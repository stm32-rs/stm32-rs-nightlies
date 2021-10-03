#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub power: crate::Reg<power::POWER_SPEC>,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    pub clkcr: crate::Reg<clkcr::CLKCR_SPEC>,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub argr: crate::Reg<argr::ARGR_SPEC>,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub cmdr: crate::Reg<cmdr::CMDR_SPEC>,
    #[doc = "0x10 - SDMMC command response register"]
    pub respcmdr: crate::Reg<respcmdr::RESPCMDR_SPEC>,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp1r: crate::Reg<resp1r::RESP1R_SPEC>,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp2r: crate::Reg<resp2r::RESP2R_SPEC>,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp3r: crate::Reg<resp3r::RESP3R_SPEC>,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp4r: crate::Reg<resp4r::RESP4R_SPEC>,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub dtimer: crate::Reg<dtimer::DTIMER_SPEC>,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub dlenr: crate::Reg<dlenr::DLENR_SPEC>,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub dctrl: crate::Reg<dctrl::DCTRL_SPEC>,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub dcntr: crate::Reg<dcntr::DCNTR_SPEC>,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub star: crate::Reg<star::STAR_SPEC>,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub maskr: crate::Reg<maskr::MASKR_SPEC>,
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    pub acktimer: crate::Reg<acktimer::ACKTIMER_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    pub idmactrlr: crate::Reg<idmactrlr::IDMACTRLR_SPEC>,
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    pub idmabsizer: crate::Reg<idmabsizer::IDMABSIZER_SPEC>,
    #[doc = "0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    pub idmabase0r: crate::Reg<idmabase0r::IDMABASE0R_SPEC>,
    #[doc = "0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    pub idmabase1r: crate::Reg<idmabase1r::IDMABASE1R_SPEC>,
    _reserved21: [u8; 0x20],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub fifor: crate::Reg<fifor::FIFOR_SPEC>,
    _reserved22: [u8; 0x0370],
    #[doc = "0x3f4 - SDMMC IP version register"]
    pub ver: crate::Reg<ver::VER_SPEC>,
    #[doc = "0x3f8 - SDMMC IP identification register"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "SDMMC power control register"]
pub mod power;
#[doc = "CLKCR register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
pub mod clkcr;
#[doc = "ARGR register accessor: an alias for `Reg<ARGR_SPEC>`"]
pub type ARGR = crate::Reg<argr::ARGR_SPEC>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod argr;
#[doc = "CMDR register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmdr;
#[doc = "RESP1R register accessor: an alias for `Reg<RESP1R_SPEC>`"]
pub type RESP1R = crate::Reg<resp1r::RESP1R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp1r;
#[doc = "RESP2R register accessor: an alias for `Reg<RESP2R_SPEC>`"]
pub type RESP2R = crate::Reg<resp2r::RESP2R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp2r;
#[doc = "RESP3R register accessor: an alias for `Reg<RESP3R_SPEC>`"]
pub type RESP3R = crate::Reg<resp3r::RESP3R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp3r;
#[doc = "RESP4R register accessor: an alias for `Reg<RESP4R_SPEC>`"]
pub type RESP4R = crate::Reg<resp4r::RESP4R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp4r;
#[doc = "DTIMER register accessor: an alias for `Reg<DTIMER_SPEC>`"]
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "DLENR register accessor: an alias for `Reg<DLENR_SPEC>`"]
pub type DLENR = crate::Reg<dlenr::DLENR_SPEC>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlenr;
#[doc = "DCTRL register accessor: an alias for `Reg<DCTRL_SPEC>`"]
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "DCNTR register accessor: an alias for `Reg<DCNTR_SPEC>`"]
pub type DCNTR = crate::Reg<dcntr::DCNTR_SPEC>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcntr;
#[doc = "STAR register accessor: an alias for `Reg<STAR_SPEC>`"]
pub type STAR = crate::Reg<star::STAR_SPEC>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod star;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "MASKR register accessor: an alias for `Reg<MASKR_SPEC>`"]
pub type MASKR = crate::Reg<maskr::MASKR_SPEC>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod maskr;
#[doc = "ACKTIMER register accessor: an alias for `Reg<ACKTIMER_SPEC>`"]
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMER_SPEC>;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod acktimer;
#[doc = "IDMACTRLR register accessor: an alias for `Reg<IDMACTRLR_SPEC>`"]
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLR_SPEC>;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod idmactrlr;
#[doc = "IDMABSIZER register accessor: an alias for `Reg<IDMABSIZER_SPEC>`"]
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZER_SPEC>;
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
pub mod idmabsizer;
#[doc = "IDMABASE0R register accessor: an alias for `Reg<IDMABASE0R_SPEC>`"]
pub type IDMABASE0R = crate::Reg<idmabase0r::IDMABASE0R_SPEC>;
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
pub mod idmabase0r;
#[doc = "IDMABASE1R register accessor: an alias for `Reg<IDMABASE1R_SPEC>`"]
pub type IDMABASE1R = crate::Reg<idmabase1r::IDMABASE1R_SPEC>;
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
pub mod idmabase1r;
#[doc = "FIFOR register accessor: an alias for `Reg<FIFOR_SPEC>`"]
pub type FIFOR = crate::Reg<fifor::FIFOR_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifor;
#[doc = "VER register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "SDMMC IP version register"]
pub mod ver;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "SDMMC IP identification register"]
pub mod id;
#[doc = "RESPCMDR register accessor: an alias for `Reg<RESPCMDR_SPEC>`"]
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDR_SPEC>;
#[doc = "SDMMC command response register"]
pub mod respcmdr;
