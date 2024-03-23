#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    argr: ARGR,
    cmdr: CMDR,
    respcmdr: RESPCMDR,
    resp1r: RESP1R,
    resp2r: RESP2R,
    resp3r: RESP3R,
    resp4r: RESP4R,
    dtimer: DTIMER,
    dlenr: DLENR,
    dctrl: DCTRL,
    dcntr: DCNTR,
    star: STAR,
    icr: ICR,
    maskr: MASKR,
    acktimer: ACKTIMER,
    _reserved17: [u8; 0x0c],
    idmactrlr: IDMACTRLR,
    idmabsizer: IDMABSIZER,
    idmabase0r: IDMABASE0R,
    idmabase1r: IDMABASE1R,
    _reserved21: [u8; 0x20],
    fifor0: FIFOR0,
    fifor1: FIFOR1,
    fifor2: FIFOR2,
    fifor3: FIFOR3,
    fifor4: FIFOR4,
    fifor5: FIFOR5,
    fifor6: FIFOR6,
    fifor7: FIFOR7,
    fifor8: FIFOR8,
    fifor9: FIFOR9,
    fifor10: FIFOR10,
    fifor11: FIFOR11,
    fifor12: FIFOR12,
    fifor13: FIFOR13,
    fifor14: FIFOR14,
    fifor15: FIFOR15,
}
impl RegisterBlock {
    #[doc = "0x00 - power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    #[doc = "0x04 - SDI clock control register"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x08 - argument register"]
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    #[doc = "0x0c - command register"]
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    #[doc = "0x10 - command response register"]
    #[inline(always)]
    pub const fn respcmdr(&self) -> &RESPCMDR {
        &self.respcmdr
    }
    #[doc = "0x14 - response 1..4 register"]
    #[inline(always)]
    pub const fn resp1r(&self) -> &RESP1R {
        &self.resp1r
    }
    #[doc = "0x18 - response 1..4 register"]
    #[inline(always)]
    pub const fn resp2r(&self) -> &RESP2R {
        &self.resp2r
    }
    #[doc = "0x1c - response 1..4 register"]
    #[inline(always)]
    pub const fn resp3r(&self) -> &RESP3R {
        &self.resp3r
    }
    #[doc = "0x20 - response 1..4 register"]
    #[inline(always)]
    pub const fn resp4r(&self) -> &RESP4R {
        &self.resp4r
    }
    #[doc = "0x24 - data timer register"]
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    #[doc = "0x28 - data length register"]
    #[inline(always)]
    pub const fn dlenr(&self) -> &DLENR {
        &self.dlenr
    }
    #[doc = "0x2c - data control register"]
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    #[doc = "0x30 - data counter register"]
    #[inline(always)]
    pub const fn dcntr(&self) -> &DCNTR {
        &self.dcntr
    }
    #[doc = "0x34 - status register"]
    #[inline(always)]
    pub const fn star(&self) -> &STAR {
        &self.star
    }
    #[doc = "0x38 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x3c - mask register"]
    #[inline(always)]
    pub const fn maskr(&self) -> &MASKR {
        &self.maskr
    }
    #[doc = "0x40 - acknowledgment timer register"]
    #[inline(always)]
    pub const fn acktimer(&self) -> &ACKTIMER {
        &self.acktimer
    }
    #[doc = "0x50 - DMA control register"]
    #[inline(always)]
    pub const fn idmactrlr(&self) -> &IDMACTRLR {
        &self.idmactrlr
    }
    #[doc = "0x54 - IDMA buffer size register"]
    #[inline(always)]
    pub const fn idmabsizer(&self) -> &IDMABSIZER {
        &self.idmabsizer
    }
    #[doc = "0x58 - IDMA buffer 0 base address register"]
    #[inline(always)]
    pub const fn idmabase0r(&self) -> &IDMABASE0R {
        &self.idmabase0r
    }
    #[doc = "0x5c - IDMA buffer 0 base address register"]
    #[inline(always)]
    pub const fn idmabase1r(&self) -> &IDMABASE1R {
        &self.idmabase1r
    }
    #[doc = "0x80 - data FIFO register 0"]
    #[inline(always)]
    pub const fn fifor0(&self) -> &FIFOR0 {
        &self.fifor0
    }
    #[doc = "0x84 - data FIFO register 1"]
    #[inline(always)]
    pub const fn fifor1(&self) -> &FIFOR1 {
        &self.fifor1
    }
    #[doc = "0x88 - data FIFO register 2"]
    #[inline(always)]
    pub const fn fifor2(&self) -> &FIFOR2 {
        &self.fifor2
    }
    #[doc = "0x8c - data FIFO register 3"]
    #[inline(always)]
    pub const fn fifor3(&self) -> &FIFOR3 {
        &self.fifor3
    }
    #[doc = "0x90 - data FIFO register 4"]
    #[inline(always)]
    pub const fn fifor4(&self) -> &FIFOR4 {
        &self.fifor4
    }
    #[doc = "0x94 - data FIFO register 5"]
    #[inline(always)]
    pub const fn fifor5(&self) -> &FIFOR5 {
        &self.fifor5
    }
    #[doc = "0x98 - data FIFO register 6"]
    #[inline(always)]
    pub const fn fifor6(&self) -> &FIFOR6 {
        &self.fifor6
    }
    #[doc = "0x9c - data FIFO register 7"]
    #[inline(always)]
    pub const fn fifor7(&self) -> &FIFOR7 {
        &self.fifor7
    }
    #[doc = "0xa0 - data FIFO register 8"]
    #[inline(always)]
    pub const fn fifor8(&self) -> &FIFOR8 {
        &self.fifor8
    }
    #[doc = "0xa4 - data FIFO register 9"]
    #[inline(always)]
    pub const fn fifor9(&self) -> &FIFOR9 {
        &self.fifor9
    }
    #[doc = "0xa8 - data FIFO register 10"]
    #[inline(always)]
    pub const fn fifor10(&self) -> &FIFOR10 {
        &self.fifor10
    }
    #[doc = "0xac - data FIFO register 11"]
    #[inline(always)]
    pub const fn fifor11(&self) -> &FIFOR11 {
        &self.fifor11
    }
    #[doc = "0xb0 - data FIFO register 12"]
    #[inline(always)]
    pub const fn fifor12(&self) -> &FIFOR12 {
        &self.fifor12
    }
    #[doc = "0xb4 - data FIFO register 13"]
    #[inline(always)]
    pub const fn fifor13(&self) -> &FIFOR13 {
        &self.fifor13
    }
    #[doc = "0xb8 - data FIFO register 14"]
    #[inline(always)]
    pub const fn fifor14(&self) -> &FIFOR14 {
        &self.fifor14
    }
    #[doc = "0xbc - data FIFO register 15"]
    #[inline(always)]
    pub const fn fifor15(&self) -> &FIFOR15 {
        &self.fifor15
    }
}
#[doc = "POWER (rw) register accessor: power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWERrs>;
#[doc = "power control register"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: SDI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "ARGR (rw) register accessor: argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argr`]
module"]
pub type ARGR = crate::Reg<argr::ARGRrs>;
#[doc = "argument register"]
pub mod argr;
#[doc = "CMDR (rw) register accessor: command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`]
module"]
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
#[doc = "command register"]
pub mod cmdr;
#[doc = "RESPCMDR (r) register accessor: command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmdr`]
module"]
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
#[doc = "command response register"]
pub mod respcmdr;
#[doc = "RESP1R (r) register accessor: response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1r`]
module"]
pub type RESP1R = crate::Reg<resp1r::RESP1Rrs>;
#[doc = "response 1..4 register"]
pub mod resp1r;
#[doc = "RESP2R (r) register accessor: response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2r`]
module"]
pub type RESP2R = crate::Reg<resp2r::RESP2Rrs>;
#[doc = "response 1..4 register"]
pub mod resp2r;
#[doc = "RESP3R (r) register accessor: response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3r`]
module"]
pub type RESP3R = crate::Reg<resp3r::RESP3Rrs>;
#[doc = "response 1..4 register"]
pub mod resp3r;
#[doc = "RESP4R (r) register accessor: response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4r`]
module"]
pub type RESP4R = crate::Reg<resp4r::RESP4Rrs>;
#[doc = "response 1..4 register"]
pub mod resp4r;
#[doc = "DTIMER (rw) register accessor: data timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "DLENR (rw) register accessor: data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlenr`]
module"]
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
#[doc = "data length register"]
pub mod dlenr;
#[doc = "DCTRL (rw) register accessor: data control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
#[doc = "data control register"]
pub mod dctrl;
#[doc = "DCNTR (r) register accessor: data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcntr`]
module"]
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
#[doc = "data counter register"]
pub mod dcntr;
#[doc = "STAR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`star::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@star`]
module"]
pub type STAR = crate::Reg<star::STARrs>;
#[doc = "status register"]
pub mod star;
#[doc = "ICR (rw) register accessor: interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "MASKR (rw) register accessor: mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskr`]
module"]
pub type MASKR = crate::Reg<maskr::MASKRrs>;
#[doc = "mask register"]
pub mod maskr;
#[doc = "ACKTIMER (rw) register accessor: acknowledgment timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acktimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acktimer`]
module"]
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
#[doc = "acknowledgment timer register"]
pub mod acktimer;
#[doc = "FIFOR0 (rw) register accessor: data FIFO register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor0`]
module"]
pub type FIFOR0 = crate::Reg<fifor0::FIFOR0rs>;
#[doc = "data FIFO register 0"]
pub mod fifor0;
#[doc = "FIFOR1 (rw) register accessor: data FIFO register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor1`]
module"]
pub type FIFOR1 = crate::Reg<fifor1::FIFOR1rs>;
#[doc = "data FIFO register 1"]
pub mod fifor1;
#[doc = "FIFOR2 (rw) register accessor: data FIFO register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor2`]
module"]
pub type FIFOR2 = crate::Reg<fifor2::FIFOR2rs>;
#[doc = "data FIFO register 2"]
pub mod fifor2;
#[doc = "FIFOR3 (rw) register accessor: data FIFO register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor3`]
module"]
pub type FIFOR3 = crate::Reg<fifor3::FIFOR3rs>;
#[doc = "data FIFO register 3"]
pub mod fifor3;
#[doc = "FIFOR4 (rw) register accessor: data FIFO register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor4`]
module"]
pub type FIFOR4 = crate::Reg<fifor4::FIFOR4rs>;
#[doc = "data FIFO register 4"]
pub mod fifor4;
#[doc = "FIFOR5 (rw) register accessor: data FIFO register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor5`]
module"]
pub type FIFOR5 = crate::Reg<fifor5::FIFOR5rs>;
#[doc = "data FIFO register 5"]
pub mod fifor5;
#[doc = "FIFOR6 (rw) register accessor: data FIFO register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor6`]
module"]
pub type FIFOR6 = crate::Reg<fifor6::FIFOR6rs>;
#[doc = "data FIFO register 6"]
pub mod fifor6;
#[doc = "FIFOR7 (rw) register accessor: data FIFO register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor7`]
module"]
pub type FIFOR7 = crate::Reg<fifor7::FIFOR7rs>;
#[doc = "data FIFO register 7"]
pub mod fifor7;
#[doc = "FIFOR8 (rw) register accessor: data FIFO register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor8`]
module"]
pub type FIFOR8 = crate::Reg<fifor8::FIFOR8rs>;
#[doc = "data FIFO register 8"]
pub mod fifor8;
#[doc = "FIFOR9 (rw) register accessor: data FIFO register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor9`]
module"]
pub type FIFOR9 = crate::Reg<fifor9::FIFOR9rs>;
#[doc = "data FIFO register 9"]
pub mod fifor9;
#[doc = "FIFOR10 (rw) register accessor: data FIFO register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor10`]
module"]
pub type FIFOR10 = crate::Reg<fifor10::FIFOR10rs>;
#[doc = "data FIFO register 10"]
pub mod fifor10;
#[doc = "FIFOR11 (rw) register accessor: data FIFO register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor11`]
module"]
pub type FIFOR11 = crate::Reg<fifor11::FIFOR11rs>;
#[doc = "data FIFO register 11"]
pub mod fifor11;
#[doc = "FIFOR12 (rw) register accessor: data FIFO register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor12`]
module"]
pub type FIFOR12 = crate::Reg<fifor12::FIFOR12rs>;
#[doc = "data FIFO register 12"]
pub mod fifor12;
#[doc = "FIFOR13 (rw) register accessor: data FIFO register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor13`]
module"]
pub type FIFOR13 = crate::Reg<fifor13::FIFOR13rs>;
#[doc = "data FIFO register 13"]
pub mod fifor13;
#[doc = "FIFOR14 (rw) register accessor: data FIFO register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor14`]
module"]
pub type FIFOR14 = crate::Reg<fifor14::FIFOR14rs>;
#[doc = "data FIFO register 14"]
pub mod fifor14;
#[doc = "FIFOR15 (rw) register accessor: data FIFO register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor15`]
module"]
pub type FIFOR15 = crate::Reg<fifor15::FIFOR15rs>;
#[doc = "data FIFO register 15"]
pub mod fifor15;
#[doc = "IDMACTRLR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmactrlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmactrlr`]
module"]
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
#[doc = "DMA control register"]
pub mod idmactrlr;
#[doc = "IDMABSIZER (rw) register accessor: IDMA buffer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabsizer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabsizer`]
module"]
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
#[doc = "IDMA buffer size register"]
pub mod idmabsizer;
#[doc = "IDMABASE0R (rw) register accessor: IDMA buffer 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabase0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabase0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabase0r`]
module"]
pub type IDMABASE0R = crate::Reg<idmabase0r::IDMABASE0Rrs>;
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase0r;
#[doc = "IDMABASE1R (rw) register accessor: IDMA buffer 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabase1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabase1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabase1r`]
module"]
pub type IDMABASE1R = crate::Reg<idmabase1r::IDMABASE1Rrs>;
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase1r;
