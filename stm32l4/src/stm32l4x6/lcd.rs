#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - frame control register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x08 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - clear register"]
    pub clr: crate::Reg<clr::CLR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14..0x54 - display memory"]
    pub ram_com: [crate::Reg<ram_com::RAM_COM_SPEC>; 8],
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "frame control register"]
pub mod fcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "clear register"]
pub mod clr;
#[doc = "RAM_COM register accessor: an alias for `Reg<RAM_COM_SPEC>`"]
pub type RAM_COM = crate::Reg<ram_com::RAM_COM_SPEC>;
#[doc = "display memory"]
pub mod ram_com;
