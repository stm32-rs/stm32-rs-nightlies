#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Timer Control Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Master Timer Interrupt Status Register"]
    pub misr: MISR,
    #[doc = "0x08 - Master Timer Interrupt Clear Register"]
    pub micr: MICR,
    #[doc = "0x0c - HRTIM Master Timer DMA / Interrupt Enable Register"]
    pub mdier: MDIER,
    #[doc = "0x10 - Master Timer Counter Register"]
    pub mcntr: MCNTR,
    #[doc = "0x14 - Master Timer Period Register"]
    pub mper: MPER,
    #[doc = "0x18 - Master Timer Repetition Register"]
    pub mrep: MREP,
    #[doc = "0x1c - Master Timer Compare 1 Register"]
    pub mcmp1r: MCMP1R,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - Master Timer Compare 2 Register"]
    pub mcmp2r: MCMP2R,
    #[doc = "0x28 - Master Timer Compare 3 Register"]
    pub mcmp3r: MCMP3R,
    #[doc = "0x2c - Master Timer Compare 4 Register"]
    pub mcmp4r: MCMP4R,
}
#[doc = "Master Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Master Timer Control Register"]
pub mod mcr;
#[doc = "Master Timer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "Master Timer Interrupt Status Register"]
pub mod misr;
#[doc = "Master Timer Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](micr) module"]
pub type MICR = crate::Reg<u32, _MICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICR;
#[doc = "`write(|w| ..)` method takes [micr::W](micr::W) writer structure"]
impl crate::Writable for MICR {}
#[doc = "Master Timer Interrupt Clear Register"]
pub mod micr;
#[doc = "HRTIM Master Timer DMA / Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdier](mdier) module"]
pub type MDIER = crate::Reg<u32, _MDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDIER;
#[doc = "`read()` method returns [mdier::R](mdier::R) reader structure"]
impl crate::Readable for MDIER {}
#[doc = "`write(|w| ..)` method takes [mdier::W](mdier::W) writer structure"]
impl crate::Writable for MDIER {}
#[doc = "HRTIM Master Timer DMA / Interrupt Enable Register"]
pub mod mdier;
#[doc = "Master Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcntr](mcntr) module"]
pub type MCNTR = crate::Reg<u32, _MCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCNTR;
#[doc = "`read()` method returns [mcntr::R](mcntr::R) reader structure"]
impl crate::Readable for MCNTR {}
#[doc = "`write(|w| ..)` method takes [mcntr::W](mcntr::W) writer structure"]
impl crate::Writable for MCNTR {}
#[doc = "Master Timer Counter Register"]
pub mod mcntr;
#[doc = "Master Timer Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mper](mper) module"]
pub type MPER = crate::Reg<u32, _MPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPER;
#[doc = "`read()` method returns [mper::R](mper::R) reader structure"]
impl crate::Readable for MPER {}
#[doc = "`write(|w| ..)` method takes [mper::W](mper::W) writer structure"]
impl crate::Writable for MPER {}
#[doc = "Master Timer Period Register"]
pub mod mper;
#[doc = "Master Timer Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrep](mrep) module"]
pub type MREP = crate::Reg<u32, _MREP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MREP;
#[doc = "`read()` method returns [mrep::R](mrep::R) reader structure"]
impl crate::Readable for MREP {}
#[doc = "`write(|w| ..)` method takes [mrep::W](mrep::W) writer structure"]
impl crate::Writable for MREP {}
#[doc = "Master Timer Repetition Register"]
pub mod mrep;
#[doc = "Master Timer Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp1r](mcmp1r) module"]
pub type MCMP1R = crate::Reg<u32, _MCMP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMP1R;
#[doc = "`read()` method returns [mcmp1r::R](mcmp1r::R) reader structure"]
impl crate::Readable for MCMP1R {}
#[doc = "`write(|w| ..)` method takes [mcmp1r::W](mcmp1r::W) writer structure"]
impl crate::Writable for MCMP1R {}
#[doc = "Master Timer Compare 1 Register"]
pub mod mcmp1r;
#[doc = "Master Timer Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp2r](mcmp2r) module"]
pub type MCMP2R = crate::Reg<u32, _MCMP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMP2R;
#[doc = "`read()` method returns [mcmp2r::R](mcmp2r::R) reader structure"]
impl crate::Readable for MCMP2R {}
#[doc = "`write(|w| ..)` method takes [mcmp2r::W](mcmp2r::W) writer structure"]
impl crate::Writable for MCMP2R {}
#[doc = "Master Timer Compare 2 Register"]
pub mod mcmp2r;
#[doc = "Master Timer Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp3r](mcmp3r) module"]
pub type MCMP3R = crate::Reg<u32, _MCMP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMP3R;
#[doc = "`read()` method returns [mcmp3r::R](mcmp3r::R) reader structure"]
impl crate::Readable for MCMP3R {}
#[doc = "`write(|w| ..)` method takes [mcmp3r::W](mcmp3r::W) writer structure"]
impl crate::Writable for MCMP3R {}
#[doc = "Master Timer Compare 3 Register"]
pub mod mcmp3r;
#[doc = "Master Timer Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp4r](mcmp4r) module"]
pub type MCMP4R = crate::Reg<u32, _MCMP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMP4R;
#[doc = "`read()` method returns [mcmp4r::R](mcmp4r::R) reader structure"]
impl crate::Readable for MCMP4R {}
#[doc = "`write(|w| ..)` method takes [mcmp4r::W](mcmp4r::W) writer structure"]
impl crate::Writable for MCMP4R {}
#[doc = "Master Timer Compare 4 Register"]
pub mod mcmp4r;
