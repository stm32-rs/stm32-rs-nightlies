#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crel: CREL,
    endn: ENDN,
    _reserved2: [u8; 0x04],
    dbtp: DBTP,
    test: TEST,
    rwd: RWD,
    cccr: CCCR,
    nbtp: NBTP,
    tscc: TSCC,
    tscv: TSCV,
    tocc: TOCC,
    tocv: TOCV,
    _reserved11: [u8; 0x10],
    ecr: ECR,
    psr: PSR,
    tdcr: TDCR,
    _reserved14: [u8; 0x04],
    ir: IR,
    ie: IE,
    ils: ILS,
    ile: ILE,
    _reserved18: [u8; 0x20],
    rxgfc: RXGFC,
    xidam: XIDAM,
    hpms: HPMS,
    _reserved21: [u8; 0x04],
    rxf0s: RXF0S,
    rxf0a: RXF0A,
    rxf1s: RXF1S,
    rxf1a: RXF1A,
    _reserved25: [u8; 0x20],
    txbc: TXBC,
    txfqs: TXFQS,
    txbrp: TXBRP,
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved36: [u8; 0x14],
    ckdiv: CKDIV,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN core release register"]
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    #[doc = "0x04 - FDCAN endian register"]
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    #[doc = "0x0c - FDCAN data bit timing and prescaler register"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    #[doc = "0x10 - FDCAN test register"]
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    #[doc = "0x14 - FDCAN RAM watchdog register"]
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    #[doc = "0x18 - FDCAN CC control register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x1c - FDCAN nominal bit timing and prescaler register"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    #[doc = "0x20 - FDCAN timestamp counter configuration register"]
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    #[doc = "0x24 - FDCAN timestamp counter value register"]
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    #[doc = "0x28 - FDCAN timeout counter configuration register"]
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    #[doc = "0x2c - FDCAN timeout counter value register"]
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    #[doc = "0x40 - FDCAN error counter register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    #[doc = "0x44 - FDCAN protocol status register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0x48 - FDCAN transmitter delay compensation register"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    #[doc = "0x50 - FDCAN interrupt register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x54 - FDCAN interrupt enable register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x58 - FDCAN interrupt line select register"]
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    #[doc = "0x5c - FDCAN interrupt line enable register"]
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    #[doc = "0x80 - FDCAN global filter configuration register"]
    #[inline(always)]
    pub const fn rxgfc(&self) -> &RXGFC {
        &self.rxgfc
    }
    #[doc = "0x84 - FDCAN extended ID and mask register"]
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    #[doc = "0x88 - FDCAN high-priority message status register"]
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 status register"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    #[doc = "0x94 - CAN Rx FIFO 0 acknowledge register"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 status register"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 acknowledge register"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/queue status register"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    #[doc = "0xc8 - FDCAN Tx buffer request pending register"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    #[doc = "0xcc - FDCAN Tx buffer add request register"]
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    #[doc = "0xd0 - FDCAN Tx buffer cancellation request register"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    #[doc = "0xd4 - FDCAN Tx buffer transmission occurred register"]
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    #[doc = "0xd8 - FDCAN Tx buffer cancellation finished register"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    #[doc = "0xdc - FDCAN Tx buffer transmission interrupt enable register"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    #[doc = "0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    #[doc = "0xe4 - FDCAN Tx event FIFO status register"]
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    #[doc = "0xe8 - FDCAN Tx event FIFO acknowledge register"]
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn ckdiv(&self) -> &CKDIV {
        &self.ckdiv
    }
}
#[doc = "CREL (r) register accessor: FDCAN core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
pub type CREL = crate::Reg<crel::CRELrs>;
#[doc = "FDCAN core release register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: FDCAN endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
pub type ENDN = crate::Reg<endn::ENDNrs>;
#[doc = "FDCAN endian register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
#[doc = "FDCAN data bit timing and prescaler register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: FDCAN test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TESTrs>;
#[doc = "FDCAN test register"]
pub mod test;
#[doc = "RWD (rw) register accessor: FDCAN RAM watchdog register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWDrs>;
#[doc = "FDCAN RAM watchdog register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: FDCAN CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCRrs>;
#[doc = "FDCAN CC control register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: FDCAN nominal bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
#[doc = "FDCAN nominal bit timing and prescaler register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
pub type TSCC = crate::Reg<tscc::TSCCrs>;
#[doc = "FDCAN timestamp counter configuration register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: FDCAN timestamp counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
pub type TSCV = crate::Reg<tscv::TSCVrs>;
#[doc = "FDCAN timestamp counter value register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: FDCAN timeout counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
pub type TOCC = crate::Reg<tocc::TOCCrs>;
#[doc = "FDCAN timeout counter configuration register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: FDCAN timeout counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
pub type TOCV = crate::Reg<tocv::TOCVrs>;
#[doc = "FDCAN timeout counter value register"]
pub mod tocv;
#[doc = "ECR (rw) register accessor: FDCAN error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECRrs>;
#[doc = "FDCAN error counter register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: FDCAN protocol status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSRrs>;
#[doc = "FDCAN protocol status register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: FDCAN transmitter delay compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
#[doc = "FDCAN transmitter delay compensation register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: FDCAN interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "FDCAN interrupt register"]
pub mod ir;
#[doc = "IE (rw) register accessor: FDCAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IErs>;
#[doc = "FDCAN interrupt enable register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: FDCAN interrupt line select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
pub type ILS = crate::Reg<ils::ILSrs>;
#[doc = "FDCAN interrupt line select register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: FDCAN interrupt line enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
pub type ILE = crate::Reg<ile::ILErs>;
#[doc = "FDCAN interrupt line enable register"]
pub mod ile;
#[doc = "RXGFC (rw) register accessor: FDCAN global filter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxgfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxgfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxgfc`]
module"]
pub type RXGFC = crate::Reg<rxgfc::RXGFCrs>;
#[doc = "FDCAN global filter configuration register"]
pub mod rxgfc;
#[doc = "XIDAM (rw) register accessor: FDCAN extended ID and mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
#[doc = "FDCAN extended ID and mask register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: FDCAN high-priority message status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMSrs>;
#[doc = "FDCAN high-priority message status register"]
pub mod hpms;
#[doc = "RXF0S (r) register accessor: FDCAN Rx FIFO 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
#[doc = "FDCAN Rx FIFO 0 status register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: CAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
#[doc = "CAN Rx FIFO 0 acknowledge register"]
pub mod rxf0a;
#[doc = "RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
#[doc = "FDCAN Rx FIFO 1 status register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
pub mod rxf1a;
#[doc = "TXBC (rw) register accessor: FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBCrs>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
#[doc = "FDCAN Tx FIFO/queue status register"]
pub mod txfqs;
#[doc = "TXBRP (r) register accessor: FDCAN Tx buffer request pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
pub type TXBRP = crate::Reg<txbrp::TXBRPrs>;
#[doc = "FDCAN Tx buffer request pending register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: FDCAN Tx buffer add request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
#[doc = "FDCAN Tx buffer add request register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
#[doc = "FDCAN Tx buffer cancellation request register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
#[doc = "FDCAN Tx buffer transmission occurred register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
#[doc = "FDCAN Tx buffer cancellation finished register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
pub mod txbcie;
#[doc = "TXEFS (r) register accessor: FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
#[doc = "FDCAN Tx event FIFO status register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
#[doc = "FDCAN Tx event FIFO acknowledge register"]
pub mod txefa;
#[doc = "CKDIV (rw) register accessor: FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckdiv`]
module"]
pub type CKDIV = crate::Reg<ckdiv::CKDIVrs>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
