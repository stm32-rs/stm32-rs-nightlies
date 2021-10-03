#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - interrupt line 0 status register"]
    pub itline0: crate::Reg<itline0::ITLINE0_SPEC>,
    #[doc = "0x84 - interrupt line 1 status register"]
    pub itline1: crate::Reg<itline1::ITLINE1_SPEC>,
    #[doc = "0x88 - interrupt line 2 status register"]
    pub itline2: crate::Reg<itline2::ITLINE2_SPEC>,
    #[doc = "0x8c - interrupt line 3 status register"]
    pub itline3: crate::Reg<itline3::ITLINE3_SPEC>,
    #[doc = "0x90 - interrupt line 4 status register"]
    pub itline4: crate::Reg<itline4::ITLINE4_SPEC>,
    #[doc = "0x94 - interrupt line 5 status register"]
    pub itline5: crate::Reg<itline5::ITLINE5_SPEC>,
    #[doc = "0x98 - interrupt line 6 status register"]
    pub itline6: crate::Reg<itline6::ITLINE6_SPEC>,
    #[doc = "0x9c - interrupt line 7 status register"]
    pub itline7: crate::Reg<itline7::ITLINE7_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0xa4 - interrupt line 9 status register"]
    pub itline9: crate::Reg<itline9::ITLINE9_SPEC>,
    #[doc = "0xa8 - interrupt line 10 status register"]
    pub itline10: crate::Reg<itline10::ITLINE10_SPEC>,
    #[doc = "0xac - interrupt line 11 status register"]
    pub itline11: crate::Reg<itline11::ITLINE11_SPEC>,
    #[doc = "0xb0 - interrupt line 12 status register"]
    pub itline12: crate::Reg<itline12::ITLINE12_SPEC>,
    #[doc = "0xb4 - interrupt line 13 status register"]
    pub itline13: crate::Reg<itline13::ITLINE13_SPEC>,
    #[doc = "0xb8 - interrupt line 14 status register"]
    pub itline14: crate::Reg<itline14::ITLINE14_SPEC>,
    #[doc = "0xbc - interrupt line 15 status register"]
    pub itline15: crate::Reg<itline15::ITLINE15_SPEC>,
    #[doc = "0xc0 - interrupt line 16 status register"]
    pub itline16: crate::Reg<itline16::ITLINE16_SPEC>,
    #[doc = "0xc4 - interrupt line 17 status register"]
    pub itline17: crate::Reg<itline17::ITLINE17_SPEC>,
    #[doc = "0xc8 - interrupt line 18 status register"]
    pub itline18: crate::Reg<itline18::ITLINE18_SPEC>,
    #[doc = "0xcc - interrupt line 19 status register"]
    pub itline19: crate::Reg<itline19::ITLINE19_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0xd4 - interrupt line 21 status register"]
    pub itline21: crate::Reg<itline21::ITLINE21_SPEC>,
    #[doc = "0xd8 - interrupt line 22 status register"]
    pub itline22: crate::Reg<itline22::ITLINE22_SPEC>,
    #[doc = "0xdc - interrupt line 23 status register"]
    pub itline23: crate::Reg<itline23::ITLINE23_SPEC>,
    #[doc = "0xe0 - interrupt line 24 status register"]
    pub itline24: crate::Reg<itline24::ITLINE24_SPEC>,
    #[doc = "0xe4 - interrupt line 25 status register"]
    pub itline25: crate::Reg<itline25::ITLINE25_SPEC>,
    #[doc = "0xe8 - interrupt line 26 status register"]
    pub itline26: crate::Reg<itline26::ITLINE26_SPEC>,
    #[doc = "0xec - interrupt line 27 status register"]
    pub itline27: crate::Reg<itline27::ITLINE27_SPEC>,
    #[doc = "0xf0 - interrupt line 28 status register"]
    pub itline28: crate::Reg<itline28::ITLINE28_SPEC>,
    #[doc = "0xf4 - interrupt line 29 status register"]
    pub itline29: crate::Reg<itline29::ITLINE29_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0xfc - interrupt line 31 status register"]
    pub itline31: crate::Reg<itline31::ITLINE31_SPEC>,
}
#[doc = "ITLINE0 register accessor: an alias for `Reg<ITLINE0_SPEC>`"]
pub type ITLINE0 = crate::Reg<itline0::ITLINE0_SPEC>;
#[doc = "interrupt line 0 status register"]
pub mod itline0;
#[doc = "ITLINE1 register accessor: an alias for `Reg<ITLINE1_SPEC>`"]
pub type ITLINE1 = crate::Reg<itline1::ITLINE1_SPEC>;
#[doc = "interrupt line 1 status register"]
pub mod itline1;
#[doc = "ITLINE2 register accessor: an alias for `Reg<ITLINE2_SPEC>`"]
pub type ITLINE2 = crate::Reg<itline2::ITLINE2_SPEC>;
#[doc = "interrupt line 2 status register"]
pub mod itline2;
#[doc = "ITLINE3 register accessor: an alias for `Reg<ITLINE3_SPEC>`"]
pub type ITLINE3 = crate::Reg<itline3::ITLINE3_SPEC>;
#[doc = "interrupt line 3 status register"]
pub mod itline3;
#[doc = "ITLINE4 register accessor: an alias for `Reg<ITLINE4_SPEC>`"]
pub type ITLINE4 = crate::Reg<itline4::ITLINE4_SPEC>;
#[doc = "interrupt line 4 status register"]
pub mod itline4;
#[doc = "ITLINE5 register accessor: an alias for `Reg<ITLINE5_SPEC>`"]
pub type ITLINE5 = crate::Reg<itline5::ITLINE5_SPEC>;
#[doc = "interrupt line 5 status register"]
pub mod itline5;
#[doc = "ITLINE6 register accessor: an alias for `Reg<ITLINE6_SPEC>`"]
pub type ITLINE6 = crate::Reg<itline6::ITLINE6_SPEC>;
#[doc = "interrupt line 6 status register"]
pub mod itline6;
#[doc = "ITLINE7 register accessor: an alias for `Reg<ITLINE7_SPEC>`"]
pub type ITLINE7 = crate::Reg<itline7::ITLINE7_SPEC>;
#[doc = "interrupt line 7 status register"]
pub mod itline7;
#[doc = "ITLINE9 register accessor: an alias for `Reg<ITLINE9_SPEC>`"]
pub type ITLINE9 = crate::Reg<itline9::ITLINE9_SPEC>;
#[doc = "interrupt line 9 status register"]
pub mod itline9;
#[doc = "ITLINE10 register accessor: an alias for `Reg<ITLINE10_SPEC>`"]
pub type ITLINE10 = crate::Reg<itline10::ITLINE10_SPEC>;
#[doc = "interrupt line 10 status register"]
pub mod itline10;
#[doc = "ITLINE11 register accessor: an alias for `Reg<ITLINE11_SPEC>`"]
pub type ITLINE11 = crate::Reg<itline11::ITLINE11_SPEC>;
#[doc = "interrupt line 11 status register"]
pub mod itline11;
#[doc = "ITLINE12 register accessor: an alias for `Reg<ITLINE12_SPEC>`"]
pub type ITLINE12 = crate::Reg<itline12::ITLINE12_SPEC>;
#[doc = "interrupt line 12 status register"]
pub mod itline12;
#[doc = "ITLINE13 register accessor: an alias for `Reg<ITLINE13_SPEC>`"]
pub type ITLINE13 = crate::Reg<itline13::ITLINE13_SPEC>;
#[doc = "interrupt line 13 status register"]
pub mod itline13;
#[doc = "ITLINE14 register accessor: an alias for `Reg<ITLINE14_SPEC>`"]
pub type ITLINE14 = crate::Reg<itline14::ITLINE14_SPEC>;
#[doc = "interrupt line 14 status register"]
pub mod itline14;
#[doc = "ITLINE15 register accessor: an alias for `Reg<ITLINE15_SPEC>`"]
pub type ITLINE15 = crate::Reg<itline15::ITLINE15_SPEC>;
#[doc = "interrupt line 15 status register"]
pub mod itline15;
#[doc = "ITLINE16 register accessor: an alias for `Reg<ITLINE16_SPEC>`"]
pub type ITLINE16 = crate::Reg<itline16::ITLINE16_SPEC>;
#[doc = "interrupt line 16 status register"]
pub mod itline16;
#[doc = "ITLINE17 register accessor: an alias for `Reg<ITLINE17_SPEC>`"]
pub type ITLINE17 = crate::Reg<itline17::ITLINE17_SPEC>;
#[doc = "interrupt line 17 status register"]
pub mod itline17;
#[doc = "ITLINE18 register accessor: an alias for `Reg<ITLINE18_SPEC>`"]
pub type ITLINE18 = crate::Reg<itline18::ITLINE18_SPEC>;
#[doc = "interrupt line 18 status register"]
pub mod itline18;
#[doc = "ITLINE19 register accessor: an alias for `Reg<ITLINE19_SPEC>`"]
pub type ITLINE19 = crate::Reg<itline19::ITLINE19_SPEC>;
#[doc = "interrupt line 19 status register"]
pub mod itline19;
#[doc = "ITLINE21 register accessor: an alias for `Reg<ITLINE21_SPEC>`"]
pub type ITLINE21 = crate::Reg<itline21::ITLINE21_SPEC>;
#[doc = "interrupt line 21 status register"]
pub mod itline21;
#[doc = "ITLINE22 register accessor: an alias for `Reg<ITLINE22_SPEC>`"]
pub type ITLINE22 = crate::Reg<itline22::ITLINE22_SPEC>;
#[doc = "interrupt line 22 status register"]
pub mod itline22;
#[doc = "ITLINE23 register accessor: an alias for `Reg<ITLINE23_SPEC>`"]
pub type ITLINE23 = crate::Reg<itline23::ITLINE23_SPEC>;
#[doc = "interrupt line 23 status register"]
pub mod itline23;
#[doc = "ITLINE24 register accessor: an alias for `Reg<ITLINE24_SPEC>`"]
pub type ITLINE24 = crate::Reg<itline24::ITLINE24_SPEC>;
#[doc = "interrupt line 24 status register"]
pub mod itline24;
#[doc = "ITLINE25 register accessor: an alias for `Reg<ITLINE25_SPEC>`"]
pub type ITLINE25 = crate::Reg<itline25::ITLINE25_SPEC>;
#[doc = "interrupt line 25 status register"]
pub mod itline25;
#[doc = "ITLINE26 register accessor: an alias for `Reg<ITLINE26_SPEC>`"]
pub type ITLINE26 = crate::Reg<itline26::ITLINE26_SPEC>;
#[doc = "interrupt line 26 status register"]
pub mod itline26;
#[doc = "ITLINE27 register accessor: an alias for `Reg<ITLINE27_SPEC>`"]
pub type ITLINE27 = crate::Reg<itline27::ITLINE27_SPEC>;
#[doc = "interrupt line 27 status register"]
pub mod itline27;
#[doc = "ITLINE28 register accessor: an alias for `Reg<ITLINE28_SPEC>`"]
pub type ITLINE28 = crate::Reg<itline28::ITLINE28_SPEC>;
#[doc = "interrupt line 28 status register"]
pub mod itline28;
#[doc = "ITLINE29 register accessor: an alias for `Reg<ITLINE29_SPEC>`"]
pub type ITLINE29 = crate::Reg<itline29::ITLINE29_SPEC>;
#[doc = "interrupt line 29 status register"]
pub mod itline29;
#[doc = "ITLINE31 register accessor: an alias for `Reg<ITLINE31_SPEC>`"]
pub type ITLINE31 = crate::Reg<itline31::ITLINE31_SPEC>;
#[doc = "interrupt line 31 status register"]
pub mod itline31;
