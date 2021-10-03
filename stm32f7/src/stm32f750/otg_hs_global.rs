#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub otg_hs_gotgctl: crate::Reg<otg_hs_gotgctl::OTG_HS_GOTGCTL_SPEC>,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub otg_hs_gotgint: crate::Reg<otg_hs_gotgint::OTG_HS_GOTGINT_SPEC>,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub otg_hs_gahbcfg: crate::Reg<otg_hs_gahbcfg::OTG_HS_GAHBCFG_SPEC>,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub otg_hs_gusbcfg: crate::Reg<otg_hs_gusbcfg::OTG_HS_GUSBCFG_SPEC>,
    #[doc = "0x10 - OTG_HS reset register"]
    pub otg_hs_grstctl: crate::Reg<otg_hs_grstctl::OTG_HS_GRSTCTL_SPEC>,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub otg_hs_gintsts: crate::Reg<otg_hs_gintsts::OTG_HS_GINTSTS_SPEC>,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub otg_hs_gintmsk: crate::Reg<otg_hs_gintmsk::OTG_HS_GINTMSK_SPEC>,
    _reserved_7_otg_hs_grxstsr: [u8; 0x04],
    _reserved_8_otg_hs_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub otg_hs_grxfsiz: crate::Reg<otg_hs_grxfsiz::OTG_HS_GRXFSIZ_SPEC>,
    _reserved_10_otg_hs: [u8; 0x04],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub otg_hs_gnptxsts: crate::Reg<otg_hs_gnptxsts::OTG_HS_GNPTXSTS_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub otg_hs_gccfg: crate::Reg<otg_hs_gccfg::OTG_HS_GCCFG_SPEC>,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub otg_hs_cid: crate::Reg<otg_hs_cid::OTG_HS_CID_SPEC>,
    _reserved14: [u8; 0x14],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_hs_glpmcfg: crate::Reg<otg_hs_glpmcfg::OTG_HS_GLPMCFG_SPEC>,
    _reserved15: [u8; 0xa8],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub otg_hs_hptxfsiz: crate::Reg<otg_hs_hptxfsiz::OTG_HS_HPTXFSIZ_SPEC>,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf1: crate::Reg<otg_hs_dieptxf1::OTG_HS_DIEPTXF1_SPEC>,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf2: crate::Reg<otg_hs_dieptxf2::OTG_HS_DIEPTXF2_SPEC>,
    _reserved18: [u8; 0x10],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf3: crate::Reg<otg_hs_dieptxf3::OTG_HS_DIEPTXF3_SPEC>,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf4: crate::Reg<otg_hs_dieptxf4::OTG_HS_DIEPTXF4_SPEC>,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf5: crate::Reg<otg_hs_dieptxf5::OTG_HS_DIEPTXF5_SPEC>,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf6: crate::Reg<otg_hs_dieptxf6::OTG_HS_DIEPTXF6_SPEC>,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf7: crate::Reg<otg_hs_dieptxf7::OTG_HS_DIEPTXF7_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_device(
        &self,
    ) -> &crate::Reg<otg_hs_grxstsr_device::OTG_HS_GRXSTSR_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<otg_hs_grxstsr_device::OTG_HS_GRXSTSR_DEVICE_SPEC>)
        }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsr_host(
        &self,
    ) -> &crate::Reg<otg_hs_grxstsr_host::OTG_HS_GRXSTSR_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<otg_hs_grxstsr_host::OTG_HS_GRXSTSR_HOST_SPEC>)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_device(
        &self,
    ) -> &crate::Reg<otg_hs_grxstsp_device::OTG_HS_GRXSTSP_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<otg_hs_grxstsp_device::OTG_HS_GRXSTSP_DEVICE_SPEC>)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_grxstsp_host(
        &self,
    ) -> &crate::Reg<otg_hs_grxstsp_host::OTG_HS_GRXSTSP_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<otg_hs_grxstsp_host::OTG_HS_GRXSTSP_HOST_SPEC>)
        }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn otg_hs_dieptxf0_device(
        &self,
    ) -> &crate::Reg<otg_hs_dieptxf0_device::OTG_HS_DIEPTXF0_DEVICE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<otg_hs_dieptxf0_device::OTG_HS_DIEPTXF0_DEVICE_SPEC>)
        }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn otg_hs_hnptxfsiz_host(
        &self,
    ) -> &crate::Reg<otg_hs_hnptxfsiz_host::OTG_HS_HNPTXFSIZ_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<otg_hs_hnptxfsiz_host::OTG_HS_HNPTXFSIZ_HOST_SPEC>)
        }
    }
}
#[doc = "OTG_HS_GOTGCTL register accessor: an alias for `Reg<OTG_HS_GOTGCTL_SPEC>`"]
pub type OTG_HS_GOTGCTL = crate::Reg<otg_hs_gotgctl::OTG_HS_GOTGCTL_SPEC>;
#[doc = "OTG_HS control and status register"]
pub mod otg_hs_gotgctl;
#[doc = "OTG_HS_GOTGINT register accessor: an alias for `Reg<OTG_HS_GOTGINT_SPEC>`"]
pub type OTG_HS_GOTGINT = crate::Reg<otg_hs_gotgint::OTG_HS_GOTGINT_SPEC>;
#[doc = "OTG_HS interrupt register"]
pub mod otg_hs_gotgint;
#[doc = "OTG_HS_GAHBCFG register accessor: an alias for `Reg<OTG_HS_GAHBCFG_SPEC>`"]
pub type OTG_HS_GAHBCFG = crate::Reg<otg_hs_gahbcfg::OTG_HS_GAHBCFG_SPEC>;
#[doc = "OTG_HS AHB configuration register"]
pub mod otg_hs_gahbcfg;
#[doc = "OTG_HS_GUSBCFG register accessor: an alias for `Reg<OTG_HS_GUSBCFG_SPEC>`"]
pub type OTG_HS_GUSBCFG = crate::Reg<otg_hs_gusbcfg::OTG_HS_GUSBCFG_SPEC>;
#[doc = "OTG_HS USB configuration register"]
pub mod otg_hs_gusbcfg;
#[doc = "OTG_HS_GRSTCTL register accessor: an alias for `Reg<OTG_HS_GRSTCTL_SPEC>`"]
pub type OTG_HS_GRSTCTL = crate::Reg<otg_hs_grstctl::OTG_HS_GRSTCTL_SPEC>;
#[doc = "OTG_HS reset register"]
pub mod otg_hs_grstctl;
#[doc = "OTG_HS_GINTSTS register accessor: an alias for `Reg<OTG_HS_GINTSTS_SPEC>`"]
pub type OTG_HS_GINTSTS = crate::Reg<otg_hs_gintsts::OTG_HS_GINTSTS_SPEC>;
#[doc = "OTG_HS core interrupt register"]
pub mod otg_hs_gintsts;
#[doc = "OTG_HS_GINTMSK register accessor: an alias for `Reg<OTG_HS_GINTMSK_SPEC>`"]
pub type OTG_HS_GINTMSK = crate::Reg<otg_hs_gintmsk::OTG_HS_GINTMSK_SPEC>;
#[doc = "OTG_HS interrupt mask register"]
pub mod otg_hs_gintmsk;
#[doc = "OTG_HS_GRXSTSR_Host register accessor: an alias for `Reg<OTG_HS_GRXSTSR_HOST_SPEC>`"]
pub type OTG_HS_GRXSTSR_HOST = crate::Reg<otg_hs_grxstsr_host::OTG_HS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod otg_hs_grxstsr_host;
#[doc = "OTG_HS_GRXSTSP_Host register accessor: an alias for `Reg<OTG_HS_GRXSTSP_HOST_SPEC>`"]
pub type OTG_HS_GRXSTSP_HOST = crate::Reg<otg_hs_grxstsp_host::OTG_HS_GRXSTSP_HOST_SPEC>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod otg_hs_grxstsp_host;
#[doc = "OTG_HS_GRXFSIZ register accessor: an alias for `Reg<OTG_HS_GRXFSIZ_SPEC>`"]
pub type OTG_HS_GRXFSIZ = crate::Reg<otg_hs_grxfsiz::OTG_HS_GRXFSIZ_SPEC>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod otg_hs_grxfsiz;
#[doc = "OTG_HS_HNPTXFSIZ_Host register accessor: an alias for `Reg<OTG_HS_HNPTXFSIZ_HOST_SPEC>`"]
pub type OTG_HS_HNPTXFSIZ_HOST = crate::Reg<otg_hs_hnptxfsiz_host::OTG_HS_HNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod otg_hs_hnptxfsiz_host;
#[doc = "OTG_HS_DIEPTXF0_Device register accessor: an alias for `Reg<OTG_HS_DIEPTXF0_DEVICE_SPEC>`"]
pub type OTG_HS_DIEPTXF0_DEVICE = crate::Reg<otg_hs_dieptxf0_device::OTG_HS_DIEPTXF0_DEVICE_SPEC>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod otg_hs_dieptxf0_device;
#[doc = "OTG_HS_GNPTXSTS register accessor: an alias for `Reg<OTG_HS_GNPTXSTS_SPEC>`"]
pub type OTG_HS_GNPTXSTS = crate::Reg<otg_hs_gnptxsts::OTG_HS_GNPTXSTS_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod otg_hs_gnptxsts;
#[doc = "OTG_HS_GCCFG register accessor: an alias for `Reg<OTG_HS_GCCFG_SPEC>`"]
pub type OTG_HS_GCCFG = crate::Reg<otg_hs_gccfg::OTG_HS_GCCFG_SPEC>;
#[doc = "OTG_HS general core configuration register"]
pub mod otg_hs_gccfg;
#[doc = "OTG_HS_CID register accessor: an alias for `Reg<OTG_HS_CID_SPEC>`"]
pub type OTG_HS_CID = crate::Reg<otg_hs_cid::OTG_HS_CID_SPEC>;
#[doc = "OTG_HS core ID register"]
pub mod otg_hs_cid;
#[doc = "OTG_HS_HPTXFSIZ register accessor: an alias for `Reg<OTG_HS_HPTXFSIZ_SPEC>`"]
pub type OTG_HS_HPTXFSIZ = crate::Reg<otg_hs_hptxfsiz::OTG_HS_HPTXFSIZ_SPEC>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod otg_hs_hptxfsiz;
#[doc = "OTG_HS_DIEPTXF1 register accessor: an alias for `Reg<OTG_HS_DIEPTXF1_SPEC>`"]
pub type OTG_HS_DIEPTXF1 = crate::Reg<otg_hs_dieptxf1::OTG_HS_DIEPTXF1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf1;
#[doc = "OTG_HS_DIEPTXF2 register accessor: an alias for `Reg<OTG_HS_DIEPTXF2_SPEC>`"]
pub type OTG_HS_DIEPTXF2 = crate::Reg<otg_hs_dieptxf2::OTG_HS_DIEPTXF2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf2;
#[doc = "OTG_HS_DIEPTXF3 register accessor: an alias for `Reg<OTG_HS_DIEPTXF3_SPEC>`"]
pub type OTG_HS_DIEPTXF3 = crate::Reg<otg_hs_dieptxf3::OTG_HS_DIEPTXF3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf3;
#[doc = "OTG_HS_DIEPTXF4 register accessor: an alias for `Reg<OTG_HS_DIEPTXF4_SPEC>`"]
pub type OTG_HS_DIEPTXF4 = crate::Reg<otg_hs_dieptxf4::OTG_HS_DIEPTXF4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf4;
#[doc = "OTG_HS_DIEPTXF5 register accessor: an alias for `Reg<OTG_HS_DIEPTXF5_SPEC>`"]
pub type OTG_HS_DIEPTXF5 = crate::Reg<otg_hs_dieptxf5::OTG_HS_DIEPTXF5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf5;
#[doc = "OTG_HS_DIEPTXF6 register accessor: an alias for `Reg<OTG_HS_DIEPTXF6_SPEC>`"]
pub type OTG_HS_DIEPTXF6 = crate::Reg<otg_hs_dieptxf6::OTG_HS_DIEPTXF6_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf6;
#[doc = "OTG_HS_DIEPTXF7 register accessor: an alias for `Reg<OTG_HS_DIEPTXF7_SPEC>`"]
pub type OTG_HS_DIEPTXF7 = crate::Reg<otg_hs_dieptxf7::OTG_HS_DIEPTXF7_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf7;
#[doc = "OTG_HS_GRXSTSR_Device register accessor: an alias for `Reg<OTG_HS_GRXSTSR_DEVICE_SPEC>`"]
pub type OTG_HS_GRXSTSR_DEVICE = crate::Reg<otg_hs_grxstsr_device::OTG_HS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod otg_hs_grxstsr_device;
#[doc = "OTG_HS_GRXSTSP_Device register accessor: an alias for `Reg<OTG_HS_GRXSTSP_DEVICE_SPEC>`"]
pub type OTG_HS_GRXSTSP_DEVICE = crate::Reg<otg_hs_grxstsp_device::OTG_HS_GRXSTSP_DEVICE_SPEC>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod otg_hs_grxstsp_device;
#[doc = "OTG_HS_GLPMCFG register accessor: an alias for `Reg<OTG_HS_GLPMCFG_SPEC>`"]
pub type OTG_HS_GLPMCFG = crate::Reg<otg_hs_glpmcfg::OTG_HS_GLPMCFG_SPEC>;
#[doc = "OTG core LPM configuration register"]
pub mod otg_hs_glpmcfg;
