#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    wrfr: WRFR,
    cwrfr: CWRFR,
    rdfr: RDFR,
    crdfr: CRDFR,
    sr: SR,
    clrfr: CLRFR,
    dinr0: DINR0,
    dinr1: DINR1,
    dinr2: DINR2,
    dinr3: DINR3,
    dinr4: DINR4,
    dinr5: DINR5,
    dinr6: DINR6,
    dinr7: DINR7,
    dinr8: DINR8,
    dinr9: DINR9,
    dinr10: DINR10,
    dinr11: DINR11,
    dinr12: DINR12,
    dinr13: DINR13,
    dinr14: DINR14,
    dinr15: DINR15,
    dinr16: DINR16,
    dinr17: DINR17,
    dinr18: DINR18,
    dinr19: DINR19,
    dinr20: DINR20,
    dinr21: DINR21,
    dinr22: DINR22,
    dinr23: DINR23,
    dinr24: DINR24,
    dinr25: DINR25,
    dinr26: DINR26,
    dinr27: DINR27,
    dinr28: DINR28,
    dinr29: DINR29,
    dinr30: DINR30,
    dinr31: DINR31,
    doutr0: DOUTR0,
    doutr1: DOUTR1,
    doutr2: DOUTR2,
    doutr3: DOUTR3,
    doutr4: DOUTR4,
    doutr5: DOUTR5,
    doutr6: DOUTR6,
    doutr7: DOUTR7,
    doutr8: DOUTR8,
    doutr9: DOUTR9,
    doutr10: DOUTR10,
    doutr11: DOUTR11,
    doutr12: DOUTR12,
    doutr13: DOUTR13,
    doutr14: DOUTR14,
    doutr15: DOUTR15,
    doutr16: DOUTR16,
    doutr17: DOUTR17,
    doutr18: DOUTR18,
    doutr19: DOUTR19,
    doutr20: DOUTR20,
    doutr21: DOUTR21,
    doutr22: DOUTR22,
    doutr23: DOUTR23,
    doutr24: DOUTR24,
    doutr25: DOUTR25,
    doutr26: DOUTR26,
    doutr27: DOUTR27,
    doutr28: DOUTR28,
    doutr29: DOUTR29,
    doutr30: DOUTR30,
    doutr31: DOUTR31,
}
impl RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - MDIOS write flag register"]
    #[inline(always)]
    pub const fn wrfr(&self) -> &WRFR {
        &self.wrfr
    }
    #[doc = "0x08 - MDIOS clear write flag register"]
    #[inline(always)]
    pub const fn cwrfr(&self) -> &CWRFR {
        &self.cwrfr
    }
    #[doc = "0x0c - MDIOS read flag register"]
    #[inline(always)]
    pub const fn rdfr(&self) -> &RDFR {
        &self.rdfr
    }
    #[doc = "0x10 - MDIOS clear read flag register"]
    #[inline(always)]
    pub const fn crdfr(&self) -> &CRDFR {
        &self.crdfr
    }
    #[doc = "0x14 - MDIOS status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - MDIOS clear flag register"]
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    #[doc = "0x1c - MDIOS input data register 0"]
    #[inline(always)]
    pub const fn dinr0(&self) -> &DINR0 {
        &self.dinr0
    }
    #[doc = "0x20 - MDIOS input data register 1"]
    #[inline(always)]
    pub const fn dinr1(&self) -> &DINR1 {
        &self.dinr1
    }
    #[doc = "0x24 - MDIOS input data register 2"]
    #[inline(always)]
    pub const fn dinr2(&self) -> &DINR2 {
        &self.dinr2
    }
    #[doc = "0x28 - MDIOS input data register 3"]
    #[inline(always)]
    pub const fn dinr3(&self) -> &DINR3 {
        &self.dinr3
    }
    #[doc = "0x2c - MDIOS input data register 4"]
    #[inline(always)]
    pub const fn dinr4(&self) -> &DINR4 {
        &self.dinr4
    }
    #[doc = "0x30 - MDIOS input data register 5"]
    #[inline(always)]
    pub const fn dinr5(&self) -> &DINR5 {
        &self.dinr5
    }
    #[doc = "0x34 - MDIOS input data register 6"]
    #[inline(always)]
    pub const fn dinr6(&self) -> &DINR6 {
        &self.dinr6
    }
    #[doc = "0x38 - MDIOS input data register 7"]
    #[inline(always)]
    pub const fn dinr7(&self) -> &DINR7 {
        &self.dinr7
    }
    #[doc = "0x3c - MDIOS input data register 8"]
    #[inline(always)]
    pub const fn dinr8(&self) -> &DINR8 {
        &self.dinr8
    }
    #[doc = "0x40 - MDIOS input data register 9"]
    #[inline(always)]
    pub const fn dinr9(&self) -> &DINR9 {
        &self.dinr9
    }
    #[doc = "0x44 - MDIOS input data register 10"]
    #[inline(always)]
    pub const fn dinr10(&self) -> &DINR10 {
        &self.dinr10
    }
    #[doc = "0x48 - MDIOS input data register 11"]
    #[inline(always)]
    pub const fn dinr11(&self) -> &DINR11 {
        &self.dinr11
    }
    #[doc = "0x4c - MDIOS input data register 12"]
    #[inline(always)]
    pub const fn dinr12(&self) -> &DINR12 {
        &self.dinr12
    }
    #[doc = "0x50 - MDIOS input data register 13"]
    #[inline(always)]
    pub const fn dinr13(&self) -> &DINR13 {
        &self.dinr13
    }
    #[doc = "0x54 - MDIOS input data register 14"]
    #[inline(always)]
    pub const fn dinr14(&self) -> &DINR14 {
        &self.dinr14
    }
    #[doc = "0x58 - MDIOS input data register 15"]
    #[inline(always)]
    pub const fn dinr15(&self) -> &DINR15 {
        &self.dinr15
    }
    #[doc = "0x5c - MDIOS input data register 16"]
    #[inline(always)]
    pub const fn dinr16(&self) -> &DINR16 {
        &self.dinr16
    }
    #[doc = "0x60 - MDIOS input data register 17"]
    #[inline(always)]
    pub const fn dinr17(&self) -> &DINR17 {
        &self.dinr17
    }
    #[doc = "0x64 - MDIOS input data register 18"]
    #[inline(always)]
    pub const fn dinr18(&self) -> &DINR18 {
        &self.dinr18
    }
    #[doc = "0x68 - MDIOS input data register 19"]
    #[inline(always)]
    pub const fn dinr19(&self) -> &DINR19 {
        &self.dinr19
    }
    #[doc = "0x6c - MDIOS input data register 20"]
    #[inline(always)]
    pub const fn dinr20(&self) -> &DINR20 {
        &self.dinr20
    }
    #[doc = "0x70 - MDIOS input data register 21"]
    #[inline(always)]
    pub const fn dinr21(&self) -> &DINR21 {
        &self.dinr21
    }
    #[doc = "0x74 - MDIOS input data register 22"]
    #[inline(always)]
    pub const fn dinr22(&self) -> &DINR22 {
        &self.dinr22
    }
    #[doc = "0x78 - MDIOS input data register 23"]
    #[inline(always)]
    pub const fn dinr23(&self) -> &DINR23 {
        &self.dinr23
    }
    #[doc = "0x7c - MDIOS input data register 24"]
    #[inline(always)]
    pub const fn dinr24(&self) -> &DINR24 {
        &self.dinr24
    }
    #[doc = "0x80 - MDIOS input data register 25"]
    #[inline(always)]
    pub const fn dinr25(&self) -> &DINR25 {
        &self.dinr25
    }
    #[doc = "0x84 - MDIOS input data register 26"]
    #[inline(always)]
    pub const fn dinr26(&self) -> &DINR26 {
        &self.dinr26
    }
    #[doc = "0x88 - MDIOS input data register 27"]
    #[inline(always)]
    pub const fn dinr27(&self) -> &DINR27 {
        &self.dinr27
    }
    #[doc = "0x8c - MDIOS input data register 28"]
    #[inline(always)]
    pub const fn dinr28(&self) -> &DINR28 {
        &self.dinr28
    }
    #[doc = "0x90 - MDIOS input data register 29"]
    #[inline(always)]
    pub const fn dinr29(&self) -> &DINR29 {
        &self.dinr29
    }
    #[doc = "0x94 - MDIOS input data register 30"]
    #[inline(always)]
    pub const fn dinr30(&self) -> &DINR30 {
        &self.dinr30
    }
    #[doc = "0x98 - MDIOS input data register 31"]
    #[inline(always)]
    pub const fn dinr31(&self) -> &DINR31 {
        &self.dinr31
    }
    #[doc = "0x9c - MDIOS output data register 0"]
    #[inline(always)]
    pub const fn doutr0(&self) -> &DOUTR0 {
        &self.doutr0
    }
    #[doc = "0xa0 - MDIOS output data register 1"]
    #[inline(always)]
    pub const fn doutr1(&self) -> &DOUTR1 {
        &self.doutr1
    }
    #[doc = "0xa4 - MDIOS output data register 2"]
    #[inline(always)]
    pub const fn doutr2(&self) -> &DOUTR2 {
        &self.doutr2
    }
    #[doc = "0xa8 - MDIOS output data register 3"]
    #[inline(always)]
    pub const fn doutr3(&self) -> &DOUTR3 {
        &self.doutr3
    }
    #[doc = "0xac - MDIOS output data register 4"]
    #[inline(always)]
    pub const fn doutr4(&self) -> &DOUTR4 {
        &self.doutr4
    }
    #[doc = "0xb0 - MDIOS output data register 5"]
    #[inline(always)]
    pub const fn doutr5(&self) -> &DOUTR5 {
        &self.doutr5
    }
    #[doc = "0xb4 - MDIOS output data register 6"]
    #[inline(always)]
    pub const fn doutr6(&self) -> &DOUTR6 {
        &self.doutr6
    }
    #[doc = "0xb8 - MDIOS output data register 7"]
    #[inline(always)]
    pub const fn doutr7(&self) -> &DOUTR7 {
        &self.doutr7
    }
    #[doc = "0xbc - MDIOS output data register 8"]
    #[inline(always)]
    pub const fn doutr8(&self) -> &DOUTR8 {
        &self.doutr8
    }
    #[doc = "0xc0 - MDIOS output data register 9"]
    #[inline(always)]
    pub const fn doutr9(&self) -> &DOUTR9 {
        &self.doutr9
    }
    #[doc = "0xc4 - MDIOS output data register 10"]
    #[inline(always)]
    pub const fn doutr10(&self) -> &DOUTR10 {
        &self.doutr10
    }
    #[doc = "0xc8 - MDIOS output data register 11"]
    #[inline(always)]
    pub const fn doutr11(&self) -> &DOUTR11 {
        &self.doutr11
    }
    #[doc = "0xcc - MDIOS output data register 12"]
    #[inline(always)]
    pub const fn doutr12(&self) -> &DOUTR12 {
        &self.doutr12
    }
    #[doc = "0xd0 - MDIOS output data register 13"]
    #[inline(always)]
    pub const fn doutr13(&self) -> &DOUTR13 {
        &self.doutr13
    }
    #[doc = "0xd4 - MDIOS output data register 14"]
    #[inline(always)]
    pub const fn doutr14(&self) -> &DOUTR14 {
        &self.doutr14
    }
    #[doc = "0xd8 - MDIOS output data register 15"]
    #[inline(always)]
    pub const fn doutr15(&self) -> &DOUTR15 {
        &self.doutr15
    }
    #[doc = "0xdc - MDIOS output data register 16"]
    #[inline(always)]
    pub const fn doutr16(&self) -> &DOUTR16 {
        &self.doutr16
    }
    #[doc = "0xe0 - MDIOS output data register 17"]
    #[inline(always)]
    pub const fn doutr17(&self) -> &DOUTR17 {
        &self.doutr17
    }
    #[doc = "0xe4 - MDIOS output data register 18"]
    #[inline(always)]
    pub const fn doutr18(&self) -> &DOUTR18 {
        &self.doutr18
    }
    #[doc = "0xe8 - MDIOS output data register 19"]
    #[inline(always)]
    pub const fn doutr19(&self) -> &DOUTR19 {
        &self.doutr19
    }
    #[doc = "0xec - MDIOS output data register 20"]
    #[inline(always)]
    pub const fn doutr20(&self) -> &DOUTR20 {
        &self.doutr20
    }
    #[doc = "0xf0 - MDIOS output data register 21"]
    #[inline(always)]
    pub const fn doutr21(&self) -> &DOUTR21 {
        &self.doutr21
    }
    #[doc = "0xf4 - MDIOS output data register 22"]
    #[inline(always)]
    pub const fn doutr22(&self) -> &DOUTR22 {
        &self.doutr22
    }
    #[doc = "0xf8 - MDIOS output data register 23"]
    #[inline(always)]
    pub const fn doutr23(&self) -> &DOUTR23 {
        &self.doutr23
    }
    #[doc = "0xfc - MDIOS output data register 24"]
    #[inline(always)]
    pub const fn doutr24(&self) -> &DOUTR24 {
        &self.doutr24
    }
    #[doc = "0x100 - MDIOS output data register 25"]
    #[inline(always)]
    pub const fn doutr25(&self) -> &DOUTR25 {
        &self.doutr25
    }
    #[doc = "0x104 - MDIOS output data register 26"]
    #[inline(always)]
    pub const fn doutr26(&self) -> &DOUTR26 {
        &self.doutr26
    }
    #[doc = "0x108 - MDIOS output data register 27"]
    #[inline(always)]
    pub const fn doutr27(&self) -> &DOUTR27 {
        &self.doutr27
    }
    #[doc = "0x10c - MDIOS output data register 28"]
    #[inline(always)]
    pub const fn doutr28(&self) -> &DOUTR28 {
        &self.doutr28
    }
    #[doc = "0x110 - MDIOS output data register 29"]
    #[inline(always)]
    pub const fn doutr29(&self) -> &DOUTR29 {
        &self.doutr29
    }
    #[doc = "0x114 - MDIOS output data register 30"]
    #[inline(always)]
    pub const fn doutr30(&self) -> &DOUTR30 {
        &self.doutr30
    }
    #[doc = "0x118 - MDIOS output data register 31"]
    #[inline(always)]
    pub const fn doutr31(&self) -> &DOUTR31 {
        &self.doutr31
    }
}
#[doc = "CR (rw) register accessor: MDIOS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "MDIOS configuration register"]
pub mod cr;
#[doc = "WRFR (r) register accessor: MDIOS write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrfr`]
module"]
pub type WRFR = crate::Reg<wrfr::WRFRrs>;
#[doc = "MDIOS write flag register"]
pub mod wrfr;
#[doc = "CWRFR (rw) register accessor: MDIOS clear write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwrfr`]
module"]
pub type CWRFR = crate::Reg<cwrfr::CWRFRrs>;
#[doc = "MDIOS clear write flag register"]
pub mod cwrfr;
#[doc = "RDFR (r) register accessor: MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdfr`]
module"]
pub type RDFR = crate::Reg<rdfr::RDFRrs>;
#[doc = "MDIOS read flag register"]
pub mod rdfr;
#[doc = "CRDFR (rw) register accessor: MDIOS clear read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crdfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crdfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crdfr`]
module"]
pub type CRDFR = crate::Reg<crdfr::CRDFRrs>;
#[doc = "MDIOS clear read flag register"]
pub mod crdfr;
#[doc = "SR (r) register accessor: MDIOS status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "MDIOS status register"]
pub mod sr;
#[doc = "CLRFR (rw) register accessor: MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrfr`]
module"]
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
#[doc = "MDIOS clear flag register"]
pub mod clrfr;
#[doc = "DINR0 (r) register accessor: MDIOS input data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr0`]
module"]
pub type DINR0 = crate::Reg<dinr0::DINR0rs>;
#[doc = "MDIOS input data register 0"]
pub mod dinr0;
#[doc = "DINR1 (r) register accessor: MDIOS input data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr1`]
module"]
pub type DINR1 = crate::Reg<dinr1::DINR1rs>;
#[doc = "MDIOS input data register 1"]
pub mod dinr1;
#[doc = "DINR2 (r) register accessor: MDIOS input data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr2`]
module"]
pub type DINR2 = crate::Reg<dinr2::DINR2rs>;
#[doc = "MDIOS input data register 2"]
pub mod dinr2;
#[doc = "DINR3 (r) register accessor: MDIOS input data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr3`]
module"]
pub type DINR3 = crate::Reg<dinr3::DINR3rs>;
#[doc = "MDIOS input data register 3"]
pub mod dinr3;
#[doc = "DINR4 (r) register accessor: MDIOS input data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr4`]
module"]
pub type DINR4 = crate::Reg<dinr4::DINR4rs>;
#[doc = "MDIOS input data register 4"]
pub mod dinr4;
#[doc = "DINR5 (r) register accessor: MDIOS input data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr5`]
module"]
pub type DINR5 = crate::Reg<dinr5::DINR5rs>;
#[doc = "MDIOS input data register 5"]
pub mod dinr5;
#[doc = "DINR6 (r) register accessor: MDIOS input data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr6`]
module"]
pub type DINR6 = crate::Reg<dinr6::DINR6rs>;
#[doc = "MDIOS input data register 6"]
pub mod dinr6;
#[doc = "DINR7 (r) register accessor: MDIOS input data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr7`]
module"]
pub type DINR7 = crate::Reg<dinr7::DINR7rs>;
#[doc = "MDIOS input data register 7"]
pub mod dinr7;
#[doc = "DINR8 (r) register accessor: MDIOS input data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr8`]
module"]
pub type DINR8 = crate::Reg<dinr8::DINR8rs>;
#[doc = "MDIOS input data register 8"]
pub mod dinr8;
#[doc = "DINR9 (r) register accessor: MDIOS input data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr9`]
module"]
pub type DINR9 = crate::Reg<dinr9::DINR9rs>;
#[doc = "MDIOS input data register 9"]
pub mod dinr9;
#[doc = "DINR10 (r) register accessor: MDIOS input data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr10`]
module"]
pub type DINR10 = crate::Reg<dinr10::DINR10rs>;
#[doc = "MDIOS input data register 10"]
pub mod dinr10;
#[doc = "DINR11 (r) register accessor: MDIOS input data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr11`]
module"]
pub type DINR11 = crate::Reg<dinr11::DINR11rs>;
#[doc = "MDIOS input data register 11"]
pub mod dinr11;
#[doc = "DINR12 (r) register accessor: MDIOS input data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr12`]
module"]
pub type DINR12 = crate::Reg<dinr12::DINR12rs>;
#[doc = "MDIOS input data register 12"]
pub mod dinr12;
#[doc = "DINR13 (r) register accessor: MDIOS input data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr13`]
module"]
pub type DINR13 = crate::Reg<dinr13::DINR13rs>;
#[doc = "MDIOS input data register 13"]
pub mod dinr13;
#[doc = "DINR14 (r) register accessor: MDIOS input data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr14`]
module"]
pub type DINR14 = crate::Reg<dinr14::DINR14rs>;
#[doc = "MDIOS input data register 14"]
pub mod dinr14;
#[doc = "DINR15 (r) register accessor: MDIOS input data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr15`]
module"]
pub type DINR15 = crate::Reg<dinr15::DINR15rs>;
#[doc = "MDIOS input data register 15"]
pub mod dinr15;
#[doc = "DINR16 (r) register accessor: MDIOS input data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr16`]
module"]
pub type DINR16 = crate::Reg<dinr16::DINR16rs>;
#[doc = "MDIOS input data register 16"]
pub mod dinr16;
#[doc = "DINR17 (r) register accessor: MDIOS input data register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr17`]
module"]
pub type DINR17 = crate::Reg<dinr17::DINR17rs>;
#[doc = "MDIOS input data register 17"]
pub mod dinr17;
#[doc = "DINR18 (r) register accessor: MDIOS input data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr18`]
module"]
pub type DINR18 = crate::Reg<dinr18::DINR18rs>;
#[doc = "MDIOS input data register 18"]
pub mod dinr18;
#[doc = "DINR19 (r) register accessor: MDIOS input data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr19`]
module"]
pub type DINR19 = crate::Reg<dinr19::DINR19rs>;
#[doc = "MDIOS input data register 19"]
pub mod dinr19;
#[doc = "DINR20 (r) register accessor: MDIOS input data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr20`]
module"]
pub type DINR20 = crate::Reg<dinr20::DINR20rs>;
#[doc = "MDIOS input data register 20"]
pub mod dinr20;
#[doc = "DINR21 (r) register accessor: MDIOS input data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr21`]
module"]
pub type DINR21 = crate::Reg<dinr21::DINR21rs>;
#[doc = "MDIOS input data register 21"]
pub mod dinr21;
#[doc = "DINR22 (r) register accessor: MDIOS input data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr22`]
module"]
pub type DINR22 = crate::Reg<dinr22::DINR22rs>;
#[doc = "MDIOS input data register 22"]
pub mod dinr22;
#[doc = "DINR23 (r) register accessor: MDIOS input data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr23`]
module"]
pub type DINR23 = crate::Reg<dinr23::DINR23rs>;
#[doc = "MDIOS input data register 23"]
pub mod dinr23;
#[doc = "DINR24 (r) register accessor: MDIOS input data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr24::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr24`]
module"]
pub type DINR24 = crate::Reg<dinr24::DINR24rs>;
#[doc = "MDIOS input data register 24"]
pub mod dinr24;
#[doc = "DINR25 (r) register accessor: MDIOS input data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr25::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr25`]
module"]
pub type DINR25 = crate::Reg<dinr25::DINR25rs>;
#[doc = "MDIOS input data register 25"]
pub mod dinr25;
#[doc = "DINR26 (r) register accessor: MDIOS input data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr26::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr26`]
module"]
pub type DINR26 = crate::Reg<dinr26::DINR26rs>;
#[doc = "MDIOS input data register 26"]
pub mod dinr26;
#[doc = "DINR27 (r) register accessor: MDIOS input data register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr27::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr27`]
module"]
pub type DINR27 = crate::Reg<dinr27::DINR27rs>;
#[doc = "MDIOS input data register 27"]
pub mod dinr27;
#[doc = "DINR28 (r) register accessor: MDIOS input data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr28::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr28`]
module"]
pub type DINR28 = crate::Reg<dinr28::DINR28rs>;
#[doc = "MDIOS input data register 28"]
pub mod dinr28;
#[doc = "DINR29 (r) register accessor: MDIOS input data register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr29::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr29`]
module"]
pub type DINR29 = crate::Reg<dinr29::DINR29rs>;
#[doc = "MDIOS input data register 29"]
pub mod dinr29;
#[doc = "DINR30 (r) register accessor: MDIOS input data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr30::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr30`]
module"]
pub type DINR30 = crate::Reg<dinr30::DINR30rs>;
#[doc = "MDIOS input data register 30"]
pub mod dinr30;
#[doc = "DINR31 (r) register accessor: MDIOS input data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr31::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr31`]
module"]
pub type DINR31 = crate::Reg<dinr31::DINR31rs>;
#[doc = "MDIOS input data register 31"]
pub mod dinr31;
#[doc = "DOUTR0 (rw) register accessor: MDIOS output data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr0`]
module"]
pub type DOUTR0 = crate::Reg<doutr0::DOUTR0rs>;
#[doc = "MDIOS output data register 0"]
pub mod doutr0;
#[doc = "DOUTR1 (rw) register accessor: MDIOS output data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr1`]
module"]
pub type DOUTR1 = crate::Reg<doutr1::DOUTR1rs>;
#[doc = "MDIOS output data register 1"]
pub mod doutr1;
#[doc = "DOUTR2 (rw) register accessor: MDIOS output data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr2`]
module"]
pub type DOUTR2 = crate::Reg<doutr2::DOUTR2rs>;
#[doc = "MDIOS output data register 2"]
pub mod doutr2;
#[doc = "DOUTR3 (rw) register accessor: MDIOS output data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr3`]
module"]
pub type DOUTR3 = crate::Reg<doutr3::DOUTR3rs>;
#[doc = "MDIOS output data register 3"]
pub mod doutr3;
#[doc = "DOUTR4 (rw) register accessor: MDIOS output data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr4`]
module"]
pub type DOUTR4 = crate::Reg<doutr4::DOUTR4rs>;
#[doc = "MDIOS output data register 4"]
pub mod doutr4;
#[doc = "DOUTR5 (rw) register accessor: MDIOS output data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr5`]
module"]
pub type DOUTR5 = crate::Reg<doutr5::DOUTR5rs>;
#[doc = "MDIOS output data register 5"]
pub mod doutr5;
#[doc = "DOUTR6 (rw) register accessor: MDIOS output data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr6`]
module"]
pub type DOUTR6 = crate::Reg<doutr6::DOUTR6rs>;
#[doc = "MDIOS output data register 6"]
pub mod doutr6;
#[doc = "DOUTR7 (rw) register accessor: MDIOS output data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr7`]
module"]
pub type DOUTR7 = crate::Reg<doutr7::DOUTR7rs>;
#[doc = "MDIOS output data register 7"]
pub mod doutr7;
#[doc = "DOUTR8 (rw) register accessor: MDIOS output data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr8`]
module"]
pub type DOUTR8 = crate::Reg<doutr8::DOUTR8rs>;
#[doc = "MDIOS output data register 8"]
pub mod doutr8;
#[doc = "DOUTR9 (rw) register accessor: MDIOS output data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr9`]
module"]
pub type DOUTR9 = crate::Reg<doutr9::DOUTR9rs>;
#[doc = "MDIOS output data register 9"]
pub mod doutr9;
#[doc = "DOUTR10 (rw) register accessor: MDIOS output data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr10`]
module"]
pub type DOUTR10 = crate::Reg<doutr10::DOUTR10rs>;
#[doc = "MDIOS output data register 10"]
pub mod doutr10;
#[doc = "DOUTR11 (rw) register accessor: MDIOS output data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr11`]
module"]
pub type DOUTR11 = crate::Reg<doutr11::DOUTR11rs>;
#[doc = "MDIOS output data register 11"]
pub mod doutr11;
#[doc = "DOUTR12 (rw) register accessor: MDIOS output data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr12`]
module"]
pub type DOUTR12 = crate::Reg<doutr12::DOUTR12rs>;
#[doc = "MDIOS output data register 12"]
pub mod doutr12;
#[doc = "DOUTR13 (rw) register accessor: MDIOS output data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr13`]
module"]
pub type DOUTR13 = crate::Reg<doutr13::DOUTR13rs>;
#[doc = "MDIOS output data register 13"]
pub mod doutr13;
#[doc = "DOUTR14 (rw) register accessor: MDIOS output data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr14`]
module"]
pub type DOUTR14 = crate::Reg<doutr14::DOUTR14rs>;
#[doc = "MDIOS output data register 14"]
pub mod doutr14;
#[doc = "DOUTR15 (rw) register accessor: MDIOS output data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr15`]
module"]
pub type DOUTR15 = crate::Reg<doutr15::DOUTR15rs>;
#[doc = "MDIOS output data register 15"]
pub mod doutr15;
#[doc = "DOUTR16 (rw) register accessor: MDIOS output data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr16`]
module"]
pub type DOUTR16 = crate::Reg<doutr16::DOUTR16rs>;
#[doc = "MDIOS output data register 16"]
pub mod doutr16;
#[doc = "DOUTR17 (rw) register accessor: MDIOS output data register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr17`]
module"]
pub type DOUTR17 = crate::Reg<doutr17::DOUTR17rs>;
#[doc = "MDIOS output data register 17"]
pub mod doutr17;
#[doc = "DOUTR18 (rw) register accessor: MDIOS output data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr18`]
module"]
pub type DOUTR18 = crate::Reg<doutr18::DOUTR18rs>;
#[doc = "MDIOS output data register 18"]
pub mod doutr18;
#[doc = "DOUTR19 (rw) register accessor: MDIOS output data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr19`]
module"]
pub type DOUTR19 = crate::Reg<doutr19::DOUTR19rs>;
#[doc = "MDIOS output data register 19"]
pub mod doutr19;
#[doc = "DOUTR20 (rw) register accessor: MDIOS output data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr20`]
module"]
pub type DOUTR20 = crate::Reg<doutr20::DOUTR20rs>;
#[doc = "MDIOS output data register 20"]
pub mod doutr20;
#[doc = "DOUTR21 (rw) register accessor: MDIOS output data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr21`]
module"]
pub type DOUTR21 = crate::Reg<doutr21::DOUTR21rs>;
#[doc = "MDIOS output data register 21"]
pub mod doutr21;
#[doc = "DOUTR22 (rw) register accessor: MDIOS output data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr22`]
module"]
pub type DOUTR22 = crate::Reg<doutr22::DOUTR22rs>;
#[doc = "MDIOS output data register 22"]
pub mod doutr22;
#[doc = "DOUTR23 (rw) register accessor: MDIOS output data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr23`]
module"]
pub type DOUTR23 = crate::Reg<doutr23::DOUTR23rs>;
#[doc = "MDIOS output data register 23"]
pub mod doutr23;
#[doc = "DOUTR24 (rw) register accessor: MDIOS output data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr24`]
module"]
pub type DOUTR24 = crate::Reg<doutr24::DOUTR24rs>;
#[doc = "MDIOS output data register 24"]
pub mod doutr24;
#[doc = "DOUTR25 (rw) register accessor: MDIOS output data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr25`]
module"]
pub type DOUTR25 = crate::Reg<doutr25::DOUTR25rs>;
#[doc = "MDIOS output data register 25"]
pub mod doutr25;
#[doc = "DOUTR26 (rw) register accessor: MDIOS output data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr26`]
module"]
pub type DOUTR26 = crate::Reg<doutr26::DOUTR26rs>;
#[doc = "MDIOS output data register 26"]
pub mod doutr26;
#[doc = "DOUTR27 (rw) register accessor: MDIOS output data register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr27`]
module"]
pub type DOUTR27 = crate::Reg<doutr27::DOUTR27rs>;
#[doc = "MDIOS output data register 27"]
pub mod doutr27;
#[doc = "DOUTR28 (rw) register accessor: MDIOS output data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr28`]
module"]
pub type DOUTR28 = crate::Reg<doutr28::DOUTR28rs>;
#[doc = "MDIOS output data register 28"]
pub mod doutr28;
#[doc = "DOUTR29 (rw) register accessor: MDIOS output data register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr29`]
module"]
pub type DOUTR29 = crate::Reg<doutr29::DOUTR29rs>;
#[doc = "MDIOS output data register 29"]
pub mod doutr29;
#[doc = "DOUTR30 (rw) register accessor: MDIOS output data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr30`]
module"]
pub type DOUTR30 = crate::Reg<doutr30::DOUTR30rs>;
#[doc = "MDIOS output data register 30"]
pub mod doutr30;
#[doc = "DOUTR31 (rw) register accessor: MDIOS output data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr31`]
module"]
pub type DOUTR31 = crate::Reg<doutr31::DOUTR31rs>;
#[doc = "MDIOS output data register 31"]
pub mod doutr31;
