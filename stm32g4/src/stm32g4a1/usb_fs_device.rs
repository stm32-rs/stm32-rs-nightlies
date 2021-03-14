#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB endpoint n register"]
    pub ep0r: EP0R,
    #[doc = "0x04 - USB endpoint n register"]
    pub ep1r: EP1R,
    #[doc = "0x08 - USB endpoint n register"]
    pub ep2r: EP2R,
    #[doc = "0x0c - USB endpoint n register"]
    pub ep3r: EP3R,
    #[doc = "0x10 - USB endpoint n register"]
    pub ep4r: EP4R,
    #[doc = "0x14 - USB endpoint n register"]
    pub ep5r: EP5R,
    #[doc = "0x18 - USB endpoint n register"]
    pub ep6r: EP6R,
    #[doc = "0x1c - USB endpoint n register"]
    pub ep7r: EP7R,
    _reserved8: [u8; 32usize],
    #[doc = "0x40 - USB control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - USB interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - USB frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - USB device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
}
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0r](ep0r) module"]
pub type EP0R = crate::Reg<u32, _EP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0R;
#[doc = "`read()` method returns [ep0r::R](ep0r::R) reader structure"]
impl crate::Readable for EP0R {}
#[doc = "`write(|w| ..)` method takes [ep0r::W](ep0r::W) writer structure"]
impl crate::Writable for EP0R {}
#[doc = "USB endpoint n register"]
pub mod ep0r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1r](ep1r) module"]
pub type EP1R = crate::Reg<u32, _EP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1R;
#[doc = "`read()` method returns [ep1r::R](ep1r::R) reader structure"]
impl crate::Readable for EP1R {}
#[doc = "`write(|w| ..)` method takes [ep1r::W](ep1r::W) writer structure"]
impl crate::Writable for EP1R {}
#[doc = "USB endpoint n register"]
pub mod ep1r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2r](ep2r) module"]
pub type EP2R = crate::Reg<u32, _EP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2R;
#[doc = "`read()` method returns [ep2r::R](ep2r::R) reader structure"]
impl crate::Readable for EP2R {}
#[doc = "`write(|w| ..)` method takes [ep2r::W](ep2r::W) writer structure"]
impl crate::Writable for EP2R {}
#[doc = "USB endpoint n register"]
pub mod ep2r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3r](ep3r) module"]
pub type EP3R = crate::Reg<u32, _EP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3R;
#[doc = "`read()` method returns [ep3r::R](ep3r::R) reader structure"]
impl crate::Readable for EP3R {}
#[doc = "`write(|w| ..)` method takes [ep3r::W](ep3r::W) writer structure"]
impl crate::Writable for EP3R {}
#[doc = "USB endpoint n register"]
pub mod ep3r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4r](ep4r) module"]
pub type EP4R = crate::Reg<u32, _EP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4R;
#[doc = "`read()` method returns [ep4r::R](ep4r::R) reader structure"]
impl crate::Readable for EP4R {}
#[doc = "`write(|w| ..)` method takes [ep4r::W](ep4r::W) writer structure"]
impl crate::Writable for EP4R {}
#[doc = "USB endpoint n register"]
pub mod ep4r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5r](ep5r) module"]
pub type EP5R = crate::Reg<u32, _EP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5R;
#[doc = "`read()` method returns [ep5r::R](ep5r::R) reader structure"]
impl crate::Readable for EP5R {}
#[doc = "`write(|w| ..)` method takes [ep5r::W](ep5r::W) writer structure"]
impl crate::Writable for EP5R {}
#[doc = "USB endpoint n register"]
pub mod ep5r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6r](ep6r) module"]
pub type EP6R = crate::Reg<u32, _EP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6R;
#[doc = "`read()` method returns [ep6r::R](ep6r::R) reader structure"]
impl crate::Readable for EP6R {}
#[doc = "`write(|w| ..)` method takes [ep6r::W](ep6r::W) writer structure"]
impl crate::Writable for EP6R {}
#[doc = "USB endpoint n register"]
pub mod ep6r;
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7r](ep7r) module"]
pub type EP7R = crate::Reg<u32, _EP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7R;
#[doc = "`read()` method returns [ep7r::R](ep7r::R) reader structure"]
impl crate::Readable for EP7R {}
#[doc = "`write(|w| ..)` method takes [ep7r::W](ep7r::W) writer structure"]
impl crate::Writable for EP7R {}
#[doc = "USB endpoint n register"]
pub mod ep7r;
#[doc = "USB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "USB control register"]
pub mod cntr;
#[doc = "USB interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](istr) module"]
pub type ISTR = crate::Reg<u32, _ISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTR;
#[doc = "`read()` method returns [istr::R](istr::R) reader structure"]
impl crate::Readable for ISTR {}
#[doc = "`write(|w| ..)` method takes [istr::W](istr::W) writer structure"]
impl crate::Writable for ISTR {}
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "USB frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnr](fnr) module"]
pub type FNR = crate::Reg<u32, _FNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNR;
#[doc = "`read()` method returns [fnr::R](fnr::R) reader structure"]
impl crate::Readable for FNR {}
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "USB device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](daddr) module"]
pub type DADDR = crate::Reg<u32, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
#[doc = "`read()` method returns [daddr::R](daddr::R) reader structure"]
impl crate::Readable for DADDR {}
#[doc = "`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure"]
impl crate::Writable for DADDR {}
#[doc = "USB device address"]
pub mod daddr;
#[doc = "Buffer table address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btable](btable) module"]
pub type BTABLE = crate::Reg<u32, _BTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTABLE;
#[doc = "`read()` method returns [btable::R](btable::R) reader structure"]
impl crate::Readable for BTABLE {}
#[doc = "`write(|w| ..)` method takes [btable::W](btable::W) writer structure"]
impl crate::Writable for BTABLE {}
#[doc = "Buffer table address"]
pub mod btable;
