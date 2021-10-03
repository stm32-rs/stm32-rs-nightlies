#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub wwdg_cr: crate::Reg<wwdg_cr::WWDG_CR_SPEC>,
    #[doc = "0x04 - Configuration register"]
    pub wwdg_cfr: crate::Reg<wwdg_cfr::WWDG_CFR_SPEC>,
    #[doc = "0x08 - Status register"]
    pub wwdg_sr: crate::Reg<wwdg_sr::WWDG_SR_SPEC>,
}
#[doc = "WWDG_CR register accessor: an alias for `Reg<WWDG_CR_SPEC>`"]
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CR_SPEC>;
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "WWDG_CFR register accessor: an alias for `Reg<WWDG_CFR_SPEC>`"]
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFR_SPEC>;
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "WWDG_SR register accessor: an alias for `Reg<WWDG_SR_SPEC>`"]
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SR_SPEC>;
#[doc = "Status register"]
pub mod wwdg_sr;
