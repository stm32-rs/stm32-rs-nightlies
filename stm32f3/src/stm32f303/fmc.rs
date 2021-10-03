#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: crate::Reg<btr1::BTR1_SPEC>,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: crate::Reg<bcr2::BCR2_SPEC>,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: crate::Reg<btr2::BTR2_SPEC>,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: crate::Reg<bcr3::BCR3_SPEC>,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: crate::Reg<btr3::BTR3_SPEC>,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: crate::Reg<bcr4::BCR4_SPEC>,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: crate::Reg<btr4::BTR4_SPEC>,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: crate::Reg<pcr2::PCR2_SPEC>,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: crate::Reg<sr2::SR2_SPEC>,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: crate::Reg<pmem2::PMEM2_SPEC>,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: crate::Reg<patt2::PATT2_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: crate::Reg<eccr2::ECCR2_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub pcr3: crate::Reg<pcr3::PCR3_SPEC>,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub sr3: crate::Reg<sr3::SR3_SPEC>,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: crate::Reg<pmem3::PMEM3_SPEC>,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: crate::Reg<patt3::PATT3_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: crate::Reg<eccr3::ECCR3_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    pub pcr4: crate::Reg<pcr4::PCR4_SPEC>,
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    pub sr4: crate::Reg<sr4::SR4_SPEC>,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: crate::Reg<pmem4::PMEM4_SPEC>,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: crate::Reg<patt4::PATT4_SPEC>,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: crate::Reg<pio4::PIO4_SPEC>,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: crate::Reg<bwtr1::BWTR1_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: crate::Reg<bwtr2::BWTR2_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: crate::Reg<bwtr3::BWTR3_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: crate::Reg<bwtr4::BWTR4_SPEC>,
}
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR1 register accessor: an alias for `Reg<BTR1_SPEC>`"]
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "BCR2 register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "BTR2 register accessor: an alias for `Reg<BTR2_SPEC>`"]
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "BCR3 register accessor: an alias for `Reg<BCR3_SPEC>`"]
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "BTR3 register accessor: an alias for `Reg<BTR3_SPEC>`"]
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "BCR4 register accessor: an alias for `Reg<BCR4_SPEC>`"]
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "BTR4 register accessor: an alias for `Reg<BTR4_SPEC>`"]
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PCR2 register accessor: an alias for `Reg<PCR2_SPEC>`"]
pub type PCR2 = crate::Reg<pcr2::PCR2_SPEC>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr2;
#[doc = "SR2 register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "FIFO status and interrupt register 2"]
pub mod sr2;
#[doc = "PMEM2 register accessor: an alias for `Reg<PMEM2_SPEC>`"]
pub type PMEM2 = crate::Reg<pmem2::PMEM2_SPEC>;
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "PATT2 register accessor: an alias for `Reg<PATT2_SPEC>`"]
pub type PATT2 = crate::Reg<patt2::PATT2_SPEC>;
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECCR2 register accessor: an alias for `Reg<ECCR2_SPEC>`"]
pub type ECCR2 = crate::Reg<eccr2::ECCR2_SPEC>;
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PCR3 register accessor: an alias for `Reg<PCR3_SPEC>`"]
pub type PCR3 = crate::Reg<pcr3::PCR3_SPEC>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr3;
#[doc = "SR3 register accessor: an alias for `Reg<SR3_SPEC>`"]
pub type SR3 = crate::Reg<sr3::SR3_SPEC>;
#[doc = "FIFO status and interrupt register 3"]
pub mod sr3;
#[doc = "PMEM3 register accessor: an alias for `Reg<PMEM3_SPEC>`"]
pub type PMEM3 = crate::Reg<pmem3::PMEM3_SPEC>;
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "PATT3 register accessor: an alias for `Reg<PATT3_SPEC>`"]
pub type PATT3 = crate::Reg<patt3::PATT3_SPEC>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECCR3 register accessor: an alias for `Reg<ECCR3_SPEC>`"]
pub type ECCR3 = crate::Reg<eccr3::ECCR3_SPEC>;
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PCR4 register accessor: an alias for `Reg<PCR4_SPEC>`"]
pub type PCR4 = crate::Reg<pcr4::PCR4_SPEC>;
#[doc = "PC Card/NAND Flash control register 4"]
pub mod pcr4;
#[doc = "SR4 register accessor: an alias for `Reg<SR4_SPEC>`"]
pub type SR4 = crate::Reg<sr4::SR4_SPEC>;
#[doc = "FIFO status and interrupt register 4"]
pub mod sr4;
#[doc = "PMEM4 register accessor: an alias for `Reg<PMEM4_SPEC>`"]
pub type PMEM4 = crate::Reg<pmem4::PMEM4_SPEC>;
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "PATT4 register accessor: an alias for `Reg<PATT4_SPEC>`"]
pub type PATT4 = crate::Reg<patt4::PATT4_SPEC>;
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "PIO4 register accessor: an alias for `Reg<PIO4_SPEC>`"]
pub type PIO4 = crate::Reg<pio4::PIO4_SPEC>;
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "BWTR1 register accessor: an alias for `Reg<BWTR1_SPEC>`"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "BWTR2 register accessor: an alias for `Reg<BWTR2_SPEC>`"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "BWTR3 register accessor: an alias for `Reg<BWTR3_SPEC>`"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "BWTR4 register accessor: an alias for `Reg<BWTR4_SPEC>`"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
