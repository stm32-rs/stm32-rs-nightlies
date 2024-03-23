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
    rxgfc: RXGFC,
    xidam: XIDAM,
    hpms: HPMS,
    _reserved21: [u8; 0x04],
    rxf0s: RXF0S,
    rxf0a: RXF0A,
    rxf1s: RXF1S,
    rxf1a: RXF1A,
    _reserved25: [u8; 0x20],
    txbc: TXBC,
    txfqs: TXFQS,
    txbrp: TXBRP,
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved36: [u8; 0x14],
    ckdiv: CKDIV,
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
    #[doc = "0x0c - This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    #[doc = "0x10 - Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    #[doc = "0x14 - The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    #[doc = "0x18 - For details about setting and resetting of single bits see Software initialization."]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x1c - FDCAN_NBTP"]
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
    #[doc = "0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x54 - The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x58 - The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    #[doc = "0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    #[doc = "0x80 - Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    #[inline(always)]
    pub const fn rxgfc(&self) -> &RXGFC {
        &self.rxgfc
    }
    #[doc = "0x84 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    #[doc = "0x88 - This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    #[doc = "0x94 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    #[doc = "0xc4 - The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    #[doc = "0xcc - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    #[doc = "0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    #[doc = "0xe4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    #[doc = "0xe8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn ckdiv(&self) -> &CKDIV {
        &self.ckdiv
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
#[doc = "DBTP (rw) register accessor: This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTPrs>;
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TESTrs>;
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
pub mod test;
#[doc = "RWD (rw) register accessor: The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWDrs>;
#[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: For details about setting and resetting of single bits see Software initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCRrs>;
#[doc = "For details about setting and resetting of single bits see Software initialization."]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: FDCAN_NBTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTPrs>;
#[doc = "FDCAN_NBTP"]
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
#[doc = "ECR (r) register accessor: FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
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
#[doc = "IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
pub mod ir;
#[doc = "IE (rw) register accessor: The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IErs>;
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
pub mod ie;
#[doc = "ILS (rw) register accessor: The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
pub type ILS = crate::Reg<ils::ILSrs>;
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
pub mod ils;
#[doc = "ILE (rw) register accessor: Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
pub type ILE = crate::Reg<ile::ILErs>;
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
pub mod ile;
#[doc = "RXGFC (rw) register accessor: Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxgfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxgfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxgfc`]
module"]
pub type RXGFC = crate::Reg<rxgfc::RXGFCrs>;
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
pub mod rxgfc;
#[doc = "XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAMrs>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMSrs>;
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
pub mod hpms;
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
#[doc = "RXF1S (r) register accessor: FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1Srs>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1Ars>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBCrs>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQSrs>;
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
pub mod txfqs;
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
#[doc = "TXBTO (r) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
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
#[doc = "TXEFS (r) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFSrs>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFArs>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "CKDIV (rw) register accessor: FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckdiv`]
module"]
pub type CKDIV = crate::Reg<ckdiv::CKDIVrs>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
