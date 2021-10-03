#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub fs_gotgctl: crate::Reg<fs_gotgctl::FS_GOTGCTL_SPEC>,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub fs_gotgint: crate::Reg<fs_gotgint::FS_GOTGINT_SPEC>,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub fs_gahbcfg: crate::Reg<fs_gahbcfg::FS_GAHBCFG_SPEC>,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub fs_gusbcfg: crate::Reg<fs_gusbcfg::FS_GUSBCFG_SPEC>,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub fs_grstctl: crate::Reg<fs_grstctl::FS_GRSTCTL_SPEC>,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub fs_gintsts: crate::Reg<fs_gintsts::FS_GINTSTS_SPEC>,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub fs_gintmsk: crate::Reg<fs_gintmsk::FS_GINTMSK_SPEC>,
    _reserved_7_fs_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub fs_grxfsiz: crate::Reg<fs_grxfsiz::FS_GRXFSIZ_SPEC>,
    _reserved_9_fs_gnptxfsiz: [u8; 0x04],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub fs_gnptxsts: crate::Reg<fs_gnptxsts::FS_GNPTXSTS_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub fs_gccfg: crate::Reg<fs_gccfg::FS_GCCFG_SPEC>,
    #[doc = "0x3c - core ID register"]
    pub fs_cid: crate::Reg<fs_cid::FS_CID_SPEC>,
    _reserved13: [u8; 0xc0],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub fs_hptxfsiz: crate::Reg<fs_hptxfsiz::FS_HPTXFSIZ_SPEC>,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub fs_dieptxf1: crate::Reg<fs_dieptxf1::FS_DIEPTXF1_SPEC>,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub fs_dieptxf2: crate::Reg<fs_dieptxf2::FS_DIEPTXF2_SPEC>,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub fs_dieptxf3: crate::Reg<fs_dieptxf3::FS_DIEPTXF3_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_host(&self) -> &crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOST_SPEC>)
        }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub fn fs_grxstsr_device(&self) -> &crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICE_SPEC>)
        }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_host(&self) -> &crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOST_SPEC>)
        }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub fn fs_gnptxfsiz_device(
        &self,
    ) -> &crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICE_SPEC>)
        }
    }
}
#[doc = "FS_GOTGCTL register accessor: an alias for `Reg<FS_GOTGCTL_SPEC>`"]
pub type FS_GOTGCTL = crate::Reg<fs_gotgctl::FS_GOTGCTL_SPEC>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "FS_GOTGINT register accessor: an alias for `Reg<FS_GOTGINT_SPEC>`"]
pub type FS_GOTGINT = crate::Reg<fs_gotgint::FS_GOTGINT_SPEC>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "FS_GAHBCFG register accessor: an alias for `Reg<FS_GAHBCFG_SPEC>`"]
pub type FS_GAHBCFG = crate::Reg<fs_gahbcfg::FS_GAHBCFG_SPEC>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "FS_GUSBCFG register accessor: an alias for `Reg<FS_GUSBCFG_SPEC>`"]
pub type FS_GUSBCFG = crate::Reg<fs_gusbcfg::FS_GUSBCFG_SPEC>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "FS_GRSTCTL register accessor: an alias for `Reg<FS_GRSTCTL_SPEC>`"]
pub type FS_GRSTCTL = crate::Reg<fs_grstctl::FS_GRSTCTL_SPEC>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "FS_GINTSTS register accessor: an alias for `Reg<FS_GINTSTS_SPEC>`"]
pub type FS_GINTSTS = crate::Reg<fs_gintsts::FS_GINTSTS_SPEC>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "FS_GINTMSK register accessor: an alias for `Reg<FS_GINTMSK_SPEC>`"]
pub type FS_GINTMSK = crate::Reg<fs_gintmsk::FS_GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "FS_GRXSTSR_Device register accessor: an alias for `Reg<FS_GRXSTSR_DEVICE_SPEC>`"]
pub type FS_GRXSTSR_DEVICE = crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "FS_GRXSTSR_Host register accessor: an alias for `Reg<FS_GRXSTSR_HOST_SPEC>`"]
pub type FS_GRXSTSR_HOST = crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "FS_GRXFSIZ register accessor: an alias for `Reg<FS_GRXFSIZ_SPEC>`"]
pub type FS_GRXFSIZ = crate::Reg<fs_grxfsiz::FS_GRXFSIZ_SPEC>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "FS_GNPTXFSIZ_Device register accessor: an alias for `Reg<FS_GNPTXFSIZ_DEVICE_SPEC>`"]
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "FS_GNPTXFSIZ_Host register accessor: an alias for `Reg<FS_GNPTXFSIZ_HOST_SPEC>`"]
pub type FS_GNPTXFSIZ_HOST = crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "FS_GNPTXSTS register accessor: an alias for `Reg<FS_GNPTXSTS_SPEC>`"]
pub type FS_GNPTXSTS = crate::Reg<fs_gnptxsts::FS_GNPTXSTS_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "FS_GCCFG register accessor: an alias for `Reg<FS_GCCFG_SPEC>`"]
pub type FS_GCCFG = crate::Reg<fs_gccfg::FS_GCCFG_SPEC>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "FS_CID register accessor: an alias for `Reg<FS_CID_SPEC>`"]
pub type FS_CID = crate::Reg<fs_cid::FS_CID_SPEC>;
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "FS_HPTXFSIZ register accessor: an alias for `Reg<FS_HPTXFSIZ_SPEC>`"]
pub type FS_HPTXFSIZ = crate::Reg<fs_hptxfsiz::FS_HPTXFSIZ_SPEC>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "FS_DIEPTXF1 register accessor: an alias for `Reg<FS_DIEPTXF1_SPEC>`"]
pub type FS_DIEPTXF1 = crate::Reg<fs_dieptxf1::FS_DIEPTXF1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "FS_DIEPTXF2 register accessor: an alias for `Reg<FS_DIEPTXF2_SPEC>`"]
pub type FS_DIEPTXF2 = crate::Reg<fs_dieptxf2::FS_DIEPTXF2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "FS_DIEPTXF3 register accessor: an alias for `Reg<FS_DIEPTXF3_SPEC>`"]
pub type FS_DIEPTXF3 = crate::Reg<fs_dieptxf3::FS_DIEPTXF3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
