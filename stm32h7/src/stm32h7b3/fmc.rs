#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x04 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub btr1: crate::Reg<btr1::BTR1_SPEC>,
    #[doc = "0x08 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
    pub bcr2: crate::Reg<bcr2::BCR2_SPEC>,
    #[doc = "0x0c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub btr2: crate::Reg<btr2::BTR2_SPEC>,
    #[doc = "0x10 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
    pub bcr3: crate::Reg<bcr3::BCR3_SPEC>,
    #[doc = "0x14 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub btr3: crate::Reg<btr3::BTR3_SPEC>,
    #[doc = "0x18 - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
    pub bcr4: crate::Reg<bcr4::BCR4_SPEC>,
    #[doc = "0x1c - This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
    pub btr4: crate::Reg<btr4::BTR4_SPEC>,
    _reserved8: [u8; 0x60],
    #[doc = "0x80 - NAND Flash control registers"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x84 - This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x88 - The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
    pub pmem: crate::Reg<pmem::PMEM_SPEC>,
    #[doc = "0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
    pub patt: crate::Reg<patt::PATT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
    pub eccr: crate::Reg<eccr::ECCR_SPEC>,
    _reserved13: [u8; 0x6c],
    #[doc = "0x104 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr1: crate::Reg<bwtr1::BWTR1_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x10c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr2: crate::Reg<bwtr2::BWTR2_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x114 - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr3: crate::Reg<bwtr3::BWTR3_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x11c - This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
    pub bwtr4: crate::Reg<bwtr4::BWTR4_SPEC>,
    _reserved17: [u8; 0x20],
    _reserved_17_sdbank1: [u8; 0x10],
    #[doc = "0x150 - This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks."]
    pub sdcmr: crate::Reg<sdcmr::SDCMR_SPEC>,
    #[doc = "0x154 - This register sets the refresh rate in number of SDCLK clock cycles between the refresh cycles by configuring the Refresh Timer Count value.Examplewhere 64 ms is the SDRAM refresh period.The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to obtain a safe margin if an internal refresh request occurs when a read request has been accepted. It corresponds to a COUNT value of 0000111000000 (448). This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This timer generates a refresh pulse when zero is reached. The COUNT value must be set at least to 41 SDRAM clock cycles.As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value programmed in the register is 0, no refresh is carried out. This register must not be reprogrammed after the initialization procedure to avoid modifying the refresh rate.Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.If a memory access is in progress, the Auto-refresh request is delayed. However, if the memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes precedence. If the memory access occurs during a refresh operation, the request is buffered to be processed when the refresh is complete.This register is common to SDRAM bank 1 and bank 2."]
    pub sdrtr: crate::Reg<sdrtr::SDRTR_SPEC>,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: crate::Reg<sdsr::SDSR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x140..0x14c - Cluster SDBANK%s, containing SDTR?, SDCR?"]
    #[inline(always)]
    pub fn sdbank1(&self) -> &SDBANK {
        unsafe { &*(((self as *const Self) as *const u8).add(320usize) as *const SDBANK) }
    }
    #[doc = "0x144..0x150 - Cluster SDBANK%s, containing SDTR?, SDCR?"]
    #[inline(always)]
    pub fn sdbank2(&self) -> &SDBANK {
        unsafe { &*(((self as *const Self) as *const u8).add(324usize) as *const SDBANK) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SDBANK {
    #[doc = "0x00 - This register contains the control parameters for each SDRAM memory bank"]
    pub sdcr: crate::Reg<self::sdbank::sdcr::SDCR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - This register contains the timing parameters of each SDRAM bank"]
    pub sdtr: crate::Reg<self::sdbank::sdtr::SDTR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster SDBANK%s, containing SDTR?, SDCR?"]
pub mod sdbank;
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
pub mod bcr1;
#[doc = "BTR1 register accessor: an alias for `Reg<BTR1_SPEC>`"]
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr1;
#[doc = "BCR2 register accessor: an alias for `Reg<BCR2_SPEC>`"]
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
pub mod bcr2;
#[doc = "BTR2 register accessor: an alias for `Reg<BTR2_SPEC>`"]
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr2;
#[doc = "BCR3 register accessor: an alias for `Reg<BCR3_SPEC>`"]
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
pub mod bcr3;
#[doc = "BTR3 register accessor: an alias for `Reg<BTR3_SPEC>`"]
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr3;
#[doc = "BCR4 register accessor: an alias for `Reg<BCR4_SPEC>`"]
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories."]
pub mod bcr4;
#[doc = "BTR4 register accessor: an alias for `Reg<BTR4_SPEC>`"]
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers)."]
pub mod btr4;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "NAND Flash control registers"]
pub mod pcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty."]
pub mod sr;
#[doc = "PMEM register accessor: an alias for `Reg<PMEM_SPEC>`"]
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access."]
pub mod pmem;
#[doc = "PATT register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature)."]
pub mod patt;
#[doc = "ECCR register accessor: an alias for `Reg<ECCR_SPEC>`"]
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
#[doc = "This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1."]
pub mod eccr;
#[doc = "BWTR1 register accessor: an alias for `Reg<BWTR1_SPEC>`"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr1;
#[doc = "BWTR2 register accessor: an alias for `Reg<BWTR2_SPEC>`"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr2;
#[doc = "BWTR3 register accessor: an alias for `Reg<BWTR3_SPEC>`"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr3;
#[doc = "BWTR4 register accessor: an alias for `Reg<BWTR4_SPEC>`"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4_SPEC>;
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access."]
pub mod bwtr4;
#[doc = "SDCMR register accessor: an alias for `Reg<SDCMR_SPEC>`"]
pub type SDCMR = crate::Reg<sdcmr::SDCMR_SPEC>;
#[doc = "This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks."]
pub mod sdcmr;
#[doc = "SDRTR register accessor: an alias for `Reg<SDRTR_SPEC>`"]
pub type SDRTR = crate::Reg<sdrtr::SDRTR_SPEC>;
#[doc = "This register sets the refresh rate in number of SDCLK clock cycles between the refresh cycles by configuring the Refresh Timer Count value.Examplewhere 64 ms is the SDRAM refresh period.The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to obtain a safe margin if an internal refresh request occurs when a read request has been accepted. It corresponds to a COUNT value of 0000111000000 (448). This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This timer generates a refresh pulse when zero is reached. The COUNT value must be set at least to 41 SDRAM clock cycles.As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value programmed in the register is 0, no refresh is carried out. This register must not be reprogrammed after the initialization procedure to avoid modifying the refresh rate.Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.If a memory access is in progress, the Auto-refresh request is delayed. However, if the memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes precedence. If the memory access occurs during a refresh operation, the request is buffered to be processed when the refresh is complete.This register is common to SDRAM bank 1 and bank 2."]
pub mod sdrtr;
#[doc = "SDSR register accessor: an alias for `Reg<SDSR_SPEC>`"]
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
#[doc = "SDRAM Status register"]
pub mod sdsr;
