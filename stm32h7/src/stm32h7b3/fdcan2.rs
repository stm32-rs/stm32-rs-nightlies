#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - FDCAN Core Release Register"]
    pub endn: ENDN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    pub dbtp: DBTP,
    #[doc = "0x10 - FDCAN Test Register"]
    pub test: TEST,
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    pub rwd: RWD,
    #[doc = "0x18 - FDCAN CC Control Register"]
    pub cccr: CCCR,
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    pub nbtp: NBTP,
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    pub tscc: TSCC,
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    pub tscv: TSCV,
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    pub tocc: TOCC,
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    pub tocv: TOCV,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - FDCAN Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    pub psr: PSR,
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    pub tdcr: TDCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - FDCAN Interrupt Register"]
    pub ir: IR,
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    pub ils: ILS,
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    pub ile: ILE,
    _reserved18: [u8; 32usize],
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    pub gfc: GFC,
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    pub xidfc: XIDFC,
    _reserved21: [u8; 4usize],
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    pub xidam: XIDAM,
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    pub hpms: HPMS,
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    pub txbc: TXBC,
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    pub txesc: TXESC,
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    pub txbto: TXBTO,
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: TXBCIE,
    _reserved43: [u8; 8usize],
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    pub txefa: TXEFA,
    _reserved46: [u8; 4usize],
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    pub tttmc: TTTMC,
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    pub ttrmc: TTRMC,
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    pub ttocf: TTOCF,
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    pub ttmlm: TTMLM,
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    pub turcf: TURCF,
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    pub ttocn: TTOCN,
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    pub ttgtp: TTGTP,
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    pub tttmk: TTTMK,
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    pub ttir: TTIR,
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    pub ttie: TTIE,
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    pub ttils: TTILS,
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    pub ttost: TTOST,
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    pub turna: TURNA,
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    pub ttlgt: TTLGT,
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    pub ttctc: TTCTC,
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    pub ttcpt: TTCPT,
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    pub ttcsm: TTCSM,
    _reserved63: [u8; 444usize],
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    pub ttts: TTTS,
}
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](crel) module"]
pub type CREL = crate::Reg<u32, _CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREL;
#[doc = "`read()` method returns [crel::R](crel::R) reader structure"]
impl crate::Readable for CREL {}
#[doc = "FDCAN Core Release Register"]
pub mod crel;
#[doc = "FDCAN Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endn](endn) module"]
pub type ENDN = crate::Reg<u32, _ENDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDN;
#[doc = "`read()` method returns [endn::R](endn::R) reader structure"]
impl crate::Readable for ENDN {}
#[doc = "FDCAN Core Release Register"]
pub mod endn;
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbtp](dbtp) module"]
pub type DBTP = crate::Reg<u32, _DBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBTP;
#[doc = "`read()` method returns [dbtp::R](dbtp::R) reader structure"]
impl crate::Readable for DBTP {}
#[doc = "`write(|w| ..)` method takes [dbtp::W](dbtp::W) writer structure"]
impl crate::Writable for DBTP {}
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "FDCAN Test Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "FDCAN Test Register"]
pub mod test;
#[doc = "FDCAN RAM Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwd](rwd) module"]
pub type RWD = crate::Reg<u32, _RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWD;
#[doc = "`read()` method returns [rwd::R](rwd::R) reader structure"]
impl crate::Readable for RWD {}
#[doc = "FDCAN RAM Watchdog Register"]
pub mod rwd;
#[doc = "FDCAN CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "FDCAN CC Control Register"]
pub mod cccr;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtp](nbtp) module"]
pub type NBTP = crate::Reg<u32, _NBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBTP;
#[doc = "`read()` method returns [nbtp::R](nbtp::R) reader structure"]
impl crate::Readable for NBTP {}
#[doc = "`write(|w| ..)` method takes [nbtp::W](nbtp::W) writer structure"]
impl crate::Writable for NBTP {}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](tscc) module"]
pub type TSCC = crate::Reg<u32, _TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCC;
#[doc = "`read()` method returns [tscc::R](tscc::R) reader structure"]
impl crate::Readable for TSCC {}
#[doc = "`write(|w| ..)` method takes [tscc::W](tscc::W) writer structure"]
impl crate::Writable for TSCC {}
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "FDCAN Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](tscv) module"]
pub type TSCV = crate::Reg<u32, _TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCV;
#[doc = "`read()` method returns [tscv::R](tscv::R) reader structure"]
impl crate::Readable for TSCV {}
#[doc = "`write(|w| ..)` method takes [tscv::W](tscv::W) writer structure"]
impl crate::Writable for TSCV {}
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "FDCAN Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](tocc) module"]
pub type TOCC = crate::Reg<u32, _TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCC;
#[doc = "`read()` method returns [tocc::R](tocc::R) reader structure"]
impl crate::Readable for TOCC {}
#[doc = "`write(|w| ..)` method takes [tocc::W](tocc::W) writer structure"]
impl crate::Writable for TOCC {}
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "FDCAN Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](tocv) module"]
pub type TOCV = crate::Reg<u32, _TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCV;
#[doc = "`read()` method returns [tocv::R](tocv::R) reader structure"]
impl crate::Readable for TOCV {}
#[doc = "`write(|w| ..)` method takes [tocv::W](tocv::W) writer structure"]
impl crate::Writable for TOCV {}
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod tocv;
#[doc = "FDCAN Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "FDCAN Error Counter Register"]
pub mod ecr;
#[doc = "FDCAN Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "`write(|w| ..)` method takes [psr::W](psr::W) writer structure"]
impl crate::Writable for PSR {}
#[doc = "FDCAN Protocol Status Register"]
pub mod psr;
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcr](tdcr) module"]
pub type TDCR = crate::Reg<u32, _TDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCR;
#[doc = "`read()` method returns [tdcr::R](tdcr::R) reader structure"]
impl crate::Readable for TDCR {}
#[doc = "`write(|w| ..)` method takes [tdcr::W](tdcr::W) writer structure"]
impl crate::Writable for TDCR {}
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "FDCAN Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "FDCAN Interrupt Register"]
pub mod ir;
#[doc = "FDCAN Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "FDCAN Interrupt Enable Register"]
pub mod ie;
#[doc = "FDCAN Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](ils) module"]
pub type ILS = crate::Reg<u32, _ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILS;
#[doc = "`read()` method returns [ils::R](ils::R) reader structure"]
impl crate::Readable for ILS {}
#[doc = "`write(|w| ..)` method takes [ils::W](ils::W) writer structure"]
impl crate::Writable for ILS {}
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod ils;
#[doc = "FDCAN Interrupt Line Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](ile) module"]
pub type ILE = crate::Reg<u32, _ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILE;
#[doc = "`read()` method returns [ile::R](ile::R) reader structure"]
impl crate::Readable for ILE {}
#[doc = "`write(|w| ..)` method takes [ile::W](ile::W) writer structure"]
impl crate::Writable for ILE {}
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod ile;
#[doc = "FDCAN Global Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](gfc) module"]
pub type GFC = crate::Reg<u32, _GFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFC;
#[doc = "`read()` method returns [gfc::R](gfc::R) reader structure"]
impl crate::Readable for GFC {}
#[doc = "`write(|w| ..)` method takes [gfc::W](gfc::W) writer structure"]
impl crate::Writable for GFC {}
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod gfc;
#[doc = "FDCAN Standard ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidfc](sidfc) module"]
pub type SIDFC = crate::Reg<u32, _SIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDFC;
#[doc = "`read()` method returns [sidfc::R](sidfc::R) reader structure"]
impl crate::Readable for SIDFC {}
#[doc = "`write(|w| ..)` method takes [sidfc::W](sidfc::W) writer structure"]
impl crate::Writable for SIDFC {}
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "FDCAN Extended ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidfc](xidfc) module"]
pub type XIDFC = crate::Reg<u32, _XIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDFC;
#[doc = "`read()` method returns [xidfc::R](xidfc::R) reader structure"]
impl crate::Readable for XIDFC {}
#[doc = "`write(|w| ..)` method takes [xidfc::W](xidfc::W) writer structure"]
impl crate::Writable for XIDFC {}
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "FDCAN Extended ID and Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidam](xidam) module"]
pub type XIDAM = crate::Reg<u32, _XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDAM;
#[doc = "`read()` method returns [xidam::R](xidam::R) reader structure"]
impl crate::Readable for XIDAM {}
#[doc = "`write(|w| ..)` method takes [xidam::W](xidam::W) writer structure"]
impl crate::Writable for XIDAM {}
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "FDCAN High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](hpms) module"]
pub type HPMS = crate::Reg<u32, _HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPMS;
#[doc = "`read()` method returns [hpms::R](hpms::R) reader structure"]
impl crate::Readable for HPMS {}
#[doc = "FDCAN High Priority Message Status Register"]
pub mod hpms;
#[doc = "FDCAN New Data 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat1](ndat1) module"]
pub type NDAT1 = crate::Reg<u32, _NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT1;
#[doc = "`read()` method returns [ndat1::R](ndat1::R) reader structure"]
impl crate::Readable for NDAT1 {}
#[doc = "`write(|w| ..)` method takes [ndat1::W](ndat1::W) writer structure"]
impl crate::Writable for NDAT1 {}
#[doc = "FDCAN New Data 1 Register"]
pub mod ndat1;
#[doc = "FDCAN New Data 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat2](ndat2) module"]
pub type NDAT2 = crate::Reg<u32, _NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT2;
#[doc = "`read()` method returns [ndat2::R](ndat2::R) reader structure"]
impl crate::Readable for NDAT2 {}
#[doc = "`write(|w| ..)` method takes [ndat2::W](ndat2::W) writer structure"]
impl crate::Writable for NDAT2 {}
#[doc = "FDCAN New Data 2 Register"]
pub mod ndat2;
#[doc = "FDCAN Rx FIFO 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](rxf0c) module"]
pub type RXF0C = crate::Reg<u32, _RXF0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0C;
#[doc = "`read()` method returns [rxf0c::R](rxf0c::R) reader structure"]
impl crate::Readable for RXF0C {}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](rxf0c::W) writer structure"]
impl crate::Writable for RXF0C {}
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](rxf0s) module"]
pub type RXF0S = crate::Reg<u32, _RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0S;
#[doc = "`read()` method returns [rxf0s::R](rxf0s::R) reader structure"]
impl crate::Readable for RXF0S {}
#[doc = "`write(|w| ..)` method takes [rxf0s::W](rxf0s::W) writer structure"]
impl crate::Writable for RXF0S {}
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0a](rxf0a) module"]
pub type RXF0A = crate::Reg<u32, _RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0A;
#[doc = "`read()` method returns [rxf0a::R](rxf0a::R) reader structure"]
impl crate::Readable for RXF0A {}
#[doc = "`write(|w| ..)` method takes [rxf0a::W](rxf0a::W) writer structure"]
impl crate::Writable for RXF0A {}
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "FDCAN Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbc](rxbc) module"]
pub type RXBC = crate::Reg<u32, _RXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBC;
#[doc = "`read()` method returns [rxbc::R](rxbc::R) reader structure"]
impl crate::Readable for RXBC {}
#[doc = "`write(|w| ..)` method takes [rxbc::W](rxbc::W) writer structure"]
impl crate::Writable for RXBC {}
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "FDCAN Rx FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1c](rxf1c) module"]
pub type RXF1C = crate::Reg<u32, _RXF1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1C;
#[doc = "`read()` method returns [rxf1c::R](rxf1c::R) reader structure"]
impl crate::Readable for RXF1C {}
#[doc = "`write(|w| ..)` method takes [rxf1c::W](rxf1c::W) writer structure"]
impl crate::Writable for RXF1C {}
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](rxf1s) module"]
pub type RXF1S = crate::Reg<u32, _RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1S;
#[doc = "`read()` method returns [rxf1s::R](rxf1s::R) reader structure"]
impl crate::Readable for RXF1S {}
#[doc = "`write(|w| ..)` method takes [rxf1s::W](rxf1s::W) writer structure"]
impl crate::Writable for RXF1S {}
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1a](rxf1a) module"]
pub type RXF1A = crate::Reg<u32, _RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1A;
#[doc = "`read()` method returns [rxf1a::R](rxf1a::R) reader structure"]
impl crate::Readable for RXF1A {}
#[doc = "`write(|w| ..)` method takes [rxf1a::W](rxf1a::W) writer structure"]
impl crate::Writable for RXF1A {}
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](rxesc) module"]
pub type RXESC = crate::Reg<u32, _RXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXESC;
#[doc = "`read()` method returns [rxesc::R](rxesc::R) reader structure"]
impl crate::Readable for RXESC {}
#[doc = "`write(|w| ..)` method takes [rxesc::W](rxesc::W) writer structure"]
impl crate::Writable for RXESC {}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod rxesc;
#[doc = "FDCAN Tx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](txbc) module"]
pub type TXBC = crate::Reg<u32, _TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBC;
#[doc = "`read()` method returns [txbc::R](txbc::R) reader structure"]
impl crate::Readable for TXBC {}
#[doc = "`write(|w| ..)` method takes [txbc::W](txbc::W) writer structure"]
impl crate::Writable for TXBC {}
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "FDCAN Tx FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](txfqs) module"]
pub type TXFQS = crate::Reg<u32, _TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFQS;
#[doc = "`read()` method returns [txfqs::R](txfqs::R) reader structure"]
impl crate::Readable for TXFQS {}
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txesc](txesc) module"]
pub type TXESC = crate::Reg<u32, _TXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXESC;
#[doc = "`read()` method returns [txesc::R](txesc::R) reader structure"]
impl crate::Readable for TXESC {}
#[doc = "`write(|w| ..)` method takes [txesc::W](txesc::W) writer structure"]
impl crate::Writable for TXESC {}
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](txbrp) module"]
pub type TXBRP = crate::Reg<u32, _TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBRP;
#[doc = "`read()` method returns [txbrp::R](txbrp::R) reader structure"]
impl crate::Readable for TXBRP {}
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "FDCAN Tx Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](txbar) module"]
pub type TXBAR = crate::Reg<u32, _TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBAR;
#[doc = "`read()` method returns [txbar::R](txbar::R) reader structure"]
impl crate::Readable for TXBAR {}
#[doc = "`write(|w| ..)` method takes [txbar::W](txbar::W) writer structure"]
impl crate::Writable for TXBAR {}
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod txbar;
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](txbcr) module"]
pub type TXBCR = crate::Reg<u32, _TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCR;
#[doc = "`read()` method returns [txbcr::R](txbcr::R) reader structure"]
impl crate::Readable for TXBCR {}
#[doc = "`write(|w| ..)` method takes [txbcr::W](txbcr::W) writer structure"]
impl crate::Writable for TXBCR {}
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](txbto) module"]
pub type TXBTO = crate::Reg<u32, _TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTO;
#[doc = "`read()` method returns [txbto::R](txbto::R) reader structure"]
impl crate::Readable for TXBTO {}
#[doc = "`write(|w| ..)` method takes [txbto::W](txbto::W) writer structure"]
impl crate::Writable for TXBTO {}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](txbcf) module"]
pub type TXBCF = crate::Reg<u32, _TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCF;
#[doc = "`read()` method returns [txbcf::R](txbcf::R) reader structure"]
impl crate::Readable for TXBCF {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](txbtie) module"]
pub type TXBTIE = crate::Reg<u32, _TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTIE;
#[doc = "`read()` method returns [txbtie::R](txbtie::R) reader structure"]
impl crate::Readable for TXBTIE {}
#[doc = "`write(|w| ..)` method takes [txbtie::W](txbtie::W) writer structure"]
impl crate::Writable for TXBTIE {}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](txbcie) module"]
pub type TXBCIE = crate::Reg<u32, _TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCIE;
#[doc = "`read()` method returns [txbcie::R](txbcie::R) reader structure"]
impl crate::Readable for TXBCIE {}
#[doc = "`write(|w| ..)` method takes [txbcie::W](txbcie::W) writer structure"]
impl crate::Writable for TXBCIE {}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "FDCAN Tx Event FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefc](txefc) module"]
pub type TXEFC = crate::Reg<u32, _TXEFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFC;
#[doc = "`read()` method returns [txefc::R](txefc::R) reader structure"]
impl crate::Readable for TXEFC {}
#[doc = "`write(|w| ..)` method takes [txefc::W](txefc::W) writer structure"]
impl crate::Writable for TXEFC {}
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "FDCAN Tx Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](txefs) module"]
pub type TXEFS = crate::Reg<u32, _TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFS;
#[doc = "`read()` method returns [txefs::R](txefs::R) reader structure"]
impl crate::Readable for TXEFS {}
#[doc = "`write(|w| ..)` method takes [txefs::W](txefs::W) writer structure"]
impl crate::Writable for TXEFS {}
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefa](txefa) module"]
pub type TXEFA = crate::Reg<u32, _TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFA;
#[doc = "`read()` method returns [txefa::R](txefa::R) reader structure"]
impl crate::Readable for TXEFA {}
#[doc = "`write(|w| ..)` method takes [txefa::W](txefa::W) writer structure"]
impl crate::Writable for TXEFA {}
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttmc](tttmc) module"]
pub type TTTMC = crate::Reg<u32, _TTTMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTTMC;
#[doc = "`read()` method returns [tttmc::R](tttmc::R) reader structure"]
impl crate::Readable for TTTMC {}
#[doc = "`write(|w| ..)` method takes [tttmc::W](tttmc::W) writer structure"]
impl crate::Writable for TTTMC {}
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod tttmc;
#[doc = "FDCAN TT Reference Message Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttrmc](ttrmc) module"]
pub type TTRMC = crate::Reg<u32, _TTRMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTRMC;
#[doc = "`read()` method returns [ttrmc::R](ttrmc::R) reader structure"]
impl crate::Readable for TTRMC {}
#[doc = "`write(|w| ..)` method takes [ttrmc::W](ttrmc::W) writer structure"]
impl crate::Writable for TTRMC {}
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod ttrmc;
#[doc = "FDCAN TT Operation Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocf](ttocf) module"]
pub type TTOCF = crate::Reg<u32, _TTOCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTOCF;
#[doc = "`read()` method returns [ttocf::R](ttocf::R) reader structure"]
impl crate::Readable for TTOCF {}
#[doc = "`write(|w| ..)` method takes [ttocf::W](ttocf::W) writer structure"]
impl crate::Writable for TTOCF {}
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod ttocf;
#[doc = "FDCAN TT Matrix Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttmlm](ttmlm) module"]
pub type TTMLM = crate::Reg<u32, _TTMLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTMLM;
#[doc = "`read()` method returns [ttmlm::R](ttmlm::R) reader structure"]
impl crate::Readable for TTMLM {}
#[doc = "`write(|w| ..)` method takes [ttmlm::W](ttmlm::W) writer structure"]
impl crate::Writable for TTMLM {}
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod ttmlm;
#[doc = "FDCAN TUR Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [turcf](turcf) module"]
pub type TURCF = crate::Reg<u32, _TURCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TURCF;
#[doc = "`read()` method returns [turcf::R](turcf::R) reader structure"]
impl crate::Readable for TURCF {}
#[doc = "`write(|w| ..)` method takes [turcf::W](turcf::W) writer structure"]
impl crate::Writable for TURCF {}
#[doc = "FDCAN TUR Configuration Register"]
pub mod turcf;
#[doc = "FDCAN TT Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocn](ttocn) module"]
pub type TTOCN = crate::Reg<u32, _TTOCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTOCN;
#[doc = "`read()` method returns [ttocn::R](ttocn::R) reader structure"]
impl crate::Readable for TTOCN {}
#[doc = "`write(|w| ..)` method takes [ttocn::W](ttocn::W) writer structure"]
impl crate::Writable for TTOCN {}
#[doc = "FDCAN TT Operation Control Register"]
pub mod ttocn;
#[doc = "FDCAN TT Global Time Preset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgtp](ttgtp) module"]
pub type TTGTP = crate::Reg<u32, _TTGTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTGTP;
#[doc = "`read()` method returns [ttgtp::R](ttgtp::R) reader structure"]
impl crate::Readable for TTGTP {}
#[doc = "`write(|w| ..)` method takes [ttgtp::W](ttgtp::W) writer structure"]
impl crate::Writable for TTGTP {}
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod ttgtp;
#[doc = "FDCAN TT Time Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttmk](tttmk) module"]
pub type TTTMK = crate::Reg<u32, _TTTMK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTTMK;
#[doc = "`read()` method returns [tttmk::R](tttmk::R) reader structure"]
impl crate::Readable for TTTMK {}
#[doc = "`write(|w| ..)` method takes [tttmk::W](tttmk::W) writer structure"]
impl crate::Writable for TTTMK {}
#[doc = "FDCAN TT Time Mark Register"]
pub mod tttmk;
#[doc = "FDCAN TT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttir](ttir) module"]
pub type TTIR = crate::Reg<u32, _TTIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTIR;
#[doc = "`read()` method returns [ttir::R](ttir::R) reader structure"]
impl crate::Readable for TTIR {}
#[doc = "`write(|w| ..)` method takes [ttir::W](ttir::W) writer structure"]
impl crate::Writable for TTIR {}
#[doc = "FDCAN TT Interrupt Register"]
pub mod ttir;
#[doc = "FDCAN TT Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttie](ttie) module"]
pub type TTIE = crate::Reg<u32, _TTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTIE;
#[doc = "`read()` method returns [ttie::R](ttie::R) reader structure"]
impl crate::Readable for TTIE {}
#[doc = "`write(|w| ..)` method takes [ttie::W](ttie::W) writer structure"]
impl crate::Writable for TTIE {}
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod ttie;
#[doc = "FDCAN TT Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttils](ttils) module"]
pub type TTILS = crate::Reg<u32, _TTILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTILS;
#[doc = "`read()` method returns [ttils::R](ttils::R) reader structure"]
impl crate::Readable for TTILS {}
#[doc = "`write(|w| ..)` method takes [ttils::W](ttils::W) writer structure"]
impl crate::Writable for TTILS {}
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod ttils;
#[doc = "FDCAN TT Operation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttost](ttost) module"]
pub type TTOST = crate::Reg<u32, _TTOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTOST;
#[doc = "`read()` method returns [ttost::R](ttost::R) reader structure"]
impl crate::Readable for TTOST {}
#[doc = "FDCAN TT Operation Status Register"]
pub mod ttost;
#[doc = "FDCAN TUR Numerator Actual Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [turna](turna) module"]
pub type TURNA = crate::Reg<u32, _TURNA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TURNA;
#[doc = "`read()` method returns [turna::R](turna::R) reader structure"]
impl crate::Readable for TURNA {}
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod turna;
#[doc = "FDCAN TT Local and Global Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttlgt](ttlgt) module"]
pub type TTLGT = crate::Reg<u32, _TTLGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTLGT;
#[doc = "`read()` method returns [ttlgt::R](ttlgt::R) reader structure"]
impl crate::Readable for TTLGT {}
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod ttlgt;
#[doc = "FDCAN TT Cycle Time and Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttctc](ttctc) module"]
pub type TTCTC = crate::Reg<u32, _TTCTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTCTC;
#[doc = "`read()` method returns [ttctc::R](ttctc::R) reader structure"]
impl crate::Readable for TTCTC {}
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod ttctc;
#[doc = "FDCAN TT Capture Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcpt](ttcpt) module"]
pub type TTCPT = crate::Reg<u32, _TTCPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTCPT;
#[doc = "`read()` method returns [ttcpt::R](ttcpt::R) reader structure"]
impl crate::Readable for TTCPT {}
#[doc = "FDCAN TT Capture Time Register"]
pub mod ttcpt;
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcsm](ttcsm) module"]
pub type TTCSM = crate::Reg<u32, _TTCSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTCSM;
#[doc = "`read()` method returns [ttcsm::R](ttcsm::R) reader structure"]
impl crate::Readable for TTCSM {}
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod ttcsm;
#[doc = "FDCAN TT Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttts](ttts) module"]
pub type TTTS = crate::Reg<u32, _TTTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTTS;
#[doc = "`read()` method returns [ttts::R](ttts::R) reader structure"]
impl crate::Readable for TTTS {}
#[doc = "`write(|w| ..)` method takes [ttts::W](ttts::W) writer structure"]
impl crate::Writable for TTTS {}
#[doc = "FDCAN TT Trigger Select Register"]
pub mod ttts;
