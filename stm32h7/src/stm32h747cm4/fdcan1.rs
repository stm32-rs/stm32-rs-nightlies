#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub crel: crate::Reg<crel::CREL_SPEC>,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    #[doc = "0x10 - FDCAN Test Register"]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    #[doc = "0x18 - FDCAN CC Control Register"]
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
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
    #[doc = "0x50 - FDCAN Interrupt Register"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    pub ils: crate::Reg<ils::ILS_SPEC>,
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved18: [u8; 0x20],
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    pub gfc: crate::Reg<gfc::GFC_SPEC>,
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    pub sidfc: crate::Reg<sidfc::SIDFC_SPEC>,
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    pub xidfc: crate::Reg<xidfc::XIDFC_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    pub ndat1: crate::Reg<ndat1::NDAT1_SPEC>,
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    pub ndat2: crate::Reg<ndat2::NDAT2_SPEC>,
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    pub rxf0c: crate::Reg<rxf0c::RXF0C_SPEC>,
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    pub rxbc: crate::Reg<rxbc::RXBC_SPEC>,
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    pub rxf1c: crate::Reg<rxf1c::RXF1C_SPEC>,
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    pub rxesc: crate::Reg<rxesc::RXESC_SPEC>,
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    pub txesc: crate::Reg<txesc::TXESC_SPEC>,
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    pub txefc: crate::Reg<txefc::TXEFC_SPEC>,
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    pub tttmc: crate::Reg<tttmc::TTTMC_SPEC>,
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    pub ttrmc: crate::Reg<ttrmc::TTRMC_SPEC>,
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    pub ttocf: crate::Reg<ttocf::TTOCF_SPEC>,
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    pub ttmlm: crate::Reg<ttmlm::TTMLM_SPEC>,
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    pub turcf: crate::Reg<turcf::TURCF_SPEC>,
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    pub ttocn: crate::Reg<ttocn::TTOCN_SPEC>,
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    pub ttgtp: crate::Reg<ttgtp::TTGTP_SPEC>,
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    pub tttmk: crate::Reg<tttmk::TTTMK_SPEC>,
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    pub ttir: crate::Reg<ttir::TTIR_SPEC>,
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    pub ttie: crate::Reg<ttie::TTIE_SPEC>,
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    pub ttils: crate::Reg<ttils::TTILS_SPEC>,
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    pub ttost: crate::Reg<ttost::TTOST_SPEC>,
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    pub turna: crate::Reg<turna::TURNA_SPEC>,
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    pub ttlgt: crate::Reg<ttlgt::TTLGT_SPEC>,
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    pub ttctc: crate::Reg<ttctc::TTCTC_SPEC>,
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    pub ttcpt: crate::Reg<ttcpt::TTCPT_SPEC>,
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    pub ttcsm: crate::Reg<ttcsm::TTCSM_SPEC>,
    _reserved63: [u8; 0x01bc],
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    pub ttts: crate::Reg<ttts::TTTS_SPEC>,
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
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "FDCAN Test Register"]
pub mod test;
#[doc = "RWD register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "FDCAN RAM Watchdog Register"]
pub mod rwd;
#[doc = "CCCR register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "FDCAN CC Control Register"]
pub mod cccr;
#[doc = "NBTP register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
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
#[doc = "FDCAN Interrupt Register"]
pub mod ir;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod ie;
#[doc = "ILS register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod ils;
#[doc = "ILE register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod ile;
#[doc = "GFC register accessor: an alias for `Reg<GFC_SPEC>`"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod gfc;
#[doc = "SIDFC register accessor: an alias for `Reg<SIDFC_SPEC>`"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "XIDFC register accessor: an alias for `Reg<XIDFC_SPEC>`"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "HPMS register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "FDCAN High Priority Message Status Register"]
pub mod hpms;
#[doc = "NDAT1 register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "FDCAN New Data 1 Register"]
pub mod ndat1;
#[doc = "NDAT2 register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "FDCAN New Data 2 Register"]
pub mod ndat2;
#[doc = "RXF0C register accessor: an alias for `Reg<RXF0C_SPEC>`"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXBC register accessor: an alias for `Reg<RXBC_SPEC>`"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "RXF1C register accessor: an alias for `Reg<RXF1C_SPEC>`"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "RXESC register accessor: an alias for `Reg<RXESC_SPEC>`"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod rxesc;
#[doc = "TXBC register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "TXESC register accessor: an alias for `Reg<TXESC_SPEC>`"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod txesc;
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
#[doc = "TXEFC register accessor: an alias for `Reg<TXEFC_SPEC>`"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "TTTMC register accessor: an alias for `Reg<TTTMC_SPEC>`"]
pub type TTTMC = crate::Reg<tttmc::TTTMC_SPEC>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod tttmc;
#[doc = "TTRMC register accessor: an alias for `Reg<TTRMC_SPEC>`"]
pub type TTRMC = crate::Reg<ttrmc::TTRMC_SPEC>;
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod ttrmc;
#[doc = "TTOCF register accessor: an alias for `Reg<TTOCF_SPEC>`"]
pub type TTOCF = crate::Reg<ttocf::TTOCF_SPEC>;
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod ttocf;
#[doc = "TTMLM register accessor: an alias for `Reg<TTMLM_SPEC>`"]
pub type TTMLM = crate::Reg<ttmlm::TTMLM_SPEC>;
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod ttmlm;
#[doc = "TURCF register accessor: an alias for `Reg<TURCF_SPEC>`"]
pub type TURCF = crate::Reg<turcf::TURCF_SPEC>;
#[doc = "FDCAN TUR Configuration Register"]
pub mod turcf;
#[doc = "TTOCN register accessor: an alias for `Reg<TTOCN_SPEC>`"]
pub type TTOCN = crate::Reg<ttocn::TTOCN_SPEC>;
#[doc = "FDCAN TT Operation Control Register"]
pub mod ttocn;
#[doc = "TTGTP register accessor: an alias for `Reg<TTGTP_SPEC>`"]
pub type TTGTP = crate::Reg<ttgtp::TTGTP_SPEC>;
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod ttgtp;
#[doc = "TTTMK register accessor: an alias for `Reg<TTTMK_SPEC>`"]
pub type TTTMK = crate::Reg<tttmk::TTTMK_SPEC>;
#[doc = "FDCAN TT Time Mark Register"]
pub mod tttmk;
#[doc = "TTIR register accessor: an alias for `Reg<TTIR_SPEC>`"]
pub type TTIR = crate::Reg<ttir::TTIR_SPEC>;
#[doc = "FDCAN TT Interrupt Register"]
pub mod ttir;
#[doc = "TTIE register accessor: an alias for `Reg<TTIE_SPEC>`"]
pub type TTIE = crate::Reg<ttie::TTIE_SPEC>;
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod ttie;
#[doc = "TTILS register accessor: an alias for `Reg<TTILS_SPEC>`"]
pub type TTILS = crate::Reg<ttils::TTILS_SPEC>;
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod ttils;
#[doc = "TTOST register accessor: an alias for `Reg<TTOST_SPEC>`"]
pub type TTOST = crate::Reg<ttost::TTOST_SPEC>;
#[doc = "FDCAN TT Operation Status Register"]
pub mod ttost;
#[doc = "TURNA register accessor: an alias for `Reg<TURNA_SPEC>`"]
pub type TURNA = crate::Reg<turna::TURNA_SPEC>;
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod turna;
#[doc = "TTLGT register accessor: an alias for `Reg<TTLGT_SPEC>`"]
pub type TTLGT = crate::Reg<ttlgt::TTLGT_SPEC>;
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod ttlgt;
#[doc = "TTCTC register accessor: an alias for `Reg<TTCTC_SPEC>`"]
pub type TTCTC = crate::Reg<ttctc::TTCTC_SPEC>;
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod ttctc;
#[doc = "TTCPT register accessor: an alias for `Reg<TTCPT_SPEC>`"]
pub type TTCPT = crate::Reg<ttcpt::TTCPT_SPEC>;
#[doc = "FDCAN TT Capture Time Register"]
pub mod ttcpt;
#[doc = "TTCSM register accessor: an alias for `Reg<TTCSM_SPEC>`"]
pub type TTCSM = crate::Reg<ttcsm::TTCSM_SPEC>;
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod ttcsm;
#[doc = "TTTS register accessor: an alias for `Reg<TTTS_SPEC>`"]
pub type TTTS = crate::Reg<ttts::TTTS_SPEC>;
#[doc = "FDCAN TT Trigger Select Register"]
pub mod ttts;
