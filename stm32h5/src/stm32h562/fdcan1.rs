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
    ///0x00 - FDCAN core release register
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FDCAN_CREL {
        &self.fdcan_crel
    }
    ///0x04 - FDCAN endian register
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FDCAN_ENDN {
        &self.fdcan_endn
    }
    ///0x0c - FDCAN data bit timing and prescaler register
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FDCAN_DBTP {
        &self.fdcan_dbtp
    }
    ///0x10 - FDCAN test register
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FDCAN_TEST {
        &self.fdcan_test
    }
    ///0x14 - FDCAN RAM watchdog register
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FDCAN_RWD {
        &self.fdcan_rwd
    }
    ///0x18 - FDCAN CC control register
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FDCAN_CCCR {
        &self.fdcan_cccr
    }
    ///0x1c - FDCAN nominal bit timing and prescaler register
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FDCAN_NBTP {
        &self.fdcan_nbtp
    }
    ///0x20 - FDCAN timestamp counter configuration register
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FDCAN_TSCC {
        &self.fdcan_tscc
    }
    ///0x24 - FDCAN timestamp counter value register
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FDCAN_TSCV {
        &self.fdcan_tscv
    }
    ///0x28 - FDCAN timeout counter configuration register
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FDCAN_TOCC {
        &self.fdcan_tocc
    }
    ///0x2c - FDCAN timeout counter value register
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FDCAN_TOCV {
        &self.fdcan_tocv
    }
    ///0x40 - FDCAN error counter register
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FDCAN_ECR {
        &self.fdcan_ecr
    }
    ///0x44 - FDCAN protocol status register
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FDCAN_PSR {
        &self.fdcan_psr
    }
    ///0x48 - FDCAN transmitter delay compensation register
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FDCAN_TDCR {
        &self.fdcan_tdcr
    }
    ///0x50 - FDCAN interrupt register
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FDCAN_IR {
        &self.fdcan_ir
    }
    ///0x54 - FDCAN interrupt enable register
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FDCAN_IE {
        &self.fdcan_ie
    }
    ///0x58 - FDCAN interrupt line select register
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FDCAN_ILS {
        &self.fdcan_ils
    }
    ///0x5c - FDCAN interrupt line enable register
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FDCAN_ILE {
        &self.fdcan_ile
    }
    ///0x80 - FDCAN global filter configuration register
    #[inline(always)]
    pub const fn fdcan_rxgfc(&self) -> &FDCAN_RXGFC {
        &self.fdcan_rxgfc
    }
    ///0x84 - FDCAN extended ID and mask register
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    ///0x88 - FDCAN high-priority message status register
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    ///0x90 - FDCAN Rx FIFO 0 status register
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    ///0x94 - CAN Rx FIFO 0 acknowledge register
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    ///0x98 - FDCAN Rx FIFO 1 status register
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    ///0x9c - FDCAN Rx FIFO 1 acknowledge register
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    ///0xc0 - FDCAN Tx buffer configuration register
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    ///0xc4 - FDCAN Tx FIFO/queue status register
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    ///0xc8 - FDCAN Tx buffer request pending register
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FDCAN_TXBRP {
        &self.fdcan_txbrp
    }
    ///0xcc - FDCAN Tx buffer add request register
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    ///0xd0 - FDCAN Tx buffer cancellation request register
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    ///0xd4 - FDCAN Tx buffer transmission occurred register
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    ///0xd8 - FDCAN Tx buffer cancellation finished register
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    ///0xdc - FDCAN Tx buffer transmission interrupt enable register
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    ///0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    ///0xe4 - FDCAN Tx event FIFO status register
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    ///0xe8 - FDCAN Tx event FIFO acknowledge register
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    ///0x100 - FDCAN CFG clock divider register
    #[inline(always)]
    pub const fn fdcan_ckdiv(&self) -> &FDCAN_CKDIV {
        &self.fdcan_ckdiv
    }
}
/**FDCAN_CREL (r) register accessor: FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`fdcan_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_CREL)

For information about available fields see [`mod@fdcan_crel`]
module*/
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CRELrs>;
///FDCAN core release register
pub mod fdcan_crel;
/**FDCAN_ENDN (r) register accessor: FDCAN endian register

You can [`read`](crate::Reg::read) this register and get [`fdcan_endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_ENDN)

For information about available fields see [`mod@fdcan_endn`]
module*/
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDNrs>;
///FDCAN endian register
pub mod fdcan_endn;
/**FDCAN_DBTP (rw) register accessor: FDCAN data bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_DBTP)

For information about available fields see [`mod@fdcan_dbtp`]
module*/
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
///FDCAN data bit timing and prescaler register
pub mod fdcan_dbtp;
/**FDCAN_TEST (rw) register accessor: FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TEST)

For information about available fields see [`mod@fdcan_test`]
module*/
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
///FDCAN test register
pub mod fdcan_test;
/**FDCAN_RWD (rw) register accessor: FDCAN RAM watchdog register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RWD)

For information about available fields see [`mod@fdcan_rwd`]
module*/
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWDrs>;
///FDCAN RAM watchdog register
pub mod fdcan_rwd;
/**FDCAN_CCCR (rw) register accessor: FDCAN CC control register

You can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_CCCR)

For information about available fields see [`mod@fdcan_cccr`]
module*/
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCRrs>;
///FDCAN CC control register
pub mod fdcan_cccr;
/**FDCAN_NBTP (rw) register accessor: FDCAN nominal bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_NBTP)

For information about available fields see [`mod@fdcan_nbtp`]
module*/
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTPrs>;
///FDCAN nominal bit timing and prescaler register
pub mod fdcan_nbtp;
/**FDCAN_TSCC (rw) register accessor: FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TSCC)

For information about available fields see [`mod@fdcan_tscc`]
module*/
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCCrs>;
///FDCAN timestamp counter configuration register
pub mod fdcan_tscc;
/**FDCAN_TSCV (rw) register accessor: FDCAN timestamp counter value register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TSCV)

For information about available fields see [`mod@fdcan_tscv`]
module*/
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCVrs>;
///FDCAN timestamp counter value register
pub mod fdcan_tscv;
/**FDCAN_TOCC (rw) register accessor: FDCAN timeout counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TOCC)

For information about available fields see [`mod@fdcan_tocc`]
module*/
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCCrs>;
///FDCAN timeout counter configuration register
pub mod fdcan_tocc;
/**FDCAN_TOCV (rw) register accessor: FDCAN timeout counter value register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TOCV)

For information about available fields see [`mod@fdcan_tocv`]
module*/
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCVrs>;
///FDCAN timeout counter value register
pub mod fdcan_tocv;
/**FDCAN_ECR (rw) register accessor: FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_ECR)

For information about available fields see [`mod@fdcan_ecr`]
module*/
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECRrs>;
///FDCAN error counter register
pub mod fdcan_ecr;
/**FDCAN_PSR (rw) register accessor: FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_PSR)

For information about available fields see [`mod@fdcan_psr`]
module*/
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSRrs>;
///FDCAN protocol status register
pub mod fdcan_psr;
/**FDCAN_TDCR (rw) register accessor: FDCAN transmitter delay compensation register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TDCR)

For information about available fields see [`mod@fdcan_tdcr`]
module*/
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
///FDCAN transmitter delay compensation register
pub mod fdcan_tdcr;
/**FDCAN_IR (rw) register accessor: FDCAN interrupt register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_IR)

For information about available fields see [`mod@fdcan_ir`]
module*/
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
///FDCAN interrupt register
pub mod fdcan_ir;
/**FDCAN_IE (rw) register accessor: FDCAN interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_IE)

For information about available fields see [`mod@fdcan_ie`]
module*/
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
///FDCAN interrupt enable register
pub mod fdcan_ie;
/**FDCAN_ILS (rw) register accessor: FDCAN interrupt line select register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_ILS)

For information about available fields see [`mod@fdcan_ils`]
module*/
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
///FDCAN interrupt line select register
pub mod fdcan_ils;
/**FDCAN_ILE (rw) register accessor: FDCAN interrupt line enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_ILE)

For information about available fields see [`mod@fdcan_ile`]
module*/
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
///FDCAN interrupt line enable register
pub mod fdcan_ile;
/**FDCAN_RXGFC (rw) register accessor: FDCAN global filter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxgfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RXGFC)

For information about available fields see [`mod@fdcan_rxgfc`]
module*/
pub type FDCAN_RXGFC = crate::Reg<fdcan_rxgfc::FDCAN_RXGFCrs>;
///FDCAN global filter configuration register
pub mod fdcan_rxgfc;
/**FDCAN_XIDAM (rw) register accessor: FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_XIDAM)

For information about available fields see [`mod@fdcan_xidam`]
module*/
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAMrs>;
///FDCAN extended ID and mask register
pub mod fdcan_xidam;
/**FDCAN_HPMS (r) register accessor: FDCAN high-priority message status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_HPMS)

For information about available fields see [`mod@fdcan_hpms`]
module*/
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMSrs>;
///FDCAN high-priority message status register
pub mod fdcan_hpms;
/**FDCAN_RXF0S (r) register accessor: FDCAN Rx FIFO 0 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RXF0S)

For information about available fields see [`mod@fdcan_rxf0s`]
module*/
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0Srs>;
///FDCAN Rx FIFO 0 status register
pub mod fdcan_rxf0s;
/**FDCAN_RXF0A (rw) register accessor: CAN Rx FIFO 0 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RXF0A)

For information about available fields see [`mod@fdcan_rxf0a`]
module*/
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0Ars>;
///CAN Rx FIFO 0 acknowledge register
pub mod fdcan_rxf0a;
/**FDCAN_RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RXF1S)

For information about available fields see [`mod@fdcan_rxf1s`]
module*/
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1Srs>;
///FDCAN Rx FIFO 1 status register
pub mod fdcan_rxf1s;
/**FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_RXF1A)

For information about available fields see [`mod@fdcan_rxf1a`]
module*/
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1Ars>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod fdcan_rxf1a;
/**FDCAN_TXBC (rw) register accessor: FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBC)

For information about available fields see [`mod@fdcan_txbc`]
module*/
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
///FDCAN Tx buffer configuration register
pub mod fdcan_txbc;
/**FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXFQS)

For information about available fields see [`mod@fdcan_txfqs`]
module*/
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
///FDCAN Tx FIFO/queue status register
pub mod fdcan_txfqs;
/**FDCAN_TXBRP (r) register accessor: FDCAN Tx buffer request pending register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBRP)

For information about available fields see [`mod@fdcan_txbrp`]
module*/
pub type FDCAN_TXBRP = crate::Reg<fdcan_txbrp::FDCAN_TXBRPrs>;
///FDCAN Tx buffer request pending register
pub mod fdcan_txbrp;
/**FDCAN_TXBAR (rw) register accessor: FDCAN Tx buffer add request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBAR)

For information about available fields see [`mod@fdcan_txbar`]
module*/
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBARrs>;
///FDCAN Tx buffer add request register
pub mod fdcan_txbar;
/**FDCAN_TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBCR)

For information about available fields see [`mod@fdcan_txbcr`]
module*/
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCRrs>;
///FDCAN Tx buffer cancellation request register
pub mod fdcan_txbcr;
/**FDCAN_TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBTO)

For information about available fields see [`mod@fdcan_txbto`]
module*/
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTOrs>;
///FDCAN Tx buffer transmission occurred register
pub mod fdcan_txbto;
/**FDCAN_TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBCF)

For information about available fields see [`mod@fdcan_txbcf`]
module*/
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCFrs>;
///FDCAN Tx buffer cancellation finished register
pub mod fdcan_txbcf;
/**FDCAN_TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBTIE)

For information about available fields see [`mod@fdcan_txbtie`]
module*/
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIErs>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod fdcan_txbtie;
/**FDCAN_TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXBCIE)

For information about available fields see [`mod@fdcan_txbcie`]
module*/
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIErs>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod fdcan_txbcie;
/**FDCAN_TXEFS (r) register accessor: FDCAN Tx event FIFO status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXEFS)

For information about available fields see [`mod@fdcan_txefs`]
module*/
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
///FDCAN Tx event FIFO status register
pub mod fdcan_txefs;
/**FDCAN_TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TXEFA)

For information about available fields see [`mod@fdcan_txefa`]
module*/
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
///FDCAN Tx event FIFO acknowledge register
pub mod fdcan_txefa;
/**FDCAN_CKDIV (rw) register accessor: FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ckdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_CKDIV)

For information about available fields see [`mod@fdcan_ckdiv`]
module*/
pub type FDCAN_CKDIV = crate::Reg<fdcan_ckdiv::FDCAN_CKDIVrs>;
///FDCAN CFG clock divider register
pub mod fdcan_ckdiv;
