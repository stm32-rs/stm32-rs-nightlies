#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    itline0: ITLINE0,
    itline1: ITLINE1,
    itline2: ITLINE2,
    itline3: ITLINE3,
    itline4: ITLINE4,
    itline5: ITLINE5,
    itline6: ITLINE6,
    itline7: ITLINE7,
    _reserved8: [u8; 0x04],
    itline9: ITLINE9,
    itline10: ITLINE10,
    itline11: ITLINE11,
    itline12: ITLINE12,
    itline13: ITLINE13,
    itline14: ITLINE14,
    itline15: ITLINE15,
    itline16: ITLINE16,
    _reserved16: [u8; 0x08],
    itline19: ITLINE19,
    _reserved17: [u8; 0x04],
    itline21: ITLINE21,
    itline22: ITLINE22,
    itline23: ITLINE23,
    itline24: ITLINE24,
    itline25: ITLINE25,
    itline26: ITLINE26,
    itline27: ITLINE27,
    itline28: ITLINE28,
    itline29: ITLINE29,
}
impl RegisterBlock {
    #[doc = "0x80 - interrupt line 0 status register"]
    #[inline(always)]
    pub const fn itline0(&self) -> &ITLINE0 {
        &self.itline0
    }
    #[doc = "0x84 - interrupt line 1 status register"]
    #[inline(always)]
    pub const fn itline1(&self) -> &ITLINE1 {
        &self.itline1
    }
    #[doc = "0x88 - interrupt line 2 status register"]
    #[inline(always)]
    pub const fn itline2(&self) -> &ITLINE2 {
        &self.itline2
    }
    #[doc = "0x8c - interrupt line 3 status register"]
    #[inline(always)]
    pub const fn itline3(&self) -> &ITLINE3 {
        &self.itline3
    }
    #[doc = "0x90 - interrupt line 4 status register"]
    #[inline(always)]
    pub const fn itline4(&self) -> &ITLINE4 {
        &self.itline4
    }
    #[doc = "0x94 - interrupt line 5 status register"]
    #[inline(always)]
    pub const fn itline5(&self) -> &ITLINE5 {
        &self.itline5
    }
    #[doc = "0x98 - interrupt line 6 status register"]
    #[inline(always)]
    pub const fn itline6(&self) -> &ITLINE6 {
        &self.itline6
    }
    #[doc = "0x9c - interrupt line 7 status register"]
    #[inline(always)]
    pub const fn itline7(&self) -> &ITLINE7 {
        &self.itline7
    }
    #[doc = "0xa4 - interrupt line 9 status register"]
    #[inline(always)]
    pub const fn itline9(&self) -> &ITLINE9 {
        &self.itline9
    }
    #[doc = "0xa8 - interrupt line 10 status register"]
    #[inline(always)]
    pub const fn itline10(&self) -> &ITLINE10 {
        &self.itline10
    }
    #[doc = "0xac - interrupt line 11 status register"]
    #[inline(always)]
    pub const fn itline11(&self) -> &ITLINE11 {
        &self.itline11
    }
    #[doc = "0xb0 - interrupt line 12 status register"]
    #[inline(always)]
    pub const fn itline12(&self) -> &ITLINE12 {
        &self.itline12
    }
    #[doc = "0xb4 - interrupt line 13 status register"]
    #[inline(always)]
    pub const fn itline13(&self) -> &ITLINE13 {
        &self.itline13
    }
    #[doc = "0xb8 - interrupt line 14 status register"]
    #[inline(always)]
    pub const fn itline14(&self) -> &ITLINE14 {
        &self.itline14
    }
    #[doc = "0xbc - interrupt line 15 status register"]
    #[inline(always)]
    pub const fn itline15(&self) -> &ITLINE15 {
        &self.itline15
    }
    #[doc = "0xc0 - interrupt line 16 status register"]
    #[inline(always)]
    pub const fn itline16(&self) -> &ITLINE16 {
        &self.itline16
    }
    #[doc = "0xcc - interrupt line 19 status register"]
    #[inline(always)]
    pub const fn itline19(&self) -> &ITLINE19 {
        &self.itline19
    }
    #[doc = "0xd4 - interrupt line 21 status register"]
    #[inline(always)]
    pub const fn itline21(&self) -> &ITLINE21 {
        &self.itline21
    }
    #[doc = "0xd8 - interrupt line 22 status register"]
    #[inline(always)]
    pub const fn itline22(&self) -> &ITLINE22 {
        &self.itline22
    }
    #[doc = "0xdc - interrupt line 23 status register"]
    #[inline(always)]
    pub const fn itline23(&self) -> &ITLINE23 {
        &self.itline23
    }
    #[doc = "0xe0 - interrupt line 24 status register"]
    #[inline(always)]
    pub const fn itline24(&self) -> &ITLINE24 {
        &self.itline24
    }
    #[doc = "0xe4 - interrupt line 25 status register"]
    #[inline(always)]
    pub const fn itline25(&self) -> &ITLINE25 {
        &self.itline25
    }
    #[doc = "0xe8 - interrupt line 26 status register"]
    #[inline(always)]
    pub const fn itline26(&self) -> &ITLINE26 {
        &self.itline26
    }
    #[doc = "0xec - interrupt line 27 status register"]
    #[inline(always)]
    pub const fn itline27(&self) -> &ITLINE27 {
        &self.itline27
    }
    #[doc = "0xf0 - interrupt line 28 status register"]
    #[inline(always)]
    pub const fn itline28(&self) -> &ITLINE28 {
        &self.itline28
    }
    #[doc = "0xf4 - interrupt line 29 status register"]
    #[inline(always)]
    pub const fn itline29(&self) -> &ITLINE29 {
        &self.itline29
    }
}
#[doc = "ITLINE0 (r) register accessor: interrupt line 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline0`]
module"]
pub type ITLINE0 = crate::Reg<itline0::ITLINE0rs>;
#[doc = "interrupt line 0 status register"]
pub mod itline0;
#[doc = "ITLINE1 (r) register accessor: interrupt line 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline1`]
module"]
pub type ITLINE1 = crate::Reg<itline1::ITLINE1rs>;
#[doc = "interrupt line 1 status register"]
pub mod itline1;
#[doc = "ITLINE2 (r) register accessor: interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline2`]
module"]
pub type ITLINE2 = crate::Reg<itline2::ITLINE2rs>;
#[doc = "interrupt line 2 status register"]
pub mod itline2;
#[doc = "ITLINE3 (r) register accessor: interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline3`]
module"]
pub type ITLINE3 = crate::Reg<itline3::ITLINE3rs>;
#[doc = "interrupt line 3 status register"]
pub mod itline3;
#[doc = "ITLINE4 (r) register accessor: interrupt line 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline4`]
module"]
pub type ITLINE4 = crate::Reg<itline4::ITLINE4rs>;
#[doc = "interrupt line 4 status register"]
pub mod itline4;
#[doc = "ITLINE5 (r) register accessor: interrupt line 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline5`]
module"]
pub type ITLINE5 = crate::Reg<itline5::ITLINE5rs>;
#[doc = "interrupt line 5 status register"]
pub mod itline5;
#[doc = "ITLINE6 (r) register accessor: interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline6`]
module"]
pub type ITLINE6 = crate::Reg<itline6::ITLINE6rs>;
#[doc = "interrupt line 6 status register"]
pub mod itline6;
#[doc = "ITLINE7 (r) register accessor: interrupt line 7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline7`]
module"]
pub type ITLINE7 = crate::Reg<itline7::ITLINE7rs>;
#[doc = "interrupt line 7 status register"]
pub mod itline7;
#[doc = "ITLINE9 (r) register accessor: interrupt line 9 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline9`]
module"]
pub type ITLINE9 = crate::Reg<itline9::ITLINE9rs>;
#[doc = "interrupt line 9 status register"]
pub mod itline9;
#[doc = "ITLINE10 (r) register accessor: interrupt line 10 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline10`]
module"]
pub type ITLINE10 = crate::Reg<itline10::ITLINE10rs>;
#[doc = "interrupt line 10 status register"]
pub mod itline10;
#[doc = "ITLINE11 (r) register accessor: interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline11`]
module"]
pub type ITLINE11 = crate::Reg<itline11::ITLINE11rs>;
#[doc = "interrupt line 11 status register"]
pub mod itline11;
#[doc = "ITLINE12 (r) register accessor: interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline12`]
module"]
pub type ITLINE12 = crate::Reg<itline12::ITLINE12rs>;
#[doc = "interrupt line 12 status register"]
pub mod itline12;
#[doc = "ITLINE13 (r) register accessor: interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline13`]
module"]
pub type ITLINE13 = crate::Reg<itline13::ITLINE13rs>;
#[doc = "interrupt line 13 status register"]
pub mod itline13;
#[doc = "ITLINE14 (r) register accessor: interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline14`]
module"]
pub type ITLINE14 = crate::Reg<itline14::ITLINE14rs>;
#[doc = "interrupt line 14 status register"]
pub mod itline14;
#[doc = "ITLINE15 (r) register accessor: interrupt line 15 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline15`]
module"]
pub type ITLINE15 = crate::Reg<itline15::ITLINE15rs>;
#[doc = "interrupt line 15 status register"]
pub mod itline15;
#[doc = "ITLINE16 (r) register accessor: interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline16`]
module"]
pub type ITLINE16 = crate::Reg<itline16::ITLINE16rs>;
#[doc = "interrupt line 16 status register"]
pub mod itline16;
#[doc = "ITLINE19 (r) register accessor: interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline19`]
module"]
pub type ITLINE19 = crate::Reg<itline19::ITLINE19rs>;
#[doc = "interrupt line 19 status register"]
pub mod itline19;
#[doc = "ITLINE21 (r) register accessor: interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline21`]
module"]
pub type ITLINE21 = crate::Reg<itline21::ITLINE21rs>;
#[doc = "interrupt line 21 status register"]
pub mod itline21;
#[doc = "ITLINE22 (r) register accessor: interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline22`]
module"]
pub type ITLINE22 = crate::Reg<itline22::ITLINE22rs>;
#[doc = "interrupt line 22 status register"]
pub mod itline22;
#[doc = "ITLINE23 (r) register accessor: interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline23`]
module"]
pub type ITLINE23 = crate::Reg<itline23::ITLINE23rs>;
#[doc = "interrupt line 23 status register"]
pub mod itline23;
#[doc = "ITLINE24 (r) register accessor: interrupt line 24 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline24`]
module"]
pub type ITLINE24 = crate::Reg<itline24::ITLINE24rs>;
#[doc = "interrupt line 24 status register"]
pub mod itline24;
#[doc = "ITLINE25 (r) register accessor: interrupt line 25 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline25`]
module"]
pub type ITLINE25 = crate::Reg<itline25::ITLINE25rs>;
#[doc = "interrupt line 25 status register"]
pub mod itline25;
#[doc = "ITLINE26 (r) register accessor: interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline26`]
module"]
pub type ITLINE26 = crate::Reg<itline26::ITLINE26rs>;
#[doc = "interrupt line 26 status register"]
pub mod itline26;
#[doc = "ITLINE27 (r) register accessor: interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline27`]
module"]
pub type ITLINE27 = crate::Reg<itline27::ITLINE27rs>;
#[doc = "interrupt line 27 status register"]
pub mod itline27;
#[doc = "ITLINE28 (r) register accessor: interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline28`]
module"]
pub type ITLINE28 = crate::Reg<itline28::ITLINE28rs>;
#[doc = "interrupt line 28 status register"]
pub mod itline28;
#[doc = "ITLINE29 (r) register accessor: interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itline29`]
module"]
pub type ITLINE29 = crate::Reg<itline29::ITLINE29rs>;
#[doc = "interrupt line 29 status register"]
pub mod itline29;
