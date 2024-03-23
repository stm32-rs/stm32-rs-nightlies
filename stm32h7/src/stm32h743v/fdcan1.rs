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
    gfc: GFC,
    sidfc: SIDFC,
    xidfc: XIDFC,
    _reserved21: [u8; 0x04],
    xidam: XIDAM,
    hpms: HPMS,
    ndat1: NDAT1,
    ndat2: NDAT2,
    rxf0c: RXF0C,
    rxf0s: RXF0S,
    rxf0a: RXF0A,
    rxbc: RXBC,
    rxf1c: RXF1C,
    rxf1s: RXF1S,
    rxf1a: RXF1A,
    rxesc: RXESC,
    txbc: TXBC,
    txfqs: TXFQS,
    txesc: TXESC,
    txbrp: TXBRP,
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    _reserved43: [u8; 0x08],
    txefc: TXEFC,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved46: [u8; 0x04],
    tttmc: TTTMC,
    ttrmc: TTRMC,
    ttocf: TTOCF,
    ttmlm: TTMLM,
    turcf: TURCF,
    ttocn: TTOCN,
    ttgtp: TTGTP,
    tttmk: TTTMK,
    ttir: TTIR,
    ttie: TTIE,
    ttils: TTILS,
    ttost: TTOST,
    turna: TURNA,
    ttlgt: TTLGT,
    ttctc: TTCTC,
    ttcpt: TTCPT,
    ttcsm: TTCSM,
    _reserved63: [u8; 0x01bc],
    ttts: TTTS,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    #[doc = "0x04 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    #[doc = "0x10 - FDCAN Test Register"]
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    #[doc = "0x18 - FDCAN CC Control Register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    #[doc = "0x40 - FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    #[doc = "0x50 - FDCAN Interrupt Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    #[inline(always)]
    pub const fn gfc(&self) -> &GFC {
        &self.gfc
    }
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn sidfc(&self) -> &SIDFC {
        &self.sidfc
    }
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn xidfc(&self) -> &XIDFC {
        &self.xidfc
    }
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    #[inline(always)]
    pub const fn ndat1(&self) -> &NDAT1 {
        &self.ndat1
    }
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    #[inline(always)]
    pub const fn ndat2(&self) -> &NDAT2 {
        &self.ndat2
    }
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    #[inline(always)]
    pub const fn rxf0c(&self) -> &RXF0C {
        &self.rxf0c
    }
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn rxbc(&self) -> &RXBC {
        &self.rxbc
    }
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    #[inline(always)]
    pub const fn rxf1c(&self) -> &RXF1C {
        &self.rxf1c
    }
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn rxesc(&self) -> &RXESC {
        &self.rxesc
    }
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn txesc(&self) -> &TXESC {
        &self.txesc
    }
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    #[inline(always)]
    pub const fn txefc(&self) -> &TXEFC {
        &self.txefc
    }
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn tttmc(&self) -> &TTTMC {
        &self.tttmc
    }
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    #[inline(always)]
    pub const fn ttrmc(&self) -> &TTRMC {
        &self.ttrmc
    }
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    #[inline(always)]
    pub const fn ttocf(&self) -> &TTOCF {
        &self.ttocf
    }
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    #[inline(always)]
    pub const fn ttmlm(&self) -> &TTMLM {
        &self.ttmlm
    }
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    #[inline(always)]
    pub const fn turcf(&self) -> &TURCF {
        &self.turcf
    }
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    #[inline(always)]
    pub const fn ttocn(&self) -> &TTOCN {
        &self.ttocn
    }
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    #[inline(always)]
    pub const fn ttgtp(&self) -> &TTGTP {
        &self.ttgtp
    }
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    #[inline(always)]
    pub const fn tttmk(&self) -> &TTTMK {
        &self.tttmk
    }
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    #[inline(always)]
    pub const fn ttir(&self) -> &TTIR {
        &self.ttir
    }
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ttie(&self) -> &TTIE {
        &self.ttie
    }
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ttils(&self) -> &TTILS {
        &self.ttils
    }
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    #[inline(always)]
    pub const fn ttost(&self) -> &TTOST {
        &self.ttost
    }
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    #[inline(always)]
    pub const fn turna(&self) -> &TURNA {
        &self.turna
    }
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    #[inline(always)]
    pub const fn ttlgt(&self) -> &TTLGT {
        &self.ttlgt
    }
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    #[inline(always)]
    pub const fn ttctc(&self) -> &TTCTC {
        &self.ttctc
    }
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    #[inline(always)]
    pub const fn ttcpt(&self) -> &TTCPT {
        &self.ttcpt
    }
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    #[inline(always)]
    pub const fn ttcsm(&self) -> &TTCSM {
        &self.ttcsm
    }
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    #[inline(always)]
    pub const fn ttts(&self) -> &TTTS {
        &self.ttts
    }
}
#[doc = "CREL (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
pub type CREL = crate::Reg<crel::CRELrs>;
#[doc = "FDCAN Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
pub type ENDN = crate::Reg<endn::ENDNrs>;
#[doc = "FDCAN Core Release Register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: FDCAN Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TESTrs>;
#[doc = "FDCAN Test Register"]
pub mod test;
#[doc = "RWD (r) register accessor: FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWDrs>;
#[doc = "FDCAN RAM Watchdog Register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: FDCAN CC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCRrs>;
#[doc = "FDCAN CC Control Register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
pub type TSCC = crate::Reg<tscc::TSCCrs>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
pub type TSCV = crate::Reg<tscv::TSCVrs>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
pub type TOCC = crate::Reg<tocc::TOCCrs>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
pub type TOCV = crate::Reg<tocv::TOCVrs>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR (rw) register accessor: FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECRrs>;
#[doc = "FDCAN Error Counter Register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSRrs>;
#[doc = "FDCAN Protocol Status Register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: FDCAN Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "FDCAN Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IErs>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
pub type ILS = crate::Reg<ils::ILSrs>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: FDCAN Interrupt Line Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
pub type ILE = crate::Reg<ile::ILErs>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod ile;
#[doc = "GFC (rw) register accessor: FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`]
module"]
pub type GFC = crate::Reg<gfc::GFCrs>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: FDCAN Standard ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`]
module"]
pub type SIDFC = crate::Reg<sidfc::SIDFCrs>;
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: FDCAN Extended ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`]
module"]
pub type XIDFC = crate::Reg<xidfc::XIDFCrs>;
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: FDCAN High Priority Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMSrs>;
#[doc = "FDCAN High Priority Message Status Register"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: FDCAN New Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`]
module"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1rs>;
#[doc = "FDCAN New Data 1 Register"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: FDCAN New Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`]
module"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2rs>;
#[doc = "FDCAN New Data 2 Register"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: FDCAN Rx FIFO 0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`]
module"]
pub type RXF0C = crate::Reg<rxf0c::RXF0Crs>;
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "RXF0S (rw) register accessor: FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`]
module"]
pub type RXBC = crate::Reg<rxbc::RXBCrs>;
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: FDCAN Rx FIFO 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`]
module"]
pub type RXF1C = crate::Reg<rxf1c::RXF1Crs>;
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "RXF1S (rw) register accessor: FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`]
module"]
pub type RXESC = crate::Reg<rxesc::RXESCrs>;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBCrs>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
pub type TXESC = crate::Reg<txesc::TXESCrs>;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
pub type TXBRP = crate::Reg<txbrp::TXBRPrs>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO (rw) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: FDCAN Tx Event FIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`]
module"]
pub type TXEFC = crate::Reg<txefc::TXEFCrs>;
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "TXEFS (rw) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "TTTMC (rw) register accessor: FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tttmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tttmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tttmc`]
module"]
pub type TTTMC = crate::Reg<tttmc::TTTMCrs>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod tttmc;
#[doc = "TTRMC (rw) register accessor: FDCAN TT Reference Message Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttrmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttrmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttrmc`]
module"]
pub type TTRMC = crate::Reg<ttrmc::TTRMCrs>;
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod ttrmc;
#[doc = "TTOCF (rw) register accessor: FDCAN TT Operation Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttocf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttocf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttocf`]
module"]
pub type TTOCF = crate::Reg<ttocf::TTOCFrs>;
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod ttocf;
#[doc = "TTMLM (rw) register accessor: FDCAN TT Matrix Limits Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttmlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttmlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttmlm`]
module"]
pub type TTMLM = crate::Reg<ttmlm::TTMLMrs>;
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod ttmlm;
#[doc = "TURCF (rw) register accessor: FDCAN TUR Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`turcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`turcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@turcf`]
module"]
pub type TURCF = crate::Reg<turcf::TURCFrs>;
#[doc = "FDCAN TUR Configuration Register"]
pub mod turcf;
#[doc = "TTOCN (rw) register accessor: FDCAN TT Operation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttocn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttocn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttocn`]
module"]
pub type TTOCN = crate::Reg<ttocn::TTOCNrs>;
#[doc = "FDCAN TT Operation Control Register"]
pub mod ttocn;
#[doc = "TTGTP (rw) register accessor: FDCAN TT Global Time Preset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttgtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttgtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttgtp`]
module"]
pub type TTGTP = crate::Reg<ttgtp::TTGTPrs>;
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod ttgtp;
#[doc = "TTTMK (rw) register accessor: FDCAN TT Time Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tttmk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tttmk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tttmk`]
module"]
pub type TTTMK = crate::Reg<tttmk::TTTMKrs>;
#[doc = "FDCAN TT Time Mark Register"]
pub mod tttmk;
#[doc = "TTIR (rw) register accessor: FDCAN TT Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttir`]
module"]
pub type TTIR = crate::Reg<ttir::TTIRrs>;
#[doc = "FDCAN TT Interrupt Register"]
pub mod ttir;
#[doc = "TTIE (rw) register accessor: FDCAN TT Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttie`]
module"]
pub type TTIE = crate::Reg<ttie::TTIErs>;
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod ttie;
#[doc = "TTILS (rw) register accessor: FDCAN TT Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttils`]
module"]
pub type TTILS = crate::Reg<ttils::TTILSrs>;
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod ttils;
#[doc = "TTOST (rw) register accessor: FDCAN TT Operation Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttost::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttost::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttost`]
module"]
pub type TTOST = crate::Reg<ttost::TTOSTrs>;
#[doc = "FDCAN TT Operation Status Register"]
pub mod ttost;
#[doc = "TURNA (r) register accessor: FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`turna::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@turna`]
module"]
pub type TURNA = crate::Reg<turna::TURNArs>;
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod turna;
#[doc = "TTLGT (r) register accessor: FDCAN TT Local and Global Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttlgt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttlgt`]
module"]
pub type TTLGT = crate::Reg<ttlgt::TTLGTrs>;
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod ttlgt;
#[doc = "TTCTC (r) register accessor: FDCAN TT Cycle Time and Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttctc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttctc`]
module"]
pub type TTCTC = crate::Reg<ttctc::TTCTCrs>;
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod ttctc;
#[doc = "TTCPT (r) register accessor: FDCAN TT Capture Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcpt`]
module"]
pub type TTCPT = crate::Reg<ttcpt::TTCPTrs>;
#[doc = "FDCAN TT Capture Time Register"]
pub mod ttcpt;
#[doc = "TTCSM (r) register accessor: FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcsm`]
module"]
pub type TTCSM = crate::Reg<ttcsm::TTCSMrs>;
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod ttcsm;
#[doc = "TTTS (rw) register accessor: FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttts`]
module"]
pub type TTTS = crate::Reg<ttts::TTTSrs>;
#[doc = "FDCAN TT Trigger Select Register"]
pub mod ttts;
