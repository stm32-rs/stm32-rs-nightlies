#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr1: FMC_BCR1,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr1: FMC_BTR1,
    #[doc = "0x08 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr2: FMC_BCR2,
    #[doc = "0x0c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr2: FMC_BTR2,
    #[doc = "0x10 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr3: FMC_BCR3,
    #[doc = "0x14 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr3: FMC_BTR3,
    #[doc = "0x18 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr4: FMC_BCR4,
    #[doc = "0x1c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr4: FMC_BTR4,
    #[doc = "0x20 - This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
    pub fmc_pcscntr: FMC_PCSCNTR,
    _reserved9: [u8; 92usize],
    #[doc = "0x80 - NAND Flash Programmable control register"]
    pub fmc_pcr: FMC_PCR,
    #[doc = "0x84 - This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
    pub fmc_sr: FMC_SR,
    #[doc = "0x88 - The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
    pub fmc_pmem: FMC_PMEM,
    #[doc = "0x8c - The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
    pub fmc_patt: FMC_PATT,
    #[doc = "0x90 - This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_hpr: FMC_HPR,
    #[doc = "0x94 - This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_heccr: FMC_HECCR,
    _reserved15: [u8; 108usize],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr1: FMC_BWTR1,
    _reserved16: [u8; 4usize],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr2: FMC_BWTR2,
    _reserved17: [u8; 4usize],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr3: FMC_BWTR3,
    _reserved18: [u8; 4usize],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr4: FMC_BWTR4,
    _reserved19: [u8; 224usize],
    #[doc = "0x200 - FMC NAND Command Sequencer Control Register"]
    pub fmc_csqcr: FMC_CSQCR,
    #[doc = "0x204 - FMC NAND Command Sequencer Configuration Register 1"]
    pub fmc_csqcfgr1: FMC_CSQCFGR1,
    #[doc = "0x208 - This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
    pub fmc_csqcfgr2: FMC_CSQCFGR2,
    #[doc = "0x20c - FMC NAND sequencer configuration register 3"]
    pub fmc_csqcfgr3: FMC_CSQCFGR3,
    #[doc = "0x210 - This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
    pub fmc_csqar1: FMC_CSQAR1,
    #[doc = "0x214 - This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
    pub fmc_csqar2: FMC_CSQAR2,
    _reserved25: [u8; 8usize],
    #[doc = "0x220 - FMC NAND Command Sequencer Interrupt Enable Register"]
    pub fmc_csqier: FMC_CSQIER,
    #[doc = "0x224 - FMC NAND Command Sequencer Interrupt Status Register"]
    pub fmc_csqisr: FMC_CSQISR,
    #[doc = "0x228 - FMC NAND Command Sequencer Interrupt Clear Register"]
    pub fmc_csqicr: FMC_CSQICR,
    _reserved28: [u8; 4usize],
    #[doc = "0x230 - This register holds a sector error mapping status when the whole transfer is complete."]
    pub fmc_csqemsr: FMC_CSQEMSR,
    _reserved29: [u8; 28usize],
    #[doc = "0x250 - FMC BCH Interrupt enable register"]
    pub fmc_bchier: FMC_BCHIER,
    #[doc = "0x254 - This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
    pub fmc_bchisr: FMC_BCHISR,
    #[doc = "0x258 - FMC BCH Interrupt Clear Register"]
    pub fmc_bchicr: FMC_BCHICR,
    _reserved32: [u8; 4usize],
    #[doc = "0x260 - These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
    pub fmc_bchpbr1: FMC_BCHPBR1,
    #[doc = "0x264 - FMC BCH Parity Bits Register 2"]
    pub fmc_bchpbr2: FMC_BCHPBR2,
    #[doc = "0x268 - FMC BCH Parity Bits Register 3"]
    pub fmc_bchpbr3: FMC_BCHPBR3,
    #[doc = "0x26c - FMC BCH Parity Bits Register 4"]
    pub fmc_bchpbr4: FMC_BCHPBR4,
    _reserved36: [u8; 12usize],
    #[doc = "0x27c - This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
    pub fmc_bchdsr0: FMC_BCHDSR0,
    #[doc = "0x280 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
    pub fmc_bchdsr1: FMC_BCHDSR1,
    #[doc = "0x284 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
    pub fmc_bchdsr2: FMC_BCHDSR2,
    #[doc = "0x288 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
    pub fmc_bchdsr3: FMC_BCHDSR3,
    #[doc = "0x28c - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
    pub fmc_bchdsr4: FMC_BCHDSR4,
    _reserved41: [u8; 348usize],
    #[doc = "0x3ec - FMC Hardware configuration register 2"]
    pub fmc_hwcfgr2: FMC_HWCFGR2,
    #[doc = "0x3f0 - FMC Hardware configuration register 1"]
    pub fmc_hwcfgr1: FMC_HWCFGR1,
    #[doc = "0x3f4 - FMC Version register"]
    pub fmc_verr: FMC_VERR,
    #[doc = "0x3f8 - FMC Identification register"]
    pub fmc_ipidr: FMC_IPIDR,
    #[doc = "0x3fc - FMC Size Identification register"]
    pub fmc_sidr: FMC_SIDR,
}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr1](fmc_bcr1) module"]
pub type FMC_BCR1 = crate::Reg<u32, _FMC_BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR1;
#[doc = "`read()` method returns [fmc_bcr1::R](fmc_bcr1::R) reader structure"]
impl crate::Readable for FMC_BCR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr1::W](fmc_bcr1::W) writer structure"]
impl crate::Writable for FMC_BCR1 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr1;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr1](fmc_btr1) module"]
pub type FMC_BTR1 = crate::Reg<u32, _FMC_BTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR1;
#[doc = "`read()` method returns [fmc_btr1::R](fmc_btr1::R) reader structure"]
impl crate::Readable for FMC_BTR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr1::W](fmc_btr1::W) writer structure"]
impl crate::Writable for FMC_BTR1 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr1;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr2](fmc_bcr2) module"]
pub type FMC_BCR2 = crate::Reg<u32, _FMC_BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR2;
#[doc = "`read()` method returns [fmc_bcr2::R](fmc_bcr2::R) reader structure"]
impl crate::Readable for FMC_BCR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr2::W](fmc_bcr2::W) writer structure"]
impl crate::Writable for FMC_BCR2 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr2;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr2](fmc_btr2) module"]
pub type FMC_BTR2 = crate::Reg<u32, _FMC_BTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR2;
#[doc = "`read()` method returns [fmc_btr2::R](fmc_btr2::R) reader structure"]
impl crate::Readable for FMC_BTR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr2::W](fmc_btr2::W) writer structure"]
impl crate::Writable for FMC_BTR2 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr2;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr3](fmc_bcr3) module"]
pub type FMC_BCR3 = crate::Reg<u32, _FMC_BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR3;
#[doc = "`read()` method returns [fmc_bcr3::R](fmc_bcr3::R) reader structure"]
impl crate::Readable for FMC_BCR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr3::W](fmc_bcr3::W) writer structure"]
impl crate::Writable for FMC_BCR3 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr3;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr3](fmc_btr3) module"]
pub type FMC_BTR3 = crate::Reg<u32, _FMC_BTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR3;
#[doc = "`read()` method returns [fmc_btr3::R](fmc_btr3::R) reader structure"]
impl crate::Readable for FMC_BTR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr3::W](fmc_btr3::W) writer structure"]
impl crate::Writable for FMC_BTR3 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr3;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bcr4](fmc_bcr4) module"]
pub type FMC_BCR4 = crate::Reg<u32, _FMC_BCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCR4;
#[doc = "`read()` method returns [fmc_bcr4::R](fmc_bcr4::R) reader structure"]
impl crate::Readable for FMC_BCR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_bcr4::W](fmc_bcr4::W) writer structure"]
impl crate::Writable for FMC_BCR4 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr4;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr4](fmc_btr4) module"]
pub type FMC_BTR4 = crate::Reg<u32, _FMC_BTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BTR4;
#[doc = "`read()` method returns [fmc_btr4::R](fmc_btr4::R) reader structure"]
impl crate::Readable for FMC_BTR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_btr4::W](fmc_btr4::W) writer structure"]
impl crate::Writable for FMC_BTR4 {}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr4;
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcscntr](fmc_pcscntr) module"]
pub type FMC_PCSCNTR = crate::Reg<u32, _FMC_PCSCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PCSCNTR;
#[doc = "`read()` method returns [fmc_pcscntr::R](fmc_pcscntr::R) reader structure"]
impl crate::Readable for FMC_PCSCNTR {}
#[doc = "`write(|w| ..)` method takes [fmc_pcscntr::W](fmc_pcscntr::W) writer structure"]
impl crate::Writable for FMC_PCSCNTR {}
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
pub mod fmc_pcscntr;
#[doc = "NAND Flash Programmable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](fmc_pcr) module"]
pub type FMC_PCR = crate::Reg<u32, _FMC_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PCR;
#[doc = "`read()` method returns [fmc_pcr::R](fmc_pcr::R) reader structure"]
impl crate::Readable for FMC_PCR {}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](fmc_pcr::W) writer structure"]
impl crate::Writable for FMC_PCR {}
#[doc = "NAND Flash Programmable control register"]
pub mod fmc_pcr;
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sr](fmc_sr) module"]
pub type FMC_SR = crate::Reg<u32, _FMC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SR;
#[doc = "`read()` method returns [fmc_sr::R](fmc_sr::R) reader structure"]
impl crate::Readable for FMC_SR {}
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
pub mod fmc_sr;
#[doc = "The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pmem](fmc_pmem) module"]
pub type FMC_PMEM = crate::Reg<u32, _FMC_PMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PMEM;
#[doc = "`read()` method returns [fmc_pmem::R](fmc_pmem::R) reader structure"]
impl crate::Readable for FMC_PMEM {}
#[doc = "`write(|w| ..)` method takes [fmc_pmem::W](fmc_pmem::W) writer structure"]
impl crate::Writable for FMC_PMEM {}
#[doc = "The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
pub mod fmc_pmem;
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_patt](fmc_patt) module"]
pub type FMC_PATT = crate::Reg<u32, _FMC_PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_PATT;
#[doc = "`read()` method returns [fmc_patt::R](fmc_patt::R) reader structure"]
impl crate::Readable for FMC_PATT {}
#[doc = "`write(|w| ..)` method takes [fmc_patt::W](fmc_patt::W) writer structure"]
impl crate::Writable for FMC_PATT {}
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
pub mod fmc_patt;
#[doc = "This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_hpr](fmc_hpr) module"]
pub type FMC_HPR = crate::Reg<u32, _FMC_HPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_HPR;
#[doc = "`read()` method returns [fmc_hpr::R](fmc_hpr::R) reader structure"]
impl crate::Readable for FMC_HPR {}
#[doc = "This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_hpr;
#[doc = "This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_heccr](fmc_heccr) module"]
pub type FMC_HECCR = crate::Reg<u32, _FMC_HECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_HECCR;
#[doc = "`read()` method returns [fmc_heccr::R](fmc_heccr::R) reader structure"]
impl crate::Readable for FMC_HECCR {}
#[doc = "This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_heccr;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr1](fmc_bwtr1) module"]
pub type FMC_BWTR1 = crate::Reg<u32, _FMC_BWTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR1;
#[doc = "`read()` method returns [fmc_bwtr1::R](fmc_bwtr1::R) reader structure"]
impl crate::Readable for FMC_BWTR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr1::W](fmc_bwtr1::W) writer structure"]
impl crate::Writable for FMC_BWTR1 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr1;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr2](fmc_bwtr2) module"]
pub type FMC_BWTR2 = crate::Reg<u32, _FMC_BWTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR2;
#[doc = "`read()` method returns [fmc_bwtr2::R](fmc_bwtr2::R) reader structure"]
impl crate::Readable for FMC_BWTR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr2::W](fmc_bwtr2::W) writer structure"]
impl crate::Writable for FMC_BWTR2 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr2;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr3](fmc_bwtr3) module"]
pub type FMC_BWTR3 = crate::Reg<u32, _FMC_BWTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR3;
#[doc = "`read()` method returns [fmc_bwtr3::R](fmc_bwtr3::R) reader structure"]
impl crate::Readable for FMC_BWTR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr3::W](fmc_bwtr3::W) writer structure"]
impl crate::Writable for FMC_BWTR3 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr3;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr4](fmc_bwtr4) module"]
pub type FMC_BWTR4 = crate::Reg<u32, _FMC_BWTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BWTR4;
#[doc = "`read()` method returns [fmc_bwtr4::R](fmc_bwtr4::R) reader structure"]
impl crate::Readable for FMC_BWTR4 {}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr4::W](fmc_bwtr4::W) writer structure"]
impl crate::Writable for FMC_BWTR4 {}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr4;
#[doc = "FMC NAND Command Sequencer Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcr](fmc_csqcr) module"]
pub type FMC_CSQCR = crate::Reg<u32, _FMC_CSQCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQCR;
#[doc = "`write(|w| ..)` method takes [fmc_csqcr::W](fmc_csqcr::W) writer structure"]
impl crate::Writable for FMC_CSQCR {}
#[doc = "FMC NAND Command Sequencer Control Register"]
pub mod fmc_csqcr;
#[doc = "FMC NAND Command Sequencer Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr1](fmc_csqcfgr1) module"]
pub type FMC_CSQCFGR1 = crate::Reg<u32, _FMC_CSQCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQCFGR1;
#[doc = "`read()` method returns [fmc_csqcfgr1::R](fmc_csqcfgr1::R) reader structure"]
impl crate::Readable for FMC_CSQCFGR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr1::W](fmc_csqcfgr1::W) writer structure"]
impl crate::Writable for FMC_CSQCFGR1 {}
#[doc = "FMC NAND Command Sequencer Configuration Register 1"]
pub mod fmc_csqcfgr1;
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr2](fmc_csqcfgr2) module"]
pub type FMC_CSQCFGR2 = crate::Reg<u32, _FMC_CSQCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQCFGR2;
#[doc = "`read()` method returns [fmc_csqcfgr2::R](fmc_csqcfgr2::R) reader structure"]
impl crate::Readable for FMC_CSQCFGR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr2::W](fmc_csqcfgr2::W) writer structure"]
impl crate::Writable for FMC_CSQCFGR2 {}
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
pub mod fmc_csqcfgr2;
#[doc = "FMC NAND sequencer configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcfgr3](fmc_csqcfgr3) module"]
pub type FMC_CSQCFGR3 = crate::Reg<u32, _FMC_CSQCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQCFGR3;
#[doc = "`read()` method returns [fmc_csqcfgr3::R](fmc_csqcfgr3::R) reader structure"]
impl crate::Readable for FMC_CSQCFGR3 {}
#[doc = "`write(|w| ..)` method takes [fmc_csqcfgr3::W](fmc_csqcfgr3::W) writer structure"]
impl crate::Writable for FMC_CSQCFGR3 {}
#[doc = "FMC NAND sequencer configuration register 3"]
pub mod fmc_csqcfgr3;
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqar1](fmc_csqar1) module"]
pub type FMC_CSQAR1 = crate::Reg<u32, _FMC_CSQAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQAR1;
#[doc = "`read()` method returns [fmc_csqar1::R](fmc_csqar1::R) reader structure"]
impl crate::Readable for FMC_CSQAR1 {}
#[doc = "`write(|w| ..)` method takes [fmc_csqar1::W](fmc_csqar1::W) writer structure"]
impl crate::Writable for FMC_CSQAR1 {}
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
pub mod fmc_csqar1;
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqar2](fmc_csqar2) module"]
pub type FMC_CSQAR2 = crate::Reg<u32, _FMC_CSQAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQAR2;
#[doc = "`read()` method returns [fmc_csqar2::R](fmc_csqar2::R) reader structure"]
impl crate::Readable for FMC_CSQAR2 {}
#[doc = "`write(|w| ..)` method takes [fmc_csqar2::W](fmc_csqar2::W) writer structure"]
impl crate::Writable for FMC_CSQAR2 {}
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
pub mod fmc_csqar2;
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqier](fmc_csqier) module"]
pub type FMC_CSQIER = crate::Reg<u32, _FMC_CSQIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQIER;
#[doc = "`read()` method returns [fmc_csqier::R](fmc_csqier::R) reader structure"]
impl crate::Readable for FMC_CSQIER {}
#[doc = "`write(|w| ..)` method takes [fmc_csqier::W](fmc_csqier::W) writer structure"]
impl crate::Writable for FMC_CSQIER {}
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register"]
pub mod fmc_csqier;
#[doc = "FMC NAND Command Sequencer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqisr](fmc_csqisr) module"]
pub type FMC_CSQISR = crate::Reg<u32, _FMC_CSQISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQISR;
#[doc = "`read()` method returns [fmc_csqisr::R](fmc_csqisr::R) reader structure"]
impl crate::Readable for FMC_CSQISR {}
#[doc = "`write(|w| ..)` method takes [fmc_csqisr::W](fmc_csqisr::W) writer structure"]
impl crate::Writable for FMC_CSQISR {}
#[doc = "FMC NAND Command Sequencer Interrupt Status Register"]
pub mod fmc_csqisr;
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqicr](fmc_csqicr) module"]
pub type FMC_CSQICR = crate::Reg<u32, _FMC_CSQICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQICR;
#[doc = "`write(|w| ..)` method takes [fmc_csqicr::W](fmc_csqicr::W) writer structure"]
impl crate::Writable for FMC_CSQICR {}
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register"]
pub mod fmc_csqicr;
#[doc = "This register holds a sector error mapping status when the whole transfer is complete.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqemsr](fmc_csqemsr) module"]
pub type FMC_CSQEMSR = crate::Reg<u32, _FMC_CSQEMSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_CSQEMSR;
#[doc = "`read()` method returns [fmc_csqemsr::R](fmc_csqemsr::R) reader structure"]
impl crate::Readable for FMC_CSQEMSR {}
#[doc = "This register holds a sector error mapping status when the whole transfer is complete."]
pub mod fmc_csqemsr;
#[doc = "FMC BCH Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchier](fmc_bchier) module"]
pub type FMC_BCHIER = crate::Reg<u32, _FMC_BCHIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHIER;
#[doc = "`read()` method returns [fmc_bchier::R](fmc_bchier::R) reader structure"]
impl crate::Readable for FMC_BCHIER {}
#[doc = "`write(|w| ..)` method takes [fmc_bchier::W](fmc_bchier::W) writer structure"]
impl crate::Writable for FMC_BCHIER {}
#[doc = "FMC BCH Interrupt enable register"]
pub mod fmc_bchier;
#[doc = "This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchisr](fmc_bchisr) module"]
pub type FMC_BCHISR = crate::Reg<u32, _FMC_BCHISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHISR;
#[doc = "`read()` method returns [fmc_bchisr::R](fmc_bchisr::R) reader structure"]
impl crate::Readable for FMC_BCHISR {}
#[doc = "This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
pub mod fmc_bchisr;
#[doc = "FMC BCH Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchicr](fmc_bchicr) module"]
pub type FMC_BCHICR = crate::Reg<u32, _FMC_BCHICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHICR;
#[doc = "`write(|w| ..)` method takes [fmc_bchicr::W](fmc_bchicr::W) writer structure"]
impl crate::Writable for FMC_BCHICR {}
#[doc = "FMC BCH Interrupt Clear Register"]
pub mod fmc_bchicr;
#[doc = "These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr1](fmc_bchpbr1) module"]
pub type FMC_BCHPBR1 = crate::Reg<u32, _FMC_BCHPBR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHPBR1;
#[doc = "`read()` method returns [fmc_bchpbr1::R](fmc_bchpbr1::R) reader structure"]
impl crate::Readable for FMC_BCHPBR1 {}
#[doc = "These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
pub mod fmc_bchpbr1;
#[doc = "FMC BCH Parity Bits Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr2](fmc_bchpbr2) module"]
pub type FMC_BCHPBR2 = crate::Reg<u32, _FMC_BCHPBR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHPBR2;
#[doc = "`read()` method returns [fmc_bchpbr2::R](fmc_bchpbr2::R) reader structure"]
impl crate::Readable for FMC_BCHPBR2 {}
#[doc = "FMC BCH Parity Bits Register 2"]
pub mod fmc_bchpbr2;
#[doc = "FMC BCH Parity Bits Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr3](fmc_bchpbr3) module"]
pub type FMC_BCHPBR3 = crate::Reg<u32, _FMC_BCHPBR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHPBR3;
#[doc = "`read()` method returns [fmc_bchpbr3::R](fmc_bchpbr3::R) reader structure"]
impl crate::Readable for FMC_BCHPBR3 {}
#[doc = "FMC BCH Parity Bits Register 3"]
pub mod fmc_bchpbr3;
#[doc = "FMC BCH Parity Bits Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr4](fmc_bchpbr4) module"]
pub type FMC_BCHPBR4 = crate::Reg<u32, _FMC_BCHPBR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHPBR4;
#[doc = "`read()` method returns [fmc_bchpbr4::R](fmc_bchpbr4::R) reader structure"]
impl crate::Readable for FMC_BCHPBR4 {}
#[doc = "FMC BCH Parity Bits Register 4"]
pub mod fmc_bchpbr4;
#[doc = "This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr0](fmc_bchdsr0) module"]
pub type FMC_BCHDSR0 = crate::Reg<u32, _FMC_BCHDSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHDSR0;
#[doc = "`read()` method returns [fmc_bchdsr0::R](fmc_bchdsr0::R) reader structure"]
impl crate::Readable for FMC_BCHDSR0 {}
#[doc = "This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
pub mod fmc_bchdsr0;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr1](fmc_bchdsr1) module"]
pub type FMC_BCHDSR1 = crate::Reg<u32, _FMC_BCHDSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHDSR1;
#[doc = "`read()` method returns [fmc_bchdsr1::R](fmc_bchdsr1::R) reader structure"]
impl crate::Readable for FMC_BCHDSR1 {}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
pub mod fmc_bchdsr1;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr2](fmc_bchdsr2) module"]
pub type FMC_BCHDSR2 = crate::Reg<u32, _FMC_BCHDSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHDSR2;
#[doc = "`read()` method returns [fmc_bchdsr2::R](fmc_bchdsr2::R) reader structure"]
impl crate::Readable for FMC_BCHDSR2 {}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
pub mod fmc_bchdsr2;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr3](fmc_bchdsr3) module"]
pub type FMC_BCHDSR3 = crate::Reg<u32, _FMC_BCHDSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHDSR3;
#[doc = "`read()` method returns [fmc_bchdsr3::R](fmc_bchdsr3::R) reader structure"]
impl crate::Readable for FMC_BCHDSR3 {}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
pub mod fmc_bchdsr3;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr4](fmc_bchdsr4) module"]
pub type FMC_BCHDSR4 = crate::Reg<u32, _FMC_BCHDSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_BCHDSR4;
#[doc = "`read()` method returns [fmc_bchdsr4::R](fmc_bchdsr4::R) reader structure"]
impl crate::Readable for FMC_BCHDSR4 {}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
pub mod fmc_bchdsr4;
#[doc = "FMC Hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_hwcfgr2](fmc_hwcfgr2) module"]
pub type FMC_HWCFGR2 = crate::Reg<u32, _FMC_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_HWCFGR2;
#[doc = "`read()` method returns [fmc_hwcfgr2::R](fmc_hwcfgr2::R) reader structure"]
impl crate::Readable for FMC_HWCFGR2 {}
#[doc = "FMC Hardware configuration register 2"]
pub mod fmc_hwcfgr2;
#[doc = "FMC Hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_hwcfgr1](fmc_hwcfgr1) module"]
pub type FMC_HWCFGR1 = crate::Reg<u32, _FMC_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_HWCFGR1;
#[doc = "`read()` method returns [fmc_hwcfgr1::R](fmc_hwcfgr1::R) reader structure"]
impl crate::Readable for FMC_HWCFGR1 {}
#[doc = "FMC Hardware configuration register 1"]
pub mod fmc_hwcfgr1;
#[doc = "FMC Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_verr](fmc_verr) module"]
pub type FMC_VERR = crate::Reg<u32, _FMC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_VERR;
#[doc = "`read()` method returns [fmc_verr::R](fmc_verr::R) reader structure"]
impl crate::Readable for FMC_VERR {}
#[doc = "FMC Version register"]
pub mod fmc_verr;
#[doc = "FMC Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ipidr](fmc_ipidr) module"]
pub type FMC_IPIDR = crate::Reg<u32, _FMC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_IPIDR;
#[doc = "`read()` method returns [fmc_ipidr::R](fmc_ipidr::R) reader structure"]
impl crate::Readable for FMC_IPIDR {}
#[doc = "FMC Identification register"]
pub mod fmc_ipidr;
#[doc = "FMC Size Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sidr](fmc_sidr) module"]
pub type FMC_SIDR = crate::Reg<u32, _FMC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SIDR;
#[doc = "`read()` method returns [fmc_sidr::R](fmc_sidr::R) reader structure"]
impl crate::Readable for FMC_SIDR {}
#[doc = "FMC Size Identification register"]
pub mod fmc_sidr;
