/******************************************************************************
 * @file     gcc_arm.ld
 * @brief    GNU Linker Script for Cortex-M based device
 * @version  V2.0.0
 * @date     21. May 2019
 ******************************************************************************/
/*
 * Copyright (c) 2009-2019 Arm Limited. All rights reserved.
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Licensed under the Apache License, Version 2.0 (the License); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an AS IS BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 *-------- <<< Use Configuration Wizard in Context Menu >>> -------------------
 */

/*---------------------- Flash Configuration ----------------------------------
  <h> Flash Configuration
    <o0> Flash Base Address <0x0-0xFFFFFFFF:8>
    <o1> Flash Size (in Bytes) <0x0-0xFFFFFFFF:8>
  </h>
  -----------------------------------------------------------------------------*/
__ROM_BASE = 0x08000000;
__ROM_SIZE = 0x00100000;

/*--------------------- Embedded RAM Configuration ----------------------------
  <h> RAM Configuration
    <o0> RAM Base Address    <0x0-0xFFFFFFFF:8>
    <o1> RAM Size (in Bytes) <0x0-0xFFFFFFFF:8>
  </h>
 -----------------------------------------------------------------------------*/
__RAM_BASE = 0x20000000;
__RAM_SIZE = 0x00018000;

/*--------------------- Stack / Heap Configuration ----------------------------
  <h> Stack / Heap Configuration
    <o0> Stack Size (in Bytes) <0x0-0xFFFFFFFF:8>
    <o1> Heap Size (in Bytes) <0x0-0xFFFFFFFF:8>
  </h>
  -----------------------------------------------------------------------------*/
/**
See Diagram below

FreeRTOS Heap 4 allocated on BSS
FreeRTOS allocation on BSS: 40K
STACK SIZE to 20K
HEAP SIZE to 10K
Remaining unallocated memory = 96 - 40 - 20 - 10 = 26K
DATA and other BSS can consume 26K
**/
__STACK_SIZE = 20K;
__HEAP_SIZE  = 10K;

/*************************************************
                                                  *
     64K RAM +-----Stack Top   +-----+ 32K RAM    *
             |    ||           |     |            *
             |    \/           |     |            *
             |     |           |     |            *
             |     |           |     |            *
             |    /\ heap      |    /\            *
             |    || overflow  |    ||            *
             .-----.           +-----Heap Start   *
             |bbbbb|           0x2000000          *
             |bbbbb| *bss                         *
             .-----.                              *
             |ddddd|                              *
             |ddddd| *data                        *
             .-----.                              *
            0x1000000                             *
                                                  *
**************************************************
*/

/*
 *-------------------- <<< end of configuration section >>> -------------------
 */

MEMORY
{
  FLASH (rx)  : ORIGIN = __ROM_BASE, LENGTH = __ROM_SIZE
  RAM   (rwx) : ORIGIN = __RAM_BASE, LENGTH = __RAM_SIZE
}

/* Linker script to place sections and symbol values. Should be used together
 * with other linker script that defines memory regions FLASH and RAM.
 * It references following symbols, which must be defined in code:
 *   Reset_Handler : Entry of reset handler
 *
 * It defines following symbols, which code can use without definition:
 *   __exidx_start
 *   __exidx_end
 *   __copy_table_start__
 *   __copy_table_end__
 *   __zero_table_start__
 *   __zero_table_end__
 *   __etext
 *   __data_start__
 *   __preinit_array_start
 *   __preinit_array_end
 *   __init_array_start
 *   __init_array_end
 *   __fini_array_start
 *   __fini_array_end
 *   __data_end__
 *   __bss_start__
 *   __bss_end__
 *   __end__
 *   end
 *   __HeapLimit
 *   __StackLimit
 *   __StackTop
 *   __stack
 */
ENTRY(Reset_Handler)

SECTIONS
{
  .text :
  {
    KEEP(*(.vector_table.exceptions))
    KEEP(*(.vector_table.interrupts))
    *(.text*)

    KEEP(*(.init))
    KEEP(*(.fini))

    /* .ctors */
    *crtbegin.o(.ctors)
    *crtbegin?.o(.ctors)
    *(EXCLUDE_FILE(*crtend?.o *crtend.o) .ctors)
    *(SORT(.ctors.*))
    *(.ctors)

    /* .dtors */
    *crtbegin.o(.dtors)
    *crtbegin?.o(.dtors)
    *(EXCLUDE_FILE(*crtend?.o *crtend.o) .dtors)
    *(SORT(.dtors.*))
    *(.dtors)

    *(.rodata*)

    KEEP(*(.eh_frame*))
  } > FLASH

  /*
   * SG veneers:
   * All SG veneers are placed in the special output section .gnu.sgstubs. Its start address
   * must be set, either with the command line option �--section-start� or in a linker script,
   * to indicate where to place these veneers in memory.
   */
/*
  .gnu.sgstubs :
  {
    . = ALIGN(32);
  } > FLASH
*/
  .ARM.extab :
  {
    *(.ARM.extab* .gnu.linkonce.armextab.*)
  } > FLASH

  __exidx_start = .;
  .ARM.exidx :
  {
    *(.ARM.exidx* .gnu.linkonce.armexidx.*)
  } > FLASH
  __exidx_end = .;

  .copy.table :
  {
    . = ALIGN(4);
    __copy_table_start__ = .;
    LONG (__etext)
    LONG (__data_start__)
    LONG ((__data_end__ - __data_start__) / 4)
    /* Add each additional data section here */
/*
    LONG (__etext2)
    LONG (__data2_start__)
    LONG (__data2_end__ - __data2_start__)
*/
    __copy_table_end__ = .;
  } > FLASH

  .zero.table :
  {
    . = ALIGN(4);
    __zero_table_start__ = .;
    LONG (__bss_start__)
    LONG ((__bss_end__ - __bss_start__) / 4)
    /* Add each additional bss section here */
/*
    LONG (__bss2_start__)
    LONG (__bss2_end__ - __bss2_start__)
*/
    __zero_table_end__ = .;
  } > FLASH

  /**
   * Location counter can end up 2byte aligned with narrow Thumb code but
   * __etext is assumed by startup code to be the LMA of a section in RAM
   * which must be 4byte aligned 
   */
  __etext = ALIGN (4);

  .data : AT (__etext)
  {
    __data_start__ = .;
    *(vtable)
    *(.data)
    *(.data.*)

    . = ALIGN(4);
    /* preinit data */
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP(*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);

    . = ALIGN(4);
    /* init data */
    PROVIDE_HIDDEN (__init_array_start = .);
    KEEP(*(SORT(.init_array.*)))
    KEEP(*(.init_array))
    PROVIDE_HIDDEN (__init_array_end = .);


    . = ALIGN(4);
    /* finit data */
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP(*(SORT(.fini_array.*)))
    KEEP(*(.fini_array))
    PROVIDE_HIDDEN (__fini_array_end = .);

    KEEP(*(.jcr*))
    . = ALIGN(4);
    /* All data end */
    __data_end__ = .;

  } > RAM

  /*
   * Secondary data section, optional
   *
   * Remember to add each additional data section
   * to the .copy.table above to asure proper
   * initialization during startup.
   */
/*
  __etext2 = ALIGN (4);

  .data2 : AT (__etext2)
  {
    . = ALIGN(4);
    __data2_start__ = .;
    *(.data2)
    *(.data2.*)
    . = ALIGN(4);
    __data2_end__ = .;

  } > RAM2
*/

  .bss :
  {
    . = ALIGN(4);
    __bss_start__ = .;
    *(.bss)
    *(.bss.*)
    *(COMMON)
    . = ALIGN(4);
    __bss_end__ = .;
  } > RAM AT > RAM

  /*
   * Secondary bss section, optional
   *
   * Remember to add each additional bss section
   * to the .zero.table above to asure proper
   * initialization during startup.
   */
/*
  .bss2 :
  {
    . = ALIGN(4);
    __bss2_start__ = .;
    *(.bss2)
    *(.bss2.*)
    . = ALIGN(4);
    __bss2_end__ = .;
  } > RAM2 AT > RAM2
*/

  .heap (COPY) :
  {
    . = ALIGN(8);
    __end__ = .;
    PROVIDE(end = .);
    . = . + __HEAP_SIZE;
    . = ALIGN(8);
    __HeapLimit = .;
  } > RAM

  .stack (ORIGIN(RAM) + LENGTH(RAM) - __STACK_SIZE) (COPY) :
  {
    . = ALIGN(8);
    __StackLimit = .;
    . = . + __STACK_SIZE;
    . = ALIGN(8);
    __StackTop = .;
  } > RAM
  PROVIDE(__stack = __StackTop);

  /* Check if data + heap + stack exceeds RAM limit */
  ASSERT(__StackLimit >= __HeapLimit, "region RAM overflowed with stack")
}

/* Exceptions */
PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);

/* Interrupts */
PROVIDE(WWDG_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(PVD_PVM_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(RTC_TAMP_STAMP_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(RTC_WKUP_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(FLASH_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(RCC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI0_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI4_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH4_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH5_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH6_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA1_CH7_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(ADC1_2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(CAN1_TX_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(CAN1_RX0_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(CAN1_RX1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(CAN1_SCE_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI9_5_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM1_BRK_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM1_UP_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM1_TRG_COM_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM1_CC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM4_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C1_EV_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C1_ER_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C2_EV_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C2_ER_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SPI1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SPI2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(USART1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(USART2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(USART3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(EXTI15_10_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(RTC_ALARM_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DFSDM1_FLT3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM8_BRK_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM8_UP_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM8_TRG_COM_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM8_CC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(ADC3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(FMC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SDMMC1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM5_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SPI3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(UART4_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(UART5_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM6_DAC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TIM7_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH3_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH4_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH5_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DFSDM1_FLT0_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DFSDM1_FLT1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DFSDM1_FLT2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(COMP_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(LPTIM1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(LPTIM2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(OTG_FS_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH6_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(DMA2_CH7_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(LPUART1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(QUADSPI_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C3_EV_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(I2C3_ER_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SAI1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SAI2_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(SWPMI1_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(TSC_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(LCD_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(AES_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(RNG_Interrupt_Handler = DefaultExceptionHandler);
PROVIDE(FPU_Interrupt_Handler = DefaultExceptionHandler);
