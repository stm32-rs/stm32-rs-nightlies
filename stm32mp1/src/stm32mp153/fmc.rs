#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr1: crate::Reg<fmc_bcr1::FMC_BCR1_SPEC>,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr1: crate::Reg<fmc_btr1::FMC_BTR1_SPEC>,
    #[doc = "0x08 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr2: crate::Reg<fmc_bcr2::FMC_BCR2_SPEC>,
    #[doc = "0x0c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr2: crate::Reg<fmc_btr2::FMC_BTR2_SPEC>,
    #[doc = "0x10 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr3: crate::Reg<fmc_bcr3::FMC_BCR3_SPEC>,
    #[doc = "0x14 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr3: crate::Reg<fmc_btr3::FMC_BTR3_SPEC>,
    #[doc = "0x18 - This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
    pub fmc_bcr4: crate::Reg<fmc_bcr4::FMC_BCR4_SPEC>,
    #[doc = "0x1c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub fmc_btr4: crate::Reg<fmc_btr4::FMC_BTR4_SPEC>,
    #[doc = "0x20 - This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
    pub fmc_pcscntr: crate::Reg<fmc_pcscntr::FMC_PCSCNTR_SPEC>,
    _reserved9: [u8; 0x5c],
    #[doc = "0x80 - NAND Flash Programmable control register"]
    pub fmc_pcr: crate::Reg<fmc_pcr::FMC_PCR_SPEC>,
    #[doc = "0x84 - This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
    pub fmc_sr: crate::Reg<fmc_sr::FMC_SR_SPEC>,
    #[doc = "0x88 - The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
    pub fmc_pmem: crate::Reg<fmc_pmem::FMC_PMEM_SPEC>,
    #[doc = "0x8c - The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
    pub fmc_patt: crate::Reg<fmc_patt::FMC_PATT_SPEC>,
    #[doc = "0x90 - This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_hpr: crate::Reg<fmc_hpr::FMC_HPR_SPEC>,
    #[doc = "0x94 - This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub fmc_heccr: crate::Reg<fmc_heccr::FMC_HECCR_SPEC>,
    _reserved15: [u8; 0x6c],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr1: crate::Reg<fmc_bwtr1::FMC_BWTR1_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr2: crate::Reg<fmc_bwtr2::FMC_BWTR2_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr3: crate::Reg<fmc_bwtr3::FMC_BWTR3_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub fmc_bwtr4: crate::Reg<fmc_bwtr4::FMC_BWTR4_SPEC>,
    _reserved19: [u8; 0xe0],
    #[doc = "0x200 - FMC NAND Command Sequencer Control Register"]
    pub fmc_csqcr: crate::Reg<fmc_csqcr::FMC_CSQCR_SPEC>,
    #[doc = "0x204 - FMC NAND Command Sequencer Configuration Register 1"]
    pub fmc_csqcfgr1: crate::Reg<fmc_csqcfgr1::FMC_CSQCFGR1_SPEC>,
    #[doc = "0x208 - This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
    pub fmc_csqcfgr2: crate::Reg<fmc_csqcfgr2::FMC_CSQCFGR2_SPEC>,
    #[doc = "0x20c - FMC NAND sequencer configuration register 3"]
    pub fmc_csqcfgr3: crate::Reg<fmc_csqcfgr3::FMC_CSQCFGR3_SPEC>,
    #[doc = "0x210 - This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
    pub fmc_csqar1: crate::Reg<fmc_csqar1::FMC_CSQAR1_SPEC>,
    #[doc = "0x214 - This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
    pub fmc_csqar2: crate::Reg<fmc_csqar2::FMC_CSQAR2_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x220 - FMC NAND Command Sequencer Interrupt Enable Register"]
    pub fmc_csqier: crate::Reg<fmc_csqier::FMC_CSQIER_SPEC>,
    #[doc = "0x224 - FMC NAND Command Sequencer Interrupt Status Register"]
    pub fmc_csqisr: crate::Reg<fmc_csqisr::FMC_CSQISR_SPEC>,
    #[doc = "0x228 - FMC NAND Command Sequencer Interrupt Clear Register"]
    pub fmc_csqicr: crate::Reg<fmc_csqicr::FMC_CSQICR_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x230 - This register holds a sector error mapping status when the whole transfer is complete."]
    pub fmc_csqemsr: crate::Reg<fmc_csqemsr::FMC_CSQEMSR_SPEC>,
    _reserved29: [u8; 0x1c],
    #[doc = "0x250 - FMC BCH Interrupt enable register"]
    pub fmc_bchier: crate::Reg<fmc_bchier::FMC_BCHIER_SPEC>,
    #[doc = "0x254 - This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
    pub fmc_bchisr: crate::Reg<fmc_bchisr::FMC_BCHISR_SPEC>,
    #[doc = "0x258 - FMC BCH Interrupt Clear Register"]
    pub fmc_bchicr: crate::Reg<fmc_bchicr::FMC_BCHICR_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x260 - These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
    pub fmc_bchpbr1: crate::Reg<fmc_bchpbr1::FMC_BCHPBR1_SPEC>,
    #[doc = "0x264 - FMC BCH Parity Bits Register 2"]
    pub fmc_bchpbr2: crate::Reg<fmc_bchpbr2::FMC_BCHPBR2_SPEC>,
    #[doc = "0x268 - FMC BCH Parity Bits Register 3"]
    pub fmc_bchpbr3: crate::Reg<fmc_bchpbr3::FMC_BCHPBR3_SPEC>,
    #[doc = "0x26c - FMC BCH Parity Bits Register 4"]
    pub fmc_bchpbr4: crate::Reg<fmc_bchpbr4::FMC_BCHPBR4_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x27c - This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
    pub fmc_bchdsr0: crate::Reg<fmc_bchdsr0::FMC_BCHDSR0_SPEC>,
    #[doc = "0x280 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
    pub fmc_bchdsr1: crate::Reg<fmc_bchdsr1::FMC_BCHDSR1_SPEC>,
    #[doc = "0x284 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
    pub fmc_bchdsr2: crate::Reg<fmc_bchdsr2::FMC_BCHDSR2_SPEC>,
    #[doc = "0x288 - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
    pub fmc_bchdsr3: crate::Reg<fmc_bchdsr3::FMC_BCHDSR3_SPEC>,
    #[doc = "0x28c - The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
    pub fmc_bchdsr4: crate::Reg<fmc_bchdsr4::FMC_BCHDSR4_SPEC>,
    _reserved41: [u8; 0x015c],
    #[doc = "0x3ec - FMC Hardware configuration register 2"]
    pub fmc_hwcfgr2: crate::Reg<fmc_hwcfgr2::FMC_HWCFGR2_SPEC>,
    #[doc = "0x3f0 - FMC Hardware configuration register 1"]
    pub fmc_hwcfgr1: crate::Reg<fmc_hwcfgr1::FMC_HWCFGR1_SPEC>,
    #[doc = "0x3f4 - FMC Version register"]
    pub fmc_verr: crate::Reg<fmc_verr::FMC_VERR_SPEC>,
    #[doc = "0x3f8 - FMC Identification register"]
    pub fmc_ipidr: crate::Reg<fmc_ipidr::FMC_IPIDR_SPEC>,
    #[doc = "0x3fc - FMC Size Identification register"]
    pub fmc_sidr: crate::Reg<fmc_sidr::FMC_SIDR_SPEC>,
}
#[doc = "FMC_BCR1 register accessor: an alias for `Reg<FMC_BCR1_SPEC>`"]
pub type FMC_BCR1 = crate::Reg<fmc_bcr1::FMC_BCR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr1;
#[doc = "FMC_BTR1 register accessor: an alias for `Reg<FMC_BTR1_SPEC>`"]
pub type FMC_BTR1 = crate::Reg<fmc_btr1::FMC_BTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr1;
#[doc = "FMC_BCR2 register accessor: an alias for `Reg<FMC_BCR2_SPEC>`"]
pub type FMC_BCR2 = crate::Reg<fmc_bcr2::FMC_BCR2_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr2;
#[doc = "FMC_BTR2 register accessor: an alias for `Reg<FMC_BTR2_SPEC>`"]
pub type FMC_BTR2 = crate::Reg<fmc_btr2::FMC_BTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr2;
#[doc = "FMC_BCR3 register accessor: an alias for `Reg<FMC_BCR3_SPEC>`"]
pub type FMC_BCR3 = crate::Reg<fmc_bcr3::FMC_BCR3_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr3;
#[doc = "FMC_BTR3 register accessor: an alias for `Reg<FMC_BTR3_SPEC>`"]
pub type FMC_BTR3 = crate::Reg<fmc_btr3::FMC_BTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr3;
#[doc = "FMC_BCR4 register accessor: an alias for `Reg<FMC_BCR4_SPEC>`"]
pub type FMC_BCR4 = crate::Reg<fmc_bcr4::FMC_BCR4_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM, FRAM and NOR Flash memories."]
pub mod fmc_bcr4;
#[doc = "FMC_BTR4 register accessor: an alias for `Reg<FMC_BTR4_SPEC>`"]
pub type FMC_BTR4 = crate::Reg<fmc_btr4::FMC_BTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod fmc_btr4;
#[doc = "FMC_PCSCNTR register accessor: an alias for `Reg<FMC_PCSCNTR_SPEC>`"]
pub type FMC_PCSCNTR = crate::Reg<fmc_pcscntr::FMC_PCSCNTR_SPEC>;
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h"]
pub mod fmc_pcscntr;
#[doc = "FMC_PCR register accessor: an alias for `Reg<FMC_PCR_SPEC>`"]
pub type FMC_PCR = crate::Reg<fmc_pcr::FMC_PCR_SPEC>;
#[doc = "NAND Flash Programmable control register"]
pub mod fmc_pcr;
#[doc = "FMC_SR register accessor: an alias for `Reg<FMC_SR_SPEC>`"]
pub type FMC_SR = crate::Reg<fmc_sr::FMC_SR_SPEC>;
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits."]
pub mod fmc_sr;
#[doc = "FMC_PMEM register accessor: an alias for `Reg<FMC_PMEM_SPEC>`"]
pub type FMC_PMEM = crate::Reg<fmc_pmem::FMC_PMEM_SPEC>;
#[doc = "The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses."]
pub mod fmc_pmem;
#[doc = "FMC_PATT register accessor: an alias for `Reg<FMC_PATT_SPEC>`"]
pub type FMC_PATT = crate::Reg<fmc_patt::FMC_PATT_SPEC>;
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function)."]
pub mod fmc_patt;
#[doc = "FMC_HPR register accessor: an alias for `Reg<FMC_HPR_SPEC>`"]
pub type FMC_HPR = crate::Reg<fmc_hpr::FMC_HPR_SPEC>;
#[doc = "This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_hpr;
#[doc = "FMC_HECCR register accessor: an alias for `Reg<FMC_HECCR_SPEC>`"]
pub type FMC_HECCR = crate::Reg<fmc_heccr::FMC_HECCR_SPEC>;
#[doc = "This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod fmc_heccr;
#[doc = "FMC_BWTR1 register accessor: an alias for `Reg<FMC_BWTR1_SPEC>`"]
pub type FMC_BWTR1 = crate::Reg<fmc_bwtr1::FMC_BWTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr1;
#[doc = "FMC_BWTR2 register accessor: an alias for `Reg<FMC_BWTR2_SPEC>`"]
pub type FMC_BWTR2 = crate::Reg<fmc_bwtr2::FMC_BWTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr2;
#[doc = "FMC_BWTR3 register accessor: an alias for `Reg<FMC_BWTR3_SPEC>`"]
pub type FMC_BWTR3 = crate::Reg<fmc_bwtr3::FMC_BWTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr3;
#[doc = "FMC_BWTR4 register accessor: an alias for `Reg<FMC_BWTR4_SPEC>`"]
pub type FMC_BWTR4 = crate::Reg<fmc_bwtr4::FMC_BWTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, FRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod fmc_bwtr4;
#[doc = "FMC_CSQCR register accessor: an alias for `Reg<FMC_CSQCR_SPEC>`"]
pub type FMC_CSQCR = crate::Reg<fmc_csqcr::FMC_CSQCR_SPEC>;
#[doc = "FMC NAND Command Sequencer Control Register"]
pub mod fmc_csqcr;
#[doc = "FMC_CSQCFGR1 register accessor: an alias for `Reg<FMC_CSQCFGR1_SPEC>`"]
pub type FMC_CSQCFGR1 = crate::Reg<fmc_csqcfgr1::FMC_CSQCFGR1_SPEC>;
#[doc = "FMC NAND Command Sequencer Configuration Register 1"]
pub mod fmc_csqcfgr1;
#[doc = "FMC_CSQCFGR2 register accessor: an alias for `Reg<FMC_CSQCFGR2_SPEC>`"]
pub type FMC_CSQCFGR2 = crate::Reg<fmc_csqcfgr2::FMC_CSQCFGR2_SPEC>;
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. ."]
pub mod fmc_csqcfgr2;
#[doc = "FMC_CSQCFGR3 register accessor: an alias for `Reg<FMC_CSQCFGR3_SPEC>`"]
pub type FMC_CSQCFGR3 = crate::Reg<fmc_csqcfgr3::FMC_CSQCFGR3_SPEC>;
#[doc = "FMC NAND sequencer configuration register 3"]
pub mod fmc_csqcfgr3;
#[doc = "FMC_CSQAR1 register accessor: an alias for `Reg<FMC_CSQAR1_SPEC>`"]
pub type FMC_CSQAR1 = crate::Reg<fmc_csqar1::FMC_CSQAR1_SPEC>;
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer."]
pub mod fmc_csqar1;
#[doc = "FMC_CSQAR2 register accessor: an alias for `Reg<FMC_CSQAR2_SPEC>`"]
pub type FMC_CSQAR2 = crate::Reg<fmc_csqar2::FMC_CSQAR2_SPEC>;
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable."]
pub mod fmc_csqar2;
#[doc = "FMC_CSQIER register accessor: an alias for `Reg<FMC_CSQIER_SPEC>`"]
pub type FMC_CSQIER = crate::Reg<fmc_csqier::FMC_CSQIER_SPEC>;
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register"]
pub mod fmc_csqier;
#[doc = "FMC_CSQISR register accessor: an alias for `Reg<FMC_CSQISR_SPEC>`"]
pub type FMC_CSQISR = crate::Reg<fmc_csqisr::FMC_CSQISR_SPEC>;
#[doc = "FMC NAND Command Sequencer Interrupt Status Register"]
pub mod fmc_csqisr;
#[doc = "FMC_CSQICR register accessor: an alias for `Reg<FMC_CSQICR_SPEC>`"]
pub type FMC_CSQICR = crate::Reg<fmc_csqicr::FMC_CSQICR_SPEC>;
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register"]
pub mod fmc_csqicr;
#[doc = "FMC_CSQEMSR register accessor: an alias for `Reg<FMC_CSQEMSR_SPEC>`"]
pub type FMC_CSQEMSR = crate::Reg<fmc_csqemsr::FMC_CSQEMSR_SPEC>;
#[doc = "This register holds a sector error mapping status when the whole transfer is complete."]
pub mod fmc_csqemsr;
#[doc = "FMC_BCHIER register accessor: an alias for `Reg<FMC_BCHIER_SPEC>`"]
pub type FMC_BCHIER = crate::Reg<fmc_bchier::FMC_BCHIER_SPEC>;
#[doc = "FMC BCH Interrupt enable register"]
pub mod fmc_bchier;
#[doc = "FMC_BCHISR register accessor: an alias for `Reg<FMC_BCHISR_SPEC>`"]
pub type FMC_BCHISR = crate::Reg<fmc_bchisr::FMC_BCHISR_SPEC>;
#[doc = "This register holds the status of BCH encoder/decoder after processing each sector. When the sequencer is used, this register is automatically cleared."]
pub mod fmc_bchisr;
#[doc = "FMC_BCHICR register accessor: an alias for `Reg<FMC_BCHICR_SPEC>`"]
pub type FMC_BCHICR = crate::Reg<fmc_bchicr::FMC_BCHICR_SPEC>;
#[doc = "FMC BCH Interrupt Clear Register"]
pub mod fmc_bchicr;
#[doc = "FMC_BCHPBR1 register accessor: an alias for `Reg<FMC_BCHPBR1_SPEC>`"]
pub type FMC_BCHPBR1 = crate::Reg<fmc_bchpbr1::FMC_BCHPBR1_SPEC>;
#[doc = "These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant."]
pub mod fmc_bchpbr1;
#[doc = "FMC_BCHPBR2 register accessor: an alias for `Reg<FMC_BCHPBR2_SPEC>`"]
pub type FMC_BCHPBR2 = crate::Reg<fmc_bchpbr2::FMC_BCHPBR2_SPEC>;
#[doc = "FMC BCH Parity Bits Register 2"]
pub mod fmc_bchpbr2;
#[doc = "FMC_BCHPBR3 register accessor: an alias for `Reg<FMC_BCHPBR3_SPEC>`"]
pub type FMC_BCHPBR3 = crate::Reg<fmc_bchpbr3::FMC_BCHPBR3_SPEC>;
#[doc = "FMC BCH Parity Bits Register 3"]
pub mod fmc_bchpbr3;
#[doc = "FMC_BCHPBR4 register accessor: an alias for `Reg<FMC_BCHPBR4_SPEC>`"]
pub type FMC_BCHPBR4 = crate::Reg<fmc_bchpbr4::FMC_BCHPBR4_SPEC>;
#[doc = "FMC BCH Parity Bits Register 4"]
pub mod fmc_bchpbr4;
#[doc = "FMC_BCHDSR0 register accessor: an alias for `Reg<FMC_BCHDSR0_SPEC>`"]
pub type FMC_BCHDSR0 = crate::Reg<fmc_bchdsr0::FMC_BCHDSR0_SPEC>;
#[doc = "This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. ."]
pub mod fmc_bchdsr0;
#[doc = "FMC_BCHDSR1 register accessor: an alias for `Reg<FMC_BCHDSR1_SPEC>`"]
pub type FMC_BCHDSR1 = crate::Reg<fmc_bchdsr1::FMC_BCHDSR1_SPEC>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors"]
pub mod fmc_bchdsr1;
#[doc = "FMC_BCHDSR2 register accessor: an alias for `Reg<FMC_BCHDSR2_SPEC>`"]
pub type FMC_BCHDSR2 = crate::Reg<fmc_bchdsr2::FMC_BCHDSR2_SPEC>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively."]
pub mod fmc_bchdsr2;
#[doc = "FMC_BCHDSR3 register accessor: an alias for `Reg<FMC_BCHDSR3_SPEC>`"]
pub type FMC_BCHDSR3 = crate::Reg<fmc_bchdsr3::FMC_BCHDSR3_SPEC>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors."]
pub mod fmc_bchdsr3;
#[doc = "FMC_BCHDSR4 register accessor: an alias for `Reg<FMC_BCHDSR4_SPEC>`"]
pub type FMC_BCHDSR4 = crate::Reg<fmc_bchdsr4::FMC_BCHDSR4_SPEC>;
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. ."]
pub mod fmc_bchdsr4;
#[doc = "FMC_HWCFGR2 register accessor: an alias for `Reg<FMC_HWCFGR2_SPEC>`"]
pub type FMC_HWCFGR2 = crate::Reg<fmc_hwcfgr2::FMC_HWCFGR2_SPEC>;
#[doc = "FMC Hardware configuration register 2"]
pub mod fmc_hwcfgr2;
#[doc = "FMC_HWCFGR1 register accessor: an alias for `Reg<FMC_HWCFGR1_SPEC>`"]
pub type FMC_HWCFGR1 = crate::Reg<fmc_hwcfgr1::FMC_HWCFGR1_SPEC>;
#[doc = "FMC Hardware configuration register 1"]
pub mod fmc_hwcfgr1;
#[doc = "FMC_VERR register accessor: an alias for `Reg<FMC_VERR_SPEC>`"]
pub type FMC_VERR = crate::Reg<fmc_verr::FMC_VERR_SPEC>;
#[doc = "FMC Version register"]
pub mod fmc_verr;
#[doc = "FMC_IPIDR register accessor: an alias for `Reg<FMC_IPIDR_SPEC>`"]
pub type FMC_IPIDR = crate::Reg<fmc_ipidr::FMC_IPIDR_SPEC>;
#[doc = "FMC Identification register"]
pub mod fmc_ipidr;
#[doc = "FMC_SIDR register accessor: an alias for `Reg<FMC_SIDR_SPEC>`"]
pub type FMC_SIDR = crate::Reg<fmc_sidr::FMC_SIDR_SPEC>;
#[doc = "FMC Size Identification register"]
pub mod fmc_sidr;
