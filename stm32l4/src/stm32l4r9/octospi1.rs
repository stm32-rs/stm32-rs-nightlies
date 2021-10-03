#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - device configuration register"]
    pub dcr1: crate::Reg<dcr1::DCR1_SPEC>,
    #[doc = "0x0c - device configuration register 2"]
    pub dcr2: crate::Reg<dcr2::DCR2_SPEC>,
    #[doc = "0x10 - device configuration register 3"]
    pub dcr3: crate::Reg<dcr3::DCR3_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x24 - flag clear register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - data length register"]
    pub dlr: crate::Reg<dlr::DLR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x48 - address register"]
    pub ar: crate::Reg<ar::AR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x50 - data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved9: [u8; 0x2c],
    #[doc = "0x80 - polling status mask register"]
    pub psmkr: crate::Reg<psmkr::PSMKR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x88 - polling status match register"]
    pub psmar: crate::Reg<psmar::PSMAR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x90 - polling interval register"]
    pub pir: crate::Reg<pir::PIR_SPEC>,
    _reserved12: [u8; 0x6c],
    #[doc = "0x100 - communication configuration register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x108 - timing configuration register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x110 - instruction register"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x120 - alternate bytes register"]
    pub abr: crate::Reg<abr::ABR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x130 - low-power timeout register"]
    pub lptr: crate::Reg<lptr::LPTR_SPEC>,
    _reserved17: [u8; 0x4c],
    #[doc = "0x180 - write communication configuration register"]
    pub wccr: crate::Reg<wccr::WCCR_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x188 - write timing configuration register"]
    pub wtcr: crate::Reg<wtcr::WTCR_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x190 - write instruction register"]
    pub wir: crate::Reg<wir::WIR_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x1a0 - write alternate bytes register"]
    pub wabr: crate::Reg<wabr::WABR_SPEC>,
    _reserved21: [u8; 0x5c],
    #[doc = "0x200 - HyperBusTM latency configuration register"]
    pub hlcr: crate::Reg<hlcr::HLCR_SPEC>,
    _reserved22: [u8; 0x01ec],
    #[doc = "0x3f0 - HW configuration register"]
    pub hwcfgr: crate::Reg<hwcfgr::HWCFGR_SPEC>,
    #[doc = "0x3f4 - version register"]
    pub ver: crate::Reg<ver::VER_SPEC>,
    #[doc = "0x3f8 - identification"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x3fc - magic ID"]
    pub mid: crate::Reg<mid::MID_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DCR1 register accessor: an alias for `Reg<DCR1_SPEC>`"]
pub type DCR1 = crate::Reg<dcr1::DCR1_SPEC>;
#[doc = "device configuration register"]
pub mod dcr1;
#[doc = "DCR2 register accessor: an alias for `Reg<DCR2_SPEC>`"]
pub type DCR2 = crate::Reg<dcr2::DCR2_SPEC>;
#[doc = "device configuration register 2"]
pub mod dcr2;
#[doc = "DCR3 register accessor: an alias for `Reg<DCR3_SPEC>`"]
pub type DCR3 = crate::Reg<dcr3::DCR3_SPEC>;
#[doc = "device configuration register 3"]
pub mod dcr3;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "DLR register accessor: an alias for `Reg<DLR_SPEC>`"]
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
#[doc = "data length register"]
pub mod dlr;
#[doc = "AR register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "address register"]
pub mod ar;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "PSMKR register accessor: an alias for `Reg<PSMKR_SPEC>`"]
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR register accessor: an alias for `Reg<PSMAR_SPEC>`"]
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "PIR register accessor: an alias for `Reg<PIR_SPEC>`"]
pub type PIR = crate::Reg<pir::PIR_SPEC>;
#[doc = "polling interval register"]
pub mod pir;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "timing configuration register"]
pub mod tcr;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "instruction register"]
pub mod ir;
#[doc = "ABR register accessor: an alias for `Reg<ABR_SPEC>`"]
pub type ABR = crate::Reg<abr::ABR_SPEC>;
#[doc = "alternate bytes register"]
pub mod abr;
#[doc = "LPTR register accessor: an alias for `Reg<LPTR_SPEC>`"]
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
#[doc = "low-power timeout register"]
pub mod lptr;
#[doc = "WCCR register accessor: an alias for `Reg<WCCR_SPEC>`"]
pub type WCCR = crate::Reg<wccr::WCCR_SPEC>;
#[doc = "write communication configuration register"]
pub mod wccr;
#[doc = "WTCR register accessor: an alias for `Reg<WTCR_SPEC>`"]
pub type WTCR = crate::Reg<wtcr::WTCR_SPEC>;
#[doc = "write timing configuration register"]
pub mod wtcr;
#[doc = "WIR register accessor: an alias for `Reg<WIR_SPEC>`"]
pub type WIR = crate::Reg<wir::WIR_SPEC>;
#[doc = "write instruction register"]
pub mod wir;
#[doc = "WABR register accessor: an alias for `Reg<WABR_SPEC>`"]
pub type WABR = crate::Reg<wabr::WABR_SPEC>;
#[doc = "write alternate bytes register"]
pub mod wabr;
#[doc = "HLCR register accessor: an alias for `Reg<HLCR_SPEC>`"]
pub type HLCR = crate::Reg<hlcr::HLCR_SPEC>;
#[doc = "HyperBusTM latency configuration register"]
pub mod hlcr;
#[doc = "HWCFGR register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "HW configuration register"]
pub mod hwcfgr;
#[doc = "VER register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "version register"]
pub mod ver;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "identification"]
pub mod id;
#[doc = "MID register accessor: an alias for `Reg<MID_SPEC>`"]
pub type MID = crate::Reg<mid::MID_SPEC>;
#[doc = "magic ID"]
pub mod mid;