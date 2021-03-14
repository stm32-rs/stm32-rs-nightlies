#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub otg_fs_gotgctl: OTG_FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub otg_fs_gotgint: OTG_FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub otg_fs_gahbcfg: OTG_FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub otg_fs_gusbcfg: OTG_FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub otg_fs_grstctl: OTG_FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub otg_fs_gintsts: OTG_FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub otg_fs_gintmsk: OTG_FS_GINTMSK,
    _reserved_7_otg_fs_grxstsr: [u8; 4usize],
    _reserved_8_otg_fs_grxstsp: [u8; 4usize],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub otg_fs_grxfsiz: OTG_FS_GRXFSIZ,
    _reserved_10_otg_fs: [u8; 4usize],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub otg_fs_hnptxsts: OTG_FS_HNPTXSTS,
    #[doc = "0x30 - OTG I2C access register"]
    pub otg_fs_gi2cctl: OTG_FS_GI2CCTL,
    _reserved13: [u8; 4usize],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub otg_fs_gccfg: OTG_FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub otg_fs_cid: OTG_FS_CID,
    _reserved15: [u8; 20usize],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_fs_glpmcfg: OTG_FS_GLPMCFG,
    #[doc = "0x58 - OTG power down register"]
    pub otg_fs_gpwrdn: OTG_FS_GPWRDN,
    _reserved17: [u8; 4usize],
    #[doc = "0x60 - OTG ADP timer, control and status register"]
    pub otg_fs_gadpctl: OTG_FS_GADPCTL,
    _reserved18: [u8; 156usize],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub otg_fs_hptxfsiz: OTG_FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
    pub otg_fs_dieptxf1: OTG_FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub otg_fs_dieptxf2: OTG_FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub otg_fs_dieptxf3: OTG_FS_DIEPTXF3,
    #[doc = "0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub otg_fs_dieptxf4: OTG_FS_DIEPTXF4,
    #[doc = "0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
    pub otg_fs_dieptxf5: OTG_FS_DIEPTXF5,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_host(&self) -> &OTG_FS_GRXSTSR_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_HOST)
        }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_host_mut(&self) -> &mut OTG_FS_GRXSTSR_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_FS_GRXSTSR_HOST)
        }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_device(&self) -> &OTG_FS_GRXSTSR_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize) as *const OTG_FS_GRXSTSR_DEVICE)
        }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsr_device_mut(&self) -> &mut OTG_FS_GRXSTSR_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut OTG_FS_GRXSTSR_DEVICE)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_host(&self) -> &OTG_FS_GRXSTSP_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_HOST)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Host mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_host_mut(&self) -> &mut OTG_FS_GRXSTSP_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_FS_GRXSTSP_HOST)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_device(&self) -> &OTG_FS_GRXSTSP_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize) as *const OTG_FS_GRXSTSP_DEVICE)
        }
    }
    #[doc = "0x20 - OTG status read and pop register (Device mode)"]
    #[inline(always)]
    pub fn otg_fs_grxstsp_device_mut(&self) -> &mut OTG_FS_GRXSTSP_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut OTG_FS_GRXSTSP_DEVICE)
        }
    }
    #[doc = "0x28 - OTG_FS Host non-periodic transmit FIFO size register"]
    #[inline(always)]
    pub fn otg_fs_hnptxfsiz_host(&self) -> &OTG_FS_HNPTXFSIZ_HOST {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_HNPTXFSIZ_HOST)
        }
    }
    #[doc = "0x28 - OTG_FS Host non-periodic transmit FIFO size register"]
    #[inline(always)]
    pub fn otg_fs_hnptxfsiz_host_mut(&self) -> &mut OTG_FS_HNPTXFSIZ_HOST {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_FS_HNPTXFSIZ_HOST)
        }
    }
    #[doc = "0x28 - OTG_FS Endpoint 0 Transmit FIFO size"]
    #[inline(always)]
    pub fn otg_fs_dieptxf0_device(&self) -> &OTG_FS_DIEPTXF0_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const OTG_FS_DIEPTXF0_DEVICE)
        }
    }
    #[doc = "0x28 - OTG_FS Endpoint 0 Transmit FIFO size"]
    #[inline(always)]
    pub fn otg_fs_dieptxf0_device_mut(&self) -> &mut OTG_FS_DIEPTXF0_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut OTG_FS_DIEPTXF0_DEVICE)
        }
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gotgctl](otg_fs_gotgctl) module"]
pub type OTG_FS_GOTGCTL = crate::Reg<u32, _OTG_FS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GOTGCTL;
#[doc = "`read()` method returns [otg_fs_gotgctl::R](otg_fs_gotgctl::R) reader structure"]
impl crate::Readable for OTG_FS_GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gotgctl::W](otg_fs_gotgctl::W) writer structure"]
impl crate::Writable for OTG_FS_GOTGCTL {}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod otg_fs_gotgctl;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gotgint](otg_fs_gotgint) module"]
pub type OTG_FS_GOTGINT = crate::Reg<u32, _OTG_FS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GOTGINT;
#[doc = "`read()` method returns [otg_fs_gotgint::R](otg_fs_gotgint::R) reader structure"]
impl crate::Readable for OTG_FS_GOTGINT {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gotgint::W](otg_fs_gotgint::W) writer structure"]
impl crate::Writable for OTG_FS_GOTGINT {}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod otg_fs_gotgint;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gahbcfg](otg_fs_gahbcfg) module"]
pub type OTG_FS_GAHBCFG = crate::Reg<u32, _OTG_FS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GAHBCFG;
#[doc = "`read()` method returns [otg_fs_gahbcfg::R](otg_fs_gahbcfg::R) reader structure"]
impl crate::Readable for OTG_FS_GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gahbcfg::W](otg_fs_gahbcfg::W) writer structure"]
impl crate::Writable for OTG_FS_GAHBCFG {}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod otg_fs_gahbcfg;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gusbcfg](otg_fs_gusbcfg) module"]
pub type OTG_FS_GUSBCFG = crate::Reg<u32, _OTG_FS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GUSBCFG;
#[doc = "`read()` method returns [otg_fs_gusbcfg::R](otg_fs_gusbcfg::R) reader structure"]
impl crate::Readable for OTG_FS_GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gusbcfg::W](otg_fs_gusbcfg::W) writer structure"]
impl crate::Writable for OTG_FS_GUSBCFG {}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod otg_fs_gusbcfg;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grstctl](otg_fs_grstctl) module"]
pub type OTG_FS_GRSTCTL = crate::Reg<u32, _OTG_FS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRSTCTL;
#[doc = "`read()` method returns [otg_fs_grstctl::R](otg_fs_grstctl::R) reader structure"]
impl crate::Readable for OTG_FS_GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_grstctl::W](otg_fs_grstctl::W) writer structure"]
impl crate::Writable for OTG_FS_GRSTCTL {}
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod otg_fs_grstctl;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gintsts](otg_fs_gintsts) module"]
pub type OTG_FS_GINTSTS = crate::Reg<u32, _OTG_FS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GINTSTS;
#[doc = "`read()` method returns [otg_fs_gintsts::R](otg_fs_gintsts::R) reader structure"]
impl crate::Readable for OTG_FS_GINTSTS {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gintsts::W](otg_fs_gintsts::W) writer structure"]
impl crate::Writable for OTG_FS_GINTSTS {}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod otg_fs_gintsts;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gintmsk](otg_fs_gintmsk) module"]
pub type OTG_FS_GINTMSK = crate::Reg<u32, _OTG_FS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GINTMSK;
#[doc = "`read()` method returns [otg_fs_gintmsk::R](otg_fs_gintmsk::R) reader structure"]
impl crate::Readable for OTG_FS_GINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gintmsk::W](otg_fs_gintmsk::W) writer structure"]
impl crate::Writable for OTG_FS_GINTMSK {}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod otg_fs_gintmsk;
#[doc = "OTG_FS Receive status debug read(Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grxstsr_device](otg_fs_grxstsr_device) module"]
pub type OTG_FS_GRXSTSR_DEVICE = crate::Reg<u32, _OTG_FS_GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSR_DEVICE;
#[doc = "`read()` method returns [otg_fs_grxstsr_device::R](otg_fs_grxstsr_device::R) reader structure"]
impl crate::Readable for OTG_FS_GRXSTSR_DEVICE {}
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod otg_fs_grxstsr_device;
#[doc = "OTG_FS Receive status debug read(Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grxstsr_host](otg_fs_grxstsr_host) module"]
pub type OTG_FS_GRXSTSR_HOST = crate::Reg<u32, _OTG_FS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSR_HOST;
#[doc = "`read()` method returns [otg_fs_grxstsr_host::R](otg_fs_grxstsr_host::R) reader structure"]
impl crate::Readable for OTG_FS_GRXSTSR_HOST {}
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod otg_fs_grxstsr_host;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grxfsiz](otg_fs_grxfsiz) module"]
pub type OTG_FS_GRXFSIZ = crate::Reg<u32, _OTG_FS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXFSIZ;
#[doc = "`read()` method returns [otg_fs_grxfsiz::R](otg_fs_grxfsiz::R) reader structure"]
impl crate::Readable for OTG_FS_GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_fs_grxfsiz::W](otg_fs_grxfsiz::W) writer structure"]
impl crate::Writable for OTG_FS_GRXFSIZ {}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod otg_fs_grxfsiz;
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf0_device](otg_fs_dieptxf0_device) module"]
pub type OTG_FS_DIEPTXF0_DEVICE = crate::Reg<u32, _OTG_FS_DIEPTXF0_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF0_DEVICE;
#[doc = "`read()` method returns [otg_fs_dieptxf0_device::R](otg_fs_dieptxf0_device::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF0_DEVICE {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf0_device::W](otg_fs_dieptxf0_device::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF0_DEVICE {}
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size"]
pub mod otg_fs_dieptxf0_device;
#[doc = "OTG_FS Host non-periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hnptxfsiz_host](otg_fs_hnptxfsiz_host) module"]
pub type OTG_FS_HNPTXFSIZ_HOST = crate::Reg<u32, _OTG_FS_HNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HNPTXFSIZ_HOST;
#[doc = "`read()` method returns [otg_fs_hnptxfsiz_host::R](otg_fs_hnptxfsiz_host::R) reader structure"]
impl crate::Readable for OTG_FS_HNPTXFSIZ_HOST {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hnptxfsiz_host::W](otg_fs_hnptxfsiz_host::W) writer structure"]
impl crate::Writable for OTG_FS_HNPTXFSIZ_HOST {}
#[doc = "OTG_FS Host non-periodic transmit FIFO size register"]
pub mod otg_fs_hnptxfsiz_host;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hnptxsts](otg_fs_hnptxsts) module"]
pub type OTG_FS_HNPTXSTS = crate::Reg<u32, _OTG_FS_HNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HNPTXSTS;
#[doc = "`read()` method returns [otg_fs_hnptxsts::R](otg_fs_hnptxsts::R) reader structure"]
impl crate::Readable for OTG_FS_HNPTXSTS {}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod otg_fs_hnptxsts;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gccfg](otg_fs_gccfg) module"]
pub type OTG_FS_GCCFG = crate::Reg<u32, _OTG_FS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GCCFG;
#[doc = "`read()` method returns [otg_fs_gccfg::R](otg_fs_gccfg::R) reader structure"]
impl crate::Readable for OTG_FS_GCCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gccfg::W](otg_fs_gccfg::W) writer structure"]
impl crate::Writable for OTG_FS_GCCFG {}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod otg_fs_gccfg;
#[doc = "core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_cid](otg_fs_cid) module"]
pub type OTG_FS_CID = crate::Reg<u32, _OTG_FS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_CID;
#[doc = "`read()` method returns [otg_fs_cid::R](otg_fs_cid::R) reader structure"]
impl crate::Readable for OTG_FS_CID {}
#[doc = "`write(|w| ..)` method takes [otg_fs_cid::W](otg_fs_cid::W) writer structure"]
impl crate::Writable for OTG_FS_CID {}
#[doc = "core ID register"]
pub mod otg_fs_cid;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hptxfsiz](otg_fs_hptxfsiz) module"]
pub type OTG_FS_HPTXFSIZ = crate::Reg<u32, _OTG_FS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPTXFSIZ;
#[doc = "`read()` method returns [otg_fs_hptxfsiz::R](otg_fs_hptxfsiz::R) reader structure"]
impl crate::Readable for OTG_FS_HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hptxfsiz::W](otg_fs_hptxfsiz::W) writer structure"]
impl crate::Writable for OTG_FS_HPTXFSIZ {}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod otg_fs_hptxfsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf1](otg_fs_dieptxf1) module"]
pub type OTG_FS_DIEPTXF1 = crate::Reg<u32, _OTG_FS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF1;
#[doc = "`read()` method returns [otg_fs_dieptxf1::R](otg_fs_dieptxf1::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf1::W](otg_fs_dieptxf1::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF1 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
pub mod otg_fs_dieptxf1;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf2](otg_fs_dieptxf2) module"]
pub type OTG_FS_DIEPTXF2 = crate::Reg<u32, _OTG_FS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF2;
#[doc = "`read()` method returns [otg_fs_dieptxf2::R](otg_fs_dieptxf2::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf2::W](otg_fs_dieptxf2::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF2 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod otg_fs_dieptxf2;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf3](otg_fs_dieptxf3) module"]
pub type OTG_FS_DIEPTXF3 = crate::Reg<u32, _OTG_FS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF3;
#[doc = "`read()` method returns [otg_fs_dieptxf3::R](otg_fs_dieptxf3::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf3::W](otg_fs_dieptxf3::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF3 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod otg_fs_dieptxf3;
#[doc = "OTG status read and pop register (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grxstsp_device](otg_fs_grxstsp_device) module"]
pub type OTG_FS_GRXSTSP_DEVICE = crate::Reg<u32, _OTG_FS_GRXSTSP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSP_DEVICE;
#[doc = "`read()` method returns [otg_fs_grxstsp_device::R](otg_fs_grxstsp_device::R) reader structure"]
impl crate::Readable for OTG_FS_GRXSTSP_DEVICE {}
#[doc = "OTG status read and pop register (Device mode)"]
pub mod otg_fs_grxstsp_device;
#[doc = "OTG status read and pop register (Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_grxstsp_host](otg_fs_grxstsp_host) module"]
pub type OTG_FS_GRXSTSP_HOST = crate::Reg<u32, _OTG_FS_GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GRXSTSP_HOST;
#[doc = "`read()` method returns [otg_fs_grxstsp_host::R](otg_fs_grxstsp_host::R) reader structure"]
impl crate::Readable for OTG_FS_GRXSTSP_HOST {}
#[doc = "OTG status read and pop register (Host mode)"]
pub mod otg_fs_grxstsp_host;
#[doc = "OTG I2C access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gi2cctl](otg_fs_gi2cctl) module"]
pub type OTG_FS_GI2CCTL = crate::Reg<u32, _OTG_FS_GI2CCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GI2CCTL;
#[doc = "`read()` method returns [otg_fs_gi2cctl::R](otg_fs_gi2cctl::R) reader structure"]
impl crate::Readable for OTG_FS_GI2CCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gi2cctl::W](otg_fs_gi2cctl::W) writer structure"]
impl crate::Writable for OTG_FS_GI2CCTL {}
#[doc = "OTG I2C access register"]
pub mod otg_fs_gi2cctl;
#[doc = "OTG power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gpwrdn](otg_fs_gpwrdn) module"]
pub type OTG_FS_GPWRDN = crate::Reg<u32, _OTG_FS_GPWRDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GPWRDN;
#[doc = "`read()` method returns [otg_fs_gpwrdn::R](otg_fs_gpwrdn::R) reader structure"]
impl crate::Readable for OTG_FS_GPWRDN {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gpwrdn::W](otg_fs_gpwrdn::W) writer structure"]
impl crate::Writable for OTG_FS_GPWRDN {}
#[doc = "OTG power down register"]
pub mod otg_fs_gpwrdn;
#[doc = "OTG ADP timer, control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gadpctl](otg_fs_gadpctl) module"]
pub type OTG_FS_GADPCTL = crate::Reg<u32, _OTG_FS_GADPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GADPCTL;
#[doc = "`read()` method returns [otg_fs_gadpctl::R](otg_fs_gadpctl::R) reader structure"]
impl crate::Readable for OTG_FS_GADPCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_gadpctl::W](otg_fs_gadpctl::W) writer structure"]
impl crate::Writable for OTG_FS_GADPCTL {}
#[doc = "OTG ADP timer, control and status register"]
pub mod otg_fs_gadpctl;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf4](otg_fs_dieptxf4) module"]
pub type OTG_FS_DIEPTXF4 = crate::Reg<u32, _OTG_FS_DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF4;
#[doc = "`read()` method returns [otg_fs_dieptxf4::R](otg_fs_dieptxf4::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf4::W](otg_fs_dieptxf4::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF4 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod otg_fs_dieptxf4;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptxf5](otg_fs_dieptxf5) module"]
pub type OTG_FS_DIEPTXF5 = crate::Reg<u32, _OTG_FS_DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTXF5;
#[doc = "`read()` method returns [otg_fs_dieptxf5::R](otg_fs_dieptxf5::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptxf5::W](otg_fs_dieptxf5::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTXF5 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
pub mod otg_fs_dieptxf5;
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_glpmcfg](otg_fs_glpmcfg) module"]
pub type OTG_FS_GLPMCFG = crate::Reg<u32, _OTG_FS_GLPMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_GLPMCFG;
#[doc = "`read()` method returns [otg_fs_glpmcfg::R](otg_fs_glpmcfg::R) reader structure"]
impl crate::Readable for OTG_FS_GLPMCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_glpmcfg::W](otg_fs_glpmcfg::W) writer structure"]
impl crate::Writable for OTG_FS_GLPMCFG {}
#[doc = "OTG core LPM configuration register"]
pub mod otg_fs_glpmcfg;
