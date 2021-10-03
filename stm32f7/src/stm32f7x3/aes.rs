#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - data input register"]
    pub dinr: crate::Reg<dinr::DINR_SPEC>,
    #[doc = "0x0c - data output register"]
    pub doutr: crate::Reg<doutr::DOUTR_SPEC>,
    #[doc = "0x10 - key register 0"]
    pub keyr0: crate::Reg<keyr0::KEYR0_SPEC>,
    #[doc = "0x14 - key register 1"]
    pub keyr1: crate::Reg<keyr1::KEYR1_SPEC>,
    #[doc = "0x18 - key register 2"]
    pub keyr2: crate::Reg<keyr2::KEYR2_SPEC>,
    #[doc = "0x1c - key register 3"]
    pub keyr3: crate::Reg<keyr3::KEYR3_SPEC>,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: crate::Reg<ivr0::IVR0_SPEC>,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: crate::Reg<ivr1::IVR1_SPEC>,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: crate::Reg<ivr2::IVR2_SPEC>,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: crate::Reg<ivr3::IVR3_SPEC>,
    #[doc = "0x30 - key register 4"]
    pub keyr4: crate::Reg<keyr4::KEYR4_SPEC>,
    #[doc = "0x34 - key register 5"]
    pub keyr5: crate::Reg<keyr5::KEYR5_SPEC>,
    #[doc = "0x38 - key register 6"]
    pub keyr6: crate::Reg<keyr6::KEYR6_SPEC>,
    #[doc = "0x3c - key register 7"]
    pub keyr7: crate::Reg<keyr7::KEYR7_SPEC>,
    #[doc = "0x40 - AES suspend register 0"]
    pub susp0r: crate::Reg<susp0r::SUSP0R_SPEC>,
    #[doc = "0x44 - AES suspend register 1"]
    pub susp1r: crate::Reg<susp1r::SUSP1R_SPEC>,
    #[doc = "0x48 - AES suspend register 2"]
    pub susp2r: crate::Reg<susp2r::SUSP2R_SPEC>,
    #[doc = "0x4c - AES suspend register 3"]
    pub susp3r: crate::Reg<susp3r::SUSP3R_SPEC>,
    #[doc = "0x50 - AES suspend register 4"]
    pub susp4r: crate::Reg<susp4r::SUSP4R_SPEC>,
    #[doc = "0x54 - AES suspend register 5"]
    pub susp5r: crate::Reg<susp5r::SUSP5R_SPEC>,
    #[doc = "0x58 - AES suspend register 6"]
    pub susp6r: crate::Reg<susp6r::SUSP6R_SPEC>,
    #[doc = "0x5c - AES suspend register 7"]
    pub susp7r: crate::Reg<susp7r::SUSP7R_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DINR register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "data input register"]
pub mod dinr;
#[doc = "DOUTR register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "data output register"]
pub mod doutr;
#[doc = "KEYR0 register accessor: an alias for `Reg<KEYR0_SPEC>`"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "KEYR1 register accessor: an alias for `Reg<KEYR1_SPEC>`"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "KEYR2 register accessor: an alias for `Reg<KEYR2_SPEC>`"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "KEYR3 register accessor: an alias for `Reg<KEYR3_SPEC>`"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "IVR0 register accessor: an alias for `Reg<IVR0_SPEC>`"]
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 register accessor: an alias for `Reg<IVR1_SPEC>`"]
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 register accessor: an alias for `Reg<IVR2_SPEC>`"]
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 register accessor: an alias for `Reg<IVR3_SPEC>`"]
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
#[doc = "initialization vector register 3"]
pub mod ivr3;
#[doc = "KEYR4 register accessor: an alias for `Reg<KEYR4_SPEC>`"]
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
#[doc = "key register 4"]
pub mod keyr4;
#[doc = "KEYR5 register accessor: an alias for `Reg<KEYR5_SPEC>`"]
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
#[doc = "key register 5"]
pub mod keyr5;
#[doc = "KEYR6 register accessor: an alias for `Reg<KEYR6_SPEC>`"]
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
#[doc = "key register 6"]
pub mod keyr6;
#[doc = "KEYR7 register accessor: an alias for `Reg<KEYR7_SPEC>`"]
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
#[doc = "key register 7"]
pub mod keyr7;
#[doc = "SUSP0R register accessor: an alias for `Reg<SUSP0R_SPEC>`"]
pub type SUSP0R = crate::Reg<susp0r::SUSP0R_SPEC>;
#[doc = "AES suspend register 0"]
pub mod susp0r;
#[doc = "SUSP1R register accessor: an alias for `Reg<SUSP1R_SPEC>`"]
pub type SUSP1R = crate::Reg<susp1r::SUSP1R_SPEC>;
#[doc = "AES suspend register 1"]
pub mod susp1r;
#[doc = "SUSP2R register accessor: an alias for `Reg<SUSP2R_SPEC>`"]
pub type SUSP2R = crate::Reg<susp2r::SUSP2R_SPEC>;
#[doc = "AES suspend register 2"]
pub mod susp2r;
#[doc = "SUSP3R register accessor: an alias for `Reg<SUSP3R_SPEC>`"]
pub type SUSP3R = crate::Reg<susp3r::SUSP3R_SPEC>;
#[doc = "AES suspend register 3"]
pub mod susp3r;
#[doc = "SUSP4R register accessor: an alias for `Reg<SUSP4R_SPEC>`"]
pub type SUSP4R = crate::Reg<susp4r::SUSP4R_SPEC>;
#[doc = "AES suspend register 4"]
pub mod susp4r;
#[doc = "SUSP5R register accessor: an alias for `Reg<SUSP5R_SPEC>`"]
pub type SUSP5R = crate::Reg<susp5r::SUSP5R_SPEC>;
#[doc = "AES suspend register 5"]
pub mod susp5r;
#[doc = "SUSP6R register accessor: an alias for `Reg<SUSP6R_SPEC>`"]
pub type SUSP6R = crate::Reg<susp6r::SUSP6R_SPEC>;
#[doc = "AES suspend register 6"]
pub mod susp6r;
#[doc = "SUSP7R register accessor: an alias for `Reg<SUSP7R_SPEC>`"]
pub type SUSP7R = crate::Reg<susp7r::SUSP7R_SPEC>;
#[doc = "AES suspend register 7"]
pub mod susp7r;
