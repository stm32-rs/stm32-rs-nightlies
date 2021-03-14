#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub fs_gotgctl: FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub fs_gotgint: FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub fs_gahbcfg: FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub fs_gusbcfg: FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub fs_grstctl: FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub fs_gintsts: FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub fs_gintmsk: FS_GINTMSK,
    _reserved_7_fs_grxstsr: [u8; 4usize],
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub fs_grxfsiz: FS_GRXFSIZ,
    _reserved_9_fs_gnptxfsiz: [u8; 4usize],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub fs_gnptxsts: FS_GNPTXSTS,
    _reserved11: [u8; 8usize],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub fs_gccfg: FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub fs_cid: FS_CID,
    _reserved13: [u8; 192usize],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub fs_hptxfsiz: FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub fs_dieptxf1: FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub fs_dieptxf2: FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub fs_dieptxf3: FS_DIEPTXF3,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_host(&self) -> &FS_GRXSTSR_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FS_GRXSTSR_HOST) }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_host_mut(&self) -> &mut FS_GRXSTSR_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FS_GRXSTSR_HOST) }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_device(&self) -> &FS_GRXSTSR_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FS_GRXSTSR_DEVICE) }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_device_mut(&self) -> &mut FS_GRXSTSR_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FS_GRXSTSR_DEVICE) }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_host(&self) -> &FS_GNPTXFSIZ_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const FS_GNPTXFSIZ_HOST) }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_host_mut(&self) -> &mut FS_GNPTXFSIZ_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut FS_GNPTXFSIZ_HOST) }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_device(&self) -> &FS_GNPTXFSIZ_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const FS_GNPTXFSIZ_DEVICE)
        }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_device_mut(&self) -> &mut FS_GNPTXFSIZ_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut FS_GNPTXFSIZ_DEVICE)
        }
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gotgctl](fs_gotgctl) module"]
pub type FS_GOTGCTL = crate::Reg<u32, _FS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GOTGCTL;
#[doc = "`read()` method returns [fs_gotgctl::R](fs_gotgctl::R) reader structure"]
impl crate::Readable for FS_GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [fs_gotgctl::W](fs_gotgctl::W) writer structure"]
impl crate::Writable for FS_GOTGCTL {}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gotgint](fs_gotgint) module"]
pub type FS_GOTGINT = crate::Reg<u32, _FS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GOTGINT;
#[doc = "`read()` method returns [fs_gotgint::R](fs_gotgint::R) reader structure"]
impl crate::Readable for FS_GOTGINT {}
#[doc = "`write(|w| ..)` method takes [fs_gotgint::W](fs_gotgint::W) writer structure"]
impl crate::Writable for FS_GOTGINT {}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gahbcfg](fs_gahbcfg) module"]
pub type FS_GAHBCFG = crate::Reg<u32, _FS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GAHBCFG;
#[doc = "`read()` method returns [fs_gahbcfg::R](fs_gahbcfg::R) reader structure"]
impl crate::Readable for FS_GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [fs_gahbcfg::W](fs_gahbcfg::W) writer structure"]
impl crate::Writable for FS_GAHBCFG {}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gusbcfg](fs_gusbcfg) module"]
pub type FS_GUSBCFG = crate::Reg<u32, _FS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GUSBCFG;
#[doc = "`read()` method returns [fs_gusbcfg::R](fs_gusbcfg::R) reader structure"]
impl crate::Readable for FS_GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [fs_gusbcfg::W](fs_gusbcfg::W) writer structure"]
impl crate::Writable for FS_GUSBCFG {}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_grstctl](fs_grstctl) module"]
pub type FS_GRSTCTL = crate::Reg<u32, _FS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRSTCTL;
#[doc = "`read()` method returns [fs_grstctl::R](fs_grstctl::R) reader structure"]
impl crate::Readable for FS_GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [fs_grstctl::W](fs_grstctl::W) writer structure"]
impl crate::Writable for FS_GRSTCTL {}
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gintsts](fs_gintsts) module"]
pub type FS_GINTSTS = crate::Reg<u32, _FS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GINTSTS;
#[doc = "`read()` method returns [fs_gintsts::R](fs_gintsts::R) reader structure"]
impl crate::Readable for FS_GINTSTS {}
#[doc = "`write(|w| ..)` method takes [fs_gintsts::W](fs_gintsts::W) writer structure"]
impl crate::Writable for FS_GINTSTS {}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gintmsk](fs_gintmsk) module"]
pub type FS_GINTMSK = crate::Reg<u32, _FS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GINTMSK;
#[doc = "`read()` method returns [fs_gintmsk::R](fs_gintmsk::R) reader structure"]
impl crate::Readable for FS_GINTMSK {}
#[doc = "`write(|w| ..)` method takes [fs_gintmsk::W](fs_gintmsk::W) writer structure"]
impl crate::Writable for FS_GINTMSK {}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "OTG_FS Receive status debug read(Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_grxstsr_device](fs_grxstsr_device) module"]
pub type FS_GRXSTSR_DEVICE = crate::Reg<u32, _FS_GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXSTSR_DEVICE;
#[doc = "`read()` method returns [fs_grxstsr_device::R](fs_grxstsr_device::R) reader structure"]
impl crate::Readable for FS_GRXSTSR_DEVICE {}
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "OTG_FS Receive status debug read(Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_grxstsr_host](fs_grxstsr_host) module"]
pub type FS_GRXSTSR_HOST = crate::Reg<u32, _FS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXSTSR_HOST;
#[doc = "`read()` method returns [fs_grxstsr_host::R](fs_grxstsr_host::R) reader structure"]
impl crate::Readable for FS_GRXSTSR_HOST {}
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_grxfsiz](fs_grxfsiz) module"]
pub type FS_GRXFSIZ = crate::Reg<u32, _FS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXFSIZ;
#[doc = "`read()` method returns [fs_grxfsiz::R](fs_grxfsiz::R) reader structure"]
impl crate::Readable for FS_GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [fs_grxfsiz::W](fs_grxfsiz::W) writer structure"]
impl crate::Writable for FS_GRXFSIZ {}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gnptxfsiz_device](fs_gnptxfsiz_device) module"]
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<u32, _FS_GNPTXFSIZ_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXFSIZ_DEVICE;
#[doc = "`read()` method returns [fs_gnptxfsiz_device::R](fs_gnptxfsiz_device::R) reader structure"]
impl crate::Readable for FS_GNPTXFSIZ_DEVICE {}
#[doc = "`write(|w| ..)` method takes [fs_gnptxfsiz_device::W](fs_gnptxfsiz_device::W) writer structure"]
impl crate::Writable for FS_GNPTXFSIZ_DEVICE {}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gnptxfsiz_host](fs_gnptxfsiz_host) module"]
pub type FS_GNPTXFSIZ_HOST = crate::Reg<u32, _FS_GNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXFSIZ_HOST;
#[doc = "`read()` method returns [fs_gnptxfsiz_host::R](fs_gnptxfsiz_host::R) reader structure"]
impl crate::Readable for FS_GNPTXFSIZ_HOST {}
#[doc = "`write(|w| ..)` method takes [fs_gnptxfsiz_host::W](fs_gnptxfsiz_host::W) writer structure"]
impl crate::Writable for FS_GNPTXFSIZ_HOST {}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gnptxsts](fs_gnptxsts) module"]
pub type FS_GNPTXSTS = crate::Reg<u32, _FS_GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXSTS;
#[doc = "`read()` method returns [fs_gnptxsts::R](fs_gnptxsts::R) reader structure"]
impl crate::Readable for FS_GNPTXSTS {}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gccfg](fs_gccfg) module"]
pub type FS_GCCFG = crate::Reg<u32, _FS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GCCFG;
#[doc = "`read()` method returns [fs_gccfg::R](fs_gccfg::R) reader structure"]
impl crate::Readable for FS_GCCFG {}
#[doc = "`write(|w| ..)` method takes [fs_gccfg::W](fs_gccfg::W) writer structure"]
impl crate::Writable for FS_GCCFG {}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_cid](fs_cid) module"]
pub type FS_CID = crate::Reg<u32, _FS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_CID;
#[doc = "`read()` method returns [fs_cid::R](fs_cid::R) reader structure"]
impl crate::Readable for FS_CID {}
#[doc = "`write(|w| ..)` method takes [fs_cid::W](fs_cid::W) writer structure"]
impl crate::Writable for FS_CID {}
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hptxfsiz](fs_hptxfsiz) module"]
pub type FS_HPTXFSIZ = crate::Reg<u32, _FS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HPTXFSIZ;
#[doc = "`read()` method returns [fs_hptxfsiz::R](fs_hptxfsiz::R) reader structure"]
impl crate::Readable for FS_HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [fs_hptxfsiz::W](fs_hptxfsiz::W) writer structure"]
impl crate::Writable for FS_HPTXFSIZ {}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_dieptxf1](fs_dieptxf1) module"]
pub type FS_DIEPTXF1 = crate::Reg<u32, _FS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF1;
#[doc = "`read()` method returns [fs_dieptxf1::R](fs_dieptxf1::R) reader structure"]
impl crate::Readable for FS_DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [fs_dieptxf1::W](fs_dieptxf1::W) writer structure"]
impl crate::Writable for FS_DIEPTXF1 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_dieptxf2](fs_dieptxf2) module"]
pub type FS_DIEPTXF2 = crate::Reg<u32, _FS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF2;
#[doc = "`read()` method returns [fs_dieptxf2::R](fs_dieptxf2::R) reader structure"]
impl crate::Readable for FS_DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [fs_dieptxf2::W](fs_dieptxf2::W) writer structure"]
impl crate::Writable for FS_DIEPTXF2 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_dieptxf3](fs_dieptxf3) module"]
pub type FS_DIEPTXF3 = crate::Reg<u32, _FS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF3;
#[doc = "`read()` method returns [fs_dieptxf3::R](fs_dieptxf3::R) reader structure"]
impl crate::Readable for FS_DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [fs_dieptxf3::W](fs_dieptxf3::W) writer structure"]
impl crate::Writable for FS_DIEPTXF3 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
