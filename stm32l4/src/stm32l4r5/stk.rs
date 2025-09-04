#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: CTRL,
    load: LOAD,
    val: VAL,
    calib: CALIB,
}
impl RegisterBlock {
    ///0x00 - SysTick control and status register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x04 - SysTick reload value register
    #[inline(always)]
    pub const fn load(&self) -> &LOAD {
        &self.load
    }
    ///0x08 - SysTick current value register
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    ///0x0c - SysTick calibration value register
    #[inline(always)]
    pub const fn calib(&self) -> &CALIB {
        &self.calib
    }
}
/**CTRL (rw) register accessor: SysTick control and status register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#STK:CTRL)

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
///SysTick control and status register
pub mod ctrl;
/**LOAD (rw) register accessor: SysTick reload value register

You can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#STK:LOAD)

For information about available fields see [`mod@load`] module*/
pub type LOAD = crate::Reg<load::LOADrs>;
///SysTick reload value register
pub mod load;
/**VAL (rw) register accessor: SysTick current value register

You can [`read`](crate::Reg::read) this register and get [`val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#STK:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///SysTick current value register
pub mod val;
/**CALIB (rw) register accessor: SysTick calibration value register

You can [`read`](crate::Reg::read) this register and get [`calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#STK:CALIB)

For information about available fields see [`mod@calib`] module*/
pub type CALIB = crate::Reg<calib::CALIBrs>;
///SysTick calibration value register
pub mod calib;
