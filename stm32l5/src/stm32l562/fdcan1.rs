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
    fdcan_rxgfc: FDCAN_RXGFC,
    fdcan_xidam: FDCAN_XIDAM,
    fdcan_hpms: FDCAN_HPMS,
    _reserved21: [u8; 0x04],
    fdcan_rxf0s: FDCAN_RXF0S,
    fdcan_rxf0a: FDCAN_RXF0A,
    fdcan_rxf1s: FDCAN_RXF1S,
    fdcan_rxf1a: FDCAN_RXF1A,
    _reserved25: [u8; 0x20],
    fdcan_txbc: FDCAN_TXBC,
    fdcan_txfqs: FDCAN_TXFQS,
    fdcan_txbrp: FDCAN_TXBRP,
    fdcan_txbar: FDCAN_TXBAR,
    fdcan_txbcr: FDCAN_TXBCR,
    fdcan_txbto: FDCAN_TXBTO,
    fdcan_txbcf: FDCAN_TXBCF,
    fdcan_txbtie: FDCAN_TXBTIE,
    fdcan_txbcie: FDCAN_TXBCIE,
    fdcan_txefs: FDCAN_TXEFS,
    fdcan_txefa: FDCAN_TXEFA,
    _reserved36: [u8; 0x14],
    fdcan_ckdiv: FDCAN_CKDIV,
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
    pub const fn fdcan_rxgfc(&self) -> &FDCAN_RXGFC {
        &self.fdcan_rxgfc
    }
    #[doc = "0x84 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    #[doc = "0x88 - FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    #[doc = "0x94 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FDCAN_TXBRP {
        &self.fdcan_txbrp
    }
    #[doc = "0xcc - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    #[doc = "0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    #[doc = "0xe4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    #[doc = "0xe8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_ckdiv(&self) -> &FDCAN_CKDIV {
        &self.fdcan_ckdiv
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
#[doc = "FDCAN_DBTP (rw) register accessor: FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_dbtp`]
module"]
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (rw) register accessor: FDCAN Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_test`]
module"]
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
#[doc = "FDCAN Test Register"]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (rw) register accessor: FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rwd`]
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
#[doc = "FDCAN_TDCR (rw) register accessor: FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tdcr`]
module"]
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (rw) register accessor: FDCAN Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ir`]
module"]
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
#[doc = "FDCAN Interrupt Register"]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (rw) register accessor: FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ie`]
module"]
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (rw) register accessor: FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ils`]
module"]
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: FDCAN Interrupt Line Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ile`]
module"]
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod fdcan_ile;
#[doc = "FDCAN_RXGFC (rw) register accessor: FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxgfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxgfc`]
module"]
pub type FDCAN_RXGFC = crate::Reg<fdcan_rxgfc::FDCAN_RXGFCrs>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod fdcan_rxgfc;
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
#[doc = "FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txfqs`]
module"]
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod fdcan_txfqs;
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
#[doc = "FDCAN_TXBTO (r) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbto`]
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
#[doc = "FDCAN_TXEFS (r) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefs`]
module"]
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefa`]
module"]
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_CKDIV (rw) register accessor: FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ckdiv`]
module"]
pub type FDCAN_CKDIV = crate::Reg<fdcan_ckdiv::FDCAN_CKDIVrs>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod fdcan_ckdiv;
#[doc = "FDCAN_TXBC (rw) register accessor: FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbc`]
module"]
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod fdcan_txbc;
