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
    _reserved36: [u8; 0x04],
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    _reserved42: [u8; 0x08],
    txefc: TXEFC,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved45: [u8; 0x04],
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
    _reserved62: [u8; 0x01bc],
    ttts: TTTS,
}
impl RegisterBlock {
    ///0x00 - FDCAN core release register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    ///0x04 - FDCAN Endian register
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    ///0x0c - This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    ///0x10 - Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    ///0x14 - The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    ///0x18 - For details about setting and resetting of single bits see Software initialization.
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x1c - This register is dedicated to the nominal bit timing used during the arbitration phase.
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
    ///0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x54 - The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    ///0x58 - This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    ///0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    ///0x80 - Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.
    #[inline(always)]
    pub const fn gfc(&self) -> &GFC {
        &self.gfc
    }
    ///0x84 - Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.
    #[inline(always)]
    pub const fn sidfc(&self) -> &SIDFC {
        &self.sidfc
    }
    ///0x88 - Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.
    #[inline(always)]
    pub const fn xidfc(&self) -> &XIDFC {
        &self.xidfc
    }
    ///0x90 - FDCAN extended ID and mask register
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    ///0x94 - This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.
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
    ///0xbc - Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.
    #[inline(always)]
    pub const fn rxesc(&self) -> &RXESC {
        &self.rxesc
    }
    ///0xc0 - FDCAN Tx buffer configuration register
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    ///0xc4 - The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    ///0xc8 - Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &gt;8 bytes are intended for CAN FD operation only.
    #[inline(always)]
    pub const fn txesc(&self) -> &TXESC {
        &self.txesc
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
    ///0x110 - The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\] is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.
    #[inline(always)]
    pub const fn turcf(&self) -> &TURCF {
        &self.turcf
    }
    ///0x114 - FDCAN TT operation control register
    #[inline(always)]
    pub const fn ttocn(&self) -> &TTOCN {
        &self.ttocn
    }
    ///0x118 - If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.
    #[inline(always)]
    pub const fn ttgtp(&self) -> &TTGTP {
        &self.ttgtp
    }
    ///0x11c - A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.
    #[inline(always)]
    pub const fn tttmk(&self) -> &TTTMK {
        &self.tttmk
    }
    ///0x120 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.
    #[inline(always)]
    pub const fn ttir(&self) -> &TTIR {
        &self.ttir
    }
    ///0x124 - The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.
    #[inline(always)]
    pub const fn ttie(&self) -> &TTIE {
        &self.ttie
    }
    ///0x128 - The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
    #[inline(always)]
    pub const fn ttils(&self) -> &TTILS {
        &self.ttils
    }
    ///0x12c - FDCAN TT operation status register
    #[inline(always)]
    pub const fn ttost(&self) -> &TTOST {
        &self.ttost
    }
    ///0x130 - There is no drift compensation in TTCAN level 1.
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
    ///0x300 - The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.
    #[inline(always)]
    pub const fn ttts(&self) -> &TTTS {
        &self.ttts
    }
}
/**CREL (r) register accessor: FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///FDCAN core release register
pub mod crel;
/**ENDN (r) register accessor: FDCAN Endian register

You can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:ENDN)

For information about available fields see [`mod@endn`] module*/
pub type ENDN = crate::Reg<endn::ENDNrs>;
///FDCAN Endian register
pub mod endn;
/**DBTP (rw) register accessor: This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.

You can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:DBTP)

For information about available fields see [`mod@dbtp`] module*/
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
///This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.
pub mod dbtp;
/**TEST (rw) register accessor: Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.

You can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TEST)

For information about available fields see [`mod@test`] module*/
pub type TEST = crate::Reg<test::TESTrs>;
///Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.
pub mod test;
/**RWD (rw) register accessor: The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.

You can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RWD)

For information about available fields see [`mod@rwd`] module*/
pub type RWD = crate::Reg<rwd::RWDrs>;
///The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.
pub mod rwd;
/**CCCR (rw) register accessor: For details about setting and resetting of single bits see Software initialization.

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///For details about setting and resetting of single bits see Software initialization.
pub mod cccr;
/**NBTP (rw) register accessor: This register is dedicated to the nominal bit timing used during the arbitration phase.

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:NBTP)

For information about available fields see [`mod@nbtp`] module*/
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
///This register is dedicated to the nominal bit timing used during the arbitration phase.
pub mod nbtp;
/**TSCC (rw) register accessor: FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TSCC)

For information about available fields see [`mod@tscc`] module*/
pub type TSCC = crate::Reg<tscc::TSCCrs>;
///FDCAN timestamp counter configuration register
pub mod tscc;
/**TSCV (rw) register accessor: FDCAN timestamp counter value register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TSCV)

For information about available fields see [`mod@tscv`] module*/
pub type TSCV = crate::Reg<tscv::TSCVrs>;
///FDCAN timestamp counter value register
pub mod tscv;
/**TOCC (rw) register accessor: FDCAN timeout counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TOCC)

For information about available fields see [`mod@tocc`] module*/
pub type TOCC = crate::Reg<tocc::TOCCrs>;
///FDCAN timeout counter configuration register
pub mod tocc;
/**TOCV (rw) register accessor: FDCAN timeout counter value register

You can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TOCV)

For information about available fields see [`mod@tocv`] module*/
pub type TOCV = crate::Reg<tocv::TOCVrs>;
///FDCAN timeout counter value register
pub mod tocv;
/**ECR (rw) register accessor: FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:ECR)

For information about available fields see [`mod@ecr`] module*/
pub type ECR = crate::Reg<ecr::ECRrs>;
///FDCAN error counter register
pub mod ecr;
/**PSR (rw) register accessor: FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///FDCAN protocol status register
pub mod psr;
/**TDCR (rw) register accessor: FDCAN transmitter delay compensation register

You can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TDCR)

For information about available fields see [`mod@tdcr`] module*/
pub type TDCR = crate::Reg<tdcr::TDCRrs>;
///FDCAN transmitter delay compensation register
pub mod tdcr;
/**IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.
pub mod ir;
/**IE (rw) register accessor: The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.
pub mod ie;
/**ILS (rw) register accessor: This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.

You can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:ILS)

For information about available fields see [`mod@ils`] module*/
pub type ILS = crate::Reg<ils::ILSrs>;
///This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
pub mod ils;
/**ILE (rw) register accessor: Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.

You can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:ILE)

For information about available fields see [`mod@ile`] module*/
pub type ILE = crate::Reg<ile::ILErs>;
///Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.
pub mod ile;
/**GFC (rw) register accessor: Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.

You can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:GFC)

For information about available fields see [`mod@gfc`] module*/
pub type GFC = crate::Reg<gfc::GFCrs>;
///Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.
pub mod gfc;
/**SIDFC (rw) register accessor: Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.

You can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:SIDFC)

For information about available fields see [`mod@sidfc`] module*/
pub type SIDFC = crate::Reg<sidfc::SIDFCrs>;
///Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.
pub mod sidfc;
/**XIDFC (rw) register accessor: Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.

You can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:XIDFC)

For information about available fields see [`mod@xidfc`] module*/
pub type XIDFC = crate::Reg<xidfc::XIDFCrs>;
///Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.
pub mod xidfc;
/**XIDAM (rw) register accessor: FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:XIDAM)

For information about available fields see [`mod@xidam`] module*/
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
///FDCAN extended ID and mask register
pub mod xidam;
/**HPMS (r) register accessor: This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.

You can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:HPMS)

For information about available fields see [`mod@hpms`] module*/
pub type HPMS = crate::Reg<hpms::HPMSrs>;
///This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.
pub mod hpms;
/**NDAT1 (rw) register accessor: FDCAN new data 1 register

You can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:NDAT1)

For information about available fields see [`mod@ndat1`] module*/
pub type NDAT1 = crate::Reg<ndat1::NDAT1rs>;
///FDCAN new data 1 register
pub mod ndat1;
/**NDAT2 (rw) register accessor: FDCAN new data 2 register

You can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:NDAT2)

For information about available fields see [`mod@ndat2`] module*/
pub type NDAT2 = crate::Reg<ndat2::NDAT2rs>;
///FDCAN new data 2 register
pub mod ndat2;
/**RXF0C (rw) register accessor: FDCAN Rx FIFO 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF0C)

For information about available fields see [`mod@rxf0c`] module*/
pub type RXF0C = crate::Reg<rxf0c::RXF0Crs>;
///FDCAN Rx FIFO 0 configuration register
pub mod rxf0c;
/**RXF0S (rw) register accessor: FDCAN Rx FIFO 0 status register

You can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF0S)

For information about available fields see [`mod@rxf0s`] module*/
pub type RXF0S = crate::Reg<rxf0s::RXF0Srs>;
///FDCAN Rx FIFO 0 status register
pub mod rxf0s;
/**RXF0A (rw) register accessor: FDCAN Rx FIFO 0 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF0A)

For information about available fields see [`mod@rxf0a`] module*/
pub type RXF0A = crate::Reg<rxf0a::RXF0Ars>;
///FDCAN Rx FIFO 0 acknowledge register
pub mod rxf0a;
/**RXBC (rw) register accessor: FDCAN Rx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXBC)

For information about available fields see [`mod@rxbc`] module*/
pub type RXBC = crate::Reg<rxbc::RXBCrs>;
///FDCAN Rx buffer configuration register
pub mod rxbc;
/**RXF1C (rw) register accessor: FDCAN Rx FIFO 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF1C)

For information about available fields see [`mod@rxf1c`] module*/
pub type RXF1C = crate::Reg<rxf1c::RXF1Crs>;
///FDCAN Rx FIFO 1 configuration register
pub mod rxf1c;
/**RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF1S)

For information about available fields see [`mod@rxf1s`] module*/
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
///FDCAN Rx FIFO 1 status register
pub mod rxf1s;
/**RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXF1A)

For information about available fields see [`mod@rxf1a`] module*/
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod rxf1a;
/**RXESC (r) register accessor: Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:RXESC)

For information about available fields see [`mod@rxesc`] module*/
pub type RXESC = crate::Reg<rxesc::RXESCrs>;
///Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.
pub mod rxesc;
/**TXBC (rw) register accessor: FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBC)

For information about available fields see [`mod@txbc`] module*/
pub type TXBC = crate::Reg<txbc::TXBCrs>;
///FDCAN Tx buffer configuration register
pub mod txbc;
/**TXFQS (r) register accessor: The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXFQS)

For information about available fields see [`mod@txfqs`] module*/
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
///The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).
pub mod txfqs;
/**TXESC (r) register accessor: Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &gt;8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`txesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXESC)

For information about available fields see [`mod@txesc`] module*/
pub type TXESC = crate::Reg<txesc::TXESCrs>;
///Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &gt;8 bytes are intended for CAN FD operation only.
pub mod txesc;
/**TXBAR (rw) register accessor: FDCAN Tx buffer add request register

You can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBAR)

For information about available fields see [`mod@txbar`] module*/
pub type TXBAR = crate::Reg<txbar::TXBARrs>;
///FDCAN Tx buffer add request register
pub mod txbar;
/**TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register

You can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBCR)

For information about available fields see [`mod@txbcr`] module*/
pub type TXBCR = crate::Reg<txbcr::TXBCRrs>;
///FDCAN Tx buffer cancellation request register
pub mod txbcr;
/**TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBTO)

For information about available fields see [`mod@txbto`] module*/
pub type TXBTO = crate::Reg<txbto::TXBTOrs>;
///FDCAN Tx buffer transmission occurred register
pub mod txbto;
/**TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBCF)

For information about available fields see [`mod@txbcf`] module*/
pub type TXBCF = crate::Reg<txbcf::TXBCFrs>;
///FDCAN Tx buffer cancellation finished register
pub mod txbcf;
/**TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBTIE)

For information about available fields see [`mod@txbtie`] module*/
pub type TXBTIE = crate::Reg<txbtie::TXBTIErs>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod txbtie;
/**TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBCIE)

For information about available fields see [`mod@txbcie`] module*/
pub type TXBCIE = crate::Reg<txbcie::TXBCIErs>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod txbcie;
/**TXEFC (rw) register accessor: FDCAN Tx event FIFO configuration register

You can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXEFC)

For information about available fields see [`mod@txefc`] module*/
pub type TXEFC = crate::Reg<txefc::TXEFCrs>;
///FDCAN Tx event FIFO configuration register
pub mod txefc;
/**TXEFS (r) register accessor: FDCAN Tx event FIFO status register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXEFS)

For information about available fields see [`mod@txefs`] module*/
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
///FDCAN Tx event FIFO status register
pub mod txefs;
/**TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXEFA)

For information about available fields see [`mod@txefa`] module*/
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
///FDCAN Tx event FIFO acknowledge register
pub mod txefa;
/**TTTMC (rw) register accessor: FDCAN TT trigger memory configuration register

You can [`read`](crate::Reg::read) this register and get [`tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTTMC)

For information about available fields see [`mod@tttmc`] module*/
pub type TTTMC = crate::Reg<tttmc::TTTMCrs>;
///FDCAN TT trigger memory configuration register
pub mod tttmc;
/**TTRMC (rw) register accessor: FDCAN TT reference message configuration register

You can [`read`](crate::Reg::read) this register and get [`ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTRMC)

For information about available fields see [`mod@ttrmc`] module*/
pub type TTRMC = crate::Reg<ttrmc::TTRMCrs>;
///FDCAN TT reference message configuration register
pub mod ttrmc;
/**TTOCF (rw) register accessor: FDCAN TT operation configuration register

You can [`read`](crate::Reg::read) this register and get [`ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTOCF)

For information about available fields see [`mod@ttocf`] module*/
pub type TTOCF = crate::Reg<ttocf::TTOCFrs>;
///FDCAN TT operation configuration register
pub mod ttocf;
/**TTMLM (rw) register accessor: FDCAN TT matrix limits register

You can [`read`](crate::Reg::read) this register and get [`ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTMLM)

For information about available fields see [`mod@ttmlm`] module*/
pub type TTMLM = crate::Reg<ttmlm::TTMLMrs>;
///FDCAN TT matrix limits register
pub mod ttmlm;
/**TURCF (rw) register accessor: The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\] is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.

You can [`read`](crate::Reg::read) this register and get [`turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TURCF)

For information about available fields see [`mod@turcf`] module*/
pub type TURCF = crate::Reg<turcf::TURCFrs>;
///The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\] is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.
pub mod turcf;
/**TTOCN (rw) register accessor: FDCAN TT operation control register

You can [`read`](crate::Reg::read) this register and get [`ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTOCN)

For information about available fields see [`mod@ttocn`] module*/
pub type TTOCN = crate::Reg<ttocn::TTOCNrs>;
///FDCAN TT operation control register
pub mod ttocn;
/**TTGTP (rw) register accessor: If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.

You can [`read`](crate::Reg::read) this register and get [`ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTGTP)

For information about available fields see [`mod@ttgtp`] module*/
pub type TTGTP = crate::Reg<ttgtp::TTGTPrs>;
///If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.
pub mod ttgtp;
/**TTTMK (rw) register accessor: A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.

You can [`read`](crate::Reg::read) this register and get [`tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTTMK)

For information about available fields see [`mod@tttmk`] module*/
pub type TTTMK = crate::Reg<tttmk::TTTMKrs>;
///A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.
pub mod tttmk;
/**TTIR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.

You can [`read`](crate::Reg::read) this register and get [`ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTIR)

For information about available fields see [`mod@ttir`] module*/
pub type TTIR = crate::Reg<ttir::TTIRrs>;
///The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.
pub mod ttir;
/**TTIE (rw) register accessor: The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.

You can [`read`](crate::Reg::read) this register and get [`ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTIE)

For information about available fields see [`mod@ttie`] module*/
pub type TTIE = crate::Reg<ttie::TTIErs>;
///The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.
pub mod ttie;
/**TTILS (rw) register accessor: The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.

You can [`read`](crate::Reg::read) this register and get [`ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTILS)

For information about available fields see [`mod@ttils`] module*/
pub type TTILS = crate::Reg<ttils::TTILSrs>;
///The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
pub mod ttils;
/**TTOST (r) register accessor: FDCAN TT operation status register

You can [`read`](crate::Reg::read) this register and get [`ttost::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTOST)

For information about available fields see [`mod@ttost`] module*/
pub type TTOST = crate::Reg<ttost::TTOSTrs>;
///FDCAN TT operation status register
pub mod ttost;
/**TURNA (r) register accessor: There is no drift compensation in TTCAN level 1.

You can [`read`](crate::Reg::read) this register and get [`turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TURNA)

For information about available fields see [`mod@turna`] module*/
pub type TURNA = crate::Reg<turna::TURNArs>;
///There is no drift compensation in TTCAN level 1.
pub mod turna;
/**TTLGT (r) register accessor: FDCAN TT local and global time register

You can [`read`](crate::Reg::read) this register and get [`ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTLGT)

For information about available fields see [`mod@ttlgt`] module*/
pub type TTLGT = crate::Reg<ttlgt::TTLGTrs>;
///FDCAN TT local and global time register
pub mod ttlgt;
/**TTCTC (r) register accessor: FDCAN TT cycle time and count register

You can [`read`](crate::Reg::read) this register and get [`ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTCTC)

For information about available fields see [`mod@ttctc`] module*/
pub type TTCTC = crate::Reg<ttctc::TTCTCrs>;
///FDCAN TT cycle time and count register
pub mod ttctc;
/**TTCPT (r) register accessor: FDCAN TT capture time register

You can [`read`](crate::Reg::read) this register and get [`ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTCPT)

For information about available fields see [`mod@ttcpt`] module*/
pub type TTCPT = crate::Reg<ttcpt::TTCPTrs>;
///FDCAN TT capture time register
pub mod ttcpt;
/**TTCSM (r) register accessor: FDCAN TT cycle sync mark register

You can [`read`](crate::Reg::read) this register and get [`ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTCSM)

For information about available fields see [`mod@ttcsm`] module*/
pub type TTCSM = crate::Reg<ttcsm::TTCSMrs>;
///FDCAN TT cycle sync mark register
pub mod ttcsm;
/**TTTS (rw) register accessor: The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.

You can [`read`](crate::Reg::read) this register and get [`ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTTS)

For information about available fields see [`mod@ttts`] module*/
pub type TTTS = crate::Reg<ttts::TTTSrs>;
///The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.
pub mod ttts;
