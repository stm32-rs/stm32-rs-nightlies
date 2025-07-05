#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - FDCAN Core Release Register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    ///0x04 - FDCAN Core Release Register
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    ///0x10 - FDCAN Test Register
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    ///0x14 - FDCAN RAM Watchdog Register
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    ///0x18 - FDCAN CC Control Register
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    ///0x24 - FDCAN Timestamp Counter Value Register
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    ///0x28 - FDCAN Timeout Counter Configuration Register
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    ///0x2c - FDCAN Timeout Counter Value Register
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    ///0x40 - FDCAN Error Counter Register
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    ///0x44 - FDCAN Protocol Status Register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    ///0x50 - FDCAN Interrupt Register
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x54 - FDCAN Interrupt Enable Register
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    ///0x58 - FDCAN Interrupt Line Select Register
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    ///0x5c - FDCAN Interrupt Line Enable Register
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    ///0x80 - FDCAN Global Filter Configuration Register
    #[inline(always)]
    pub const fn gfc(&self) -> &GFC {
        &self.gfc
    }
    ///0x84 - FDCAN Standard ID Filter Configuration Register
    #[inline(always)]
    pub const fn sidfc(&self) -> &SIDFC {
        &self.sidfc
    }
    ///0x88 - FDCAN Extended ID Filter Configuration Register
    #[inline(always)]
    pub const fn xidfc(&self) -> &XIDFC {
        &self.xidfc
    }
    ///0x90 - FDCAN Extended ID and Mask Register
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    ///0x94 - FDCAN High Priority Message Status Register
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    ///0x98 - FDCAN New Data 1 Register
    #[inline(always)]
    pub const fn ndat1(&self) -> &NDAT1 {
        &self.ndat1
    }
    ///0x9c - FDCAN New Data 2 Register
    #[inline(always)]
    pub const fn ndat2(&self) -> &NDAT2 {
        &self.ndat2
    }
    ///0xa0 - FDCAN Rx FIFO 0 Configuration Register
    #[inline(always)]
    pub const fn rxf0c(&self) -> &RXF0C {
        &self.rxf0c
    }
    ///0xa4 - FDCAN Rx FIFO 0 Status Register
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    ///0xa8 - CAN Rx FIFO 0 Acknowledge Register
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    ///0xac - FDCAN Rx Buffer Configuration Register
    #[inline(always)]
    pub const fn rxbc(&self) -> &RXBC {
        &self.rxbc
    }
    ///0xb0 - FDCAN Rx FIFO 1 Configuration Register
    #[inline(always)]
    pub const fn rxf1c(&self) -> &RXF1C {
        &self.rxf1c
    }
    ///0xb4 - FDCAN Rx FIFO 1 Status Register
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    ///0xb8 - FDCAN Rx FIFO 1 Acknowledge Register
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    ///0xbc - FDCAN Rx Buffer Element Size Configuration Register
    #[inline(always)]
    pub const fn rxesc(&self) -> &RXESC {
        &self.rxesc
    }
    ///0xc0 - FDCAN Tx Buffer Configuration Register
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    ///0xc8 - FDCAN Tx Buffer Element Size Configuration Register
    #[inline(always)]
    pub const fn txesc(&self) -> &TXESC {
        &self.txesc
    }
    ///0xcc - FDCAN Tx Buffer Request Pending Register
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    ///0xd0 - FDCAN Tx Buffer Add Request Register
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    ///0xd4 - FDCAN Tx Buffer Cancellation Request Register
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    ///0xd8 - FDCAN Tx Buffer Transmission Occurred Register
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    ///0xdc - FDCAN Tx Buffer Cancellation Finished Register
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    ///0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    ///0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    ///0xf0 - FDCAN Tx Event FIFO Configuration Register
    #[inline(always)]
    pub const fn txefc(&self) -> &TXEFC {
        &self.txefc
    }
    ///0xf4 - FDCAN Tx Event FIFO Status Register
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    ///0xf8 - FDCAN Tx Event FIFO Acknowledge Register
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    ///0x100 - FDCAN TT Trigger Memory Configuration Register
    #[inline(always)]
    pub const fn tttmc(&self) -> &TTTMC {
        &self.tttmc
    }
    ///0x104 - FDCAN TT Reference Message Configuration Register
    #[inline(always)]
    pub const fn ttrmc(&self) -> &TTRMC {
        &self.ttrmc
    }
    ///0x108 - FDCAN TT Operation Configuration Register
    #[inline(always)]
    pub const fn ttocf(&self) -> &TTOCF {
        &self.ttocf
    }
    ///0x10c - FDCAN TT Matrix Limits Register
    #[inline(always)]
    pub const fn ttmlm(&self) -> &TTMLM {
        &self.ttmlm
    }
    ///0x110 - FDCAN TUR Configuration Register
    #[inline(always)]
    pub const fn turcf(&self) -> &TURCF {
        &self.turcf
    }
    ///0x114 - FDCAN TT Operation Control Register
    #[inline(always)]
    pub const fn ttocn(&self) -> &TTOCN {
        &self.ttocn
    }
    ///0x118 - FDCAN TT Global Time Preset Register
    #[inline(always)]
    pub const fn ttgtp(&self) -> &TTGTP {
        &self.ttgtp
    }
    ///0x11c - FDCAN TT Time Mark Register
    #[inline(always)]
    pub const fn tttmk(&self) -> &TTTMK {
        &self.tttmk
    }
    ///0x120 - FDCAN TT Interrupt Register
    #[inline(always)]
    pub const fn ttir(&self) -> &TTIR {
        &self.ttir
    }
    ///0x124 - FDCAN TT Interrupt Enable Register
    #[inline(always)]
    pub const fn ttie(&self) -> &TTIE {
        &self.ttie
    }
    ///0x128 - FDCAN TT Interrupt Line Select Register
    #[inline(always)]
    pub const fn ttils(&self) -> &TTILS {
        &self.ttils
    }
    ///0x12c - FDCAN TT Operation Status Register
    #[inline(always)]
    pub const fn ttost(&self) -> &TTOST {
        &self.ttost
    }
    ///0x130 - FDCAN TUR Numerator Actual Register
    #[inline(always)]
    pub const fn turna(&self) -> &TURNA {
        &self.turna
    }
    ///0x134 - FDCAN TT Local and Global Time Register
    #[inline(always)]
    pub const fn ttlgt(&self) -> &TTLGT {
        &self.ttlgt
    }
    ///0x138 - FDCAN TT Cycle Time and Count Register
    #[inline(always)]
    pub const fn ttctc(&self) -> &TTCTC {
        &self.ttctc
    }
    ///0x13c - FDCAN TT Capture Time Register
    #[inline(always)]
    pub const fn ttcpt(&self) -> &TTCPT {
        &self.ttcpt
    }
    ///0x140 - FDCAN TT Cycle Sync Mark Register
    #[inline(always)]
    pub const fn ttcsm(&self) -> &TTCSM {
        &self.ttcsm
    }
    ///0x300 - FDCAN TT Trigger Select Register
    #[inline(always)]
    pub const fn ttts(&self) -> &TTTS {
        &self.ttts
    }
}
/**CREL (r) register accessor: FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///FDCAN Core Release Register
pub mod crel;
/**ENDN (r) register accessor: FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:ENDN)

For information about available fields see [`mod@endn`] module*/
pub type ENDN = crate::Reg<endn::ENDNrs>;
///FDCAN Core Release Register
pub mod endn;
/**DBTP (rw) register accessor: FDCAN Data Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:DBTP)

For information about available fields see [`mod@dbtp`] module*/
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod dbtp;
/**TEST (rw) register accessor: FDCAN Test Register

You can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TEST)

For information about available fields see [`mod@test`] module*/
pub type TEST = crate::Reg<test::TESTrs>;
///FDCAN Test Register
pub mod test;
/**RWD (r) register accessor: FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`rwd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RWD)

For information about available fields see [`mod@rwd`] module*/
pub type RWD = crate::Reg<rwd::RWDrs>;
///FDCAN RAM Watchdog Register
pub mod rwd;
/**CCCR (rw) register accessor: FDCAN CC Control Register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///FDCAN CC Control Register
pub mod cccr;
/**NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:NBTP)

For information about available fields see [`mod@nbtp`] module*/
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod nbtp;
/**TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TSCC)

For information about available fields see [`mod@tscc`] module*/
pub type TSCC = crate::Reg<tscc::TSCCrs>;
///FDCAN Timestamp Counter Configuration Register
pub mod tscc;
/**TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TSCV)

For information about available fields see [`mod@tscv`] module*/
pub type TSCV = crate::Reg<tscv::TSCVrs>;
///FDCAN Timestamp Counter Value Register
pub mod tscv;
/**TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TOCC)

For information about available fields see [`mod@tocc`] module*/
pub type TOCC = crate::Reg<tocc::TOCCrs>;
///FDCAN Timeout Counter Configuration Register
pub mod tocc;
/**TOCV (rw) register accessor: FDCAN Timeout Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TOCV)

For information about available fields see [`mod@tocv`] module*/
pub type TOCV = crate::Reg<tocv::TOCVrs>;
///FDCAN Timeout Counter Value Register
pub mod tocv;
/**ECR (rw) register accessor: FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:ECR)

For information about available fields see [`mod@ecr`] module*/
pub type ECR = crate::Reg<ecr::ECRrs>;
///FDCAN Error Counter Register
pub mod ecr;
/**PSR (rw) register accessor: FDCAN Protocol Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///FDCAN Protocol Status Register
pub mod psr;
/**TDCR (rw) register accessor: FDCAN Transmitter Delay Compensation Register

You can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TDCR)

For information about available fields see [`mod@tdcr`] module*/
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
///FDCAN Transmitter Delay Compensation Register
pub mod tdcr;
/**IR (rw) register accessor: FDCAN Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///FDCAN Interrupt Register
pub mod ir;
/**IE (rw) register accessor: FDCAN Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///FDCAN Interrupt Enable Register
pub mod ie;
/**ILS (rw) register accessor: FDCAN Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:ILS)

For information about available fields see [`mod@ils`] module*/
pub type ILS = crate::Reg<ils::ILSrs>;
///FDCAN Interrupt Line Select Register
pub mod ils;
/**ILE (rw) register accessor: FDCAN Interrupt Line Enable Register

You can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:ILE)

For information about available fields see [`mod@ile`] module*/
pub type ILE = crate::Reg<ile::ILErs>;
///FDCAN Interrupt Line Enable Register
pub mod ile;
/**GFC (rw) register accessor: FDCAN Global Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:GFC)

For information about available fields see [`mod@gfc`] module*/
pub type GFC = crate::Reg<gfc::GFCrs>;
///FDCAN Global Filter Configuration Register
pub mod gfc;
/**SIDFC (rw) register accessor: FDCAN Standard ID Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:SIDFC)

For information about available fields see [`mod@sidfc`] module*/
pub type SIDFC = crate::Reg<sidfc::SIDFCrs>;
///FDCAN Standard ID Filter Configuration Register
pub mod sidfc;
/**XIDFC (rw) register accessor: FDCAN Extended ID Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:XIDFC)

For information about available fields see [`mod@xidfc`] module*/
pub type XIDFC = crate::Reg<xidfc::XIDFCrs>;
///FDCAN Extended ID Filter Configuration Register
pub mod xidfc;
/**XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register

You can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:XIDAM)

For information about available fields see [`mod@xidam`] module*/
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
///FDCAN Extended ID and Mask Register
pub mod xidam;
/**HPMS (r) register accessor: FDCAN High Priority Message Status Register

You can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:HPMS)

For information about available fields see [`mod@hpms`] module*/
pub type HPMS = crate::Reg<hpms::HPMSrs>;
///FDCAN High Priority Message Status Register
pub mod hpms;
/**NDAT1 (rw) register accessor: FDCAN New Data 1 Register

You can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:NDAT1)

For information about available fields see [`mod@ndat1`] module*/
pub type NDAT1 = crate::Reg<ndat1::NDAT1rs>;
///FDCAN New Data 1 Register
pub mod ndat1;
/**NDAT2 (rw) register accessor: FDCAN New Data 2 Register

You can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:NDAT2)

For information about available fields see [`mod@ndat2`] module*/
pub type NDAT2 = crate::Reg<ndat2::NDAT2rs>;
///FDCAN New Data 2 Register
pub mod ndat2;
/**RXF0C (rw) register accessor: FDCAN Rx FIFO 0 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF0C)

For information about available fields see [`mod@rxf0c`] module*/
pub type RXF0C = crate::Reg<rxf0c::RXF0Crs>;
///FDCAN Rx FIFO 0 Configuration Register
pub mod rxf0c;
/**RXF0S (rw) register accessor: FDCAN Rx FIFO 0 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF0S)

For information about available fields see [`mod@rxf0s`] module*/
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
///FDCAN Rx FIFO 0 Status Register
pub mod rxf0s;
/**RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF0A)

For information about available fields see [`mod@rxf0a`] module*/
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod rxf0a;
/**RXBC (rw) register accessor: FDCAN Rx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXBC)

For information about available fields see [`mod@rxbc`] module*/
pub type RXBC = crate::Reg<rxbc::RXBCrs>;
///FDCAN Rx Buffer Configuration Register
pub mod rxbc;
/**RXF1C (rw) register accessor: FDCAN Rx FIFO 1 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF1C)

For information about available fields see [`mod@rxf1c`] module*/
pub type RXF1C = crate::Reg<rxf1c::RXF1Crs>;
///FDCAN Rx FIFO 1 Configuration Register
pub mod rxf1c;
/**RXF1S (rw) register accessor: FDCAN Rx FIFO 1 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF1S)

For information about available fields see [`mod@rxf1s`] module*/
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
///FDCAN Rx FIFO 1 Status Register
pub mod rxf1s;
/**RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXF1A)

For information about available fields see [`mod@rxf1a`] module*/
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod rxf1a;
/**RXESC (rw) register accessor: FDCAN Rx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RXESC)

For information about available fields see [`mod@rxesc`] module*/
pub type RXESC = crate::Reg<rxesc::RXESCrs>;
///FDCAN Rx Buffer Element Size Configuration Register
pub mod rxesc;
/**TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBC)

For information about available fields see [`mod@txbc`] module*/
pub type TXBC = crate::Reg<txbc::TXBCrs>;
///FDCAN Tx Buffer Configuration Register
pub mod txbc;
/**TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXFQS)

For information about available fields see [`mod@txfqs`] module*/
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
///FDCAN Tx FIFO/Queue Status Register
pub mod txfqs;
/**TXESC (rw) register accessor: FDCAN Tx Buffer Element Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXESC)

For information about available fields see [`mod@txesc`] module*/
pub type TXESC = crate::Reg<txesc::TXESCrs>;
///FDCAN Tx Buffer Element Size Configuration Register
pub mod txesc;
/**TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register

You can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBRP)

For information about available fields see [`mod@txbrp`] module*/
pub type TXBRP = crate::Reg<txbrp::TXBRPrs>;
///FDCAN Tx Buffer Request Pending Register
pub mod txbrp;
/**TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register

You can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBAR)

For information about available fields see [`mod@txbar`] module*/
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
///FDCAN Tx Buffer Add Request Register
pub mod txbar;
/**TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register

You can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBCR)

For information about available fields see [`mod@txbcr`] module*/
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod txbcr;
/**TXBTO (rw) register accessor: FDCAN Tx Buffer Transmission Occurred Register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBTO)

For information about available fields see [`mod@txbto`] module*/
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod txbto;
/**TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register

You can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBCF)

For information about available fields see [`mod@txbcf`] module*/
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod txbcf;
/**TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBTIE)

For information about available fields see [`mod@txbtie`] module*/
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod txbtie;
/**TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXBCIE)

For information about available fields see [`mod@txbcie`] module*/
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod txbcie;
/**TXEFC (rw) register accessor: FDCAN Tx Event FIFO Configuration Register

You can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXEFC)

For information about available fields see [`mod@txefc`] module*/
pub type TXEFC = crate::Reg<txefc::TXEFCrs>;
///FDCAN Tx Event FIFO Configuration Register
pub mod txefc;
/**TXEFS (rw) register accessor: FDCAN Tx Event FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXEFS)

For information about available fields see [`mod@txefs`] module*/
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
///FDCAN Tx Event FIFO Status Register
pub mod txefs;
/**TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register

You can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TXEFA)

For information about available fields see [`mod@txefa`] module*/
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod txefa;
/**TTTMC (rw) register accessor: FDCAN TT Trigger Memory Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTTMC)

For information about available fields see [`mod@tttmc`] module*/
pub type TTTMC = crate::Reg<tttmc::TTTMCrs>;
///FDCAN TT Trigger Memory Configuration Register
pub mod tttmc;
/**TTRMC (rw) register accessor: FDCAN TT Reference Message Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTRMC)

For information about available fields see [`mod@ttrmc`] module*/
pub type TTRMC = crate::Reg<ttrmc::TTRMCrs>;
///FDCAN TT Reference Message Configuration Register
pub mod ttrmc;
/**TTOCF (rw) register accessor: FDCAN TT Operation Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTOCF)

For information about available fields see [`mod@ttocf`] module*/
pub type TTOCF = crate::Reg<ttocf::TTOCFrs>;
///FDCAN TT Operation Configuration Register
pub mod ttocf;
/**TTMLM (rw) register accessor: FDCAN TT Matrix Limits Register

You can [`read`](crate::Reg::read) this register and get [`ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTMLM)

For information about available fields see [`mod@ttmlm`] module*/
pub type TTMLM = crate::Reg<ttmlm::TTMLMrs>;
///FDCAN TT Matrix Limits Register
pub mod ttmlm;
/**TURCF (rw) register accessor: FDCAN TUR Configuration Register

You can [`read`](crate::Reg::read) this register and get [`turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TURCF)

For information about available fields see [`mod@turcf`] module*/
pub type TURCF = crate::Reg<turcf::TURCFrs>;
///FDCAN TUR Configuration Register
pub mod turcf;
/**TTOCN (rw) register accessor: FDCAN TT Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTOCN)

For information about available fields see [`mod@ttocn`] module*/
pub type TTOCN = crate::Reg<ttocn::TTOCNrs>;
///FDCAN TT Operation Control Register
pub mod ttocn;
/**TTGTP (rw) register accessor: FDCAN TT Global Time Preset Register

You can [`read`](crate::Reg::read) this register and get [`ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTGTP)

For information about available fields see [`mod@ttgtp`] module*/
pub type TTGTP = crate::Reg<ttgtp::TTGTPrs>;
///FDCAN TT Global Time Preset Register
pub mod ttgtp;
/**TTTMK (rw) register accessor: FDCAN TT Time Mark Register

You can [`read`](crate::Reg::read) this register and get [`tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTTMK)

For information about available fields see [`mod@tttmk`] module*/
pub type TTTMK = crate::Reg<tttmk::TTTMKrs>;
///FDCAN TT Time Mark Register
pub mod tttmk;
/**TTIR (rw) register accessor: FDCAN TT Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTIR)

For information about available fields see [`mod@ttir`] module*/
pub type TTIR = crate::Reg<ttir::TTIRrs>;
///FDCAN TT Interrupt Register
pub mod ttir;
/**TTIE (rw) register accessor: FDCAN TT Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTIE)

For information about available fields see [`mod@ttie`] module*/
pub type TTIE = crate::Reg<ttie::TTIErs>;
///FDCAN TT Interrupt Enable Register
pub mod ttie;
/**TTILS (rw) register accessor: FDCAN TT Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTILS)

For information about available fields see [`mod@ttils`] module*/
pub type TTILS = crate::Reg<ttils::TTILSrs>;
///FDCAN TT Interrupt Line Select Register
pub mod ttils;
/**TTOST (rw) register accessor: FDCAN TT Operation Status Register

You can [`read`](crate::Reg::read) this register and get [`ttost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTOST)

For information about available fields see [`mod@ttost`] module*/
pub type TTOST = crate::Reg<ttost::TTOSTrs>;
///FDCAN TT Operation Status Register
pub mod ttost;
/**TURNA (r) register accessor: FDCAN TUR Numerator Actual Register

You can [`read`](crate::Reg::read) this register and get [`turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TURNA)

For information about available fields see [`mod@turna`] module*/
pub type TURNA = crate::Reg<turna::TURNArs>;
///FDCAN TUR Numerator Actual Register
pub mod turna;
/**TTLGT (r) register accessor: FDCAN TT Local and Global Time Register

You can [`read`](crate::Reg::read) this register and get [`ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTLGT)

For information about available fields see [`mod@ttlgt`] module*/
pub type TTLGT = crate::Reg<ttlgt::TTLGTrs>;
///FDCAN TT Local and Global Time Register
pub mod ttlgt;
/**TTCTC (r) register accessor: FDCAN TT Cycle Time and Count Register

You can [`read`](crate::Reg::read) this register and get [`ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTCTC)

For information about available fields see [`mod@ttctc`] module*/
pub type TTCTC = crate::Reg<ttctc::TTCTCrs>;
///FDCAN TT Cycle Time and Count Register
pub mod ttctc;
/**TTCPT (r) register accessor: FDCAN TT Capture Time Register

You can [`read`](crate::Reg::read) this register and get [`ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTCPT)

For information about available fields see [`mod@ttcpt`] module*/
pub type TTCPT = crate::Reg<ttcpt::TTCPTrs>;
///FDCAN TT Capture Time Register
pub mod ttcpt;
/**TTCSM (r) register accessor: FDCAN TT Cycle Sync Mark Register

You can [`read`](crate::Reg::read) this register and get [`ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTCSM)

For information about available fields see [`mod@ttcsm`] module*/
pub type TTCSM = crate::Reg<ttcsm::TTCSMrs>;
///FDCAN TT Cycle Sync Mark Register
pub mod ttcsm;
/**TTTS (rw) register accessor: FDCAN TT Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTTS)

For information about available fields see [`mod@ttts`] module*/
pub type TTTS = crate::Reg<ttts::TTTSrs>;
///FDCAN TT Trigger Select Register
pub mod ttts;
