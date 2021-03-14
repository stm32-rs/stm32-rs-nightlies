#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub otg_hs_gotgctl: OTG_HS_GOTGCTL,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub otg_hs_gotgint: OTG_HS_GOTGINT,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub otg_hs_gahbcfg: OTG_HS_GAHBCFG,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub otg_hs_gusbcfg: OTG_HS_GUSBCFG,
    #[doc = "0x10 - OTG_HS reset register"]
    pub otg_hs_grstctl: OTG_HS_GRSTCTL,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub otg_hs_gintsts: OTG_HS_GINTSTS,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub otg_hs_gintmsk: OTG_HS_GINTMSK,
    _reserved_7_otg_hs_grxstsr: [u8; 4usize],
    _reserved_8_otg_hs_grxstsp: [u8; 4usize],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub otg_hs_grxfsiz: OTG_HS_GRXFSIZ,
    _reserved_10_otg_hs: [u8; 4usize],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub otg_hs_gnptxsts: OTG_HS_GNPTXSTS,
    _reserved12: [u8; 8usize],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub otg_hs_gccfg: OTG_HS_GCCFG,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub otg_hs_cid: OTG_HS_CID,
    _reserved14: [u8; 192usize],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub otg_hs_hptxfsiz: OTG_HS_HPTXFSIZ,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf1: OTG_HS_DIEPTXF1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf2: OTG_HS_DIEPTXF2,
    _reserved17: [u8; 16usize],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf3: OTG_HS_DIEPTXF3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf4: OTG_HS_DIEPTXF4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf5: OTG_HS_DIEPTXF5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf6: OTG_HS_DIEPTXF6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf7: OTG_HS_DIEPTXF7,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_peripheral(&self) -> &OTG_HS_GRXSTSR_PERIPHERAL {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const OTG_HS_GRXSTSR_PERIPHERAL)
        }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_peripheral_mut(&self) -> &mut OTG_HS_GRXSTSR_PERIPHERAL {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize)
                as *mut OTG_HS_GRXSTSR_PERIPHERAL)
        }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_host(&self) -> &OTG_HS_GRXSTSR_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_HS_GRXSTSR_HOST)
        }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_host_mut(&self) -> &mut OTG_HS_GRXSTSR_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_HS_GRXSTSR_HOST)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_peripheral(&self) -> &OTG_HS_GRXSTSP_PERIPHERAL {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const OTG_HS_GRXSTSP_PERIPHERAL)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_peripheral_mut(&self) -> &mut OTG_HS_GRXSTSP_PERIPHERAL {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize)
                as *mut OTG_HS_GRXSTSP_PERIPHERAL)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_host(&self) -> &OTG_HS_GRXSTSP_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_HS_GRXSTSP_HOST)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_host_mut(&self) -> &mut OTG_HS_GRXSTSP_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_HS_GRXSTSP_HOST)
        }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_tx0fsiz_peripheral(&self) -> &OTG_HS_TX0FSIZ_PERIPHERAL {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const OTG_HS_TX0FSIZ_PERIPHERAL)
        }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_tx0fsiz_peripheral_mut(&self) -> &mut OTG_HS_TX0FSIZ_PERIPHERAL {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize)
                as *mut OTG_HS_TX0FSIZ_PERIPHERAL)
        }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_gnptxfsiz_host(&self) -> &OTG_HS_GNPTXFSIZ_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_HS_GNPTXFSIZ_HOST)
        }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_gnptxfsiz_host_mut(&self) -> &mut OTG_HS_GNPTXFSIZ_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_HS_GNPTXFSIZ_HOST)
        }
    }
}
#[doc = "OTG_HS control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gotgctl](otg_hs_gotgctl) module"]
pub type OTG_HS_GOTGCTL = crate::Reg<u32, _OTG_HS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GOTGCTL;
#[doc = "`read()` method returns [otg_hs_gotgctl::R](otg_hs_gotgctl::R) reader structure"]
impl crate::Readable for OTG_HS_GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gotgctl::W](otg_hs_gotgctl::W) writer structure"]
impl crate::Writable for OTG_HS_GOTGCTL {}
#[doc = "OTG_HS control and status register"]
pub mod otg_hs_gotgctl;
#[doc = "OTG_HS interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gotgint](otg_hs_gotgint) module"]
pub type OTG_HS_GOTGINT = crate::Reg<u32, _OTG_HS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GOTGINT;
#[doc = "`read()` method returns [otg_hs_gotgint::R](otg_hs_gotgint::R) reader structure"]
impl crate::Readable for OTG_HS_GOTGINT {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gotgint::W](otg_hs_gotgint::W) writer structure"]
impl crate::Writable for OTG_HS_GOTGINT {}
#[doc = "OTG_HS interrupt register"]
pub mod otg_hs_gotgint;
#[doc = "OTG_HS AHB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gahbcfg](otg_hs_gahbcfg) module"]
pub type OTG_HS_GAHBCFG = crate::Reg<u32, _OTG_HS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GAHBCFG;
#[doc = "`read()` method returns [otg_hs_gahbcfg::R](otg_hs_gahbcfg::R) reader structure"]
impl crate::Readable for OTG_HS_GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gahbcfg::W](otg_hs_gahbcfg::W) writer structure"]
impl crate::Writable for OTG_HS_GAHBCFG {}
#[doc = "OTG_HS AHB configuration register"]
pub mod otg_hs_gahbcfg;
#[doc = "OTG_HS USB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gusbcfg](otg_hs_gusbcfg) module"]
pub type OTG_HS_GUSBCFG = crate::Reg<u32, _OTG_HS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GUSBCFG;
#[doc = "`read()` method returns [otg_hs_gusbcfg::R](otg_hs_gusbcfg::R) reader structure"]
impl crate::Readable for OTG_HS_GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gusbcfg::W](otg_hs_gusbcfg::W) writer structure"]
impl crate::Writable for OTG_HS_GUSBCFG {}
#[doc = "OTG_HS USB configuration register"]
pub mod otg_hs_gusbcfg;
#[doc = "OTG_HS reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grstctl](otg_hs_grstctl) module"]
pub type OTG_HS_GRSTCTL = crate::Reg<u32, _OTG_HS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRSTCTL;
#[doc = "`read()` method returns [otg_hs_grstctl::R](otg_hs_grstctl::R) reader structure"]
impl crate::Readable for OTG_HS_GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [otg_hs_grstctl::W](otg_hs_grstctl::W) writer structure"]
impl crate::Writable for OTG_HS_GRSTCTL {}
#[doc = "OTG_HS reset register"]
pub mod otg_hs_grstctl;
#[doc = "OTG_HS core interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gintsts](otg_hs_gintsts) module"]
pub type OTG_HS_GINTSTS = crate::Reg<u32, _OTG_HS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GINTSTS;
#[doc = "`read()` method returns [otg_hs_gintsts::R](otg_hs_gintsts::R) reader structure"]
impl crate::Readable for OTG_HS_GINTSTS {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gintsts::W](otg_hs_gintsts::W) writer structure"]
impl crate::Writable for OTG_HS_GINTSTS {}
#[doc = "OTG_HS core interrupt register"]
pub mod otg_hs_gintsts;
#[doc = "OTG_HS interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gintmsk](otg_hs_gintmsk) module"]
pub type OTG_HS_GINTMSK = crate::Reg<u32, _OTG_HS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GINTMSK;
#[doc = "`read()` method returns [otg_hs_gintmsk::R](otg_hs_gintmsk::R) reader structure"]
impl crate::Readable for OTG_HS_GINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gintmsk::W](otg_hs_gintmsk::W) writer structure"]
impl crate::Writable for OTG_HS_GINTMSK {}
#[doc = "OTG_HS interrupt mask register"]
pub mod otg_hs_gintmsk;
#[doc = "OTG_HS Receive status debug read register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grxstsr_host](otg_hs_grxstsr_host) module"]
pub type OTG_HS_GRXSTSR_HOST = crate::Reg<u32, _OTG_HS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSR_HOST;
#[doc = "`read()` method returns [otg_hs_grxstsr_host::R](otg_hs_grxstsr_host::R) reader structure"]
impl crate::Readable for OTG_HS_GRXSTSR_HOST {}
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod otg_hs_grxstsr_host;
#[doc = "OTG_HS status read and pop register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grxstsp_host](otg_hs_grxstsp_host) module"]
pub type OTG_HS_GRXSTSP_HOST = crate::Reg<u32, _OTG_HS_GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSP_HOST;
#[doc = "`read()` method returns [otg_hs_grxstsp_host::R](otg_hs_grxstsp_host::R) reader structure"]
impl crate::Readable for OTG_HS_GRXSTSP_HOST {}
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod otg_hs_grxstsp_host;
#[doc = "OTG_HS Receive FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grxfsiz](otg_hs_grxfsiz) module"]
pub type OTG_HS_GRXFSIZ = crate::Reg<u32, _OTG_HS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXFSIZ;
#[doc = "`read()` method returns [otg_hs_grxfsiz::R](otg_hs_grxfsiz::R) reader structure"]
impl crate::Readable for OTG_HS_GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_hs_grxfsiz::W](otg_hs_grxfsiz::W) writer structure"]
impl crate::Writable for OTG_HS_GRXFSIZ {}
#[doc = "OTG_HS Receive FIFO size register"]
pub mod otg_hs_grxfsiz;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gnptxfsiz_host](otg_hs_gnptxfsiz_host) module"]
pub type OTG_HS_GNPTXFSIZ_HOST = crate::Reg<u32, _OTG_HS_GNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GNPTXFSIZ_HOST;
#[doc = "`read()` method returns [otg_hs_gnptxfsiz_host::R](otg_hs_gnptxfsiz_host::R) reader structure"]
impl crate::Readable for OTG_HS_GNPTXFSIZ_HOST {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gnptxfsiz_host::W](otg_hs_gnptxfsiz_host::W) writer structure"]
impl crate::Writable for OTG_HS_GNPTXFSIZ_HOST {}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod otg_hs_gnptxfsiz_host;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_tx0fsiz_peripheral](otg_hs_tx0fsiz_peripheral) module"]
pub type OTG_HS_TX0FSIZ_PERIPHERAL = crate::Reg<u32, _OTG_HS_TX0FSIZ_PERIPHERAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_TX0FSIZ_PERIPHERAL;
#[doc = "`read()` method returns [otg_hs_tx0fsiz_peripheral::R](otg_hs_tx0fsiz_peripheral::R) reader structure"]
impl crate::Readable for OTG_HS_TX0FSIZ_PERIPHERAL {}
#[doc = "`write(|w| ..)` method takes [otg_hs_tx0fsiz_peripheral::W](otg_hs_tx0fsiz_peripheral::W) writer structure"]
impl crate::Writable for OTG_HS_TX0FSIZ_PERIPHERAL {}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod otg_hs_tx0fsiz_peripheral;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gnptxsts](otg_hs_gnptxsts) module"]
pub type OTG_HS_GNPTXSTS = crate::Reg<u32, _OTG_HS_GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GNPTXSTS;
#[doc = "`read()` method returns [otg_hs_gnptxsts::R](otg_hs_gnptxsts::R) reader structure"]
impl crate::Readable for OTG_HS_GNPTXSTS {}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod otg_hs_gnptxsts;
#[doc = "OTG_HS general core configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_gccfg](otg_hs_gccfg) module"]
pub type OTG_HS_GCCFG = crate::Reg<u32, _OTG_HS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GCCFG;
#[doc = "`read()` method returns [otg_hs_gccfg::R](otg_hs_gccfg::R) reader structure"]
impl crate::Readable for OTG_HS_GCCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hs_gccfg::W](otg_hs_gccfg::W) writer structure"]
impl crate::Writable for OTG_HS_GCCFG {}
#[doc = "OTG_HS general core configuration register"]
pub mod otg_hs_gccfg;
#[doc = "OTG_HS core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_cid](otg_hs_cid) module"]
pub type OTG_HS_CID = crate::Reg<u32, _OTG_HS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_CID;
#[doc = "`read()` method returns [otg_hs_cid::R](otg_hs_cid::R) reader structure"]
impl crate::Readable for OTG_HS_CID {}
#[doc = "`write(|w| ..)` method takes [otg_hs_cid::W](otg_hs_cid::W) writer structure"]
impl crate::Writable for OTG_HS_CID {}
#[doc = "OTG_HS core ID register"]
pub mod otg_hs_cid;
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hptxfsiz](otg_hs_hptxfsiz) module"]
pub type OTG_HS_HPTXFSIZ = crate::Reg<u32, _OTG_HS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPTXFSIZ;
#[doc = "`read()` method returns [otg_hs_hptxfsiz::R](otg_hs_hptxfsiz::R) reader structure"]
impl crate::Readable for OTG_HS_HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hptxfsiz::W](otg_hs_hptxfsiz::W) writer structure"]
impl crate::Writable for OTG_HS_HPTXFSIZ {}
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod otg_hs_hptxfsiz;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf1](otg_hs_dieptxf1) module"]
pub type OTG_HS_DIEPTXF1 = crate::Reg<u32, _OTG_HS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF1;
#[doc = "`read()` method returns [otg_hs_dieptxf1::R](otg_hs_dieptxf1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf1::W](otg_hs_dieptxf1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF1 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf1;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf2](otg_hs_dieptxf2) module"]
pub type OTG_HS_DIEPTXF2 = crate::Reg<u32, _OTG_HS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF2;
#[doc = "`read()` method returns [otg_hs_dieptxf2::R](otg_hs_dieptxf2::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf2::W](otg_hs_dieptxf2::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF2 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf2;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf3](otg_hs_dieptxf3) module"]
pub type OTG_HS_DIEPTXF3 = crate::Reg<u32, _OTG_HS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF3;
#[doc = "`read()` method returns [otg_hs_dieptxf3::R](otg_hs_dieptxf3::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf3::W](otg_hs_dieptxf3::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF3 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf3;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf4](otg_hs_dieptxf4) module"]
pub type OTG_HS_DIEPTXF4 = crate::Reg<u32, _OTG_HS_DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF4;
#[doc = "`read()` method returns [otg_hs_dieptxf4::R](otg_hs_dieptxf4::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf4::W](otg_hs_dieptxf4::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF4 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf4;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf5](otg_hs_dieptxf5) module"]
pub type OTG_HS_DIEPTXF5 = crate::Reg<u32, _OTG_HS_DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF5;
#[doc = "`read()` method returns [otg_hs_dieptxf5::R](otg_hs_dieptxf5::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf5::W](otg_hs_dieptxf5::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF5 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf5;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf6](otg_hs_dieptxf6) module"]
pub type OTG_HS_DIEPTXF6 = crate::Reg<u32, _OTG_HS_DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF6;
#[doc = "`read()` method returns [otg_hs_dieptxf6::R](otg_hs_dieptxf6::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf6::W](otg_hs_dieptxf6::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF6 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf6;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dieptxf7](otg_hs_dieptxf7) module"]
pub type OTG_HS_DIEPTXF7 = crate::Reg<u32, _OTG_HS_DIEPTXF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPTXF7;
#[doc = "`read()` method returns [otg_hs_dieptxf7::R](otg_hs_dieptxf7::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_dieptxf7::W](otg_hs_dieptxf7::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF7 {}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf7;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grxstsr_peripheral](otg_hs_grxstsr_peripheral) module"]
pub type OTG_HS_GRXSTSR_PERIPHERAL = crate::Reg<u32, _OTG_HS_GRXSTSR_PERIPHERAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSR_PERIPHERAL;
#[doc = "`read()` method returns [otg_hs_grxstsr_peripheral::R](otg_hs_grxstsr_peripheral::R) reader structure"]
impl crate::Readable for OTG_HS_GRXSTSR_PERIPHERAL {}
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod otg_hs_grxstsr_peripheral;
#[doc = "OTG_HS status read and pop register (peripheral mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_grxstsp_peripheral](otg_hs_grxstsp_peripheral) module"]
pub type OTG_HS_GRXSTSP_PERIPHERAL = crate::Reg<u32, _OTG_HS_GRXSTSP_PERIPHERAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_GRXSTSP_PERIPHERAL;
#[doc = "`read()` method returns [otg_hs_grxstsp_peripheral::R](otg_hs_grxstsp_peripheral::R) reader structure"]
impl crate::Readable for OTG_HS_GRXSTSP_PERIPHERAL {}
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod otg_hs_grxstsp_peripheral;
