#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: BTR4,
    _reserved8: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: PCR2,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: SR2,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: PMEM2,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: PATT2,
    _reserved12: [u8; 4usize],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: ECCR2,
    _reserved13: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub pcr3: PCR3,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub sr3: SR3,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: PMEM3,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: PATT3,
    _reserved17: [u8; 4usize],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: ECCR3,
    _reserved18: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    pub pcr4: PCR4,
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    pub sr4: SR4,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: PMEM4,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: PATT4,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved23: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR1,
    _reserved24: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: BWTR2,
    _reserved25: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: BWTR3,
    _reserved26: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: BWTR4,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr1](bcr1) module"]
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
#[doc = "`read()` method returns [bcr1::R](bcr1::R) reader structure"]
impl crate::Readable for BCR1 {}
#[doc = "`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure"]
impl crate::Writable for BCR1 {}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr1](btr1) module"]
pub type BTR1 = crate::Reg<u32, _BTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR1;
#[doc = "`read()` method returns [btr1::R](btr1::R) reader structure"]
impl crate::Readable for BTR1 {}
#[doc = "`write(|w| ..)` method takes [btr1::W](btr1::W) writer structure"]
impl crate::Writable for BTR1 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr2](bcr2) module"]
pub type BCR2 = crate::Reg<u32, _BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR2;
#[doc = "`read()` method returns [bcr2::R](bcr2::R) reader structure"]
impl crate::Readable for BCR2 {}
#[doc = "`write(|w| ..)` method takes [bcr2::W](bcr2::W) writer structure"]
impl crate::Writable for BCR2 {}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr2](btr2) module"]
pub type BTR2 = crate::Reg<u32, _BTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR2;
#[doc = "`read()` method returns [btr2::R](btr2::R) reader structure"]
impl crate::Readable for BTR2 {}
#[doc = "`write(|w| ..)` method takes [btr2::W](btr2::W) writer structure"]
impl crate::Writable for BTR2 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr3](bcr3) module"]
pub type BCR3 = crate::Reg<u32, _BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR3;
#[doc = "`read()` method returns [bcr3::R](bcr3::R) reader structure"]
impl crate::Readable for BCR3 {}
#[doc = "`write(|w| ..)` method takes [bcr3::W](bcr3::W) writer structure"]
impl crate::Writable for BCR3 {}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr3](btr3) module"]
pub type BTR3 = crate::Reg<u32, _BTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR3;
#[doc = "`read()` method returns [btr3::R](btr3::R) reader structure"]
impl crate::Readable for BTR3 {}
#[doc = "`write(|w| ..)` method takes [btr3::W](btr3::W) writer structure"]
impl crate::Writable for BTR3 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr4](bcr4) module"]
pub type BCR4 = crate::Reg<u32, _BCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR4;
#[doc = "`read()` method returns [bcr4::R](bcr4::R) reader structure"]
impl crate::Readable for BCR4 {}
#[doc = "`write(|w| ..)` method takes [bcr4::W](bcr4::W) writer structure"]
impl crate::Writable for BCR4 {}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr4](btr4) module"]
pub type BTR4 = crate::Reg<u32, _BTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR4;
#[doc = "`read()` method returns [btr4::R](btr4::R) reader structure"]
impl crate::Readable for BTR4 {}
#[doc = "`write(|w| ..)` method takes [btr4::W](btr4::W) writer structure"]
impl crate::Writable for BTR4 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PC Card/NAND Flash control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr2](pcr2) module"]
pub type PCR2 = crate::Reg<u32, _PCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR2;
#[doc = "`read()` method returns [pcr2::R](pcr2::R) reader structure"]
impl crate::Readable for PCR2 {}
#[doc = "`write(|w| ..)` method takes [pcr2::W](pcr2::W) writer structure"]
impl crate::Writable for PCR2 {}
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr2;
#[doc = "FIFO status and interrupt register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](sr2) module"]
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
#[doc = "`read()` method returns [sr2::R](sr2::R) reader structure"]
impl crate::Readable for SR2 {}
#[doc = "`write(|w| ..)` method takes [sr2::W](sr2::W) writer structure"]
impl crate::Writable for SR2 {}
#[doc = "FIFO status and interrupt register 2"]
pub mod sr2;
#[doc = "Common memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem2](pmem2) module"]
pub type PMEM2 = crate::Reg<u32, _PMEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM2;
#[doc = "`read()` method returns [pmem2::R](pmem2::R) reader structure"]
impl crate::Readable for PMEM2 {}
#[doc = "`write(|w| ..)` method takes [pmem2::W](pmem2::W) writer structure"]
impl crate::Writable for PMEM2 {}
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "Attribute memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt2](patt2) module"]
pub type PATT2 = crate::Reg<u32, _PATT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT2;
#[doc = "`read()` method returns [patt2::R](patt2::R) reader structure"]
impl crate::Readable for PATT2 {}
#[doc = "`write(|w| ..)` method takes [patt2::W](patt2::W) writer structure"]
impl crate::Writable for PATT2 {}
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECC result register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr2](eccr2) module"]
pub type ECCR2 = crate::Reg<u32, _ECCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR2;
#[doc = "`read()` method returns [eccr2::R](eccr2::R) reader structure"]
impl crate::Readable for ECCR2 {}
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PC Card/NAND Flash control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr3](pcr3) module"]
pub type PCR3 = crate::Reg<u32, _PCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR3;
#[doc = "`read()` method returns [pcr3::R](pcr3::R) reader structure"]
impl crate::Readable for PCR3 {}
#[doc = "`write(|w| ..)` method takes [pcr3::W](pcr3::W) writer structure"]
impl crate::Writable for PCR3 {}
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr3;
#[doc = "FIFO status and interrupt register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr3](sr3) module"]
pub type SR3 = crate::Reg<u32, _SR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR3;
#[doc = "`read()` method returns [sr3::R](sr3::R) reader structure"]
impl crate::Readable for SR3 {}
#[doc = "`write(|w| ..)` method takes [sr3::W](sr3::W) writer structure"]
impl crate::Writable for SR3 {}
#[doc = "FIFO status and interrupt register 3"]
pub mod sr3;
#[doc = "Common memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem3](pmem3) module"]
pub type PMEM3 = crate::Reg<u32, _PMEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM3;
#[doc = "`read()` method returns [pmem3::R](pmem3::R) reader structure"]
impl crate::Readable for PMEM3 {}
#[doc = "`write(|w| ..)` method takes [pmem3::W](pmem3::W) writer structure"]
impl crate::Writable for PMEM3 {}
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "Attribute memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt3](patt3) module"]
pub type PATT3 = crate::Reg<u32, _PATT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT3;
#[doc = "`read()` method returns [patt3::R](patt3::R) reader structure"]
impl crate::Readable for PATT3 {}
#[doc = "`write(|w| ..)` method takes [patt3::W](patt3::W) writer structure"]
impl crate::Writable for PATT3 {}
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECC result register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr3](eccr3) module"]
pub type ECCR3 = crate::Reg<u32, _ECCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR3;
#[doc = "`read()` method returns [eccr3::R](eccr3::R) reader structure"]
impl crate::Readable for ECCR3 {}
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PC Card/NAND Flash control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr4](pcr4) module"]
pub type PCR4 = crate::Reg<u32, _PCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR4;
#[doc = "`read()` method returns [pcr4::R](pcr4::R) reader structure"]
impl crate::Readable for PCR4 {}
#[doc = "`write(|w| ..)` method takes [pcr4::W](pcr4::W) writer structure"]
impl crate::Writable for PCR4 {}
#[doc = "PC Card/NAND Flash control register 4"]
pub mod pcr4;
#[doc = "FIFO status and interrupt register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr4](sr4) module"]
pub type SR4 = crate::Reg<u32, _SR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR4;
#[doc = "`read()` method returns [sr4::R](sr4::R) reader structure"]
impl crate::Readable for SR4 {}
#[doc = "`write(|w| ..)` method takes [sr4::W](sr4::W) writer structure"]
impl crate::Writable for SR4 {}
#[doc = "FIFO status and interrupt register 4"]
pub mod sr4;
#[doc = "Common memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem4](pmem4) module"]
pub type PMEM4 = crate::Reg<u32, _PMEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM4;
#[doc = "`read()` method returns [pmem4::R](pmem4::R) reader structure"]
impl crate::Readable for PMEM4 {}
#[doc = "`write(|w| ..)` method takes [pmem4::W](pmem4::W) writer structure"]
impl crate::Writable for PMEM4 {}
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "Attribute memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt4](patt4) module"]
pub type PATT4 = crate::Reg<u32, _PATT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT4;
#[doc = "`read()` method returns [patt4::R](patt4::R) reader structure"]
impl crate::Readable for PATT4 {}
#[doc = "`write(|w| ..)` method takes [patt4::W](patt4::W) writer structure"]
impl crate::Writable for PATT4 {}
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "I/O space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio4](pio4) module"]
pub type PIO4 = crate::Reg<u32, _PIO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO4;
#[doc = "`read()` method returns [pio4::R](pio4::R) reader structure"]
impl crate::Readable for PIO4 {}
#[doc = "`write(|w| ..)` method takes [pio4::W](pio4::W) writer structure"]
impl crate::Writable for PIO4 {}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr1](bwtr1) module"]
pub type BWTR1 = crate::Reg<u32, _BWTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR1;
#[doc = "`read()` method returns [bwtr1::R](bwtr1::R) reader structure"]
impl crate::Readable for BWTR1 {}
#[doc = "`write(|w| ..)` method takes [bwtr1::W](bwtr1::W) writer structure"]
impl crate::Writable for BWTR1 {}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr2](bwtr2) module"]
pub type BWTR2 = crate::Reg<u32, _BWTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR2;
#[doc = "`read()` method returns [bwtr2::R](bwtr2::R) reader structure"]
impl crate::Readable for BWTR2 {}
#[doc = "`write(|w| ..)` method takes [bwtr2::W](bwtr2::W) writer structure"]
impl crate::Writable for BWTR2 {}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr3](bwtr3) module"]
pub type BWTR3 = crate::Reg<u32, _BWTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR3;
#[doc = "`read()` method returns [bwtr3::R](bwtr3::R) reader structure"]
impl crate::Readable for BWTR3 {}
#[doc = "`write(|w| ..)` method takes [bwtr3::W](bwtr3::W) writer structure"]
impl crate::Writable for BWTR3 {}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr4](bwtr4) module"]
pub type BWTR4 = crate::Reg<u32, _BWTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR4;
#[doc = "`read()` method returns [bwtr4::R](bwtr4::R) reader structure"]
impl crate::Readable for BWTR4 {}
#[doc = "`write(|w| ..)` method takes [bwtr4::W](bwtr4::W) writer structure"]
impl crate::Writable for BWTR4 {}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
