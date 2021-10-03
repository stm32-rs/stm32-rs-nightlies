#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    #[doc = "0x08 - Status register (WWDG_SR)"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register (WWDG_CR)"]
pub mod cr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register (WWDG_SR)"]
pub mod sr;
