#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to control the PLL of the HS PHY."]
    pub usbphyc_pll: crate::Reg<usbphyc_pll::USBPHYC_PLL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - This register is used to control the switch between controllers for the HS PHY."]
    pub usbphyc_misc: crate::Reg<usbphyc_misc::USBPHYC_MISC_SPEC>,
    _reserved2: [u8; 0x0100],
    #[doc = "0x10c - This register is used to control the tune interface of the HS PHY, port #x."]
    pub usbphyc_tune1: crate::Reg<usbphyc_tune1::USBPHYC_TUNE1_SPEC>,
    _reserved3: [u8; 0xfc],
    #[doc = "0x20c - This register is used to control the tune interface of the HS PHY, port #x."]
    pub usbphyc_tune2: crate::Reg<usbphyc_tune2::USBPHYC_TUNE2_SPEC>,
    _reserved4: [u8; 0x0dec],
    #[doc = "0xffc - This register defines the version of this IP."]
    pub usbphyc_verr: crate::Reg<usbphyc_verr::USBPHYC_VERR_SPEC>,
}
#[doc = "USBPHYC_PLL register accessor: an alias for `Reg<USBPHYC_PLL_SPEC>`"]
pub type USBPHYC_PLL = crate::Reg<usbphyc_pll::USBPHYC_PLL_SPEC>;
#[doc = "This register is used to control the PLL of the HS PHY."]
pub mod usbphyc_pll;
#[doc = "USBPHYC_MISC register accessor: an alias for `Reg<USBPHYC_MISC_SPEC>`"]
pub type USBPHYC_MISC = crate::Reg<usbphyc_misc::USBPHYC_MISC_SPEC>;
#[doc = "This register is used to control the switch between controllers for the HS PHY."]
pub mod usbphyc_misc;
#[doc = "USBPHYC_TUNE1 register accessor: an alias for `Reg<USBPHYC_TUNE1_SPEC>`"]
pub type USBPHYC_TUNE1 = crate::Reg<usbphyc_tune1::USBPHYC_TUNE1_SPEC>;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune1;
#[doc = "USBPHYC_TUNE2 register accessor: an alias for `Reg<USBPHYC_TUNE2_SPEC>`"]
pub type USBPHYC_TUNE2 = crate::Reg<usbphyc_tune2::USBPHYC_TUNE2_SPEC>;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune2;
#[doc = "USBPHYC_VERR register accessor: an alias for `Reg<USBPHYC_VERR_SPEC>`"]
pub type USBPHYC_VERR = crate::Reg<usbphyc_verr::USBPHYC_VERR_SPEC>;
#[doc = "This register defines the version of this IP."]
pub mod usbphyc_verr;
