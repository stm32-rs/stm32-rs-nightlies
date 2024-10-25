#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - FDCAN Core Release Register
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FDCAN_CREL {
        &self.fdcan_crel
    }
    ///0x04 - FDCAN Core Release Register
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FDCAN_ENDN {
        &self.fdcan_endn
    }
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FDCAN_DBTP {
        &self.fdcan_dbtp
    }
    ///0x10 - FDCAN Test Register
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FDCAN_TEST {
        &self.fdcan_test
    }
    ///0x14 - FDCAN RAM Watchdog Register
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FDCAN_RWD {
        &self.fdcan_rwd
    }
    ///0x18 - FDCAN CC Control Register
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FDCAN_CCCR {
        &self.fdcan_cccr
    }
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FDCAN_NBTP {
        &self.fdcan_nbtp
    }
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FDCAN_TSCC {
        &self.fdcan_tscc
    }
    ///0x24 - FDCAN Timestamp Counter Value Register
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FDCAN_TSCV {
        &self.fdcan_tscv
    }
    ///0x28 - FDCAN Timeout Counter Configuration Register
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FDCAN_TOCC {
        &self.fdcan_tocc
    }
    ///0x2c - FDCAN Timeout Counter Value Register
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FDCAN_TOCV {
        &self.fdcan_tocv
    }
    ///0x40 - FDCAN Error Counter Register
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FDCAN_ECR {
        &self.fdcan_ecr
    }
    ///0x44 - FDCAN Protocol Status Register
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FDCAN_PSR {
        &self.fdcan_psr
    }
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FDCAN_TDCR {
        &self.fdcan_tdcr
    }
    ///0x50 - FDCAN Interrupt Register
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FDCAN_IR {
        &self.fdcan_ir
    }
    ///0x54 - FDCAN Interrupt Enable Register
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FDCAN_IE {
        &self.fdcan_ie
    }
    ///0x58 - FDCAN Interrupt Line Select Register
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FDCAN_ILS {
        &self.fdcan_ils
    }
    ///0x5c - FDCAN Interrupt Line Enable Register
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FDCAN_ILE {
        &self.fdcan_ile
    }
    ///0x80 - FDCAN Global Filter Configuration Register
    #[inline(always)]
    pub const fn fdcan_gfc(&self) -> &FDCAN_GFC {
        &self.fdcan_gfc
    }
    ///0x84 - FDCAN Standard ID Filter Configuration Register
    #[inline(always)]
    pub const fn fdcan_sidfc(&self) -> &FDCAN_SIDFC {
        &self.fdcan_sidfc
    }
    ///0x88 - FDCAN Extended ID Filter Configuration Register
    #[inline(always)]
    pub const fn fdcan_xidfc(&self) -> &FDCAN_XIDFC {
        &self.fdcan_xidfc
    }
    ///0x90 - FDCAN Extended ID and Mask Register
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    ///0x94 - FDCAN High Priority Message Status Register
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    ///0x98 - FDCAN New Data 1 Register
    #[inline(always)]
    pub const fn fdcan_ndat1(&self) -> &FDCAN_NDAT1 {
        &self.fdcan_ndat1
    }
    ///0x9c - FDCAN New Data 2 Register
    #[inline(always)]
    pub const fn fdcan_ndat2(&self) -> &FDCAN_NDAT2 {
        &self.fdcan_ndat2
    }
    ///0xa0 - FDCAN Rx FIFO 0 Configuration Register
    #[inline(always)]
    pub const fn fdcan_rxf0c(&self) -> &FDCAN_RXF0C {
        &self.fdcan_rxf0c
    }
    ///0xa4 - FDCAN Rx FIFO 0 Status Register
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    ///0xa8 - CAN Rx FIFO 0 Acknowledge Register
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    ///0xac - FDCAN Rx Buffer Configuration Register
    #[inline(always)]
    pub const fn fdcan_rxbc(&self) -> &FDCAN_RXBC {
        &self.fdcan_rxbc
    }
    ///0xb0 - FDCAN Rx FIFO 1 Configuration Register
    #[inline(always)]
    pub const fn fdcan_rxf1c(&self) -> &FDCAN_RXF1C {
        &self.fdcan_rxf1c
    }
    ///0xb4 - FDCAN Rx FIFO 1 Status Register
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    ///0xb8 - FDCAN Rx FIFO 1 Acknowledge Register
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    ///0xbc - FDCAN Rx Buffer Element Size Configuration Register
    #[inline(always)]
    pub const fn fdcan_rxesc(&self) -> &FDCAN_RXESC {
        &self.fdcan_rxesc
    }
    ///0xc0 - FDCAN Tx Buffer Configuration Register
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    ///0xc8 - FDCAN Tx Buffer Element Size Configuration Register
    #[inline(always)]
    pub const fn fdcan_txesc(&self) -> &FDCAN_TXESC {
        &self.fdcan_txesc
    }
    ///0xcc - FDCAN Tx Buffer Request Pending Register
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FDCAN_TXBRP {
        &self.fdcan_txbrp
    }
    ///0xd0 - FDCAN Tx Buffer Add Request Register
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    ///0xd4 - FDCAN Tx Buffer Cancellation Request Register
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    ///0xd8 - FDCAN Tx Buffer Transmission Occurred Register
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    ///0xdc - FDCAN Tx Buffer Cancellation Finished Register
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    ///0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    ///0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    ///0xf0 - FDCAN Tx Event FIFO Configuration Register
    #[inline(always)]
    pub const fn fdcan_txefc(&self) -> &FDCAN_TXEFC {
        &self.fdcan_txefc
    }
    ///0xf4 - FDCAN Tx Event FIFO Status Register
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    ///0xf8 - FDCAN Tx Event FIFO Acknowledge Register
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    ///0x100 - FDCAN TT Trigger Memory Configuration Register
    #[inline(always)]
    pub const fn fdcan_tttmc(&self) -> &FDCAN_TTTMC {
        &self.fdcan_tttmc
    }
    ///0x104 - FDCAN TT Reference Message Configuration Register
    #[inline(always)]
    pub const fn fdcan_ttrmc(&self) -> &FDCAN_TTRMC {
        &self.fdcan_ttrmc
    }
    ///0x108 - FDCAN TT Operation Configuration Register
    #[inline(always)]
    pub const fn fdcan_ttocf(&self) -> &FDCAN_TTOCF {
        &self.fdcan_ttocf
    }
    ///0x10c - FDCAN TT Matrix Limits Register
    #[inline(always)]
    pub const fn fdcan_ttmlm(&self) -> &FDCAN_TTMLM {
        &self.fdcan_ttmlm
    }
    ///0x110 - FDCAN TUR Configuration Register
    #[inline(always)]
    pub const fn fdcan_turcf(&self) -> &FDCAN_TURCF {
        &self.fdcan_turcf
    }
    ///0x114 - FDCAN TT Operation Control Register
    #[inline(always)]
    pub const fn fdcan_ttocn(&self) -> &FDCAN_TTOCN {
        &self.fdcan_ttocn
    }
    ///0x118 - FDCAN TT Global Time Preset Register
    #[inline(always)]
    pub const fn can_ttgtp(&self) -> &CAN_TTGTP {
        &self.can_ttgtp
    }
    ///0x11c - FDCAN TT Time Mark Register
    #[inline(always)]
    pub const fn fdcan_tttmk(&self) -> &FDCAN_TTTMK {
        &self.fdcan_tttmk
    }
    ///0x120 - FDCAN TT Interrupt Register
    #[inline(always)]
    pub const fn fdcan_ttir(&self) -> &FDCAN_TTIR {
        &self.fdcan_ttir
    }
    ///0x124 - FDCAN TT Interrupt Enable Register
    #[inline(always)]
    pub const fn fdcan_ttie(&self) -> &FDCAN_TTIE {
        &self.fdcan_ttie
    }
    ///0x128 - FDCAN TT Interrupt Line Select Register
    #[inline(always)]
    pub const fn fdcan_ttils(&self) -> &FDCAN_TTILS {
        &self.fdcan_ttils
    }
    ///0x12c - FDCAN TT Operation Status Register
    #[inline(always)]
    pub const fn fdcan_ttost(&self) -> &FDCAN_TTOST {
        &self.fdcan_ttost
    }
    ///0x130 - FDCAN TUR Numerator Actual Register
    #[inline(always)]
    pub const fn fdcan_turna(&self) -> &FDCAN_TURNA {
        &self.fdcan_turna
    }
    ///0x134 - FDCAN TT Local and Global Time Register
    #[inline(always)]
    pub const fn fdcan_ttlgt(&self) -> &FDCAN_TTLGT {
        &self.fdcan_ttlgt
    }
    ///0x138 - FDCAN TT Cycle Time and Count Register
    #[inline(always)]
    pub const fn fdcan_ttctc(&self) -> &FDCAN_TTCTC {
        &self.fdcan_ttctc
    }
    ///0x13c - FDCAN TT Capture Time Register
    #[inline(always)]
    pub const fn fdcan_ttcpt(&self) -> &FDCAN_TTCPT {
        &self.fdcan_ttcpt
    }
    ///0x140 - FDCAN TT Cycle Sync Mark Register
    #[inline(always)]
    pub const fn fdcan_ttcsm(&self) -> &FDCAN_TTCSM {
        &self.fdcan_ttcsm
    }
    ///0x300 - FDCAN TT Trigger Select Register
    #[inline(always)]
    pub const fn fdcan_ttts(&self) -> &FDCAN_TTTS {
        &self.fdcan_ttts
    }
}
/**FDCAN_CREL (r) register accessor: FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_CREL)

For information about available fields see [`mod@fdcan_crel`]
module*/
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CRELrs>;
///FDCAN Core Release Register
pub mod fdcan_crel;
/**FDCAN_ENDN (r) register accessor: FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_ENDN)

For information about available fields see [`mod@fdcan_endn`]
module*/
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDNrs>;
///FDCAN Core Release Register
pub mod fdcan_endn;
/**FDCAN_DBTP (r) register accessor: FDCAN Data Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_DBTP)

For information about available fields see [`mod@fdcan_dbtp`]
module*/
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod fdcan_dbtp;
/**FDCAN_TEST (r) register accessor: FDCAN Test Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TEST)

For information about available fields see [`mod@fdcan_test`]
module*/
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
///FDCAN Test Register
pub mod fdcan_test;
/**FDCAN_RWD (r) register accessor: FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RWD)

For information about available fields see [`mod@fdcan_rwd`]
module*/
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWDrs>;
///FDCAN RAM Watchdog Register
pub mod fdcan_rwd;
/**FDCAN_CCCR (rw) register accessor: FDCAN CC Control Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_CCCR)

For information about available fields see [`mod@fdcan_cccr`]
module*/
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCRrs>;
///FDCAN CC Control Register
pub mod fdcan_cccr;
/**FDCAN_NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_NBTP)

For information about available fields see [`mod@fdcan_nbtp`]
module*/
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTPrs>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod fdcan_nbtp;
/**FDCAN_TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TSCC)

For information about available fields see [`mod@fdcan_tscc`]
module*/
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCCrs>;
///FDCAN Timestamp Counter Configuration Register
pub mod fdcan_tscc;
/**FDCAN_TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TSCV)

For information about available fields see [`mod@fdcan_tscv`]
module*/
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCVrs>;
///FDCAN Timestamp Counter Value Register
pub mod fdcan_tscv;
/**FDCAN_TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TOCC)

For information about available fields see [`mod@fdcan_tocc`]
module*/
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCCrs>;
///FDCAN Timeout Counter Configuration Register
pub mod fdcan_tocc;
/**FDCAN_TOCV (rw) register accessor: FDCAN Timeout Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TOCV)

For information about available fields see [`mod@fdcan_tocv`]
module*/
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCVrs>;
///FDCAN Timeout Counter Value Register
pub mod fdcan_tocv;
/**FDCAN_ECR (rw) register accessor: FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_ECR)

For information about available fields see [`mod@fdcan_ecr`]
module*/
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECRrs>;
///FDCAN Error Counter Register
pub mod fdcan_ecr;
/**FDCAN_PSR (rw) register accessor: FDCAN Protocol Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_PSR)

For information about available fields see [`mod@fdcan_psr`]
module*/
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSRrs>;
///FDCAN Protocol Status Register
pub mod fdcan_psr;
/**FDCAN_TDCR (r) register accessor: FDCAN Transmitter Delay Compensation Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TDCR)

For information about available fields see [`mod@fdcan_tdcr`]
module*/
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
///FDCAN Transmitter Delay Compensation Register
pub mod fdcan_tdcr;
/**FDCAN_IR (r) register accessor: FDCAN Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_IR)

For information about available fields see [`mod@fdcan_ir`]
module*/
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
///FDCAN Interrupt Register
pub mod fdcan_ir;
/**FDCAN_IE (rw) register accessor: FDCAN Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_IE)

For information about available fields see [`mod@fdcan_ie`]
module*/
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
///FDCAN Interrupt Enable Register
pub mod fdcan_ie;
/**FDCAN_ILS (r) register accessor: FDCAN Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_ILS)

For information about available fields see [`mod@fdcan_ils`]
module*/
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
///FDCAN Interrupt Line Select Register
pub mod fdcan_ils;
/**FDCAN_ILE (rw) register accessor: FDCAN Interrupt Line Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_ILE)

For information about available fields see [`mod@fdcan_ile`]
module*/
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
///FDCAN Interrupt Line Enable Register
pub mod fdcan_ile;
/**FDCAN_GFC (rw) register accessor: FDCAN Global Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_GFC)

For information about available fields see [`mod@fdcan_gfc`]
module*/
pub type FDCAN_GFC = crate::Reg<fdcan_gfc::FDCAN_GFCrs>;
///FDCAN Global Filter Configuration Register
pub mod fdcan_gfc;
/**FDCAN_SIDFC (rw) register accessor: FDCAN Standard ID Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_SIDFC)

For information about available fields see [`mod@fdcan_sidfc`]
module*/
pub type FDCAN_SIDFC = crate::Reg<fdcan_sidfc::FDCAN_SIDFCrs>;
///FDCAN Standard ID Filter Configuration Register
pub mod fdcan_sidfc;
/**FDCAN_XIDFC (rw) register accessor: FDCAN Extended ID Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_XIDFC)

For information about available fields see [`mod@fdcan_xidfc`]
module*/
pub type FDCAN_XIDFC = crate::Reg<fdcan_xidfc::FDCAN_XIDFCrs>;
///FDCAN Extended ID Filter Configuration Register
pub mod fdcan_xidfc;
/**FDCAN_XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_XIDAM)

For information about available fields see [`mod@fdcan_xidam`]
module*/
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAMrs>;
///FDCAN Extended ID and Mask Register
pub mod fdcan_xidam;
/**FDCAN_HPMS (r) register accessor: FDCAN High Priority Message Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_HPMS)

For information about available fields see [`mod@fdcan_hpms`]
module*/
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMSrs>;
///FDCAN High Priority Message Status Register
pub mod fdcan_hpms;
/**FDCAN_NDAT1 (r) register accessor: FDCAN New Data 1 Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_NDAT1)

For information about available fields see [`mod@fdcan_ndat1`]
module*/
pub type FDCAN_NDAT1 = crate::Reg<fdcan_ndat1::FDCAN_NDAT1rs>;
///FDCAN New Data 1 Register
pub mod fdcan_ndat1;
/**FDCAN_NDAT2 (r) register accessor: FDCAN New Data 2 Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_NDAT2)

For information about available fields see [`mod@fdcan_ndat2`]
module*/
pub type FDCAN_NDAT2 = crate::Reg<fdcan_ndat2::FDCAN_NDAT2rs>;
///FDCAN New Data 2 Register
pub mod fdcan_ndat2;
/**FDCAN_RXF0C (rw) register accessor: FDCAN Rx FIFO 0 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF0C)

For information about available fields see [`mod@fdcan_rxf0c`]
module*/
pub type FDCAN_RXF0C = crate::Reg<fdcan_rxf0c::FDCAN_RXF0Crs>;
///FDCAN Rx FIFO 0 Configuration Register
pub mod fdcan_rxf0c;
/**FDCAN_RXF0S (rw) register accessor: FDCAN Rx FIFO 0 Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF0S)

For information about available fields see [`mod@fdcan_rxf0s`]
module*/
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0Srs>;
///FDCAN Rx FIFO 0 Status Register
pub mod fdcan_rxf0s;
/**FDCAN_RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF0A)

For information about available fields see [`mod@fdcan_rxf0a`]
module*/
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0Ars>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod fdcan_rxf0a;
/**FDCAN_RXBC (rw) register accessor: FDCAN Rx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXBC)

For information about available fields see [`mod@fdcan_rxbc`]
module*/
pub type FDCAN_RXBC = crate::Reg<fdcan_rxbc::FDCAN_RXBCrs>;
///FDCAN Rx Buffer Configuration Register
pub mod fdcan_rxbc;
/**FDCAN_RXF1C (rw) register accessor: FDCAN Rx FIFO 1 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF1C)

For information about available fields see [`mod@fdcan_rxf1c`]
module*/
pub type FDCAN_RXF1C = crate::Reg<fdcan_rxf1c::FDCAN_RXF1Crs>;
///FDCAN Rx FIFO 1 Configuration Register
pub mod fdcan_rxf1c;
/**FDCAN_RXF1S (rw) register accessor: FDCAN Rx FIFO 1 Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF1S)

For information about available fields see [`mod@fdcan_rxf1s`]
module*/
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1Srs>;
///FDCAN Rx FIFO 1 Status Register
pub mod fdcan_rxf1s;
/**FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXF1A)

For information about available fields see [`mod@fdcan_rxf1a`]
module*/
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1Ars>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod fdcan_rxf1a;
/**FDCAN_RXESC (rw) register accessor: FDCAN Rx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_RXESC)

For information about available fields see [`mod@fdcan_rxesc`]
module*/
pub type FDCAN_RXESC = crate::Reg<fdcan_rxesc::FDCAN_RXESCrs>;
///FDCAN Rx Buffer Element Size Configuration Register
pub mod fdcan_rxesc;
/**FDCAN_TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBC)

For information about available fields see [`mod@fdcan_txbc`]
module*/
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
///FDCAN Tx Buffer Configuration Register
pub mod fdcan_txbc;
/**FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXFQS)

For information about available fields see [`mod@fdcan_txfqs`]
module*/
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
///FDCAN Tx FIFO/Queue Status Register
pub mod fdcan_txfqs;
/**FDCAN_TXESC (rw) register accessor: FDCAN Tx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXESC)

For information about available fields see [`mod@fdcan_txesc`]
module*/
pub type FDCAN_TXESC = crate::Reg<fdcan_txesc::FDCAN_TXESCrs>;
///FDCAN Tx Buffer Element Size Configuration Register
pub mod fdcan_txesc;
/**FDCAN_TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBRP)

For information about available fields see [`mod@fdcan_txbrp`]
module*/
pub type FDCAN_TXBRP = crate::Reg<fdcan_txbrp::FDCAN_TXBRPrs>;
///FDCAN Tx Buffer Request Pending Register
pub mod fdcan_txbrp;
/**FDCAN_TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBAR)

For information about available fields see [`mod@fdcan_txbar`]
module*/
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBARrs>;
///FDCAN Tx Buffer Add Request Register
pub mod fdcan_txbar;
/**FDCAN_TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBCR)

For information about available fields see [`mod@fdcan_txbcr`]
module*/
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCRrs>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod fdcan_txbcr;
/**FDCAN_TXBTO (rw) register accessor: FDCAN Tx Buffer Transmission Occurred Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBTO)

For information about available fields see [`mod@fdcan_txbto`]
module*/
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTOrs>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod fdcan_txbto;
/**FDCAN_TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBCF)

For information about available fields see [`mod@fdcan_txbcf`]
module*/
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCFrs>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod fdcan_txbcf;
/**FDCAN_TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBTIE)

For information about available fields see [`mod@fdcan_txbtie`]
module*/
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIErs>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod fdcan_txbtie;
/**FDCAN_TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBCIE)

For information about available fields see [`mod@fdcan_txbcie`]
module*/
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIErs>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod fdcan_txbcie;
/**FDCAN_TXEFC (rw) register accessor: FDCAN Tx Event FIFO Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXEFC)

For information about available fields see [`mod@fdcan_txefc`]
module*/
pub type FDCAN_TXEFC = crate::Reg<fdcan_txefc::FDCAN_TXEFCrs>;
///FDCAN Tx Event FIFO Configuration Register
pub mod fdcan_txefc;
/**FDCAN_TXEFS (rw) register accessor: FDCAN Tx Event FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXEFS)

For information about available fields see [`mod@fdcan_txefs`]
module*/
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
///FDCAN Tx Event FIFO Status Register
pub mod fdcan_txefs;
/**FDCAN_TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXEFA)

For information about available fields see [`mod@fdcan_txefa`]
module*/
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod fdcan_txefa;
/**FDCAN_TTTMC (rw) register accessor: FDCAN TT Trigger Memory Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTTMC)

For information about available fields see [`mod@fdcan_tttmc`]
module*/
pub type FDCAN_TTTMC = crate::Reg<fdcan_tttmc::FDCAN_TTTMCrs>;
///FDCAN TT Trigger Memory Configuration Register
pub mod fdcan_tttmc;
/**FDCAN_TTRMC (rw) register accessor: FDCAN TT Reference Message Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTRMC)

For information about available fields see [`mod@fdcan_ttrmc`]
module*/
pub type FDCAN_TTRMC = crate::Reg<fdcan_ttrmc::FDCAN_TTRMCrs>;
///FDCAN TT Reference Message Configuration Register
pub mod fdcan_ttrmc;
/**FDCAN_TTOCF (rw) register accessor: FDCAN TT Operation Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTOCF)

For information about available fields see [`mod@fdcan_ttocf`]
module*/
pub type FDCAN_TTOCF = crate::Reg<fdcan_ttocf::FDCAN_TTOCFrs>;
///FDCAN TT Operation Configuration Register
pub mod fdcan_ttocf;
/**FDCAN_TTMLM (rw) register accessor: FDCAN TT Matrix Limits Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTMLM)

For information about available fields see [`mod@fdcan_ttmlm`]
module*/
pub type FDCAN_TTMLM = crate::Reg<fdcan_ttmlm::FDCAN_TTMLMrs>;
///FDCAN TT Matrix Limits Register
pub mod fdcan_ttmlm;
/**FDCAN_TURCF (rw) register accessor: FDCAN TUR Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TURCF)

For information about available fields see [`mod@fdcan_turcf`]
module*/
pub type FDCAN_TURCF = crate::Reg<fdcan_turcf::FDCAN_TURCFrs>;
///FDCAN TUR Configuration Register
pub mod fdcan_turcf;
/**FDCAN_TTOCN (rw) register accessor: FDCAN TT Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTOCN)

For information about available fields see [`mod@fdcan_ttocn`]
module*/
pub type FDCAN_TTOCN = crate::Reg<fdcan_ttocn::FDCAN_TTOCNrs>;
///FDCAN TT Operation Control Register
pub mod fdcan_ttocn;
/**CAN_TTGTP (rw) register accessor: FDCAN TT Global Time Preset Register

You can [`read`](crate::Reg::read) this register and get [`can_ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:CAN_TTGTP)

For information about available fields see [`mod@can_ttgtp`]
module*/
pub type CAN_TTGTP = crate::Reg<can_ttgtp::CAN_TTGTPrs>;
///FDCAN TT Global Time Preset Register
pub mod can_ttgtp;
/**FDCAN_TTTMK (rw) register accessor: FDCAN TT Time Mark Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTTMK)

For information about available fields see [`mod@fdcan_tttmk`]
module*/
pub type FDCAN_TTTMK = crate::Reg<fdcan_tttmk::FDCAN_TTTMKrs>;
///FDCAN TT Time Mark Register
pub mod fdcan_tttmk;
/**FDCAN_TTIR (rw) register accessor: FDCAN TT Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTIR)

For information about available fields see [`mod@fdcan_ttir`]
module*/
pub type FDCAN_TTIR = crate::Reg<fdcan_ttir::FDCAN_TTIRrs>;
///FDCAN TT Interrupt Register
pub mod fdcan_ttir;
/**FDCAN_TTIE (rw) register accessor: FDCAN TT Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTIE)

For information about available fields see [`mod@fdcan_ttie`]
module*/
pub type FDCAN_TTIE = crate::Reg<fdcan_ttie::FDCAN_TTIErs>;
///FDCAN TT Interrupt Enable Register
pub mod fdcan_ttie;
/**FDCAN_TTILS (rw) register accessor: FDCAN TT Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTILS)

For information about available fields see [`mod@fdcan_ttils`]
module*/
pub type FDCAN_TTILS = crate::Reg<fdcan_ttils::FDCAN_TTILSrs>;
///FDCAN TT Interrupt Line Select Register
pub mod fdcan_ttils;
/**FDCAN_TTOST (r) register accessor: FDCAN TT Operation Status Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttost::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTOST)

For information about available fields see [`mod@fdcan_ttost`]
module*/
pub type FDCAN_TTOST = crate::Reg<fdcan_ttost::FDCAN_TTOSTrs>;
///FDCAN TT Operation Status Register
pub mod fdcan_ttost;
/**FDCAN_TURNA (r) register accessor: FDCAN TUR Numerator Actual Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TURNA)

For information about available fields see [`mod@fdcan_turna`]
module*/
pub type FDCAN_TURNA = crate::Reg<fdcan_turna::FDCAN_TURNArs>;
///FDCAN TUR Numerator Actual Register
pub mod fdcan_turna;
/**FDCAN_TTLGT (r) register accessor: FDCAN TT Local and Global Time Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTLGT)

For information about available fields see [`mod@fdcan_ttlgt`]
module*/
pub type FDCAN_TTLGT = crate::Reg<fdcan_ttlgt::FDCAN_TTLGTrs>;
///FDCAN TT Local and Global Time Register
pub mod fdcan_ttlgt;
/**FDCAN_TTCTC (r) register accessor: FDCAN TT Cycle Time and Count Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTCTC)

For information about available fields see [`mod@fdcan_ttctc`]
module*/
pub type FDCAN_TTCTC = crate::Reg<fdcan_ttctc::FDCAN_TTCTCrs>;
///FDCAN TT Cycle Time and Count Register
pub mod fdcan_ttctc;
/**FDCAN_TTCPT (r) register accessor: FDCAN TT Capture Time Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTCPT)

For information about available fields see [`mod@fdcan_ttcpt`]
module*/
pub type FDCAN_TTCPT = crate::Reg<fdcan_ttcpt::FDCAN_TTCPTrs>;
///FDCAN TT Capture Time Register
pub mod fdcan_ttcpt;
/**FDCAN_TTCSM (r) register accessor: FDCAN TT Cycle Sync Mark Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTCSM)

For information about available fields see [`mod@fdcan_ttcsm`]
module*/
pub type FDCAN_TTCSM = crate::Reg<fdcan_ttcsm::FDCAN_TTCSMrs>;
///FDCAN TT Cycle Sync Mark Register
pub mod fdcan_ttcsm;
/**FDCAN_TTTS (rw) register accessor: FDCAN TT Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTTS)

For information about available fields see [`mod@fdcan_ttts`]
module*/
pub type FDCAN_TTTS = crate::Reg<fdcan_ttts::FDCAN_TTTSrs>;
///FDCAN TT Trigger Select Register
pub mod fdcan_ttts;
