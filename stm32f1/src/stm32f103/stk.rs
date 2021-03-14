#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - SysTick reload value register"]
    pub load_: LOAD_,
    #[doc = "0x08 - SysTick current value register"]
    pub val: VAL,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "SysTick control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SysTick control and status register"]
pub mod ctrl;
#[doc = "SysTick reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load_](load_) module"]
pub type LOAD_ = crate::Reg<u32, _LOAD_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD_;
#[doc = "`read()` method returns [load_::R](load_::R) reader structure"]
impl crate::Readable for LOAD_ {}
#[doc = "`write(|w| ..)` method takes [load_::W](load_::W) writer structure"]
impl crate::Writable for LOAD_ {}
#[doc = "SysTick reload value register"]
pub mod load_;
#[doc = "SysTick current value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [val](val) module"]
pub type VAL = crate::Reg<u32, _VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VAL;
#[doc = "`read()` method returns [val::R](val::R) reader structure"]
impl crate::Readable for VAL {}
#[doc = "`write(|w| ..)` method takes [val::W](val::W) writer structure"]
impl crate::Writable for VAL {}
#[doc = "SysTick current value register"]
pub mod val;
#[doc = "SysTick calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](calib) module"]
pub type CALIB = crate::Reg<u32, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
#[doc = "`read()` method returns [calib::R](calib::R) reader structure"]
impl crate::Readable for CALIB {}
#[doc = "`write(|w| ..)` method takes [calib::W](calib::W) writer structure"]
impl crate::Writable for CALIB {}
#[doc = "SysTick calibration value register"]
pub mod calib;
