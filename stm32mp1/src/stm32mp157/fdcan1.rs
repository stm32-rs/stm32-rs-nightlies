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
    #[doc = "0x00 - FDCAN core release register"]
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FDCAN_CREL {
        &self.fdcan_crel
    }
    #[doc = "0x04 - FDCAN Endian register"]
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FDCAN_ENDN {
        &self.fdcan_endn
    }
    #[doc = "0x0c - This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FDCAN_DBTP {
        &self.fdcan_dbtp
    }
    #[doc = "0x10 - Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FDCAN_TEST {
        &self.fdcan_test
    }
    #[doc = "0x14 - The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock."]
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FDCAN_RWD {
        &self.fdcan_rwd
    }
    #[doc = "0x18 - For details about setting and resetting of single bits see Software initialization."]
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FDCAN_CCCR {
        &self.fdcan_cccr
    }
    #[doc = "0x1c - This register is dedicated to the nominal bit timing used during the arbitration phase."]
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FDCAN_NBTP {
        &self.fdcan_nbtp
    }
    #[doc = "0x20 - FDCAN timestamp counter configuration register"]
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FDCAN_TSCC {
        &self.fdcan_tscc
    }
    #[doc = "0x24 - FDCAN timestamp counter value register"]
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FDCAN_TSCV {
        &self.fdcan_tscv
    }
    #[doc = "0x28 - FDCAN timeout counter configuration register"]
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FDCAN_TOCC {
        &self.fdcan_tocc
    }
    #[doc = "0x2c - FDCAN timeout counter value register"]
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FDCAN_TOCV {
        &self.fdcan_tocv
    }
    #[doc = "0x40 - FDCAN error counter register"]
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FDCAN_ECR {
        &self.fdcan_ecr
    }
    #[doc = "0x44 - FDCAN protocol status register"]
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FDCAN_PSR {
        &self.fdcan_psr
    }
    #[doc = "0x48 - FDCAN transmitter delay compensation register"]
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FDCAN_TDCR {
        &self.fdcan_tdcr
    }
    #[doc = "0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FDCAN_IR {
        &self.fdcan_ir
    }
    #[doc = "0x54 - The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line."]
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FDCAN_IE {
        &self.fdcan_ie
    }
    #[doc = "0x58 - This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1."]
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FDCAN_ILS {
        &self.fdcan_ils
    }
    #[doc = "0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FDCAN_ILE {
        &self.fdcan_ile
    }
    #[doc = "0x80 - Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path."]
    #[inline(always)]
    pub const fn fdcan_gfc(&self) -> &FDCAN_GFC {
        &self.fdcan_gfc
    }
    #[doc = "0x84 - Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708."]
    #[inline(always)]
    pub const fn fdcan_sidfc(&self) -> &FDCAN_SIDFC {
        &self.fdcan_sidfc
    }
    #[doc = "0x88 - Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path."]
    #[inline(always)]
    pub const fn fdcan_xidfc(&self) -> &FDCAN_XIDFC {
        &self.fdcan_xidfc
    }
    #[doc = "0x90 - FDCAN extended ID and mask register"]
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FDCAN_XIDAM {
        &self.fdcan_xidam
    }
    #[doc = "0x94 - This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FDCAN_HPMS {
        &self.fdcan_hpms
    }
    #[doc = "0x98 - FDCAN new data 1 register"]
    #[inline(always)]
    pub const fn fdcan_ndat1(&self) -> &FDCAN_NDAT1 {
        &self.fdcan_ndat1
    }
    #[doc = "0x9c - FDCAN new data 2 register"]
    #[inline(always)]
    pub const fn fdcan_ndat2(&self) -> &FDCAN_NDAT2 {
        &self.fdcan_ndat2
    }
    #[doc = "0xa0 - FDCAN Rx FIFO 0 configuration register"]
    #[inline(always)]
    pub const fn fdcan_rxf0c(&self) -> &FDCAN_RXF0C {
        &self.fdcan_rxf0c
    }
    #[doc = "0xa4 - FDCAN Rx FIFO 0 status register"]
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FDCAN_RXF0S {
        &self.fdcan_rxf0s
    }
    #[doc = "0xa8 - FDCAN Rx FIFO 0 acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FDCAN_RXF0A {
        &self.fdcan_rxf0a
    }
    #[doc = "0xac - FDCAN Rx buffer configuration register"]
    #[inline(always)]
    pub const fn fdcan_rxbc(&self) -> &FDCAN_RXBC {
        &self.fdcan_rxbc
    }
    #[doc = "0xb0 - FDCAN Rx FIFO 1 configuration register"]
    #[inline(always)]
    pub const fn fdcan_rxf1c(&self) -> &FDCAN_RXF1C {
        &self.fdcan_rxf1c
    }
    #[doc = "0xb4 - FDCAN Rx FIFO 1 status register"]
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FDCAN_RXF1S {
        &self.fdcan_rxf1s
    }
    #[doc = "0xb8 - FDCAN Rx FIFO 1 acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FDCAN_RXF1A {
        &self.fdcan_rxf1a
    }
    #[doc = "0xbc - Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only."]
    #[inline(always)]
    pub const fn fdcan_rxesc(&self) -> &FDCAN_RXESC {
        &self.fdcan_rxesc
    }
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FDCAN_TXBC {
        &self.fdcan_txbc
    }
    #[doc = "0xc4 - The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated)."]
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FDCAN_TXFQS {
        &self.fdcan_txfqs
    }
    #[doc = "0xc8 - Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only."]
    #[inline(always)]
    pub const fn fdcan_txesc(&self) -> &FDCAN_TXESC {
        &self.fdcan_txesc
    }
    #[doc = "0xd0 - FDCAN Tx buffer add request register"]
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FDCAN_TXBAR {
        &self.fdcan_txbar
    }
    #[doc = "0xd4 - FDCAN Tx buffer cancellation request register"]
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FDCAN_TXBCR {
        &self.fdcan_txbcr
    }
    #[doc = "0xd8 - FDCAN Tx buffer transmission occurred register"]
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FDCAN_TXBTO {
        &self.fdcan_txbto
    }
    #[doc = "0xdc - FDCAN Tx buffer cancellation finished register"]
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FDCAN_TXBCF {
        &self.fdcan_txbcf
    }
    #[doc = "0xe0 - FDCAN Tx buffer transmission interrupt enable register"]
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FDCAN_TXBTIE {
        &self.fdcan_txbtie
    }
    #[doc = "0xe4 - FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FDCAN_TXBCIE {
        &self.fdcan_txbcie
    }
    #[doc = "0xf0 - FDCAN Tx event FIFO configuration register"]
    #[inline(always)]
    pub const fn fdcan_txefc(&self) -> &FDCAN_TXEFC {
        &self.fdcan_txefc
    }
    #[doc = "0xf4 - FDCAN Tx event FIFO status register"]
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FDCAN_TXEFS {
        &self.fdcan_txefs
    }
    #[doc = "0xf8 - FDCAN Tx event FIFO acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FDCAN_TXEFA {
        &self.fdcan_txefa
    }
    #[doc = "0x100 - FDCAN TT trigger memory configuration register"]
    #[inline(always)]
    pub const fn fdcan_tttmc(&self) -> &FDCAN_TTTMC {
        &self.fdcan_tttmc
    }
    #[doc = "0x104 - FDCAN TT reference message configuration register"]
    #[inline(always)]
    pub const fn fdcan_ttrmc(&self) -> &FDCAN_TTRMC {
        &self.fdcan_ttrmc
    }
    #[doc = "0x108 - FDCAN TT operation configuration register"]
    #[inline(always)]
    pub const fn fdcan_ttocf(&self) -> &FDCAN_TTOCF {
        &self.fdcan_ttocf
    }
    #[doc = "0x10c - FDCAN TT matrix limits register"]
    #[inline(always)]
    pub const fn fdcan_ttmlm(&self) -> &FDCAN_TTMLM {
        &self.fdcan_ttmlm
    }
    #[doc = "0x110 - The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\\[17:16\\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\\[15:0\\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process."]
    #[inline(always)]
    pub const fn fdcan_turcf(&self) -> &FDCAN_TURCF {
        &self.fdcan_turcf
    }
    #[doc = "0x114 - FDCAN TT operation control register"]
    #[inline(always)]
    pub const fn fdcan_ttocn(&self) -> &FDCAN_TTOCN {
        &self.fdcan_ttocn
    }
    #[doc = "0x118 - If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master."]
    #[inline(always)]
    pub const fn fdcan_ttgtp(&self) -> &FDCAN_TTGTP {
        &self.fdcan_ttgtp
    }
    #[doc = "0x11c - A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM."]
    #[inline(always)]
    pub const fn fdcan_tttmk(&self) -> &FDCAN_TTTMK {
        &self.fdcan_tttmk
    }
    #[doc = "0x120 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register."]
    #[inline(always)]
    pub const fn fdcan_ttir(&self) -> &FDCAN_TTIR {
        &self.fdcan_ttir
    }
    #[doc = "0x124 - The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt."]
    #[inline(always)]
    pub const fn fdcan_ttie(&self) -> &FDCAN_TTIE {
        &self.fdcan_ttie
    }
    #[doc = "0x128 - The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1."]
    #[inline(always)]
    pub const fn fdcan_ttils(&self) -> &FDCAN_TTILS {
        &self.fdcan_ttils
    }
    #[doc = "0x12c - FDCAN TT operation status register"]
    #[inline(always)]
    pub const fn fdcan_ttost(&self) -> &FDCAN_TTOST {
        &self.fdcan_ttost
    }
    #[doc = "0x130 - There is no drift compensation in TTCAN level 1."]
    #[inline(always)]
    pub const fn fdcan_turna(&self) -> &FDCAN_TURNA {
        &self.fdcan_turna
    }
    #[doc = "0x134 - FDCAN TT local and global time register"]
    #[inline(always)]
    pub const fn fdcan_ttlgt(&self) -> &FDCAN_TTLGT {
        &self.fdcan_ttlgt
    }
    #[doc = "0x138 - FDCAN TT cycle time and count register"]
    #[inline(always)]
    pub const fn fdcan_ttctc(&self) -> &FDCAN_TTCTC {
        &self.fdcan_ttctc
    }
    #[doc = "0x13c - FDCAN TT capture time register"]
    #[inline(always)]
    pub const fn fdcan_ttcpt(&self) -> &FDCAN_TTCPT {
        &self.fdcan_ttcpt
    }
    #[doc = "0x140 - FDCAN TT cycle sync mark register"]
    #[inline(always)]
    pub const fn fdcan_ttcsm(&self) -> &FDCAN_TTCSM {
        &self.fdcan_ttcsm
    }
    #[doc = "0x300 - The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger."]
    #[inline(always)]
    pub const fn fdcan_ttts(&self) -> &FDCAN_TTTS {
        &self.fdcan_ttts
    }
}
#[doc = "FDCAN_CREL (r) register accessor: FDCAN core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_crel`]
module"]
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CRELrs>;
#[doc = "FDCAN core release register"]
pub mod fdcan_crel;
#[doc = "FDCAN_ENDN (r) register accessor: FDCAN Endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_endn`]
module"]
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDNrs>;
#[doc = "FDCAN Endian register"]
pub mod fdcan_endn;
#[doc = "FDCAN_DBTP (rw) register accessor: This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_dbtp`]
module"]
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTPrs>;
#[doc = "This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (rw) register accessor: Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_test`]
module"]
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TESTrs>;
#[doc = "Write access to this register has to be enabled by setting bit FDCAN_CCCR.TEST to 1. All register functions are set to their reset values when bit FDCAN_CCCR.TEST is reset. Loop back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (rw) register accessor: The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rwd`]
module"]
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWDrs>;
#[doc = "The RAM watchdog monitors the READY output of the message RAM. A message RAM access starts the message RAM watchdog counter with the value configured by the FDCAN_RWD.WDC bits. The counter is reloaded with FDCAN_RWD.WDC bits when the message RAM signals successful completion by activating its READY output. In case there is no response from the message RAM until the counter has counted down to 0, the counter stops and interrupt flag FDCAN_IR.WDI bit is set. The RAM watchdog counter is clocked by the fdcan_pclk clock."]
pub mod fdcan_rwd;
#[doc = "FDCAN_CCCR (rw) register accessor: For details about setting and resetting of single bits see Software initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_cccr`]
module"]
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCRrs>;
#[doc = "For details about setting and resetting of single bits see Software initialization."]
pub mod fdcan_cccr;
#[doc = "FDCAN_NBTP (rw) register accessor: This register is dedicated to the nominal bit timing used during the arbitration phase.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_nbtp`]
module"]
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTPrs>;
#[doc = "This register is dedicated to the nominal bit timing used during the arbitration phase."]
pub mod fdcan_nbtp;
#[doc = "FDCAN_TSCC (rw) register accessor: FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscc`]
module"]
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCCrs>;
#[doc = "FDCAN timestamp counter configuration register"]
pub mod fdcan_tscc;
#[doc = "FDCAN_TSCV (rw) register accessor: FDCAN timestamp counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscv`]
module"]
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCVrs>;
#[doc = "FDCAN timestamp counter value register"]
pub mod fdcan_tscv;
#[doc = "FDCAN_TOCC (rw) register accessor: FDCAN timeout counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocc`]
module"]
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCCrs>;
#[doc = "FDCAN timeout counter configuration register"]
pub mod fdcan_tocc;
#[doc = "FDCAN_TOCV (rw) register accessor: FDCAN timeout counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocv`]
module"]
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCVrs>;
#[doc = "FDCAN timeout counter value register"]
pub mod fdcan_tocv;
#[doc = "FDCAN_ECR (rw) register accessor: FDCAN error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ecr`]
module"]
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECRrs>;
#[doc = "FDCAN error counter register"]
pub mod fdcan_ecr;
#[doc = "FDCAN_PSR (rw) register accessor: FDCAN protocol status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_psr`]
module"]
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSRrs>;
#[doc = "FDCAN protocol status register"]
pub mod fdcan_psr;
#[doc = "FDCAN_TDCR (rw) register accessor: FDCAN transmitter delay compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tdcr`]
module"]
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCRrs>;
#[doc = "FDCAN transmitter delay compensation register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ir`]
module"]
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IRrs>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (rw) register accessor: The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ie`]
module"]
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IErs>;
#[doc = "The settings in the interrupt enable register determine which status changes in the interrupt register will be signaled on an interrupt line."]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (rw) register accessor: This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ils`]
module"]
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILSrs>;
#[doc = "This register assigns an interrupt generated by a specific interrupt flag from the interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1."]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ile`]
module"]
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILErs>;
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
pub mod fdcan_ile;
#[doc = "FDCAN_GFC (rw) register accessor: Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_gfc`]
module"]
pub type FDCAN_GFC = crate::Reg<fdcan_gfc::FDCAN_GFCrs>;
#[doc = "Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path."]
pub mod fdcan_gfc;
#[doc = "FDCAN_SIDFC (rw) register accessor: Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_sidfc`]
module"]
pub type FDCAN_SIDFC = crate::Reg<fdcan_sidfc::FDCAN_SIDFCrs>;
#[doc = "Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708."]
pub mod fdcan_sidfc;
#[doc = "FDCAN_XIDFC (rw) register accessor: Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidfc`]
module"]
pub type FDCAN_XIDFC = crate::Reg<fdcan_xidfc::FDCAN_XIDFCrs>;
#[doc = "Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path."]
pub mod fdcan_xidfc;
#[doc = "FDCAN_XIDAM (rw) register accessor: FDCAN extended ID and mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidam`]
module"]
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAMrs>;
#[doc = "FDCAN extended ID and mask register"]
pub mod fdcan_xidam;
#[doc = "FDCAN_HPMS (r) register accessor: This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_hpms`]
module"]
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMSrs>;
#[doc = "This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
pub mod fdcan_hpms;
#[doc = "FDCAN_NDAT1 (rw) register accessor: FDCAN new data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat1`]
module"]
pub type FDCAN_NDAT1 = crate::Reg<fdcan_ndat1::FDCAN_NDAT1rs>;
#[doc = "FDCAN new data 1 register"]
pub mod fdcan_ndat1;
#[doc = "FDCAN_NDAT2 (rw) register accessor: FDCAN new data 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat2`]
module"]
pub type FDCAN_NDAT2 = crate::Reg<fdcan_ndat2::FDCAN_NDAT2rs>;
#[doc = "FDCAN new data 2 register"]
pub mod fdcan_ndat2;
#[doc = "FDCAN_RXF0C (rw) register accessor: FDCAN Rx FIFO 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0c`]
module"]
pub type FDCAN_RXF0C = crate::Reg<fdcan_rxf0c::FDCAN_RXF0Crs>;
#[doc = "FDCAN Rx FIFO 0 configuration register"]
pub mod fdcan_rxf0c;
#[doc = "FDCAN_RXF0S (rw) register accessor: FDCAN Rx FIFO 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0s`]
module"]
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0Srs>;
#[doc = "FDCAN Rx FIFO 0 status register"]
pub mod fdcan_rxf0s;
#[doc = "FDCAN_RXF0A (rw) register accessor: FDCAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0a`]
module"]
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0Ars>;
#[doc = "FDCAN Rx FIFO 0 acknowledge register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN_RXBC (rw) register accessor: FDCAN Rx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxbc`]
module"]
pub type FDCAN_RXBC = crate::Reg<fdcan_rxbc::FDCAN_RXBCrs>;
#[doc = "FDCAN Rx buffer configuration register"]
pub mod fdcan_rxbc;
#[doc = "FDCAN_RXF1C (rw) register accessor: FDCAN Rx FIFO 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1c`]
module"]
pub type FDCAN_RXF1C = crate::Reg<fdcan_rxf1c::FDCAN_RXF1Crs>;
#[doc = "FDCAN Rx FIFO 1 configuration register"]
pub mod fdcan_rxf1c;
#[doc = "FDCAN_RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1s`]
module"]
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1Srs>;
#[doc = "FDCAN Rx FIFO 1 status register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1a`]
module"]
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1Ars>;
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN_RXESC (r) register accessor: Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxesc`]
module"]
pub type FDCAN_RXESC = crate::Reg<fdcan_rxesc::FDCAN_RXESCrs>;
#[doc = "Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only."]
pub mod fdcan_rxesc;
#[doc = "FDCAN_TXBC (rw) register accessor: FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbc`]
module"]
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBCrs>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod fdcan_txbc;
#[doc = "FDCAN_TXFQS (r) register accessor: The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txfqs`]
module"]
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQSrs>;
#[doc = "The Tx FIFO/queue status is related to the pending Tx requests listed in register FDCAN_TXBRP. Therefore the effect of add/cancellation requests may be delayed due to a running Tx scan (FDCAN_TXBRP not yet updated)."]
pub mod fdcan_txfqs;
#[doc = "FDCAN_TXESC (r) register accessor: Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txesc`]
module"]
pub type FDCAN_TXESC = crate::Reg<fdcan_txesc::FDCAN_TXESCrs>;
#[doc = "Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only."]
pub mod fdcan_txesc;
#[doc = "FDCAN_TXBAR (rw) register accessor: FDCAN Tx buffer add request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbar`]
module"]
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBARrs>;
#[doc = "FDCAN Tx buffer add request register"]
pub mod fdcan_txbar;
#[doc = "FDCAN_TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcr`]
module"]
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCRrs>;
#[doc = "FDCAN Tx buffer cancellation request register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN_TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbto`]
module"]
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTOrs>;
#[doc = "FDCAN Tx buffer transmission occurred register"]
pub mod fdcan_txbto;
#[doc = "FDCAN_TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcf`]
module"]
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCFrs>;
#[doc = "FDCAN Tx buffer cancellation finished register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN_TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbtie`]
module"]
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIErs>;
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN_TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcie`]
module"]
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIErs>;
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN_TXEFC (rw) register accessor: FDCAN Tx event FIFO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefc`]
module"]
pub type FDCAN_TXEFC = crate::Reg<fdcan_txefc::FDCAN_TXEFCrs>;
#[doc = "FDCAN Tx event FIFO configuration register"]
pub mod fdcan_txefc;
#[doc = "FDCAN_TXEFS (r) register accessor: FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefs`]
module"]
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFSrs>;
#[doc = "FDCAN Tx event FIFO status register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefa`]
module"]
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFArs>;
#[doc = "FDCAN Tx event FIFO acknowledge register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_TTTMC (rw) register accessor: FDCAN TT trigger memory configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmc`]
module"]
pub type FDCAN_TTTMC = crate::Reg<fdcan_tttmc::FDCAN_TTTMCrs>;
#[doc = "FDCAN TT trigger memory configuration register"]
pub mod fdcan_tttmc;
#[doc = "FDCAN_TTRMC (rw) register accessor: FDCAN TT reference message configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttrmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttrmc`]
module"]
pub type FDCAN_TTRMC = crate::Reg<fdcan_ttrmc::FDCAN_TTRMCrs>;
#[doc = "FDCAN TT reference message configuration register"]
pub mod fdcan_ttrmc;
#[doc = "FDCAN_TTOCF (rw) register accessor: FDCAN TT operation configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocf`]
module"]
pub type FDCAN_TTOCF = crate::Reg<fdcan_ttocf::FDCAN_TTOCFrs>;
#[doc = "FDCAN TT operation configuration register"]
pub mod fdcan_ttocf;
#[doc = "FDCAN_TTMLM (rw) register accessor: FDCAN TT matrix limits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttmlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttmlm`]
module"]
pub type FDCAN_TTMLM = crate::Reg<fdcan_ttmlm::FDCAN_TTMLMrs>;
#[doc = "FDCAN TT matrix limits register"]
pub mod fdcan_ttmlm;
#[doc = "FDCAN_TURCF (rw) register accessor: The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\\[17:16\\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\\[15:0\\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_turcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_turcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turcf`]
module"]
pub type FDCAN_TURCF = crate::Reg<fdcan_turcf::FDCAN_TURCFrs>;
#[doc = "The length of the NTU is given by: NTU = CAN clock period x NC/DC. NC is an 18-bit value. Its high part, NCH\\[17:16\\]
is hard wired to 0b01. Therefore the range of NC extends from 0x10000 to 0x1FFFF. The value configured by NCL is the initial value for FDCAN_TURNA.NAV\\[15:0\\]. DC is set to 0x1000 by hardware reset and it may not be written to 0x0000. Level 1: NC 4 * DC and NTU = CAN bit time Levels 0 and 2: NC 8 * DC The actual value of FDCAN_TUR may be changed by the clock drift compensation function of TTCAN level 0 and level 2 in order to adjust the node local view of the NTU to the time master view of the NTU. DC will not be changed by the automatic drift compensation, FDCAN_TURNA.NAV may be adjusted around NC in the range of the synchronization deviation limit given by FDCAN_TTOCF.LDSDL. NC and DC should be programmed to the largest suitable values in achieve the best computational accuracy for the drift compensation process."]
pub mod fdcan_turcf;
#[doc = "FDCAN_TTOCN (rw) register accessor: FDCAN TT operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocn`]
module"]
pub type FDCAN_TTOCN = crate::Reg<fdcan_ttocn::FDCAN_TTOCNrs>;
#[doc = "FDCAN TT operation control register"]
pub mod fdcan_ttocn;
#[doc = "FDCAN_TTGTP (rw) register accessor: If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttgtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttgtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttgtp`]
module"]
pub type FDCAN_TTGTP = crate::Reg<fdcan_ttgtp::FDCAN_TTGTPrs>;
#[doc = "If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master."]
pub mod fdcan_ttgtp;
#[doc = "FDCAN_TTTMK (rw) register accessor: A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmk`]
module"]
pub type FDCAN_TTTMK = crate::Reg<fdcan_tttmk::FDCAN_TTTMKrs>;
#[doc = "A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM."]
pub mod fdcan_tttmk;
#[doc = "FDCAN_TTIR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttir`]
module"]
pub type FDCAN_TTIR = crate::Reg<fdcan_ttir::FDCAN_TTIRrs>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register."]
pub mod fdcan_ttir;
#[doc = "FDCAN_TTIE (rw) register accessor: The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttie`]
module"]
pub type FDCAN_TTIE = crate::Reg<fdcan_ttie::FDCAN_TTIErs>;
#[doc = "The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt."]
pub mod fdcan_ttie;
#[doc = "FDCAN_TTILS (rw) register accessor: The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttils`]
module"]
pub type FDCAN_TTILS = crate::Reg<fdcan_ttils::FDCAN_TTILSrs>;
#[doc = "The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1."]
pub mod fdcan_ttils;
#[doc = "FDCAN_TTOST (r) register accessor: FDCAN TT operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttost::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttost`]
module"]
pub type FDCAN_TTOST = crate::Reg<fdcan_ttost::FDCAN_TTOSTrs>;
#[doc = "FDCAN TT operation status register"]
pub mod fdcan_ttost;
#[doc = "FDCAN_TURNA (r) register accessor: There is no drift compensation in TTCAN level 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_turna::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turna`]
module"]
pub type FDCAN_TURNA = crate::Reg<fdcan_turna::FDCAN_TURNArs>;
#[doc = "There is no drift compensation in TTCAN level 1."]
pub mod fdcan_turna;
#[doc = "FDCAN_TTLGT (r) register accessor: FDCAN TT local and global time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttlgt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttlgt`]
module"]
pub type FDCAN_TTLGT = crate::Reg<fdcan_ttlgt::FDCAN_TTLGTrs>;
#[doc = "FDCAN TT local and global time register"]
pub mod fdcan_ttlgt;
#[doc = "FDCAN_TTCTC (r) register accessor: FDCAN TT cycle time and count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttctc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttctc`]
module"]
pub type FDCAN_TTCTC = crate::Reg<fdcan_ttctc::FDCAN_TTCTCrs>;
#[doc = "FDCAN TT cycle time and count register"]
pub mod fdcan_ttctc;
#[doc = "FDCAN_TTCPT (r) register accessor: FDCAN TT capture time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcpt`]
module"]
pub type FDCAN_TTCPT = crate::Reg<fdcan_ttcpt::FDCAN_TTCPTrs>;
#[doc = "FDCAN TT capture time register"]
pub mod fdcan_ttcpt;
#[doc = "FDCAN_TTCSM (r) register accessor: FDCAN TT cycle sync mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcsm`]
module"]
pub type FDCAN_TTCSM = crate::Reg<fdcan_ttcsm::FDCAN_TTCSMrs>;
#[doc = "FDCAN TT cycle sync mark register"]
pub mod fdcan_ttcsm;
#[doc = "FDCAN_TTTS (rw) register accessor: The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttts`]
module"]
pub type FDCAN_TTTS = crate::Reg<fdcan_ttts::FDCAN_TTTSrs>;
#[doc = "The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger."]
pub mod fdcan_ttts;
