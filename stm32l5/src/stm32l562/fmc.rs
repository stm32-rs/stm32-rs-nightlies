#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_BCR1"]
    pub fmc_bcr1: FMC_BCR1,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr1: FMC_BTR1,
    #[doc = "0x08 - FMC_BCR2"]
    pub fmc_bcr2: FMC_BCR2,
    #[doc = "0x0c - FMC_BTR2"]
    pub fmc_btr2: FMC_BTR2,
    #[doc = "0x10 - >FMC_BCR3"]
    pub fmc_bcr3: FMC_BCR3,
    #[doc = "0x14 - FMC_BTR3"]
    pub fmc_btr3: FMC_BTR3,
    #[doc = "0x18 - >FMC_BCR4"]
    pub fmc_bcr4: FMC_BCR4,
    #[doc = "0x1c - FMC_BTR4"]
    pub fmc_btr4: FMC_BTR4,
    #[doc = "0x20 - PCSCNTR"]
    pub pcscntr: PCSCNTR,
    _reserved9: [u8; 92usize],
    #[doc = "0x80 - NAND Flash control registers"]
    pub fmc_pcr: FMC_PCR,
    #[doc = "0x84 - This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
    pub fmc_sr: FMC_SR,
    #[doc = "0x88 - The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
    pub fmc_pmem: FMC_PMEM,
    #[doc = "0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
    pub fmc_patt: FMC_PATT,
    _reserved13: [u8; 4usize],
    #[doc = "0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_eccr: FMC_ECCR,
    _reserved14: [u8; 108usize],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr1: FMC_BWTR1,
    _reserved15: [u8; 4usize],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr2: FMC_BWTR2,
    _reserved16: [u8; 4usize],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr3: FMC_BWTR3,
    _reserved17: [u8; 4usize],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr4: FMC_BWTR4,
}
#[doc = "FMC_BCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr1](fmc_bcr1) module"]
pub type FMC_BCR1 = crate::Reg<u32, _FMC_BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR1;
#[doc = "`read()` method returns [fmc_bcr1::R](fmc_bcr1::R) reader structure"]
impl crate::Readable for FMC_BCR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr1::W](fmc_bcr1::W) writer structure"]
impl crate::Writable for FMC_BCR1 {}
#[doc = "FMC_BCR1"]
pub mod fmc_bcr1;
#[doc = "FMC_BCR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr2](fmc_bcr2) module"]
pub type FMC_BCR2 = crate::Reg<u32, _FMC_BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR2;
#[doc = "`read()` method returns [fmc_bcr2::R](fmc_bcr2::R) reader structure"]
impl crate::Readable for FMC_BCR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr2::W](fmc_bcr2::W) writer structure"]
impl crate::Writable for FMC_BCR2 {}
#[doc = "FMC_BCR2"]
pub mod fmc_bcr2;
#[doc = ">FMC_BCR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr3](fmc_bcr3) module"]
pub type FMC_BCR3 = crate::Reg<u32, _FMC_BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR3;
#[doc = "`read()` method returns [fmc_bcr3::R](fmc_bcr3::R) reader structure"]
impl crate::Readable for FMC_BCR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr3::W](fmc_bcr3::W) writer structure"]
impl crate::Writable for FMC_BCR3 {}
#[doc = ">FMC_BCR3"]
pub mod fmc_bcr3;
#[doc = ">FMC_BCR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr4](fmc_bcr4) module"]
pub type FMC_BCR4 = crate::Reg<u32, _FMC_BCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR4;
#[doc = "`read()` method returns [fmc_bcr4::R](fmc_bcr4::R) reader structure"]
impl crate::Readable for FMC_BCR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr4::W](fmc_bcr4::W) writer structure"]
impl crate::Writable for FMC_BCR4 {}
#[doc = ">FMC_BCR4"]
pub mod fmc_bcr4;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr1](fmc_btr1) module"]
pub type FMC_BTR1 = crate::Reg<u32, _FMC_BTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR1;
#[doc = "`read()` method returns [fmc_btr1::R](fmc_btr1::R) reader structure"]
impl crate::Readable for FMC_BTR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr1::W](fmc_btr1::W) writer structure"]
impl crate::Writable for FMC_BTR1 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr1;
#[doc = "FMC_BTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr2](fmc_btr2) module"]
pub type FMC_BTR2 = crate::Reg<u32, _FMC_BTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR2;
#[doc = "`read()` method returns [fmc_btr2::R](fmc_btr2::R) reader structure"]
impl crate::Readable for FMC_BTR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr2::W](fmc_btr2::W) writer structure"]
impl crate::Writable for FMC_BTR2 {}
#[doc = "FMC_BTR2"]
pub mod fmc_btr2;
#[doc = "FMC_BTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr3](fmc_btr3) module"]
pub type FMC_BTR3 = crate::Reg<u32, _FMC_BTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR3;
#[doc = "`read()` method returns [fmc_btr3::R](fmc_btr3::R) reader structure"]
impl crate::Readable for FMC_BTR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr3::W](fmc_btr3::W) writer structure"]
impl crate::Writable for FMC_BTR3 {}
#[doc = "FMC_BTR3"]
pub mod fmc_btr3;
#[doc = "FMC_BTR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr4](fmc_btr4) module"]
pub type FMC_BTR4 = crate::Reg<u32, _FMC_BTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR4;
#[doc = "`read()` method returns [fmc_btr4::R](fmc_btr4::R) reader structure"]
impl crate::Readable for FMC_BTR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr4::W](fmc_btr4::W) writer structure"]
impl crate::Writable for FMC_BTR4 {}
#[doc = "FMC_BTR4"]
pub mod fmc_btr4;
#[doc = "NAND Flash control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](fmc_pcr) module"]
pub type FMC_PCR = crate::Reg<u32, _FMC_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PCR;
#[doc = "`read()` method returns [fmc_pcr::R](fmc_pcr::R) reader structure"]
impl crate::Readable for FMC_PCR {}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](fmc_pcr::W) writer structure"]
impl crate::Writable for FMC_PCR {}
#[doc = "NAND Flash control registers"]
pub mod fmc_pcr;
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sr](fmc_sr) module"]
pub type FMC_SR = crate::Reg<u32, _FMC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SR;
#[doc = "`read()` method returns [fmc_sr::R](fmc_sr::R) reader structure"]
impl crate::Readable for FMC_SR {}
#[doc = "`write(|w| ..)` method takes [fmc_sr::W](fmc_sr::W) writer structure"]
impl crate::Writable for FMC_SR {}
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
pub mod fmc_sr;
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pmem](fmc_pmem) module"]
pub type FMC_PMEM = crate::Reg<u32, _FMC_PMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PMEM;
#[doc = "`read()` method returns [fmc_pmem::R](fmc_pmem::R) reader structure"]
impl crate::Readable for FMC_PMEM {}
#[doc = "`write(|w| ..)` method takes [fmc_pmem::W](fmc_pmem::W) writer structure"]
impl crate::Writable for FMC_PMEM {}
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
pub mod fmc_pmem;
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_patt](fmc_patt) module"]
pub type FMC_PATT = crate::Reg<u32, _FMC_PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PATT;
#[doc = "`read()` method returns [fmc_patt::R](fmc_patt::R) reader structure"]
impl crate::Readable for FMC_PATT {}
#[doc = "`write(|w| ..)` method takes [fmc_patt::W](fmc_patt::W) writer structure"]
impl crate::Writable for FMC_PATT {}
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
pub mod fmc_patt;
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_eccr](fmc_eccr) module"]
pub type FMC_ECCR = crate::Reg<u32, _FMC_ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_ECCR;
#[doc = "`read()` method returns [fmc_eccr::R](fmc_eccr::R) reader structure"]
impl crate::Readable for FMC_ECCR {}
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_eccr;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr1](fmc_bwtr1) module"]
pub type FMC_BWTR1 = crate::Reg<u32, _FMC_BWTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR1;
#[doc = "`read()` method returns [fmc_bwtr1::R](fmc_bwtr1::R) reader structure"]
impl crate::Readable for FMC_BWTR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr1::W](fmc_bwtr1::W) writer structure"]
impl crate::Writable for FMC_BWTR1 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr1;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr2](fmc_bwtr2) module"]
pub type FMC_BWTR2 = crate::Reg<u32, _FMC_BWTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR2;
#[doc = "`read()` method returns [fmc_bwtr2::R](fmc_bwtr2::R) reader structure"]
impl crate::Readable for FMC_BWTR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr2::W](fmc_bwtr2::W) writer structure"]
impl crate::Writable for FMC_BWTR2 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr2;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr3](fmc_bwtr3) module"]
pub type FMC_BWTR3 = crate::Reg<u32, _FMC_BWTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR3;
#[doc = "`read()` method returns [fmc_bwtr3::R](fmc_bwtr3::R) reader structure"]
impl crate::Readable for FMC_BWTR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr3::W](fmc_bwtr3::W) writer structure"]
impl crate::Writable for FMC_BWTR3 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr3;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr4](fmc_bwtr4) module"]
pub type FMC_BWTR4 = crate::Reg<u32, _FMC_BWTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR4;
#[doc = "`read()` method returns [fmc_bwtr4::R](fmc_bwtr4::R) reader structure"]
impl crate::Readable for FMC_BWTR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr4::W](fmc_bwtr4::W) writer structure"]
impl crate::Writable for FMC_BWTR4 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr4;
#[doc = "PCSCNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcscntr](pcscntr) module"]
pub type PCSCNTR = crate::Reg<u32, _PCSCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSCNTR;
#[doc = "`read()` method returns [pcscntr::R](pcscntr::R) reader structure"]
impl crate::Readable for PCSCNTR {}
#[doc = "`write(|w| ..)` method takes [pcscntr::W](pcscntr::W) writer structure"]
impl crate::Writable for PCSCNTR {}
#[doc = "PCSCNTR"]
pub mod pcscntr;
