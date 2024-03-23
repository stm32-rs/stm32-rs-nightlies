#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    gtpr: GTPR,
    rtor: RTOR,
    rqr: RQR,
    isr: ISR,
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x0c - baud rate register"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x10 - guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    #[doc = "0x14 - receiver timeout register"]
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    #[doc = "0x18 - request register"]
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    #[doc = "0x1c - interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x20 - interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x24 - receive data register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    #[doc = "0x28 - transmit data register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    #[doc = "0x2c - prescaler register"]
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "BRR (rw) register accessor: baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "baud rate register"]
pub mod brr;
#[doc = "GTPR (rw) register accessor: guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtpr`]
module"]
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
#[doc = "guard time and prescaler register"]
pub mod gtpr;
#[doc = "RTOR (rw) register accessor: receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtor`]
module"]
pub type RTOR = crate::Reg<rtor::RTORrs>;
#[doc = "receiver timeout register"]
pub mod rtor;
#[doc = "RQR (rw) register accessor: request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqr`]
module"]
pub type RQR = crate::Reg<rqr::RQRrs>;
#[doc = "request register"]
pub mod rqr;
#[doc = "ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "interrupt flag clear register"]
pub mod icr;
#[doc = "RDR (r) register accessor: receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDRrs>;
#[doc = "receive data register"]
pub mod rdr;
#[doc = "TDR (rw) register accessor: transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDRrs>;
#[doc = "transmit data register"]
pub mod tdr;
#[doc = "PRESC (rw) register accessor: prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presc`]
module"]
pub type PRESC = crate::Reg<presc::PRESCrs>;
#[doc = "prescaler register"]
pub mod presc;
