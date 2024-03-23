#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fdcan_crel: FDCAN_CREL,
    fdcan_endn: FDCAN_ENDN,
    _reserved2: [u8; 0x04],
    fdcan_dbtp: FDCAN_DBTP,
    fdcan_test: FDCAN_TEST,
    fdcan_rwd: FDCAN_RWD,
    fdcan_cccr: FDCAN_CCCR,
    fdcan_nbtp: FDCAN_NBTP,
    fdcan_tscc: FDCAN_TSCC,
    fdcan_tscv: FDCAN_TSCV,
    fdcan_tocc: FDCAN_TOCC,
    fdcan_tocv: FDCAN_TOCV,
    _reserved11: [u8; 0x10],
    fdcan_ecr: FDCAN_ECR,
    fdcan_psr: FDCAN_PSR,
    fdcan_tdcr: FDCAN_TDCR,
    _reserved14: [u8; 0x04],
    fdcan_ir: FDCAN_IR,
    fdcan_ie: FDCAN_IE,
    fdcan_ils: FDCAN_ILS,
    fdcan_ile: FDCAN_ILE,
    _reserved18: [u8; 0x20],
    fdcan_gfc: FDCAN_GFC,
    fdcan_sidfc: FDCAN_SIDFC,
    fdcan_xidfc: FDCAN_XIDFC,
    _reserved21: [u8; 0x04],
    fdcan_xidam: FDCAN_XIDAM,
    fdcan_hpms: FDCAN_HPMS,
    fdcan_ndat1: FDCAN_NDAT1,
    fdcan_ndat2: FDCAN_NDAT2,
    fdcan_rxf0c: FDCAN_RXF0C,
    fdcan_rxf0s: FDCAN_RXF0S,
    fdcan_rxf0a: FDCAN_RXF0A,
    fdcan_rxbc: FDCAN_RXBC,
    fdcan_rxf1c: FDCAN_RXF1C,
    fdcan_rxf1s: FDCAN_RXF1S,
    fdcan_rxf1a: FDCAN_RXF1A,
    fdcan_rxesc: FDCAN_RXESC,
    fdcan_txbc: FDCAN_TXBC,
    fdcan_txfqs: FDCAN_TXFQS,
    fdcan_txesc: FDCAN_TXESC,
    fdcan_txbrp: FDCAN_TXBRP,
    fdcan_txbar: FDCAN_TXBAR,
    fdcan_txbcr: FDCAN_TXBCR,
    fdcan_txbto: FDCAN_TXBTO,
    fdcan_txbcf: FDCAN_TXBCF,
    fdcan_txbtie: FDCAN_TXBTIE,
    fdcan_txbcie: FDCAN_TXBCIE,
    _reserved43: [u8; 0x08],
    fdcan_txefc: FDCAN_TXEFC,
    fdcan_txefs: FDCAN_TXEFS,
    fdcan_txefa: FDCAN_TXEFA,
    _reserved46: [u8; 0x04],
    fdcan_tttmc: FDCAN_TTTMC,
    fdcan_ttrmc: FDCAN_TTRMC,
    fdcan_ttocf: FDCAN_TTOCF,
    fdcan_ttmlm: FDCAN_TTMLM,
    fdcan_turcf: FDCAN_TURCF,
    fdcan_ttocn: FDCAN_TTOCN,
    can_ttgtp: CAN_TTGTP,
    fdcan_tttmk: FDCAN_TTTMK,
    fdcan_ttir: FDCAN_TTIR,
    fdcan_ttie: FDCAN_TTIE,
    fdcan_ttils: FDCAN_TTILS,
    fdcan_ttost: FDCAN_TTOST,
    fdcan_turna: FDCAN_TURNA,
    fdcan_ttlgt: FDCAN_TTLGT,
    fdcan_ttctc: FDCAN_TTCTC,
    fdcan_ttcpt: FDCAN_TTCPT,
    fdcan_ttcsm: FDCAN_TTCSM,
    _reserved63: [u8; 0x01bc],
    fdcan_ttts: FDCAN_TTTS,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FDCAN_CREL {
        &self.fdcan_crel
    }
    #[doc = "0x04 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FDCAN_ENDN {
        &self.fdcan_endn
    }
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FDCAN_DBTP {
        &self.fdcan_dbtp
    }
    #[doc = "0x10 - FDCAN Test Register"]
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FDCAN_TEST {
        &self.fdcan_test
    }
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FDCAN_RWD {
        &self.fdcan_rwd
    }
    #[doc = "0x18 - FDCAN CC Control Register"]
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FDCAN_CCCR {
        &self.fdcan_cccr
    }
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FDCAN_NBTP {
        &self.fdcan_nbtp
    }
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FDCAN_TSCC {
        &self.fdcan_tscc
    }
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FDCAN_TSCV {
        &self.fdcan_tscv
    }
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FDCAN_TOCC {
        &self.fdcan_tocc
    }
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FDCAN_TOCV {
        &self.fdcan_tocv
    }
    #[doc = "0x40 - FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FDCAN_ECR {
        &self.fdcan_ecr
    }
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FDCAN_PSR {
        &self.fdcan_psr
    }
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FDCAN_TDCR {
        &self.fdcan_tdcr
    }
    #[doc = "0x50 - FDCAN Interrupt Register"]
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FDCAN_IR {
        &self.fdcan_ir
    }
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FDCAN_IE {
        &self.fdcan_ie
    }
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FDCAN_ILS {
        &self.fdcan_ils
    }
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FDCAN_ILE {
        &self.fdcan_ile
    }
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_gfc(&self) -> &FDCAN_GFC {
        &self.fdcan_gfc
    }
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_sidfc(&self) -> &FDCAN_SIDFC {
        &self.fdcan_sidfc
    }
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_xidfc(&self) -> &FDCAN_XIDFC {
        &self.fdcan_xidfc
    }
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    #[inline(always)]
    pub const fn fdcan_ndat1(&self) -> &FDCAN_NDAT1 {
        &self.fdcan_ndat1
    }
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    #[inline(always)]
    pub const fn fdcan_ndat2(&self) -> &FDCAN_NDAT2 {
        &self.fdcan_ndat2
    }
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0c(&self) -> &FDCAN_RXF0C {
        &self.fdcan_rxf0c
    }
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxbc(&self) -> &FDCAN_RXBC {
        &self.fdcan_rxbc
    }
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1c(&self) -> &FDCAN_RXF1C {
        &self.fdcan_rxf1c
    }
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxesc(&self) -> &FDCAN_RXESC {
        &self.fdcan_rxesc
    }
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txesc(&self) -> &FDCAN_TXESC {
        &self.fdcan_txesc
    }
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FDCAN_TXBRP {
        &self.fdcan_txbrp
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txefc(&self) -> &FDCAN_TXEFC {
        &self.fdcan_txefc
    }
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tttmc(&self) -> &FDCAN_TTTMC {
        &self.fdcan_tttmc
    }
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_ttrmc(&self) -> &FDCAN_TTRMC {
        &self.fdcan_ttrmc
    }
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_ttocf(&self) -> &FDCAN_TTOCF {
        &self.fdcan_ttocf
    }
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    #[inline(always)]
    pub const fn fdcan_ttmlm(&self) -> &FDCAN_TTMLM {
        &self.fdcan_ttmlm
    }
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_turcf(&self) -> &FDCAN_TURCF {
        &self.fdcan_turcf
    }
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    #[inline(always)]
    pub const fn fdcan_ttocn(&self) -> &FDCAN_TTOCN {
        &self.fdcan_ttocn
    }
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    #[inline(always)]
    pub const fn can_ttgtp(&self) -> &CAN_TTGTP {
        &self.can_ttgtp
    }
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    #[inline(always)]
    pub const fn fdcan_tttmk(&self) -> &FDCAN_TTTMK {
        &self.fdcan_tttmk
    }
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    #[inline(always)]
    pub const fn fdcan_ttir(&self) -> &FDCAN_TTIR {
        &self.fdcan_ttir
    }
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ttie(&self) -> &FDCAN_TTIE {
        &self.fdcan_ttie
    }
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn fdcan_ttils(&self) -> &FDCAN_TTILS {
        &self.fdcan_ttils
    }
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    #[inline(always)]
    pub const fn fdcan_ttost(&self) -> &FDCAN_TTOST {
        &self.fdcan_ttost
    }
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    #[inline(always)]
    pub const fn fdcan_turna(&self) -> &FDCAN_TURNA {
        &self.fdcan_turna
    }
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    #[inline(always)]
    pub const fn fdcan_ttlgt(&self) -> &FDCAN_TTLGT {
        &self.fdcan_ttlgt
    }
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    #[inline(always)]
    pub const fn fdcan_ttctc(&self) -> &FDCAN_TTCTC {
        &self.fdcan_ttctc
    }
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    #[inline(always)]
    pub const fn fdcan_ttcpt(&self) -> &FDCAN_TTCPT {
        &self.fdcan_ttcpt
    }
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    #[inline(always)]
    pub const fn fdcan_ttcsm(&self) -> &FDCAN_TTCSM {
        &self.fdcan_ttcsm
    }
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    #[inline(always)]
    pub const fn fdcan_ttts(&self) -> &FDCAN_TTTS {
        &self.fdcan_ttts
    }
}
#[doc = "FDCAN_CREL (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_crel`]
module"]
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CRELrs>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_crel;
#[doc = "FDCAN_ENDN (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_endn`]
module"]
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDNrs>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_endn;
#[doc = "FDCAN_DBTP (r) register accessor: FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_dbtp`]
module"]
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (r) register accessor: FDCAN Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_test`]
module"]
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
#[doc = "FDCAN Test Register"]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (r) register accessor: FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rwd`]
module"]
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWDrs>;
#[doc = "FDCAN RAM Watchdog Register"]
pub mod fdcan_rwd;
#[doc = "FDCAN_CCCR (rw) register accessor: FDCAN CC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_cccr`]
module"]
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCRrs>;
#[doc = "FDCAN CC Control Register"]
pub mod fdcan_cccr;
#[doc = "FDCAN_NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_nbtp`]
module"]
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTPrs>;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod fdcan_nbtp;
#[doc = "FDCAN_TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscc`]
module"]
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCCrs>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod fdcan_tscc;
#[doc = "FDCAN_TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscv`]
module"]
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCVrs>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod fdcan_tscv;
#[doc = "FDCAN_TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocc`]
module"]
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCCrs>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod fdcan_tocc;
#[doc = "FDCAN_TOCV (rw) register accessor: FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocv`]
module"]
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCVrs>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod fdcan_tocv;
#[doc = "FDCAN_ECR (rw) register accessor: FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ecr`]
module"]
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECRrs>;
#[doc = "FDCAN Error Counter Register"]
pub mod fdcan_ecr;
#[doc = "FDCAN_PSR (rw) register accessor: FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_psr`]
module"]
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSRrs>;
#[doc = "FDCAN Protocol Status Register"]
pub mod fdcan_psr;
#[doc = "FDCAN_TDCR (r) register accessor: FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tdcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tdcr`]
module"]
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (r) register accessor: FDCAN Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ir`]
module"]
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
#[doc = "FDCAN Interrupt Register"]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (rw) register accessor: FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ie`]
module"]
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (r) register accessor: FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ils`]
module"]
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: FDCAN Interrupt Line Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ile`]
module"]
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod fdcan_ile;
#[doc = "FDCAN_GFC (rw) register accessor: FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_gfc`]
module"]
pub type FDCAN_GFC = crate::Reg<fdcan_gfc::FDCAN_GFCrs>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod fdcan_gfc;
#[doc = "FDCAN_SIDFC (rw) register accessor: FDCAN Standard ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_sidfc`]
module"]
pub type FDCAN_SIDFC = crate::Reg<fdcan_sidfc::FDCAN_SIDFCrs>;
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod fdcan_sidfc;
#[doc = "FDCAN_XIDFC (rw) register accessor: FDCAN Extended ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidfc`]
module"]
pub type FDCAN_XIDFC = crate::Reg<fdcan_xidfc::FDCAN_XIDFCrs>;
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod fdcan_xidfc;
#[doc = "FDCAN_XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidam`]
module"]
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAMrs>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod fdcan_xidam;
#[doc = "FDCAN_HPMS (r) register accessor: FDCAN High Priority Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_hpms`]
module"]
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMSrs>;
#[doc = "FDCAN High Priority Message Status Register"]
pub mod fdcan_hpms;
#[doc = "FDCAN_NDAT1 (r) register accessor: FDCAN New Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ndat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat1`]
module"]
pub type FDCAN_NDAT1 = crate::Reg<fdcan_ndat1::FDCAN_NDAT1rs>;
#[doc = "FDCAN New Data 1 Register"]
pub mod fdcan_ndat1;
#[doc = "FDCAN_NDAT2 (r) register accessor: FDCAN New Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ndat2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat2`]
module"]
pub type FDCAN_NDAT2 = crate::Reg<fdcan_ndat2::FDCAN_NDAT2rs>;
#[doc = "FDCAN New Data 2 Register"]
pub mod fdcan_ndat2;
#[doc = "FDCAN_RXF0C (rw) register accessor: FDCAN Rx FIFO 0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0c`]
module"]
pub type FDCAN_RXF0C = crate::Reg<fdcan_rxf0c::FDCAN_RXF0Crs>;
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod fdcan_rxf0c;
#[doc = "FDCAN_RXF0S (rw) register accessor: FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0s`]
module"]
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0Srs>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod fdcan_rxf0s;
#[doc = "FDCAN_RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0a`]
module"]
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0Ars>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN_RXBC (rw) register accessor: FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxbc`]
module"]
pub type FDCAN_RXBC = crate::Reg<fdcan_rxbc::FDCAN_RXBCrs>;
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod fdcan_rxbc;
#[doc = "FDCAN_RXF1C (rw) register accessor: FDCAN Rx FIFO 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1c`]
module"]
pub type FDCAN_RXF1C = crate::Reg<fdcan_rxf1c::FDCAN_RXF1Crs>;
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod fdcan_rxf1c;
#[doc = "FDCAN_RXF1S (rw) register accessor: FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1s`]
module"]
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1Srs>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1a`]
module"]
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1Ars>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN_RXESC (rw) register accessor: FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxesc`]
module"]
pub type FDCAN_RXESC = crate::Reg<fdcan_rxesc::FDCAN_RXESCrs>;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod fdcan_rxesc;
#[doc = "FDCAN_TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbc`]
module"]
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod fdcan_txbc;
#[doc = "FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txfqs`]
module"]
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod fdcan_txfqs;
#[doc = "FDCAN_TXESC (rw) register accessor: FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txesc`]
module"]
pub type FDCAN_TXESC = crate::Reg<fdcan_txesc::FDCAN_TXESCrs>;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod fdcan_txesc;
#[doc = "FDCAN_TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbrp`]
module"]
pub type FDCAN_TXBRP = crate::Reg<fdcan_txbrp::FDCAN_TXBRPrs>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod fdcan_txbrp;
#[doc = "FDCAN_TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbar`]
module"]
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBARrs>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod fdcan_txbar;
#[doc = "FDCAN_TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcr`]
module"]
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCRrs>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN_TXBTO (rw) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbto`]
module"]
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTOrs>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod fdcan_txbto;
#[doc = "FDCAN_TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcf`]
module"]
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCFrs>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN_TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbtie`]
module"]
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIErs>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN_TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcie`]
module"]
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIErs>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN_TXEFC (rw) register accessor: FDCAN Tx Event FIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefc`]
module"]
pub type FDCAN_TXEFC = crate::Reg<fdcan_txefc::FDCAN_TXEFCrs>;
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod fdcan_txefc;
#[doc = "FDCAN_TXEFS (rw) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefs`]
module"]
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefa`]
module"]
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_TTTMC (rw) register accessor: FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmc`]
module"]
pub type FDCAN_TTTMC = crate::Reg<fdcan_tttmc::FDCAN_TTTMCrs>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod fdcan_tttmc;
#[doc = "FDCAN_TTRMC (rw) register accessor: FDCAN TT Reference Message Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttrmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttrmc`]
module"]
pub type FDCAN_TTRMC = crate::Reg<fdcan_ttrmc::FDCAN_TTRMCrs>;
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod fdcan_ttrmc;
#[doc = "FDCAN_TTOCF (rw) register accessor: FDCAN TT Operation Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocf`]
module"]
pub type FDCAN_TTOCF = crate::Reg<fdcan_ttocf::FDCAN_TTOCFrs>;
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod fdcan_ttocf;
#[doc = "FDCAN_TTMLM (rw) register accessor: FDCAN TT Matrix Limits Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttmlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttmlm`]
module"]
pub type FDCAN_TTMLM = crate::Reg<fdcan_ttmlm::FDCAN_TTMLMrs>;
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod fdcan_ttmlm;
#[doc = "FDCAN_TURCF (rw) register accessor: FDCAN TUR Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_turcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_turcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turcf`]
module"]
pub type FDCAN_TURCF = crate::Reg<fdcan_turcf::FDCAN_TURCFrs>;
#[doc = "FDCAN TUR Configuration Register"]
pub mod fdcan_turcf;
#[doc = "FDCAN_TTOCN (rw) register accessor: FDCAN TT Operation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocn`]
module"]
pub type FDCAN_TTOCN = crate::Reg<fdcan_ttocn::FDCAN_TTOCNrs>;
#[doc = "FDCAN TT Operation Control Register"]
pub mod fdcan_ttocn;
#[doc = "CAN_TTGTP (rw) register accessor: FDCAN TT Global Time Preset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ttgtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ttgtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ttgtp`]
module"]
pub type CAN_TTGTP = crate::Reg<can_ttgtp::CAN_TTGTPrs>;
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod can_ttgtp;
#[doc = "FDCAN_TTTMK (rw) register accessor: FDCAN TT Time Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmk`]
module"]
pub type FDCAN_TTTMK = crate::Reg<fdcan_tttmk::FDCAN_TTTMKrs>;
#[doc = "FDCAN TT Time Mark Register"]
pub mod fdcan_tttmk;
#[doc = "FDCAN_TTIR (rw) register accessor: FDCAN TT Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttir`]
module"]
pub type FDCAN_TTIR = crate::Reg<fdcan_ttir::FDCAN_TTIRrs>;
#[doc = "FDCAN TT Interrupt Register"]
pub mod fdcan_ttir;
#[doc = "FDCAN_TTIE (rw) register accessor: FDCAN TT Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttie`]
module"]
pub type FDCAN_TTIE = crate::Reg<fdcan_ttie::FDCAN_TTIErs>;
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod fdcan_ttie;
#[doc = "FDCAN_TTILS (rw) register accessor: FDCAN TT Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttils`]
module"]
pub type FDCAN_TTILS = crate::Reg<fdcan_ttils::FDCAN_TTILSrs>;
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod fdcan_ttils;
#[doc = "FDCAN_TTOST (r) register accessor: FDCAN TT Operation Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttost::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttost`]
module"]
pub type FDCAN_TTOST = crate::Reg<fdcan_ttost::FDCAN_TTOSTrs>;
#[doc = "FDCAN TT Operation Status Register"]
pub mod fdcan_ttost;
#[doc = "FDCAN_TURNA (r) register accessor: FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_turna::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turna`]
module"]
pub type FDCAN_TURNA = crate::Reg<fdcan_turna::FDCAN_TURNArs>;
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod fdcan_turna;
#[doc = "FDCAN_TTLGT (r) register accessor: FDCAN TT Local and Global Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttlgt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttlgt`]
module"]
pub type FDCAN_TTLGT = crate::Reg<fdcan_ttlgt::FDCAN_TTLGTrs>;
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod fdcan_ttlgt;
#[doc = "FDCAN_TTCTC (r) register accessor: FDCAN TT Cycle Time and Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttctc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttctc`]
module"]
pub type FDCAN_TTCTC = crate::Reg<fdcan_ttctc::FDCAN_TTCTCrs>;
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod fdcan_ttctc;
#[doc = "FDCAN_TTCPT (r) register accessor: FDCAN TT Capture Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcpt`]
module"]
pub type FDCAN_TTCPT = crate::Reg<fdcan_ttcpt::FDCAN_TTCPTrs>;
#[doc = "FDCAN TT Capture Time Register"]
pub mod fdcan_ttcpt;
#[doc = "FDCAN_TTCSM (r) register accessor: FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcsm`]
module"]
pub type FDCAN_TTCSM = crate::Reg<fdcan_ttcsm::FDCAN_TTCSMrs>;
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod fdcan_ttcsm;
#[doc = "FDCAN_TTTS (rw) register accessor: FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttts`]
module"]
pub type FDCAN_TTTS = crate::Reg<fdcan_ttts::FDCAN_TTTSrs>;
#[doc = "FDCAN TT Trigger Select Register"]
pub mod fdcan_ttts;
