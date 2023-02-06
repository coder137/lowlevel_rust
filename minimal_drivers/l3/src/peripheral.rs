/// Peripheral trait to convert Peripheral Address to Peripheral Type
/// For example:
/// struct GPIOA_Port;
/// impl PeriphRef<GPIO_TypeDef, GPIOA_BASE> for GPIOA_Port {}
///
/// Access internal port using GPIOA_Port::get()
/// -> &'static mut GPIO_TypeDef;
pub trait Peripheral<T, const B: u32> {
    fn get() -> &'static mut T {
        let safe = B as *mut T;
        let mutable_ref = unsafe { &mut *(safe) };
        mutable_ref
    }
}
