#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - OTG_HS reset register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 4usize],
    _reserved_8_grxstsp: [u8; 4usize],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub grxfsiz: GRXFSIZ,
    _reserved_10_hnptxfsiz_host: [u8; 4usize],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 8usize],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub cid: CID,
    _reserved14: [u8; 20usize],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub glpmcfg: GLPMCFG,
    _reserved15: [u8; 168usize],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf2: DIEPTXF2,
    _reserved18: [u8; 16usize],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf6: DIEPTXF6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf7: DIEPTXF7,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_DEVICE) }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn grxstsr_device_mut(&self) -> &mut GRXSTSR_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_DEVICE) }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_HOST) }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn grxstsr_host_mut(&self) -> &mut GRXSTSR_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_HOST) }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_DEVICE) }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn grxstsp_device_mut(&self) -> &mut GRXSTSP_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_DEVICE) }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_HOST) }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn grxstsp_host_mut(&self) -> &mut GRXSTSP_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_HOST) }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn dieptxf0_device(&self) -> &DIEPTXF0_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const DIEPTXF0_DEVICE) }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn dieptxf0_device_mut(&self) -> &mut DIEPTXF0_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut DIEPTXF0_DEVICE) }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn hnptxfsiz_host(&self) -> &HNPTXFSIZ_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HNPTXFSIZ_HOST) }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn hnptxfsiz_host_mut(&self) -> &mut HNPTXFSIZ_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HNPTXFSIZ_HOST) }
    }
}
#[doc = "OTG_HS control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](gotgctl) module"]
pub type GOTGCTL = crate::Reg<u32, _GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGCTL;
#[doc = "`read()` method returns [gotgctl::R](gotgctl::R) reader structure"]
impl crate::Readable for GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](gotgctl::W) writer structure"]
impl crate::Writable for GOTGCTL {}
#[doc = "OTG_HS control and status register"]
pub mod gotgctl;
#[doc = "OTG_HS interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgint](gotgint) module"]
pub type GOTGINT = crate::Reg<u32, _GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGINT;
#[doc = "`read()` method returns [gotgint::R](gotgint::R) reader structure"]
impl crate::Readable for GOTGINT {}
#[doc = "`write(|w| ..)` method takes [gotgint::W](gotgint::W) writer structure"]
impl crate::Writable for GOTGINT {}
#[doc = "OTG_HS interrupt register"]
pub mod gotgint;
#[doc = "OTG_HS AHB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](gahbcfg) module"]
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
#[doc = "`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure"]
impl crate::Readable for GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure"]
impl crate::Writable for GAHBCFG {}
#[doc = "OTG_HS AHB configuration register"]
pub mod gahbcfg;
#[doc = "OTG_HS USB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](gusbcfg) module"]
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
#[doc = "`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure"]
impl crate::Readable for GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure"]
impl crate::Writable for GUSBCFG {}
#[doc = "OTG_HS USB configuration register"]
pub mod gusbcfg;
#[doc = "OTG_HS reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "OTG_HS reset register"]
pub mod grstctl;
#[doc = "OTG_HS core interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](gintsts) module"]
pub type GINTSTS = crate::Reg<u32, _GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS;
#[doc = "`read()` method returns [gintsts::R](gintsts::R) reader structure"]
impl crate::Readable for GINTSTS {}
#[doc = "`write(|w| ..)` method takes [gintsts::W](gintsts::W) writer structure"]
impl crate::Writable for GINTSTS {}
#[doc = "OTG_HS core interrupt register"]
pub mod gintsts;
#[doc = "OTG_HS interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](gintmsk) module"]
pub type GINTMSK = crate::Reg<u32, _GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK;
#[doc = "`read()` method returns [gintmsk::R](gintmsk::R) reader structure"]
impl crate::Readable for GINTMSK {}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](gintmsk::W) writer structure"]
impl crate::Writable for GINTMSK {}
#[doc = "OTG_HS interrupt mask register"]
pub mod gintmsk;
#[doc = "OTG_HS Receive status debug read register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsr_host](grxstsr_host) module"]
pub type GRXSTSR_HOST = crate::Reg<u32, _GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_HOST;
#[doc = "`read()` method returns [grxstsr_host::R](grxstsr_host::R) reader structure"]
impl crate::Readable for GRXSTSR_HOST {}
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod grxstsr_host;
#[doc = "OTG_HS status read and pop register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp_host](grxstsp_host) module"]
pub type GRXSTSP_HOST = crate::Reg<u32, _GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_HOST;
#[doc = "`read()` method returns [grxstsp_host::R](grxstsp_host::R) reader structure"]
impl crate::Readable for GRXSTSP_HOST {}
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod grxstsp_host;
#[doc = "OTG_HS Receive FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxfsiz](grxfsiz) module"]
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
#[doc = "`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure"]
impl crate::Readable for GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure"]
impl crate::Writable for GRXFSIZ {}
#[doc = "OTG_HS Receive FIFO size register"]
pub mod grxfsiz;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptxfsiz_host](hnptxfsiz_host) module"]
pub type HNPTXFSIZ_HOST = crate::Reg<u32, _HNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HNPTXFSIZ_HOST;
#[doc = "`read()` method returns [hnptxfsiz_host::R](hnptxfsiz_host::R) reader structure"]
impl crate::Readable for HNPTXFSIZ_HOST {}
#[doc = "`write(|w| ..)` method takes [hnptxfsiz_host::W](hnptxfsiz_host::W) writer structure"]
impl crate::Writable for HNPTXFSIZ_HOST {}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod hnptxfsiz_host;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf0_device](dieptxf0_device) module"]
pub type DIEPTXF0_DEVICE = crate::Reg<u32, _DIEPTXF0_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF0_DEVICE;
#[doc = "`read()` method returns [dieptxf0_device::R](dieptxf0_device::R) reader structure"]
impl crate::Readable for DIEPTXF0_DEVICE {}
#[doc = "`write(|w| ..)` method takes [dieptxf0_device::W](dieptxf0_device::W) writer structure"]
impl crate::Writable for DIEPTXF0_DEVICE {}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod dieptxf0_device;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxsts](gnptxsts) module"]
pub type GNPTXSTS = crate::Reg<u32, _GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXSTS;
#[doc = "`read()` method returns [gnptxsts::R](gnptxsts::R) reader structure"]
impl crate::Readable for GNPTXSTS {}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod gnptxsts;
#[doc = "OTG_HS general core configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gccfg](gccfg) module"]
pub type GCCFG = crate::Reg<u32, _GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCFG;
#[doc = "`read()` method returns [gccfg::R](gccfg::R) reader structure"]
impl crate::Readable for GCCFG {}
#[doc = "`write(|w| ..)` method takes [gccfg::W](gccfg::W) writer structure"]
impl crate::Writable for GCCFG {}
#[doc = "OTG_HS general core configuration register"]
pub mod gccfg;
#[doc = "OTG_HS core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid](cid) module"]
pub type CID = crate::Reg<u32, _CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID;
#[doc = "`read()` method returns [cid::R](cid::R) reader structure"]
impl crate::Readable for CID {}
#[doc = "`write(|w| ..)` method takes [cid::W](cid::W) writer structure"]
impl crate::Writable for CID {}
#[doc = "OTG_HS core ID register"]
pub mod cid;
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxfsiz](hptxfsiz) module"]
pub type HPTXFSIZ = crate::Reg<u32, _HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXFSIZ;
#[doc = "`read()` method returns [hptxfsiz::R](hptxfsiz::R) reader structure"]
impl crate::Readable for HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [hptxfsiz::W](hptxfsiz::W) writer structure"]
impl crate::Writable for HPTXFSIZ {}
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](dieptxf1) module"]
pub type DIEPTXF1 = crate::Reg<u32, _DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF1;
#[doc = "`read()` method returns [dieptxf1::R](dieptxf1::R) reader structure"]
impl crate::Readable for DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](dieptxf1::W) writer structure"]
impl crate::Writable for DIEPTXF1 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf1;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf2](dieptxf2) module"]
pub type DIEPTXF2 = crate::Reg<u32, _DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF2;
#[doc = "`read()` method returns [dieptxf2::R](dieptxf2::R) reader structure"]
impl crate::Readable for DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](dieptxf2::W) writer structure"]
impl crate::Writable for DIEPTXF2 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf2;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf3](dieptxf3) module"]
pub type DIEPTXF3 = crate::Reg<u32, _DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF3;
#[doc = "`read()` method returns [dieptxf3::R](dieptxf3::R) reader structure"]
impl crate::Readable for DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](dieptxf3::W) writer structure"]
impl crate::Writable for DIEPTXF3 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf3;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf4](dieptxf4) module"]
pub type DIEPTXF4 = crate::Reg<u32, _DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF4;
#[doc = "`read()` method returns [dieptxf4::R](dieptxf4::R) reader structure"]
impl crate::Readable for DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [dieptxf4::W](dieptxf4::W) writer structure"]
impl crate::Writable for DIEPTXF4 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf4;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf5](dieptxf5) module"]
pub type DIEPTXF5 = crate::Reg<u32, _DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF5;
#[doc = "`read()` method returns [dieptxf5::R](dieptxf5::R) reader structure"]
impl crate::Readable for DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](dieptxf5::W) writer structure"]
impl crate::Writable for DIEPTXF5 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf5;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf6](dieptxf6) module"]
pub type DIEPTXF6 = crate::Reg<u32, _DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF6;
#[doc = "`read()` method returns [dieptxf6::R](dieptxf6::R) reader structure"]
impl crate::Readable for DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [dieptxf6::W](dieptxf6::W) writer structure"]
impl crate::Writable for DIEPTXF6 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf6;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf7](dieptxf7) module"]
pub type DIEPTXF7 = crate::Reg<u32, _DIEPTXF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF7;
#[doc = "`read()` method returns [dieptxf7::R](dieptxf7::R) reader structure"]
impl crate::Readable for DIEPTXF7 {}
#[doc = "`write(|w| ..)` method takes [dieptxf7::W](dieptxf7::W) writer structure"]
impl crate::Writable for DIEPTXF7 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf7;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsr_device](grxstsr_device) module"]
pub type GRXSTSR_DEVICE = crate::Reg<u32, _GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_DEVICE;
#[doc = "`read()` method returns [grxstsr_device::R](grxstsr_device::R) reader structure"]
impl crate::Readable for GRXSTSR_DEVICE {}
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod grxstsr_device;
#[doc = "OTG_HS status read and pop register (peripheral mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp_device](grxstsp_device) module"]
pub type GRXSTSP_DEVICE = crate::Reg<u32, _GRXSTSP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_DEVICE;
#[doc = "`read()` method returns [grxstsp_device::R](grxstsp_device::R) reader structure"]
impl crate::Readable for GRXSTSP_DEVICE {}
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod grxstsp_device;
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glpmcfg](glpmcfg) module"]
pub type GLPMCFG = crate::Reg<u32, _GLPMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLPMCFG;
#[doc = "`read()` method returns [glpmcfg::R](glpmcfg::R) reader structure"]
impl crate::Readable for GLPMCFG {}
#[doc = "`write(|w| ..)` method takes [glpmcfg::W](glpmcfg::W) writer structure"]
impl crate::Writable for GLPMCFG {}
#[doc = "OTG core LPM configuration register"]
pub mod glpmcfg;
