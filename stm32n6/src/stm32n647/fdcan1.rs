#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_crel: [u8; 0x04],
    _reserved_1_endn: [u8; 0x04],
    ccu_cstat: CCU_CSTAT,
    _reserved_3_dbtp: [u8; 0x04],
    _reserved_4_test: [u8; 0x04],
    _reserved_5_rwd: [u8; 0x04],
    cccr: CCCR,
    nbtp: NBTP,
    tscc: TSCC,
    tscv: TSCV,
    tocc: TOCC,
    tocv: TOCV,
    _reserved12: [u8; 0x10],
    ecr: ECR,
    psr: PSR,
    tdcr: TDCR,
    _reserved15: [u8; 0x04],
    ir: IR,
    ie: IE,
    ils: ILS,
    ile: ILE,
    _reserved19: [u8; 0x20],
    gfc: GFC,
    sidfc: SIDFC,
    xidfc: XIDFC,
    _reserved22: [u8; 0x04],
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
    _reserved44: [u8; 0x08],
    txefc: TXEFC,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved47: [u8; 0x04],
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
    _reserved64: [u8; 0x01bc],
    ttts: TTTS,
}
impl RegisterBlock {
    ///0x00 - FDCAN core release register
    #[inline(always)]
    pub const fn ccu_crel(&self) -> &CCU_CREL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - FDCAN core release register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - FDCAN Endian register
    #[inline(always)]
    pub const fn ccu_ccfg(&self) -> &CCU_CCFG {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - FDCAN Endian register
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - Calibration status register
    #[inline(always)]
    pub const fn ccu_cstat(&self) -> &CCU_CSTAT {
        &self.ccu_cstat
    }
    ///0x0c - FDCAN data bit timing and prescaler register
    #[inline(always)]
    pub const fn ccu_cwd(&self) -> &CCU_CWD {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - FDCAN data bit timing and prescaler register
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 - FDCAN test register
    #[inline(always)]
    pub const fn ccu_ir(&self) -> &CCU_IR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - FDCAN test register
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x14 - FDCAN RAM watchdog register
    #[inline(always)]
    pub const fn ccu_ie(&self) -> &CCU_IE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - FDCAN RAM watchdog register
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x18 - FDCAN CC control register
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x1c - FDCAN nominal bit timing and prescaler register
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    ///0x20 - FDCAN timestamp counter configuration register
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    ///0x24 - FDCAN timestamp counter value register
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    ///0x28 - FDCAN timeout counter configuration register
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    ///0x2c - FDCAN timeout counter value register
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    ///0x40 - FDCAN error counter register
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    ///0x44 - FDCAN protocol status register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0x48 - FDCAN transmitter delay compensation register
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    ///0x50 - FDCAN interrupt register
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x54 - FDCAN interrupt enable register
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    ///0x58 - FDCAN interrupt line select register
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    ///0x5c - FDCAN interrupt line enable register
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    ///0x80 - FDCAN global filter configuration register
    #[inline(always)]
    pub const fn gfc(&self) -> &GFC {
        &self.gfc
    }
    ///0x84 - FDCAN standard ID filter configuration register
    #[inline(always)]
    pub const fn sidfc(&self) -> &SIDFC {
        &self.sidfc
    }
    ///0x88 - FDCAN extended ID filter configuration register
    #[inline(always)]
    pub const fn xidfc(&self) -> &XIDFC {
        &self.xidfc
    }
    ///0x90 - FDCAN extended ID and mask register
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    ///0x94 - FDCAN high priority message status register
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    ///0x98 - FDCAN new data 1 register
    #[inline(always)]
    pub const fn ndat1(&self) -> &NDAT1 {
        &self.ndat1
    }
    ///0x9c - FDCAN new data 2 register
    #[inline(always)]
    pub const fn ndat2(&self) -> &NDAT2 {
        &self.ndat2
    }
    ///0xa0 - FDCAN Rx FIFO 0 configuration register
    #[inline(always)]
    pub const fn rxf0c(&self) -> &RXF0C {
        &self.rxf0c
    }
    ///0xa4 - FDCAN Rx FIFO 0 status register
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    ///0xa8 - FDCAN Rx FIFO 0 acknowledge register
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    ///0xac - FDCAN Rx buffer configuration register
    #[inline(always)]
    pub const fn rxbc(&self) -> &RXBC {
        &self.rxbc
    }
    ///0xb0 - FDCAN Rx FIFO 1 configuration register
    #[inline(always)]
    pub const fn rxf1c(&self) -> &RXF1C {
        &self.rxf1c
    }
    ///0xb4 - FDCAN Rx FIFO 1 status register
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    ///0xb8 - FDCAN Rx FIFO 1 acknowledge register
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    ///0xbc - FDCAN Rx buffer element size configuration register
    #[inline(always)]
    pub const fn rxesc(&self) -> &RXESC {
        &self.rxesc
    }
    ///0xc0 - FDCAN Tx buffer configuration register
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    ///0xc4 - FDCAN Tx FIFO/queue status register
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    ///0xc8 - FDCAN Tx buffer element size configuration register
    #[inline(always)]
    pub const fn txesc(&self) -> &TXESC {
        &self.txesc
    }
    ///0xcc - FDCAN Tx buffer request pending register
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    ///0xd0 - FDCAN Tx buffer add request register
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    ///0xd4 - FDCAN Tx buffer cancellation request register
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    ///0xd8 - FDCAN Tx buffer transmission occurred register
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    ///0xdc - FDCAN Tx buffer cancellation finished register
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    ///0xe0 - FDCAN Tx buffer transmission interrupt enable register
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    ///0xe4 - FDCAN Tx buffer cancellation finished interrupt enable register
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    ///0xf0 - FDCAN Tx event FIFO configuration register
    #[inline(always)]
    pub const fn txefc(&self) -> &TXEFC {
        &self.txefc
    }
    ///0xf4 - FDCAN Tx event FIFO status register
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    ///0xf8 - FDCAN Tx event FIFO acknowledge register
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    ///0x100 - FDCAN TT trigger memory configuration register
    #[inline(always)]
    pub const fn tttmc(&self) -> &TTTMC {
        &self.tttmc
    }
    ///0x104 - FDCAN TT reference message configuration register
    #[inline(always)]
    pub const fn ttrmc(&self) -> &TTRMC {
        &self.ttrmc
    }
    ///0x108 - FDCAN TT operation configuration register
    #[inline(always)]
    pub const fn ttocf(&self) -> &TTOCF {
        &self.ttocf
    }
    ///0x10c - FDCAN TT matrix limits register
    #[inline(always)]
    pub const fn ttmlm(&self) -> &TTMLM {
        &self.ttmlm
    }
    ///0x110 - FDCAN TUR configuration register
    #[inline(always)]
    pub const fn turcf(&self) -> &TURCF {
        &self.turcf
    }
    ///0x114 - FDCAN TT operation control register
    #[inline(always)]
    pub const fn ttocn(&self) -> &TTOCN {
        &self.ttocn
    }
    ///0x118 - FDCAN TT global time preset register
    #[inline(always)]
    pub const fn ttgtp(&self) -> &TTGTP {
        &self.ttgtp
    }
    ///0x11c - FDCAN TT time mark register
    #[inline(always)]
    pub const fn tttmk(&self) -> &TTTMK {
        &self.tttmk
    }
    ///0x120 - FDCAN TT interrupt register
    #[inline(always)]
    pub const fn ttir(&self) -> &TTIR {
        &self.ttir
    }
    ///0x124 - FDCAN TT interrupt enable register
    #[inline(always)]
    pub const fn ttie(&self) -> &TTIE {
        &self.ttie
    }
    ///0x128 - FDCAN TT interrupt line select register
    #[inline(always)]
    pub const fn ttils(&self) -> &TTILS {
        &self.ttils
    }
    ///0x12c - FDCAN TT operation status register
    #[inline(always)]
    pub const fn ttost(&self) -> &TTOST {
        &self.ttost
    }
    ///0x130 - FDCAN TUR numerator actual register
    #[inline(always)]
    pub const fn turna(&self) -> &TURNA {
        &self.turna
    }
    ///0x134 - FDCAN TT local and global time register
    #[inline(always)]
    pub const fn ttlgt(&self) -> &TTLGT {
        &self.ttlgt
    }
    ///0x138 - FDCAN TT cycle time and count register
    #[inline(always)]
    pub const fn ttctc(&self) -> &TTCTC {
        &self.ttctc
    }
    ///0x13c - FDCAN TT capture time register
    #[inline(always)]
    pub const fn ttcpt(&self) -> &TTCPT {
        &self.ttcpt
    }
    ///0x140 - FDCAN TT cycle sync mark register
    #[inline(always)]
    pub const fn ttcsm(&self) -> &TTCSM {
        &self.ttcsm
    }
    ///0x300 - FDCAN TT trigger select register
    #[inline(always)]
    pub const fn ttts(&self) -> &TTTS {
        &self.ttts
    }
}
/**CREL (r) register accessor: FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///FDCAN core release register
pub mod crel;
/**CCU_CREL (r) register accessor: FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`ccu_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_CREL)

For information about available fields see [`mod@ccu_crel`] module*/
pub type CCU_CREL = crate::Reg<ccu_crel::CCU_CRELrs>;
///FDCAN core release register
pub mod ccu_crel;
/**ENDN (r) register accessor: FDCAN Endian register

You can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:ENDN)

For information about available fields see [`mod@endn`] module*/
pub type ENDN = crate::Reg<endn::ENDNrs>;
///FDCAN Endian register
pub mod endn;
/**CCU_CCFG (rw) register accessor: FDCAN Endian register

You can [`read`](crate::Reg::read) this register and get [`ccu_ccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_CCFG)

For information about available fields see [`mod@ccu_ccfg`] module*/
pub type CCU_CCFG = crate::Reg<ccu_ccfg::CCU_CCFGrs>;
///FDCAN Endian register
pub mod ccu_ccfg;
/**CCU_CSTAT (r) register accessor: Calibration status register

You can [`read`](crate::Reg::read) this register and get [`ccu_cstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_CSTAT)

For information about available fields see [`mod@ccu_cstat`] module*/
pub type CCU_CSTAT = crate::Reg<ccu_cstat::CCU_CSTATrs>;
///Calibration status register
pub mod ccu_cstat;
/**DBTP (rw) register accessor: FDCAN data bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:DBTP)

For information about available fields see [`mod@dbtp`] module*/
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
///FDCAN data bit timing and prescaler register
pub mod dbtp;
/**CCU_CWD (rw) register accessor: FDCAN data bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`ccu_cwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_cwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_CWD)

For information about available fields see [`mod@ccu_cwd`] module*/
pub type CCU_CWD = crate::Reg<ccu_cwd::CCU_CWDrs>;
///FDCAN data bit timing and prescaler register
pub mod ccu_cwd;
/**TEST (rw) register accessor: FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TEST)

For information about available fields see [`mod@test`] module*/
pub type TEST = crate::Reg<test::TESTrs>;
///FDCAN test register
pub mod test;
/**CCU_IR (rw) register accessor: FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`ccu_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_IR)

For information about available fields see [`mod@ccu_ir`] module*/
pub type CCU_IR = crate::Reg<ccu_ir::CCU_IRrs>;
///FDCAN test register
pub mod ccu_ir;
/**RWD (rw) register accessor: FDCAN RAM watchdog register

You can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RWD)

For information about available fields see [`mod@rwd`] module*/
pub type RWD = crate::Reg<rwd::RWDrs>;
///FDCAN RAM watchdog register
pub mod rwd;
/**CCU_IE (rw) register accessor: FDCAN RAM watchdog register

You can [`read`](crate::Reg::read) this register and get [`ccu_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_IE)

For information about available fields see [`mod@ccu_ie`] module*/
pub type CCU_IE = crate::Reg<ccu_ie::CCU_IErs>;
///FDCAN RAM watchdog register
pub mod ccu_ie;
/**CCCR (rw) register accessor: FDCAN CC control register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///FDCAN CC control register
pub mod cccr;
/**NBTP (rw) register accessor: FDCAN nominal bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:NBTP)

For information about available fields see [`mod@nbtp`] module*/
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
///FDCAN nominal bit timing and prescaler register
pub mod nbtp;
/**TSCC (rw) register accessor: FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TSCC)

For information about available fields see [`mod@tscc`] module*/
pub type TSCC = crate::Reg<tscc::TSCCrs>;
///FDCAN timestamp counter configuration register
pub mod tscc;
/**TSCV (rw) register accessor: FDCAN timestamp counter value register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TSCV)

For information about available fields see [`mod@tscv`] module*/
pub type TSCV = crate::Reg<tscv::TSCVrs>;
///FDCAN timestamp counter value register
pub mod tscv;
/**TOCC (rw) register accessor: FDCAN timeout counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TOCC)

For information about available fields see [`mod@tocc`] module*/
pub type TOCC = crate::Reg<tocc::TOCCrs>;
///FDCAN timeout counter configuration register
pub mod tocc;
/**TOCV (rw) register accessor: FDCAN timeout counter value register

You can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TOCV)

For information about available fields see [`mod@tocv`] module*/
pub type TOCV = crate::Reg<tocv::TOCVrs>;
///FDCAN timeout counter value register
pub mod tocv;
/**ECR (rw) register accessor: FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:ECR)

For information about available fields see [`mod@ecr`] module*/
pub type ECR = crate::Reg<ecr::ECRrs>;
///FDCAN error counter register
pub mod ecr;
/**PSR (rw) register accessor: FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///FDCAN protocol status register
pub mod psr;
/**TDCR (rw) register accessor: FDCAN transmitter delay compensation register

You can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TDCR)

For information about available fields see [`mod@tdcr`] module*/
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
///FDCAN transmitter delay compensation register
pub mod tdcr;
/**IR (rw) register accessor: FDCAN interrupt register

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///FDCAN interrupt register
pub mod ir;
/**IE (rw) register accessor: FDCAN interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///FDCAN interrupt enable register
pub mod ie;
/**ILS (rw) register accessor: FDCAN interrupt line select register

You can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:ILS)

For information about available fields see [`mod@ils`] module*/
pub type ILS = crate::Reg<ils::ILSrs>;
///FDCAN interrupt line select register
pub mod ils;
/**ILE (rw) register accessor: FDCAN interrupt line enable register

You can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:ILE)

For information about available fields see [`mod@ile`] module*/
pub type ILE = crate::Reg<ile::ILErs>;
///FDCAN interrupt line enable register
pub mod ile;
/**GFC (rw) register accessor: FDCAN global filter configuration register

You can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:GFC)

For information about available fields see [`mod@gfc`] module*/
pub type GFC = crate::Reg<gfc::GFCrs>;
///FDCAN global filter configuration register
pub mod gfc;
/**SIDFC (rw) register accessor: FDCAN standard ID filter configuration register

You can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:SIDFC)

For information about available fields see [`mod@sidfc`] module*/
pub type SIDFC = crate::Reg<sidfc::SIDFCrs>;
///FDCAN standard ID filter configuration register
pub mod sidfc;
/**XIDFC (rw) register accessor: FDCAN extended ID filter configuration register

You can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:XIDFC)

For information about available fields see [`mod@xidfc`] module*/
pub type XIDFC = crate::Reg<xidfc::XIDFCrs>;
///FDCAN extended ID filter configuration register
pub mod xidfc;
/**XIDAM (rw) register accessor: FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:XIDAM)

For information about available fields see [`mod@xidam`] module*/
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
///FDCAN extended ID and mask register
pub mod xidam;
/**HPMS (r) register accessor: FDCAN high priority message status register

You can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:HPMS)

For information about available fields see [`mod@hpms`] module*/
pub type HPMS = crate::Reg<hpms::HPMSrs>;
///FDCAN high priority message status register
pub mod hpms;
/**NDAT1 (rw) register accessor: FDCAN new data 1 register

You can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:NDAT1)

For information about available fields see [`mod@ndat1`] module*/
pub type NDAT1 = crate::Reg<ndat1::NDAT1rs>;
///FDCAN new data 1 register
pub mod ndat1;
/**NDAT2 (rw) register accessor: FDCAN new data 2 register

You can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:NDAT2)

For information about available fields see [`mod@ndat2`] module*/
pub type NDAT2 = crate::Reg<ndat2::NDAT2rs>;
///FDCAN new data 2 register
pub mod ndat2;
/**RXF0C (rw) register accessor: FDCAN Rx FIFO 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF0C)

For information about available fields see [`mod@rxf0c`] module*/
pub type RXF0C = crate::Reg<rxf0c::RXF0Crs>;
///FDCAN Rx FIFO 0 configuration register
pub mod rxf0c;
/**RXF0S (rw) register accessor: FDCAN Rx FIFO 0 status register

You can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF0S)

For information about available fields see [`mod@rxf0s`] module*/
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
///FDCAN Rx FIFO 0 status register
pub mod rxf0s;
/**RXF0A (rw) register accessor: FDCAN Rx FIFO 0 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF0A)

For information about available fields see [`mod@rxf0a`] module*/
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
///FDCAN Rx FIFO 0 acknowledge register
pub mod rxf0a;
/**RXBC (rw) register accessor: FDCAN Rx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXBC)

For information about available fields see [`mod@rxbc`] module*/
pub type RXBC = crate::Reg<rxbc::RXBCrs>;
///FDCAN Rx buffer configuration register
pub mod rxbc;
/**RXF1C (rw) register accessor: FDCAN Rx FIFO 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF1C)

For information about available fields see [`mod@rxf1c`] module*/
pub type RXF1C = crate::Reg<rxf1c::RXF1Crs>;
///FDCAN Rx FIFO 1 configuration register
pub mod rxf1c;
/**RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF1S)

For information about available fields see [`mod@rxf1s`] module*/
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
///FDCAN Rx FIFO 1 status register
pub mod rxf1s;
/**RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXF1A)

For information about available fields see [`mod@rxf1a`] module*/
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod rxf1a;
/**RXESC (r) register accessor: FDCAN Rx buffer element size configuration register

You can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:RXESC)

For information about available fields see [`mod@rxesc`] module*/
pub type RXESC = crate::Reg<rxesc::RXESCrs>;
///FDCAN Rx buffer element size configuration register
pub mod rxesc;
/**TXBC (rw) register accessor: FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBC)

For information about available fields see [`mod@txbc`] module*/
pub type TXBC = crate::Reg<txbc::TXBCrs>;
///FDCAN Tx buffer configuration register
pub mod txbc;
/**TXFQS (r) register accessor: FDCAN Tx FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXFQS)

For information about available fields see [`mod@txfqs`] module*/
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
///FDCAN Tx FIFO/queue status register
pub mod txfqs;
/**TXESC (r) register accessor: FDCAN Tx buffer element size configuration register

You can [`read`](crate::Reg::read) this register and get [`txesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXESC)

For information about available fields see [`mod@txesc`] module*/
pub type TXESC = crate::Reg<txesc::TXESCrs>;
///FDCAN Tx buffer element size configuration register
pub mod txesc;
/**TXBRP (r) register accessor: FDCAN Tx buffer request pending register

You can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBRP)

For information about available fields see [`mod@txbrp`] module*/
pub type TXBRP = crate::Reg<txbrp::TXBRPrs>;
///FDCAN Tx buffer request pending register
pub mod txbrp;
/**TXBAR (rw) register accessor: FDCAN Tx buffer add request register

You can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBAR)

For information about available fields see [`mod@txbar`] module*/
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
///FDCAN Tx buffer add request register
pub mod txbar;
/**TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register

You can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBCR)

For information about available fields see [`mod@txbcr`] module*/
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
///FDCAN Tx buffer cancellation request register
pub mod txbcr;
/**TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBTO)

For information about available fields see [`mod@txbto`] module*/
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
///FDCAN Tx buffer transmission occurred register
pub mod txbto;
/**TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBCF)

For information about available fields see [`mod@txbcf`] module*/
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
///FDCAN Tx buffer cancellation finished register
pub mod txbcf;
/**TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBTIE)

For information about available fields see [`mod@txbtie`] module*/
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod txbtie;
/**TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXBCIE)

For information about available fields see [`mod@txbcie`] module*/
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod txbcie;
/**TXEFC (rw) register accessor: FDCAN Tx event FIFO configuration register

You can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXEFC)

For information about available fields see [`mod@txefc`] module*/
pub type TXEFC = crate::Reg<txefc::TXEFCrs>;
///FDCAN Tx event FIFO configuration register
pub mod txefc;
/**TXEFS (r) register accessor: FDCAN Tx event FIFO status register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXEFS)

For information about available fields see [`mod@txefs`] module*/
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
///FDCAN Tx event FIFO status register
pub mod txefs;
/**TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TXEFA)

For information about available fields see [`mod@txefa`] module*/
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
///FDCAN Tx event FIFO acknowledge register
pub mod txefa;
/**TTTMC (rw) register accessor: FDCAN TT trigger memory configuration register

You can [`read`](crate::Reg::read) this register and get [`tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTTMC)

For information about available fields see [`mod@tttmc`] module*/
pub type TTTMC = crate::Reg<tttmc::TTTMCrs>;
///FDCAN TT trigger memory configuration register
pub mod tttmc;
/**TTRMC (rw) register accessor: FDCAN TT reference message configuration register

You can [`read`](crate::Reg::read) this register and get [`ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTRMC)

For information about available fields see [`mod@ttrmc`] module*/
pub type TTRMC = crate::Reg<ttrmc::TTRMCrs>;
///FDCAN TT reference message configuration register
pub mod ttrmc;
/**TTOCF (rw) register accessor: FDCAN TT operation configuration register

You can [`read`](crate::Reg::read) this register and get [`ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTOCF)

For information about available fields see [`mod@ttocf`] module*/
pub type TTOCF = crate::Reg<ttocf::TTOCFrs>;
///FDCAN TT operation configuration register
pub mod ttocf;
/**TTMLM (rw) register accessor: FDCAN TT matrix limits register

You can [`read`](crate::Reg::read) this register and get [`ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTMLM)

For information about available fields see [`mod@ttmlm`] module*/
pub type TTMLM = crate::Reg<ttmlm::TTMLMrs>;
///FDCAN TT matrix limits register
pub mod ttmlm;
/**TURCF (rw) register accessor: FDCAN TUR configuration register

You can [`read`](crate::Reg::read) this register and get [`turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TURCF)

For information about available fields see [`mod@turcf`] module*/
pub type TURCF = crate::Reg<turcf::TURCFrs>;
///FDCAN TUR configuration register
pub mod turcf;
/**TTOCN (rw) register accessor: FDCAN TT operation control register

You can [`read`](crate::Reg::read) this register and get [`ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTOCN)

For information about available fields see [`mod@ttocn`] module*/
pub type TTOCN = crate::Reg<ttocn::TTOCNrs>;
///FDCAN TT operation control register
pub mod ttocn;
/**TTGTP (rw) register accessor: FDCAN TT global time preset register

You can [`read`](crate::Reg::read) this register and get [`ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTGTP)

For information about available fields see [`mod@ttgtp`] module*/
pub type TTGTP = crate::Reg<ttgtp::TTGTPrs>;
///FDCAN TT global time preset register
pub mod ttgtp;
/**TTTMK (rw) register accessor: FDCAN TT time mark register

You can [`read`](crate::Reg::read) this register and get [`tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTTMK)

For information about available fields see [`mod@tttmk`] module*/
pub type TTTMK = crate::Reg<tttmk::TTTMKrs>;
///FDCAN TT time mark register
pub mod tttmk;
/**TTIR (rw) register accessor: FDCAN TT interrupt register

You can [`read`](crate::Reg::read) this register and get [`ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTIR)

For information about available fields see [`mod@ttir`] module*/
pub type TTIR = crate::Reg<ttir::TTIRrs>;
///FDCAN TT interrupt register
pub mod ttir;
/**TTIE (rw) register accessor: FDCAN TT interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTIE)

For information about available fields see [`mod@ttie`] module*/
pub type TTIE = crate::Reg<ttie::TTIErs>;
///FDCAN TT interrupt enable register
pub mod ttie;
/**TTILS (rw) register accessor: FDCAN TT interrupt line select register

You can [`read`](crate::Reg::read) this register and get [`ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTILS)

For information about available fields see [`mod@ttils`] module*/
pub type TTILS = crate::Reg<ttils::TTILSrs>;
///FDCAN TT interrupt line select register
pub mod ttils;
/**TTOST (r) register accessor: FDCAN TT operation status register

You can [`read`](crate::Reg::read) this register and get [`ttost::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTOST)

For information about available fields see [`mod@ttost`] module*/
pub type TTOST = crate::Reg<ttost::TTOSTrs>;
///FDCAN TT operation status register
pub mod ttost;
/**TURNA (r) register accessor: FDCAN TUR numerator actual register

You can [`read`](crate::Reg::read) this register and get [`turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TURNA)

For information about available fields see [`mod@turna`] module*/
pub type TURNA = crate::Reg<turna::TURNArs>;
///FDCAN TUR numerator actual register
pub mod turna;
/**TTLGT (r) register accessor: FDCAN TT local and global time register

You can [`read`](crate::Reg::read) this register and get [`ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTLGT)

For information about available fields see [`mod@ttlgt`] module*/
pub type TTLGT = crate::Reg<ttlgt::TTLGTrs>;
///FDCAN TT local and global time register
pub mod ttlgt;
/**TTCTC (r) register accessor: FDCAN TT cycle time and count register

You can [`read`](crate::Reg::read) this register and get [`ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTCTC)

For information about available fields see [`mod@ttctc`] module*/
pub type TTCTC = crate::Reg<ttctc::TTCTCrs>;
///FDCAN TT cycle time and count register
pub mod ttctc;
/**TTCPT (r) register accessor: FDCAN TT capture time register

You can [`read`](crate::Reg::read) this register and get [`ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTCPT)

For information about available fields see [`mod@ttcpt`] module*/
pub type TTCPT = crate::Reg<ttcpt::TTCPTrs>;
///FDCAN TT capture time register
pub mod ttcpt;
/**TTCSM (r) register accessor: FDCAN TT cycle sync mark register

You can [`read`](crate::Reg::read) this register and get [`ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTCSM)

For information about available fields see [`mod@ttcsm`] module*/
pub type TTCSM = crate::Reg<ttcsm::TTCSMrs>;
///FDCAN TT cycle sync mark register
pub mod ttcsm;
/**TTTS (rw) register accessor: FDCAN TT trigger select register

You can [`read`](crate::Reg::read) this register and get [`ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTTS)

For information about available fields see [`mod@ttts`] module*/
pub type TTTS = crate::Reg<ttts::TTTSrs>;
///FDCAN TT trigger select register
pub mod ttts;
