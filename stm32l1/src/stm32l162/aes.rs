#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - Data input register"]
    pub dinr: crate::Reg<dinr::DINR_SPEC>,
    #[doc = "0x0c - Data output register"]
    pub doutr: crate::Reg<doutr::DOUTR_SPEC>,
    #[doc = "0x10 - AES Key register 0"]
    pub keyr0: crate::Reg<keyr0::KEYR0_SPEC>,
    #[doc = "0x14 - AES Key register 1"]
    pub keyr1: crate::Reg<keyr1::KEYR1_SPEC>,
    #[doc = "0x18 - AES Key register 2"]
    pub keyr2: crate::Reg<keyr2::KEYR2_SPEC>,
    #[doc = "0x1c - AES Key register 3"]
    pub keyr3: crate::Reg<keyr3::KEYR3_SPEC>,
    #[doc = "0x20 - Initialization Vector Register 0"]
    pub ivr0: crate::Reg<ivr0::IVR0_SPEC>,
    #[doc = "0x24 - Initialization Vector Register 1"]
    pub ivr1: crate::Reg<ivr1::IVR1_SPEC>,
    #[doc = "0x28 - Initialization Vector Register 2"]
    pub ivr2: crate::Reg<ivr2::IVR2_SPEC>,
    #[doc = "0x2c - Initialization Vector Register 3"]
    pub ivr3: crate::Reg<ivr3::IVR3_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "DINR register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "Data input register"]
pub mod dinr;
#[doc = "DOUTR register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "Data output register"]
pub mod doutr;
#[doc = "KEYR0 register accessor: an alias for `Reg<KEYR0_SPEC>`"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
#[doc = "AES Key register 0"]
pub mod keyr0;
#[doc = "KEYR1 register accessor: an alias for `Reg<KEYR1_SPEC>`"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
#[doc = "AES Key register 1"]
pub mod keyr1;
#[doc = "KEYR2 register accessor: an alias for `Reg<KEYR2_SPEC>`"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
#[doc = "AES Key register 2"]
pub mod keyr2;
#[doc = "KEYR3 register accessor: an alias for `Reg<KEYR3_SPEC>`"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
#[doc = "AES Key register 3"]
pub mod keyr3;
#[doc = "IVR0 register accessor: an alias for `Reg<IVR0_SPEC>`"]
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
#[doc = "Initialization Vector Register 0"]
pub mod ivr0;
#[doc = "IVR1 register accessor: an alias for `Reg<IVR1_SPEC>`"]
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
#[doc = "Initialization Vector Register 1"]
pub mod ivr1;
#[doc = "IVR2 register accessor: an alias for `Reg<IVR2_SPEC>`"]
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
#[doc = "Initialization Vector Register 2"]
pub mod ivr2;
#[doc = "IVR3 register accessor: an alias for `Reg<IVR3_SPEC>`"]
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
#[doc = "Initialization Vector Register 3"]
pub mod ivr3;
