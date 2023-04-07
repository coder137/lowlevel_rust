#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused_imports)]

#[cfg(not(test))]
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn spin_delay(delay: u32) {
    use core::arch::asm;

    let mut mdelay = delay;
    while mdelay != 0 {
        unsafe {
            asm!("nop");
        }
        mdelay -= 1;
    }
}

#[cfg(not(test))]
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[no_mangle]
fn main() -> ! {
    use core::{
        fmt::Write,
        ops::Add,
        pin::pin,
        ptr,
        sync::atomic::{AtomicBool, Ordering},
        time::Duration,
    };
    use l0::*;
    use l2::{
        block_on, heapless::spsc::Queue, join_tasks, sleep_via_timer, sleep_via_wait, wait,
        AsyncMutex, AsyncTask,
    };
    use l3::*;
    use l4::*;

    // GPIOA Pin 5
    fn configure_gpio_output() -> impl GpioOut {
        let gpioa_peripheral = GPIOA_GLOBAL.take();
        // Configure GPIOA port and Pin 5 as output
        let gpio_out_at_pin5 = gpioa_peripheral.configure_for_output(5);
        gpio_out_at_pin5
    }

    // GPIOC Pin 13
    fn configure_gpio_input() -> impl GpioIn {
        let gpioc_peripheral = GPIOC_GLOBAL.take();
        // Configure GPIOC port and Pin 13 as input
        let gpio_in_at_pin13 = gpioc_peripheral.configure_for_input(13);
        gpio_in_at_pin13
    }

    // GPIOB Pin 6, 7
    fn configure_usart_rx_tx() -> impl UsartBufferedInOut {
        let gpiob_peripheral = GPIOB_GLOBAL.take();
        // Configure GPIOB port Pin 6 and Pin 7 for USART
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 6);
        gpiob_peripheral.configure_for_usart(GPIOAlternate::AF7, 7);

        let usart1_rx_tx = USART1_GLOBAL
            .take()
            .configure_buffered_rx_tx(unsafe { &mut RX_BUF }, unsafe { &mut TX_BUF });
        usart1_rx_tx
    }

    fn configure_usart_rx_tx_interrupt() {
        nvic::enable_irq(Interrupt::USART1);
    }

    // Start
    let mut rcc_register = RCC_GLOBAL.take().get_register();
    // Activate clock control for GPIOA, GPIOB and GPIOC and USART1EN
    rcc_register.set_ahb2enr(RCC_AHB2ENR::GPIOAEN | RCC_AHB2ENR::GPIOBEN | RCC_AHB2ENR::GPIOCEN);
    rcc_register.set_apb2enr(RCC_APB2ENR::USART1EN | RCC_APB2ENR::SYSCFGEN);

    // LED module
    let mut gpio_output = configure_gpio_output();
    let mut led = Led::new(&mut gpio_output);

    // Button module
    static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    let gpio_in = configure_gpio_input();
    let button = Button::new(&gpio_in, GpioValue::High);

    // USART
    let usart1_rx_tx = AsyncMutex::new(configure_usart_rx_tx());

    // NOTE, Queue implementation is very heavy
    // Uses 4 bytes per character
    static mut RX_BUF: Queue<char, 64> = Queue::new();
    static mut TX_BUF: Queue<char, 64> = Queue::new();
    static IS_NEWLINE: AtomicBool = AtomicBool::new(false);
    #[no_mangle]
    extern "C" fn USART1_Interrupt_Handler() {
        let usart1_port = USART1_PORT::port();
        let isr_data = read_register!(usart1_port.ISR);
        const RXNE: u32 = 5;
        // const TC: u32 = 6;
        const TXE: u32 = 7;
        const TXEIE: u32 = 7;
        if (isr_data >> RXNE) & 0x01 == 1 {
            // Read data
            let rdr_data = read_register!(usart1_port.RDR) as u8 as char;
            if rdr_data == '\n' || rdr_data == '\r' {
                IS_NEWLINE.store(true, Ordering::SeqCst);
            }
            unsafe { RX_BUF.enqueue(rdr_data).unwrap() };
        }

        if (isr_data >> TXE) & 0x01 == 1 {
            unsafe {
                match TX_BUF.dequeue() {
                    Some(data) => {
                        write_register!(usart1_port.TDR, data as u16);
                    }
                    None => {
                        // Reset the CR1 TXEIE register
                        write_assign_register!(usart1_port.CR1, &, !(1 << TXEIE));
                    }
                }
            };
        }
    }
    configure_usart_rx_tx_interrupt();

    // Async task here
    let async_button_press = async {
        let mut counter = 0;
        loop {
            // Wait for button to be released
            wait(|| !button.pressed()).await;
            // Wait for button to be pressed
            wait(|| button.pressed()).await;
            let mut serial = usart1_rx_tx.lock().await;
            let current_time = get_current_time();
            serial
                .write_fmt(format_args!("Button {counter} {:?}\r\n", current_time))
                .unwrap();
            counter += 1;
        }
    };

    let async_newline_recv = async {
        loop {
            wait(|| IS_NEWLINE.load(Ordering::SeqCst)).await;
            let mut serial = usart1_rx_tx.lock().await;

            serial.write_str("Printing\r\n").unwrap();
            while serial.size() != 0 {
                let c = serial.try_read_character().unwrap();
                serial.write_char(c).unwrap();
            }
            serial.write_str("\r\n").unwrap();
            IS_NEWLINE.store(false, Ordering::SeqCst);
        }
    };

    let async_print_time = async {
        loop {
            sleep_via_timer(Duration::from_millis(1000)).await;
            let current_time = get_current_time();
            let mut serial = usart1_rx_tx.lock().await;

            serial
                .write_fmt(format_args!("Time: {:?}\r\n", current_time))
                .unwrap();
        }
    };

    let async_button_press = pin!(async_button_press);
    let async_newline_recv = pin!(async_newline_recv);
    let async_print_time = pin!(async_print_time);

    block_on(async {
        join_tasks([
            AsyncTask::new(async_button_press),
            AsyncTask::new(async_newline_recv),
            AsyncTask::new(async_print_time),
        ])
        .await;
    });

    // TODO, Remove this
    // TODO, Add time based apis based on Systick
    const TIME: u32 = 100_000;
    loop {
        if BUTTON_PRESSED.load(Ordering::SeqCst) {
            led.on();
            spin_delay(TIME);
            led.off();
            BUTTON_PRESSED.store(false, Ordering::SeqCst);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_tests_work() {
        assert_eq!(1, 1);
    }
}
