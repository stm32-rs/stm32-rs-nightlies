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
    _reserved13: [u8; 8usize],
    #[doc = "0x60 - RAMECC monitor 3 configuration register"]
    pub m3cr: M3CR,
    #[doc = "0x64 - RAMECC monitor 3 status register"]
    pub m3sr: M3SR,
    #[doc = "0x68 - RAMECC monitor 3 failing address register"]
    pub m3far: M3FAR,
    #[doc = "0x6c - RAMECC monitor 3 failing data low register"]
    pub m3fdrl: M3FDRL,
    #[doc = "0x70 - RAMECC monitor 3 failing data high register"]
    pub m3fdrh: M3FDRH,
    #[doc = "0x74 - RAMECC monitor 3 failing error code register"]
    pub m3fecr: M3FECR,
    _reserved19: [u8; 8usize],
    #[doc = "0x80 - RAMECC monitor 4 configuration register"]
    pub m4cr: M4CR,
    #[doc = "0x84 - RAMECC monitor 4 status register"]
    pub m4sr: M4SR,
    #[doc = "0x88 - RAMECC monitor 4 failing address register"]
    pub m4far: M4FAR,
    #[doc = "0x8c - RAMECC monitor 4 failing data low register"]
    pub m4fdrl: M4FDRL,
    #[doc = "0x90 - RAMECC monitor 4 failing data high register"]
    pub m4fdrh: M4FDRH,
    #[doc = "0x94 - RAMECC monitor 4 failing error code register"]
    pub m4fecr: M4FECR,
    _reserved25: [u8; 8usize],
    #[doc = "0xa0 - RAMECC monitor 5 configuration register"]
    pub m5cr: M5CR,
    #[doc = "0xa4 - RAMECC monitor 5 status register"]
    pub m5sr: M5SR,
    #[doc = "0xa8 - RAMECC monitor 5 failing address register"]
    pub m5far: M5FAR,
    #[doc = "0xac - RAMECC monitor 5 failing data low register"]
    pub m5fdrl: M5FDRL,
    #[doc = "0xb0 - RAMECC monitor 5 failing data high register"]
    pub m5fdrh: M5FDRH,
    #[doc = "0xb4 - RAMECC monitor 5 failing error code register"]
    pub m5fecr: M5FECR,
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
#[doc = "RAMECC monitor 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3cr](m3cr) module"]
pub type M3CR = crate::Reg<u32, _M3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3CR;
#[doc = "`read()` method returns [m3cr::R](m3cr::R) reader structure"]
impl crate::Readable for M3CR {}
#[doc = "`write(|w| ..)` method takes [m3cr::W](m3cr::W) writer structure"]
impl crate::Writable for M3CR {}
#[doc = "RAMECC monitor 3 configuration register"]
pub mod m3cr;
#[doc = "RAMECC monitor 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3sr](m3sr) module"]
pub type M3SR = crate::Reg<u32, _M3SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3SR;
#[doc = "`read()` method returns [m3sr::R](m3sr::R) reader structure"]
impl crate::Readable for M3SR {}
#[doc = "`write(|w| ..)` method takes [m3sr::W](m3sr::W) writer structure"]
impl crate::Writable for M3SR {}
#[doc = "RAMECC monitor 3 status register"]
pub mod m3sr;
#[doc = "RAMECC monitor 3 failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3far](m3far) module"]
pub type M3FAR = crate::Reg<u32, _M3FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3FAR;
#[doc = "`read()` method returns [m3far::R](m3far::R) reader structure"]
impl crate::Readable for M3FAR {}
#[doc = "`write(|w| ..)` method takes [m3far::W](m3far::W) writer structure"]
impl crate::Writable for M3FAR {}
#[doc = "RAMECC monitor 3 failing address register"]
pub mod m3far;
#[doc = "RAMECC monitor 3 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3fdrl](m3fdrl) module"]
pub type M3FDRL = crate::Reg<u32, _M3FDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3FDRL;
#[doc = "`read()` method returns [m3fdrl::R](m3fdrl::R) reader structure"]
impl crate::Readable for M3FDRL {}
#[doc = "`write(|w| ..)` method takes [m3fdrl::W](m3fdrl::W) writer structure"]
impl crate::Writable for M3FDRL {}
#[doc = "RAMECC monitor 3 failing data low register"]
pub mod m3fdrl;
#[doc = "RAMECC monitor 3 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3fdrh](m3fdrh) module"]
pub type M3FDRH = crate::Reg<u32, _M3FDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3FDRH;
#[doc = "`read()` method returns [m3fdrh::R](m3fdrh::R) reader structure"]
impl crate::Readable for M3FDRH {}
#[doc = "`write(|w| ..)` method takes [m3fdrh::W](m3fdrh::W) writer structure"]
impl crate::Writable for M3FDRH {}
#[doc = "RAMECC monitor 3 failing data high register"]
pub mod m3fdrh;
#[doc = "RAMECC monitor 3 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3fecr](m3fecr) module"]
pub type M3FECR = crate::Reg<u32, _M3FECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3FECR;
#[doc = "`read()` method returns [m3fecr::R](m3fecr::R) reader structure"]
impl crate::Readable for M3FECR {}
#[doc = "`write(|w| ..)` method takes [m3fecr::W](m3fecr::W) writer structure"]
impl crate::Writable for M3FECR {}
#[doc = "RAMECC monitor 3 failing error code register"]
pub mod m3fecr;
#[doc = "RAMECC monitor 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4cr](m4cr) module"]
pub type M4CR = crate::Reg<u32, _M4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4CR;
#[doc = "`read()` method returns [m4cr::R](m4cr::R) reader structure"]
impl crate::Readable for M4CR {}
#[doc = "`write(|w| ..)` method takes [m4cr::W](m4cr::W) writer structure"]
impl crate::Writable for M4CR {}
#[doc = "RAMECC monitor 4 configuration register"]
pub mod m4cr;
#[doc = "RAMECC monitor 4 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4sr](m4sr) module"]
pub type M4SR = crate::Reg<u32, _M4SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4SR;
#[doc = "`read()` method returns [m4sr::R](m4sr::R) reader structure"]
impl crate::Readable for M4SR {}
#[doc = "`write(|w| ..)` method takes [m4sr::W](m4sr::W) writer structure"]
impl crate::Writable for M4SR {}
#[doc = "RAMECC monitor 4 status register"]
pub mod m4sr;
#[doc = "RAMECC monitor 4 failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4far](m4far) module"]
pub type M4FAR = crate::Reg<u32, _M4FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4FAR;
#[doc = "`read()` method returns [m4far::R](m4far::R) reader structure"]
impl crate::Readable for M4FAR {}
#[doc = "`write(|w| ..)` method takes [m4far::W](m4far::W) writer structure"]
impl crate::Writable for M4FAR {}
#[doc = "RAMECC monitor 4 failing address register"]
pub mod m4far;
#[doc = "RAMECC monitor 4 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fdrl](m4fdrl) module"]
pub type M4FDRL = crate::Reg<u32, _M4FDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4FDRL;
#[doc = "`read()` method returns [m4fdrl::R](m4fdrl::R) reader structure"]
impl crate::Readable for M4FDRL {}
#[doc = "`write(|w| ..)` method takes [m4fdrl::W](m4fdrl::W) writer structure"]
impl crate::Writable for M4FDRL {}
#[doc = "RAMECC monitor 4 failing data low register"]
pub mod m4fdrl;
#[doc = "RAMECC monitor 4 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fdrh](m4fdrh) module"]
pub type M4FDRH = crate::Reg<u32, _M4FDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4FDRH;
#[doc = "`read()` method returns [m4fdrh::R](m4fdrh::R) reader structure"]
impl crate::Readable for M4FDRH {}
#[doc = "`write(|w| ..)` method takes [m4fdrh::W](m4fdrh::W) writer structure"]
impl crate::Writable for M4FDRH {}
#[doc = "RAMECC monitor 4 failing data high register"]
pub mod m4fdrh;
#[doc = "RAMECC monitor 4 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fecr](m4fecr) module"]
pub type M4FECR = crate::Reg<u32, _M4FECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4FECR;
#[doc = "`read()` method returns [m4fecr::R](m4fecr::R) reader structure"]
impl crate::Readable for M4FECR {}
#[doc = "`write(|w| ..)` method takes [m4fecr::W](m4fecr::W) writer structure"]
impl crate::Writable for M4FECR {}
#[doc = "RAMECC monitor 4 failing error code register"]
pub mod m4fecr;
#[doc = "RAMECC monitor 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5cr](m5cr) module"]
pub type M5CR = crate::Reg<u32, _M5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5CR;
#[doc = "`read()` method returns [m5cr::R](m5cr::R) reader structure"]
impl crate::Readable for M5CR {}
#[doc = "`write(|w| ..)` method takes [m5cr::W](m5cr::W) writer structure"]
impl crate::Writable for M5CR {}
#[doc = "RAMECC monitor 5 configuration register"]
pub mod m5cr;
#[doc = "RAMECC monitor 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5sr](m5sr) module"]
pub type M5SR = crate::Reg<u32, _M5SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5SR;
#[doc = "`read()` method returns [m5sr::R](m5sr::R) reader structure"]
impl crate::Readable for M5SR {}
#[doc = "`write(|w| ..)` method takes [m5sr::W](m5sr::W) writer structure"]
impl crate::Writable for M5SR {}
#[doc = "RAMECC monitor 5 status register"]
pub mod m5sr;
#[doc = "RAMECC monitor 5 failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5far](m5far) module"]
pub type M5FAR = crate::Reg<u32, _M5FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5FAR;
#[doc = "`read()` method returns [m5far::R](m5far::R) reader structure"]
impl crate::Readable for M5FAR {}
#[doc = "`write(|w| ..)` method takes [m5far::W](m5far::W) writer structure"]
impl crate::Writable for M5FAR {}
#[doc = "RAMECC monitor 5 failing address register"]
pub mod m5far;
#[doc = "RAMECC monitor 5 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fdrl](m5fdrl) module"]
pub type M5FDRL = crate::Reg<u32, _M5FDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5FDRL;
#[doc = "`read()` method returns [m5fdrl::R](m5fdrl::R) reader structure"]
impl crate::Readable for M5FDRL {}
#[doc = "`write(|w| ..)` method takes [m5fdrl::W](m5fdrl::W) writer structure"]
impl crate::Writable for M5FDRL {}
#[doc = "RAMECC monitor 5 failing data low register"]
pub mod m5fdrl;
#[doc = "RAMECC monitor 5 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fdrh](m5fdrh) module"]
pub type M5FDRH = crate::Reg<u32, _M5FDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5FDRH;
#[doc = "`read()` method returns [m5fdrh::R](m5fdrh::R) reader structure"]
impl crate::Readable for M5FDRH {}
#[doc = "`write(|w| ..)` method takes [m5fdrh::W](m5fdrh::W) writer structure"]
impl crate::Writable for M5FDRH {}
#[doc = "RAMECC monitor 5 failing data high register"]
pub mod m5fdrh;
#[doc = "RAMECC monitor 5 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fecr](m5fecr) module"]
pub type M5FECR = crate::Reg<u32, _M5FECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5FECR;
#[doc = "`read()` method returns [m5fecr::R](m5fecr::R) reader structure"]
impl crate::Readable for M5FECR {}
#[doc = "`write(|w| ..)` method takes [m5fecr::W](m5fecr::W) writer structure"]
impl crate::Writable for M5FECR {}
#[doc = "RAMECC monitor 5 failing error code register"]
pub mod m5fecr;
