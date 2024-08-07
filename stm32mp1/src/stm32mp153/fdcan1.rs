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
    _reserved36: [u8; 0x04],
    fdcan_txbar: FDCAN_TXBAR,
    fdcan_txbcr: FDCAN_TXBCR,
    fdcan_txbto: FDCAN_TXBTO,
    fdcan_txbcf: FDCAN_TXBCF,
    fdcan_txbtie: FDCAN_TXBTIE,
    fdcan_txbcie: FDCAN_TXBCIE,
    _reserved42: [u8; 0x08],
    fdcan_txefc: FDCAN_TXEFC,
    fdcan_txefs: FDCAN_TXEFS,
    fdcan_txefa: FDCAN_TXEFA,
    _reserved45: [u8; 0x04],
    fdcan_tttmc: FDCAN_TTTMC,
    fdcan_ttrmc: FDCAN_TTRMC,
    fdcan_ttocf: FDCAN_TTOCF,
    fdcan_ttmlm: FDCAN_TTMLM,
    fdcan_turcf: FDCAN_TURCF,
    fdcan_ttocn: FDCAN_TTOCN,
    fdcan_ttgtp: FDCAN_TTGTP,
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
    _reserved62: [u8; 0x01bc],
    fdcan_ttts: FDCAN_TTTS,
}
impl RegisterBlock {
    ///0x00 - FDCAN core release register
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FDCAN_CREL {
        &self.fdcan_crel
    }
    ///0x04 - FDCAN Endian register
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FDCAN_ENDN {
        &self.fdcan_endn
    }
    ///0x0c - This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FDCAN_DBTP {
        &self.fdcan_dbtp
    }
    ///0x10 - Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FDCAN_TEST {
        &self.fdcan_test
    }
    ///0x14 - The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FDCAN_RWD {
        &self.fdcan_rwd
    }
    ///0x18 - For details about setting and resetting of single bits see Software initialization.
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FDCAN_CCCR {
        &self.fdcan_cccr
    }
    ///0x1c - This register is dedicated to the nominal bit timing used during the arbitration phase.
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
    ///0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FDCAN_IR {
        &self.fdcan_ir
    }
    ///0x54 - The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FDCAN_IE {
        &self.fdcan_ie
    }
    ///0x58 - This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FDCAN_ILS {
        &self.fdcan_ils
    }
    ///0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FDCAN_ILE {
        &self.fdcan_ile
    }
    ///0x80 - Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.
    #[inline(always)]
    pub const fn fdcan_gfc(&self) -> &FDCAN_GFC {
        &self.fdcan_gfc
    }
    ///0x84 - Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.
    #[inline(always)]
    pub const fn fdcan_sidfc(&self) -> &FDCAN_SIDFC {
        &self.fdcan_sidfc
    }
    ///0x88 - Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.
    #[inline(always)]
    pub const fn fdcan_xidfc(&self) -> &FDCAN_XIDFC {
        &self.fdcan_xidfc
    }
    ///0x90 - FDCAN extended ID and mask register
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    ///0x94 - This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    ///0x98 - FDCAN new data 1 register
    #[inline(always)]
    pub const fn fdcan_ndat1(&self) -> &FDCAN_NDAT1 {
        &self.fdcan_ndat1
    }
    ///0x9c - FDCAN new data 2 register
    #[inline(always)]
    pub const fn fdcan_ndat2(&self) -> &FDCAN_NDAT2 {
        &self.fdcan_ndat2
    }
    ///0xa0 - FDCAN Rx FIFO 0 configuration register
    #[inline(always)]
    pub const fn fdcan_rxf0c(&self) -> &FDCAN_RXF0C {
        &self.fdcan_rxf0c
    }
    ///0xa4 - FDCAN Rx FIFO 0 status register
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    ///0xa8 - FDCAN Rx FIFO 0 acknowledge register
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    ///0xac - FDCAN Rx buffer configuration register
    #[inline(always)]
    pub const fn fdcan_rxbc(&self) -> &FDCAN_RXBC {
        &self.fdcan_rxbc
    }
    ///0xb0 - FDCAN Rx FIFO 1 configuration register
    #[inline(always)]
    pub const fn fdcan_rxf1c(&self) -> &FDCAN_RXF1C {
        &self.fdcan_rxf1c
    }
    ///0xb4 - FDCAN Rx FIFO 1 status register
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    ///0xb8 - FDCAN Rx FIFO 1 acknowledge register
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    ///0xbc - Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.
    #[inline(always)]
    pub const fn fdcan_rxesc(&self) -> &FDCAN_RXESC {
        &self.fdcan_rxesc
    }
    ///0xc0 - FDCAN Tx buffer configuration register
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    ///0xc4 - The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    ///0xc8 - Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.
    #[inline(always)]
    pub const fn fdcan_txesc(&self) -> &FDCAN_TXESC {
        &self.fdcan_txesc
    }
    ///0xd0 - FDCAN Tx buffer add request register
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    ///0xd4 - FDCAN Tx buffer cancellation request register
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    ///0xd8 - FDCAN Tx buffer transmission occurred register
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    ///0xdc - FDCAN Tx buffer cancellation finished register
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    ///0xe0 - FDCAN Tx buffer transmission interrupt enable register
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    ///0xe4 - FDCAN Tx buffer cancellation finished interrupt enable register
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    ///0xf0 - FDCAN Tx event FIFO configuration register
    #[inline(always)]
    pub const fn fdcan_txefc(&self) -> &FDCAN_TXEFC {
        &self.fdcan_txefc
    }
    ///0xf4 - FDCAN Tx event FIFO status register
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    ///0xf8 - FDCAN Tx event FIFO acknowledge register
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    ///0x100 - FDCAN TT trigger memory configuration register
    #[inline(always)]
    pub const fn fdcan_tttmc(&self) -> &FDCAN_TTTMC {
        &self.fdcan_tttmc
    }
    ///0x104 - FDCAN TT reference message configuration register
    #[inline(always)]
    pub const fn fdcan_ttrmc(&self) -> &FDCAN_TTRMC {
        &self.fdcan_ttrmc
    }
    ///0x108 - FDCAN TT operation configuration register
    #[inline(always)]
    pub const fn fdcan_ttocf(&self) -> &FDCAN_TTOCF {
        &self.fdcan_ttocf
    }
    ///0x10c - FDCAN TT matrix limits register
    #[inline(always)]
    pub const fn fdcan_ttmlm(&self) -> &FDCAN_TTMLM {
        &self.fdcan_ttmlm
    }
    /**0x110 - The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\]
    is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.*/
    #[inline(always)]
    pub const fn fdcan_turcf(&self) -> &FDCAN_TURCF {
        &self.fdcan_turcf
    }
    ///0x114 - FDCAN TT operation control register
    #[inline(always)]
    pub const fn fdcan_ttocn(&self) -> &FDCAN_TTOCN {
        &self.fdcan_ttocn
    }
    ///0x118 - If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.
    #[inline(always)]
    pub const fn fdcan_ttgtp(&self) -> &FDCAN_TTGTP {
        &self.fdcan_ttgtp
    }
    ///0x11c - A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.
    #[inline(always)]
    pub const fn fdcan_tttmk(&self) -> &FDCAN_TTTMK {
        &self.fdcan_tttmk
    }
    ///0x120 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.
    #[inline(always)]
    pub const fn fdcan_ttir(&self) -> &FDCAN_TTIR {
        &self.fdcan_ttir
    }
    ///0x124 - The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.
    #[inline(always)]
    pub const fn fdcan_ttie(&self) -> &FDCAN_TTIE {
        &self.fdcan_ttie
    }
    ///0x128 - The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
    #[inline(always)]
    pub const fn fdcan_ttils(&self) -> &FDCAN_TTILS {
        &self.fdcan_ttils
    }
    ///0x12c - FDCAN TT operation status register
    #[inline(always)]
    pub const fn fdcan_ttost(&self) -> &FDCAN_TTOST {
        &self.fdcan_ttost
    }
    ///0x130 - There is no drift compensation in TTCAN level 1.
    #[inline(always)]
    pub const fn fdcan_turna(&self) -> &FDCAN_TURNA {
        &self.fdcan_turna
    }
    ///0x134 - FDCAN TT local and global time register
    #[inline(always)]
    pub const fn fdcan_ttlgt(&self) -> &FDCAN_TTLGT {
        &self.fdcan_ttlgt
    }
    ///0x138 - FDCAN TT cycle time and count register
    #[inline(always)]
    pub const fn fdcan_ttctc(&self) -> &FDCAN_TTCTC {
        &self.fdcan_ttctc
    }
    ///0x13c - FDCAN TT capture time register
    #[inline(always)]
    pub const fn fdcan_ttcpt(&self) -> &FDCAN_TTCPT {
        &self.fdcan_ttcpt
    }
    ///0x140 - FDCAN TT cycle sync mark register
    #[inline(always)]
    pub const fn fdcan_ttcsm(&self) -> &FDCAN_TTCSM {
        &self.fdcan_ttcsm
    }
    ///0x300 - The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.
    #[inline(always)]
    pub const fn fdcan_ttts(&self) -> &FDCAN_TTTS {
        &self.fdcan_ttts
    }
}
/**FDCAN_CREL (r) register accessor: FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`fdcan_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_CREL)

For information about available fields see [`mod@fdcan_crel`]
module*/
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CRELrs>;
///FDCAN core release register
pub mod fdcan_crel;
/**FDCAN_ENDN (r) register accessor: FDCAN Endian register

You can [`read`](crate::Reg::read) this register and get [`fdcan_endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_ENDN)

For information about available fields see [`mod@fdcan_endn`]
module*/
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDNrs>;
///FDCAN Endian register
pub mod fdcan_endn;
/**FDCAN_DBTP (rw) register accessor: This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.

You can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_DBTP)

For information about available fields see [`mod@fdcan_dbtp`]
module*/
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
///This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.
pub mod fdcan_dbtp;
/**FDCAN_TEST (rw) register accessor: Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.

You can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TEST)

For information about available fields see [`mod@fdcan_test`]
module*/
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
///Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.
pub mod fdcan_test;
/**FDCAN_RWD (rw) register accessor: The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.

You can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RWD)

For information about available fields see [`mod@fdcan_rwd`]
module*/
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWDrs>;
///The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.
pub mod fdcan_rwd;
/**FDCAN_CCCR (rw) register accessor: For details about setting and resetting of single bits see Software initialization.

You can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_CCCR)

For information about available fields see [`mod@fdcan_cccr`]
module*/
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCRrs>;
///For details about setting and resetting of single bits see Software initialization.
pub mod fdcan_cccr;
/**FDCAN_NBTP (rw) register accessor: This register is dedicated to the nominal bit timing used during the arbitration phase.

You can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_NBTP)

For information about available fields see [`mod@fdcan_nbtp`]
module*/
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTPrs>;
///This register is dedicated to the nominal bit timing used during the arbitration phase.
pub mod fdcan_nbtp;
/**FDCAN_TSCC (rw) register accessor: FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TSCC)

For information about available fields see [`mod@fdcan_tscc`]
module*/
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCCrs>;
///FDCAN timestamp counter configuration register
pub mod fdcan_tscc;
/**FDCAN_TSCV (rw) register accessor: FDCAN timestamp counter value register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TSCV)

For information about available fields see [`mod@fdcan_tscv`]
module*/
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCVrs>;
///FDCAN timestamp counter value register
pub mod fdcan_tscv;
/**FDCAN_TOCC (rw) register accessor: FDCAN timeout counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TOCC)

For information about available fields see [`mod@fdcan_tocc`]
module*/
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCCrs>;
///FDCAN timeout counter configuration register
pub mod fdcan_tocc;
/**FDCAN_TOCV (rw) register accessor: FDCAN timeout counter value register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TOCV)

For information about available fields see [`mod@fdcan_tocv`]
module*/
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCVrs>;
///FDCAN timeout counter value register
pub mod fdcan_tocv;
/**FDCAN_ECR (rw) register accessor: FDCAN error counter register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_ECR)

For information about available fields see [`mod@fdcan_ecr`]
module*/
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECRrs>;
///FDCAN error counter register
pub mod fdcan_ecr;
/**FDCAN_PSR (rw) register accessor: FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_PSR)

For information about available fields see [`mod@fdcan_psr`]
module*/
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSRrs>;
///FDCAN protocol status register
pub mod fdcan_psr;
/**FDCAN_TDCR (rw) register accessor: FDCAN transmitter delay compensation register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TDCR)

For information about available fields see [`mod@fdcan_tdcr`]
module*/
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
///FDCAN transmitter delay compensation register
pub mod fdcan_tdcr;
/**FDCAN_IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_IR)

For information about available fields see [`mod@fdcan_ir`]
module*/
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
///The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.
pub mod fdcan_ir;
/**FDCAN_IE (rw) register accessor: The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_IE)

For information about available fields see [`mod@fdcan_ie`]
module*/
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
///The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.
pub mod fdcan_ie;
/**FDCAN_ILS (rw) register accessor: This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_ILS)

For information about available fields see [`mod@fdcan_ils`]
module*/
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
///This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
pub mod fdcan_ils;
/**FDCAN_ILE (rw) register accessor: Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_ILE)

For information about available fields see [`mod@fdcan_ile`]
module*/
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
///Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.
pub mod fdcan_ile;
/**FDCAN_GFC (rw) register accessor: Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.

You can [`read`](crate::Reg::read) this register and get [`fdcan_gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_GFC)

For information about available fields see [`mod@fdcan_gfc`]
module*/
pub type FDCAN_GFC = crate::Reg<fdcan_gfc::FDCAN_GFCrs>;
///Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.
pub mod fdcan_gfc;
/**FDCAN_SIDFC (rw) register accessor: Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.

You can [`read`](crate::Reg::read) this register and get [`fdcan_sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_SIDFC)

For information about available fields see [`mod@fdcan_sidfc`]
module*/
pub type FDCAN_SIDFC = crate::Reg<fdcan_sidfc::FDCAN_SIDFCrs>;
///Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.
pub mod fdcan_sidfc;
/**FDCAN_XIDFC (rw) register accessor: Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_XIDFC)

For information about available fields see [`mod@fdcan_xidfc`]
module*/
pub type FDCAN_XIDFC = crate::Reg<fdcan_xidfc::FDCAN_XIDFCrs>;
///Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.
pub mod fdcan_xidfc;
/**FDCAN_XIDAM (rw) register accessor: FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_XIDAM)

For information about available fields see [`mod@fdcan_xidam`]
module*/
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAMrs>;
///FDCAN extended ID and mask register
pub mod fdcan_xidam;
/**FDCAN_HPMS (r) register accessor: This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.

You can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_HPMS)

For information about available fields see [`mod@fdcan_hpms`]
module*/
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMSrs>;
///This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.
pub mod fdcan_hpms;
/**FDCAN_NDAT1 (rw) register accessor: FDCAN new data 1 register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_NDAT1)

For information about available fields see [`mod@fdcan_ndat1`]
module*/
pub type FDCAN_NDAT1 = crate::Reg<fdcan_ndat1::FDCAN_NDAT1rs>;
///FDCAN new data 1 register
pub mod fdcan_ndat1;
/**FDCAN_NDAT2 (rw) register accessor: FDCAN new data 2 register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_NDAT2)

For information about available fields see [`mod@fdcan_ndat2`]
module*/
pub type FDCAN_NDAT2 = crate::Reg<fdcan_ndat2::FDCAN_NDAT2rs>;
///FDCAN new data 2 register
pub mod fdcan_ndat2;
/**FDCAN_RXF0C (rw) register accessor: FDCAN Rx FIFO 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF0C)

For information about available fields see [`mod@fdcan_rxf0c`]
module*/
pub type FDCAN_RXF0C = crate::Reg<fdcan_rxf0c::FDCAN_RXF0Crs>;
///FDCAN Rx FIFO 0 configuration register
pub mod fdcan_rxf0c;
/**FDCAN_RXF0S (rw) register accessor: FDCAN Rx FIFO 0 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF0S)

For information about available fields see [`mod@fdcan_rxf0s`]
module*/
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0Srs>;
///FDCAN Rx FIFO 0 status register
pub mod fdcan_rxf0s;
/**FDCAN_RXF0A (rw) register accessor: FDCAN Rx FIFO 0 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF0A)

For information about available fields see [`mod@fdcan_rxf0a`]
module*/
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0Ars>;
///FDCAN Rx FIFO 0 acknowledge register
pub mod fdcan_rxf0a;
/**FDCAN_RXBC (rw) register accessor: FDCAN Rx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXBC)

For information about available fields see [`mod@fdcan_rxbc`]
module*/
pub type FDCAN_RXBC = crate::Reg<fdcan_rxbc::FDCAN_RXBCrs>;
///FDCAN Rx buffer configuration register
pub mod fdcan_rxbc;
/**FDCAN_RXF1C (rw) register accessor: FDCAN Rx FIFO 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF1C)

For information about available fields see [`mod@fdcan_rxf1c`]
module*/
pub type FDCAN_RXF1C = crate::Reg<fdcan_rxf1c::FDCAN_RXF1Crs>;
///FDCAN Rx FIFO 1 configuration register
pub mod fdcan_rxf1c;
/**FDCAN_RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF1S)

For information about available fields see [`mod@fdcan_rxf1s`]
module*/
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1Srs>;
///FDCAN Rx FIFO 1 status register
pub mod fdcan_rxf1s;
/**FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXF1A)

For information about available fields see [`mod@fdcan_rxf1a`]
module*/
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1Ars>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod fdcan_rxf1a;
/**FDCAN_RXESC (r) register accessor: Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_RXESC)

For information about available fields see [`mod@fdcan_rxesc`]
module*/
pub type FDCAN_RXESC = crate::Reg<fdcan_rxesc::FDCAN_RXESCrs>;
///Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.
pub mod fdcan_rxesc;
/**FDCAN_TXBC (rw) register accessor: FDCAN Tx buffer configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBC)

For information about available fields see [`mod@fdcan_txbc`]
module*/
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
///FDCAN Tx buffer configuration register
pub mod fdcan_txbc;
/**FDCAN_TXFQS (r) register accessor: The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).

You can [`read`](crate::Reg::read) this register and get [`fdcan_txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXFQS)

For information about available fields see [`mod@fdcan_txfqs`]
module*/
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
///The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).
pub mod fdcan_txfqs;
/**FDCAN_TXESC (r) register accessor: Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXESC)

For information about available fields see [`mod@fdcan_txesc`]
module*/
pub type FDCAN_TXESC = crate::Reg<fdcan_txesc::FDCAN_TXESCrs>;
///Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.
pub mod fdcan_txesc;
/**FDCAN_TXBAR (rw) register accessor: FDCAN Tx buffer add request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBAR)

For information about available fields see [`mod@fdcan_txbar`]
module*/
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBARrs>;
///FDCAN Tx buffer add request register
pub mod fdcan_txbar;
/**FDCAN_TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBCR)

For information about available fields see [`mod@fdcan_txbcr`]
module*/
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCRrs>;
///FDCAN Tx buffer cancellation request register
pub mod fdcan_txbcr;
/**FDCAN_TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBTO)

For information about available fields see [`mod@fdcan_txbto`]
module*/
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTOrs>;
///FDCAN Tx buffer transmission occurred register
pub mod fdcan_txbto;
/**FDCAN_TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBCF)

For information about available fields see [`mod@fdcan_txbcf`]
module*/
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCFrs>;
///FDCAN Tx buffer cancellation finished register
pub mod fdcan_txbcf;
/**FDCAN_TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBTIE)

For information about available fields see [`mod@fdcan_txbtie`]
module*/
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIErs>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod fdcan_txbtie;
/**FDCAN_TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXBCIE)

For information about available fields see [`mod@fdcan_txbcie`]
module*/
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIErs>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod fdcan_txbcie;
/**FDCAN_TXEFC (rw) register accessor: FDCAN Tx event FIFO configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXEFC)

For information about available fields see [`mod@fdcan_txefc`]
module*/
pub type FDCAN_TXEFC = crate::Reg<fdcan_txefc::FDCAN_TXEFCrs>;
///FDCAN Tx event FIFO configuration register
pub mod fdcan_txefc;
/**FDCAN_TXEFS (r) register accessor: FDCAN Tx event FIFO status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXEFS)

For information about available fields see [`mod@fdcan_txefs`]
module*/
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
///FDCAN Tx event FIFO status register
pub mod fdcan_txefs;
/**FDCAN_TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXEFA)

For information about available fields see [`mod@fdcan_txefa`]
module*/
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
///FDCAN Tx event FIFO acknowledge register
pub mod fdcan_txefa;
/**FDCAN_TTTMC (rw) register accessor: FDCAN TT trigger memory configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTTMC)

For information about available fields see [`mod@fdcan_tttmc`]
module*/
pub type FDCAN_TTTMC = crate::Reg<fdcan_tttmc::FDCAN_TTTMCrs>;
///FDCAN TT trigger memory configuration register
pub mod fdcan_tttmc;
/**FDCAN_TTRMC (rw) register accessor: FDCAN TT reference message configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTRMC)

For information about available fields see [`mod@fdcan_ttrmc`]
module*/
pub type FDCAN_TTRMC = crate::Reg<fdcan_ttrmc::FDCAN_TTRMCrs>;
///FDCAN TT reference message configuration register
pub mod fdcan_ttrmc;
/**FDCAN_TTOCF (rw) register accessor: FDCAN TT operation configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTOCF)

For information about available fields see [`mod@fdcan_ttocf`]
module*/
pub type FDCAN_TTOCF = crate::Reg<fdcan_ttocf::FDCAN_TTOCFrs>;
///FDCAN TT operation configuration register
pub mod fdcan_ttocf;
/**FDCAN_TTMLM (rw) register accessor: FDCAN TT matrix limits register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTMLM)

For information about available fields see [`mod@fdcan_ttmlm`]
module*/
pub type FDCAN_TTMLM = crate::Reg<fdcan_ttmlm::FDCAN_TTMLMrs>;
///FDCAN TT matrix limits register
pub mod fdcan_ttmlm;
/**FDCAN_TURCF (rw) register accessor: The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.

You can [`read`](crate::Reg::read) this register and get [`fdcan_turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TURCF)

For information about available fields see [`mod@fdcan_turcf`]
module*/
pub type FDCAN_TURCF = crate::Reg<fdcan_turcf::FDCAN_TURCFrs>;
/**The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\[17:16\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\[15:0\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.*/
pub mod fdcan_turcf;
/**FDCAN_TTOCN (rw) register accessor: FDCAN TT operation control register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTOCN)

For information about available fields see [`mod@fdcan_ttocn`]
module*/
pub type FDCAN_TTOCN = crate::Reg<fdcan_ttocn::FDCAN_TTOCNrs>;
///FDCAN TT operation control register
pub mod fdcan_ttocn;
/**FDCAN_TTGTP (rw) register accessor: If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTGTP)

For information about available fields see [`mod@fdcan_ttgtp`]
module*/
pub type FDCAN_TTGTP = crate::Reg<fdcan_ttgtp::FDCAN_TTGTPrs>;
///If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.
pub mod fdcan_ttgtp;
/**FDCAN_TTTMK (rw) register accessor: A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.

You can [`read`](crate::Reg::read) this register and get [`fdcan_tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTTMK)

For information about available fields see [`mod@fdcan_tttmk`]
module*/
pub type FDCAN_TTTMK = crate::Reg<fdcan_tttmk::FDCAN_TTTMKrs>;
///A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.
pub mod fdcan_tttmk;
/**FDCAN_TTIR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTIR)

For information about available fields see [`mod@fdcan_ttir`]
module*/
pub type FDCAN_TTIR = crate::Reg<fdcan_ttir::FDCAN_TTIRrs>;
///The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.
pub mod fdcan_ttir;
/**FDCAN_TTIE (rw) register accessor: The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTIE)

For information about available fields see [`mod@fdcan_ttie`]
module*/
pub type FDCAN_TTIE = crate::Reg<fdcan_ttie::FDCAN_TTIErs>;
///The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.
pub mod fdcan_ttie;
/**FDCAN_TTILS (rw) register accessor: The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTILS)

For information about available fields see [`mod@fdcan_ttils`]
module*/
pub type FDCAN_TTILS = crate::Reg<fdcan_ttils::FDCAN_TTILSrs>;
///The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.
pub mod fdcan_ttils;
/**FDCAN_TTOST (r) register accessor: FDCAN TT operation status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttost::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTOST)

For information about available fields see [`mod@fdcan_ttost`]
module*/
pub type FDCAN_TTOST = crate::Reg<fdcan_ttost::FDCAN_TTOSTrs>;
///FDCAN TT operation status register
pub mod fdcan_ttost;
/**FDCAN_TURNA (r) register accessor: There is no drift compensation in TTCAN level 1.

You can [`read`](crate::Reg::read) this register and get [`fdcan_turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TURNA)

For information about available fields see [`mod@fdcan_turna`]
module*/
pub type FDCAN_TURNA = crate::Reg<fdcan_turna::FDCAN_TURNArs>;
///There is no drift compensation in TTCAN level 1.
pub mod fdcan_turna;
/**FDCAN_TTLGT (r) register accessor: FDCAN TT local and global time register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTLGT)

For information about available fields see [`mod@fdcan_ttlgt`]
module*/
pub type FDCAN_TTLGT = crate::Reg<fdcan_ttlgt::FDCAN_TTLGTrs>;
///FDCAN TT local and global time register
pub mod fdcan_ttlgt;
/**FDCAN_TTCTC (r) register accessor: FDCAN TT cycle time and count register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTCTC)

For information about available fields see [`mod@fdcan_ttctc`]
module*/
pub type FDCAN_TTCTC = crate::Reg<fdcan_ttctc::FDCAN_TTCTCrs>;
///FDCAN TT cycle time and count register
pub mod fdcan_ttctc;
/**FDCAN_TTCPT (r) register accessor: FDCAN TT capture time register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTCPT)

For information about available fields see [`mod@fdcan_ttcpt`]
module*/
pub type FDCAN_TTCPT = crate::Reg<fdcan_ttcpt::FDCAN_TTCPTrs>;
///FDCAN TT capture time register
pub mod fdcan_ttcpt;
/**FDCAN_TTCSM (r) register accessor: FDCAN TT cycle sync mark register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTCSM)

For information about available fields see [`mod@fdcan_ttcsm`]
module*/
pub type FDCAN_TTCSM = crate::Reg<fdcan_ttcsm::FDCAN_TTCSMrs>;
///FDCAN TT cycle sync mark register
pub mod fdcan_ttcsm;
/**FDCAN_TTTS (rw) register accessor: The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TTTS)

For information about available fields see [`mod@fdcan_ttts`]
module*/
pub type FDCAN_TTTS = crate::Reg<fdcan_ttts::FDCAN_TTTSrs>;
///The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.
pub mod fdcan_ttts;
