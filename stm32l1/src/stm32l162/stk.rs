#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    load_: LOAD_,
    val: VAL,
    calib: CALIB,
}
impl RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - SysTick reload value register"]
    #[inline(always)]
    pub const fn load_(&self) -> &LOAD_ {
        &self.load_
    }
    #[doc = "0x08 - SysTick current value register"]
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    #[doc = "0x0c - SysTick calibration value register"]
    #[inline(always)]
    pub const fn calib(&self) -> &CALIB {
        &self.calib
    }
}
#[doc = "CTRL (rw) register accessor: SysTick control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
#[doc = "SysTick control and status register"]
pub mod ctrl;
#[doc = "LOAD_ (rw) register accessor: SysTick reload value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_`]
module"]
pub type LOAD_ = crate::Reg<load_::LOAD_rs>;
#[doc = "SysTick reload value register"]
pub mod load_;
#[doc = "VAL (rw) register accessor: SysTick current value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@val`]
module"]
pub type VAL = crate::Reg<val::VALrs>;
#[doc = "SysTick current value register"]
pub mod val;
#[doc = "CALIB (rw) register accessor: SysTick calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
pub type CALIB = crate::Reg<calib::CALIBrs>;
#[doc = "SysTick calibration value register"]
pub mod calib;
