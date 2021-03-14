#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RAMECC interrupt enable register"]
    pub ier: IER,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - RAMECC monitor 1 configuration register"]
    pub m1cr: M1CR,
    #[doc = "0x24 - RAMECC monitor 1 status register"]
    pub m1sr: M1SR,
    #[doc = "0x28 - RAMECC monitor 1 failing address register"]
    pub m1far: M1FAR,
    #[doc = "0x2c - RAMECC monitor 1 failing data low register"]
    pub m1fdrl: M1FDRL,
    #[doc = "0x30 - RAMECC monitor 1 failing data high register"]
    pub m1fdrh: M1FDRH,
    #[doc = "0x34 - RAMECC monitor 1 failing error code register"]
    pub m1fecr: M1FECR,
    _reserved7: [u8; 8usize],
    #[doc = "0x40 - RAMECC monitor 2 configuration register"]
    pub m2cr: M2CR,
    #[doc = "0x44 - RAMECC monitor 2 status register"]
    pub m2sr: M2SR,
    #[doc = "0x48 - RAMECC monitor 2 failing address register"]
    pub m2far: M2FAR,
    #[doc = "0x4c - RAMECC monitor 2 failing data low register"]
    pub m2fdrl: M2FDRL,
    #[doc = "0x50 - RAMECC monitor 2 failing data high register"]
    pub m2fdrh: M2FDRH,
    #[doc = "0x54 - RAMECC monitor 2 failing error code register"]
    pub m2fecr: M2FECR,
}
#[doc = "RAMECC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "RAMECC interrupt enable register"]
pub mod ier;
#[doc = "RAMECC monitor 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1cr](m1cr) module"]
pub type M1CR = crate::Reg<u32, _M1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1CR;
#[doc = "`read()` method returns [m1cr::R](m1cr::R) reader structure"]
impl crate::Readable for M1CR {}
#[doc = "`write(|w| ..)` method takes [m1cr::W](m1cr::W) writer structure"]
impl crate::Writable for M1CR {}
#[doc = "RAMECC monitor 1 configuration register"]
pub mod m1cr;
#[doc = "RAMECC monitor 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1sr](m1sr) module"]
pub type M1SR = crate::Reg<u32, _M1SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1SR;
#[doc = "`read()` method returns [m1sr::R](m1sr::R) reader structure"]
impl crate::Readable for M1SR {}
#[doc = "`write(|w| ..)` method takes [m1sr::W](m1sr::W) writer structure"]
impl crate::Writable for M1SR {}
#[doc = "RAMECC monitor 1 status register"]
pub mod m1sr;
#[doc = "RAMECC monitor 1 failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1far](m1far) module"]
pub type M1FAR = crate::Reg<u32, _M1FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1FAR;
#[doc = "`read()` method returns [m1far::R](m1far::R) reader structure"]
impl crate::Readable for M1FAR {}
#[doc = "`write(|w| ..)` method takes [m1far::W](m1far::W) writer structure"]
impl crate::Writable for M1FAR {}
#[doc = "RAMECC monitor 1 failing address register"]
pub mod m1far;
#[doc = "RAMECC monitor 1 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1fdrl](m1fdrl) module"]
pub type M1FDRL = crate::Reg<u32, _M1FDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1FDRL;
#[doc = "`read()` method returns [m1fdrl::R](m1fdrl::R) reader structure"]
impl crate::Readable for M1FDRL {}
#[doc = "`write(|w| ..)` method takes [m1fdrl::W](m1fdrl::W) writer structure"]
impl crate::Writable for M1FDRL {}
#[doc = "RAMECC monitor 1 failing data low register"]
pub mod m1fdrl;
#[doc = "RAMECC monitor 1 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1fdrh](m1fdrh) module"]
pub type M1FDRH = crate::Reg<u32, _M1FDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1FDRH;
#[doc = "`read()` method returns [m1fdrh::R](m1fdrh::R) reader structure"]
impl crate::Readable for M1FDRH {}
#[doc = "`write(|w| ..)` method takes [m1fdrh::W](m1fdrh::W) writer structure"]
impl crate::Writable for M1FDRH {}
#[doc = "RAMECC monitor 1 failing data high register"]
pub mod m1fdrh;
#[doc = "RAMECC monitor 1 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1fecr](m1fecr) module"]
pub type M1FECR = crate::Reg<u32, _M1FECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1FECR;
#[doc = "`read()` method returns [m1fecr::R](m1fecr::R) reader structure"]
impl crate::Readable for M1FECR {}
#[doc = "`write(|w| ..)` method takes [m1fecr::W](m1fecr::W) writer structure"]
impl crate::Writable for M1FECR {}
#[doc = "RAMECC monitor 1 failing error code register"]
pub mod m1fecr;
#[doc = "RAMECC monitor 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2cr](m2cr) module"]
pub type M2CR = crate::Reg<u32, _M2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2CR;
#[doc = "`read()` method returns [m2cr::R](m2cr::R) reader structure"]
impl crate::Readable for M2CR {}
#[doc = "`write(|w| ..)` method takes [m2cr::W](m2cr::W) writer structure"]
impl crate::Writable for M2CR {}
#[doc = "RAMECC monitor 2 configuration register"]
pub mod m2cr;
#[doc = "RAMECC monitor 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2sr](m2sr) module"]
pub type M2SR = crate::Reg<u32, _M2SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2SR;
#[doc = "`read()` method returns [m2sr::R](m2sr::R) reader structure"]
impl crate::Readable for M2SR {}
#[doc = "`write(|w| ..)` method takes [m2sr::W](m2sr::W) writer structure"]
impl crate::Writable for M2SR {}
#[doc = "RAMECC monitor 2 status register"]
pub mod m2sr;
#[doc = "RAMECC monitor 2 failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2far](m2far) module"]
pub type M2FAR = crate::Reg<u32, _M2FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2FAR;
#[doc = "`read()` method returns [m2far::R](m2far::R) reader structure"]
impl crate::Readable for M2FAR {}
#[doc = "`write(|w| ..)` method takes [m2far::W](m2far::W) writer structure"]
impl crate::Writable for M2FAR {}
#[doc = "RAMECC monitor 2 failing address register"]
pub mod m2far;
#[doc = "RAMECC monitor 2 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fdrl](m2fdrl) module"]
pub type M2FDRL = crate::Reg<u32, _M2FDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2FDRL;
#[doc = "`read()` method returns [m2fdrl::R](m2fdrl::R) reader structure"]
impl crate::Readable for M2FDRL {}
#[doc = "`write(|w| ..)` method takes [m2fdrl::W](m2fdrl::W) writer structure"]
impl crate::Writable for M2FDRL {}
#[doc = "RAMECC monitor 2 failing data low register"]
pub mod m2fdrl;
#[doc = "RAMECC monitor 2 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fdrh](m2fdrh) module"]
pub type M2FDRH = crate::Reg<u32, _M2FDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2FDRH;
#[doc = "`read()` method returns [m2fdrh::R](m2fdrh::R) reader structure"]
impl crate::Readable for M2FDRH {}
#[doc = "`write(|w| ..)` method takes [m2fdrh::W](m2fdrh::W) writer structure"]
impl crate::Writable for M2FDRH {}
#[doc = "RAMECC monitor 2 failing data high register"]
pub mod m2fdrh;
#[doc = "RAMECC monitor 2 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fecr](m2fecr) module"]
pub type M2FECR = crate::Reg<u32, _M2FECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2FECR;
#[doc = "`read()` method returns [m2fecr::R](m2fecr::R) reader structure"]
impl crate::Readable for M2FECR {}
#[doc = "`write(|w| ..)` method takes [m2fecr::W](m2fecr::W) writer structure"]
impl crate::Writable for M2FECR {}
#[doc = "RAMECC monitor 2 failing error code register"]
pub mod m2fecr;
