#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub crel: crate::Reg<crel::CREL_SPEC>,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    #[doc = "0x10 - Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x14 - The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    #[doc = "0x18 - For details about setting and resetting of single bits see Software initialization."]
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    #[doc = "0x1c - FDCAN_NBTP"]
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - FDCAN Error Counter Register"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x54 - The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x58 - The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    pub ils: crate::Reg<ils::ILS_SPEC>,
    #[doc = "0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved18: [u8; 0x20],
    #[doc = "0x80 - Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    pub rxgfc: crate::Reg<rxgfc::RXGFC_SPEC>,
    #[doc = "0x84 - FDCAN Extended ID and Mask Register"]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    #[doc = "0x88 - This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - FDCAN Rx FIFO 0 Status Register"]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    #[doc = "0x94 - CAN Rx FIFO 0 Acknowledge Register"]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    #[doc = "0x98 - FDCAN Rx FIFO 1 Status Register"]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    #[doc = "0x9c - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    _reserved25: [u8; 0x20],
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    #[doc = "0xc4 - The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    #[doc = "0xc8 - FDCAN Tx Buffer Request Pending Register"]
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    #[doc = "0xcc - FDCAN Tx Buffer Add Request Register"]
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    #[doc = "0xd0 - FDCAN Tx Buffer Cancellation Request Register"]
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    #[doc = "0xd4 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    #[doc = "0xd8 - FDCAN Tx Buffer Cancellation Finished Register"]
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    #[doc = "0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    #[doc = "0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    #[doc = "0xe4 - FDCAN Tx Event FIFO Status Register"]
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    #[doc = "0xe8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
    _reserved36: [u8; 0x14],
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    pub ckdiv: crate::Reg<ckdiv::CKDIV_SPEC>,
}
#[doc = "CREL register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "FDCAN Core Release Register"]
pub mod crel;
#[doc = "ENDN register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "FDCAN Core Release Register"]
pub mod endn;
#[doc = "DBTP register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
pub mod dbtp;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
pub mod test;
#[doc = "RWD register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
pub mod rwd;
#[doc = "CCCR register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "For details about setting and resetting of single bits see Software initialization."]
pub mod cccr;
#[doc = "NBTP register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "FDCAN_NBTP"]
pub mod nbtp;
#[doc = "TSCC register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "FDCAN Error Counter Register"]
pub mod ecr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "FDCAN Protocol Status Register"]
pub mod psr;
#[doc = "TDCR register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
pub mod ir;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
pub mod ie;
#[doc = "ILS register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
pub mod ils;
#[doc = "ILE register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
pub mod ile;
#[doc = "RXGFC register accessor: an alias for `Reg<RXGFC_SPEC>`"]
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
pub mod rxgfc;
#[doc = "XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "HPMS register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
pub mod hpms;
#[doc = "RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "TXBC register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
pub mod txfqs;
#[doc = "TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "CKDIV register accessor: an alias for `Reg<CKDIV_SPEC>`"]
pub type CKDIV = crate::Reg<ckdiv::CKDIV_SPEC>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
