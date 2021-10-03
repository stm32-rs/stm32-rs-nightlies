#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub sdmmc_power: crate::Reg<sdmmc_power::SDMMC_POWER_SPEC>,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
    pub sdmmc_clkcr: crate::Reg<sdmmc_clkcr::SDMMC_CLKCR_SPEC>,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub sdmmc_argr: crate::Reg<sdmmc_argr::SDMMC_ARGR_SPEC>,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub sdmmc_cmdr: crate::Reg<sdmmc_cmdr::SDMMC_CMDR_SPEC>,
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
    pub sdmmc_respcmdr: crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDR_SPEC>,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp1r: crate::Reg<sdmmc_resp1r::SDMMC_RESP1R_SPEC>,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp2r: crate::Reg<sdmmc_resp2r::SDMMC_RESP2R_SPEC>,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp3r: crate::Reg<sdmmc_resp3r::SDMMC_RESP3R_SPEC>,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp4r: crate::Reg<sdmmc_resp4r::SDMMC_RESP4R_SPEC>,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub sdmmc_dtimer: crate::Reg<sdmmc_dtimer::SDMMC_DTIMER_SPEC>,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub sdmmc_dlenr: crate::Reg<sdmmc_dlenr::SDMMC_DLENR_SPEC>,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub sdmmc_dctrl: crate::Reg<sdmmc_dctrl::SDMMC_DCTRL_SPEC>,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub sdmmc_dcntr: crate::Reg<sdmmc_dcntr::SDMMC_DCNTR_SPEC>,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub sdmmc_star: crate::Reg<sdmmc_star::SDMMC_STAR_SPEC>,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub sdmmc_icr: crate::Reg<sdmmc_icr::SDMMC_ICR_SPEC>,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub sdmmc_maskr: crate::Reg<sdmmc_maskr::SDMMC_MASKR_SPEC>,
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    pub sdmmc_acktimer: crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMER_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    pub sdmmc_idmactrlr: crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLR_SPEC>,
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
    pub sdmmc_idmabsizer: crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZER_SPEC>,
    #[doc = "0x58 - The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
    pub sdmmc_idmabaser: crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASER_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0x64 - SDMMC IDMA linked list address register"]
    pub sdmmc_idmalar: crate::Reg<sdmmc_idmalar::SDMMC_IDMALAR_SPEC>,
    #[doc = "0x68 - SDMMC IDMA linked list memory base register"]
    pub sdmmc_idmabar: crate::Reg<sdmmc_idmabar::SDMMC_IDMABAR_SPEC>,
    _reserved22: [u8; 0x14],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor0: crate::Reg<sdmmc_fifor0::SDMMC_FIFOR0_SPEC>,
    #[doc = "0x84 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor1: crate::Reg<sdmmc_fifor1::SDMMC_FIFOR1_SPEC>,
    #[doc = "0x88 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor2: crate::Reg<sdmmc_fifor2::SDMMC_FIFOR2_SPEC>,
    #[doc = "0x8c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor3: crate::Reg<sdmmc_fifor3::SDMMC_FIFOR3_SPEC>,
    #[doc = "0x90 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor4: crate::Reg<sdmmc_fifor4::SDMMC_FIFOR4_SPEC>,
    #[doc = "0x94 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor5: crate::Reg<sdmmc_fifor5::SDMMC_FIFOR5_SPEC>,
    #[doc = "0x98 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor6: crate::Reg<sdmmc_fifor6::SDMMC_FIFOR6_SPEC>,
    #[doc = "0x9c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor7: crate::Reg<sdmmc_fifor7::SDMMC_FIFOR7_SPEC>,
    #[doc = "0xa0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor8: crate::Reg<sdmmc_fifor8::SDMMC_FIFOR8_SPEC>,
    #[doc = "0xa4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor9: crate::Reg<sdmmc_fifor9::SDMMC_FIFOR9_SPEC>,
    #[doc = "0xa8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor10: crate::Reg<sdmmc_fifor10::SDMMC_FIFOR10_SPEC>,
    #[doc = "0xac - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor11: crate::Reg<sdmmc_fifor11::SDMMC_FIFOR11_SPEC>,
    #[doc = "0xb0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor12: crate::Reg<sdmmc_fifor12::SDMMC_FIFOR12_SPEC>,
    #[doc = "0xb4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor13: crate::Reg<sdmmc_fifor13::SDMMC_FIFOR13_SPEC>,
    #[doc = "0xb8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor14: crate::Reg<sdmmc_fifor14::SDMMC_FIFOR14_SPEC>,
    #[doc = "0xbc - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor15: crate::Reg<sdmmc_fifor15::SDMMC_FIFOR15_SPEC>,
    _reserved38: [u8; 0x0334],
    #[doc = "0x3f4 - SDMMC version register"]
    pub sdmmc_verr: crate::Reg<sdmmc_verr::SDMMC_VERR_SPEC>,
    #[doc = "0x3f8 - SDMMC identification register"]
    pub sdmmc_ipidr: crate::Reg<sdmmc_ipidr::SDMMC_IPIDR_SPEC>,
    #[doc = "0x3fc - SDMMC size ID register"]
    pub sdmmc_sidr: crate::Reg<sdmmc_sidr::SDMMC_SIDR_SPEC>,
}
#[doc = "SDMMC_POWER register accessor: an alias for `Reg<SDMMC_POWER_SPEC>`"]
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWER_SPEC>;
#[doc = "SDMMC power control register"]
pub mod sdmmc_power;
#[doc = "SDMMC_CLKCR register accessor: an alias for `Reg<SDMMC_CLKCR_SPEC>`"]
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCR_SPEC>;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
pub mod sdmmc_clkcr;
#[doc = "SDMMC_ARGR register accessor: an alias for `Reg<SDMMC_ARGR_SPEC>`"]
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGR_SPEC>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod sdmmc_argr;
#[doc = "SDMMC_CMDR register accessor: an alias for `Reg<SDMMC_CMDR_SPEC>`"]
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDR_SPEC>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod sdmmc_cmdr;
#[doc = "SDMMC_RESPCMDR register accessor: an alias for `Reg<SDMMC_RESPCMDR_SPEC>`"]
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDR_SPEC>;
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod sdmmc_respcmdr;
#[doc = "SDMMC_RESP1R register accessor: an alias for `Reg<SDMMC_RESP1R_SPEC>`"]
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp1r;
#[doc = "SDMMC_RESP2R register accessor: an alias for `Reg<SDMMC_RESP2R_SPEC>`"]
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp2r;
#[doc = "SDMMC_RESP3R register accessor: an alias for `Reg<SDMMC_RESP3R_SPEC>`"]
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp3r;
#[doc = "SDMMC_RESP4R register accessor: an alias for `Reg<SDMMC_RESP4R_SPEC>`"]
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp4r;
#[doc = "SDMMC_DTIMER register accessor: an alias for `Reg<SDMMC_DTIMER_SPEC>`"]
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMER_SPEC>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod sdmmc_dtimer;
#[doc = "SDMMC_DLENR register accessor: an alias for `Reg<SDMMC_DLENR_SPEC>`"]
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENR_SPEC>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod sdmmc_dlenr;
#[doc = "SDMMC_DCTRL register accessor: an alias for `Reg<SDMMC_DCTRL_SPEC>`"]
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRL_SPEC>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod sdmmc_dctrl;
#[doc = "SDMMC_DCNTR register accessor: an alias for `Reg<SDMMC_DCNTR_SPEC>`"]
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTR_SPEC>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod sdmmc_dcntr;
#[doc = "SDMMC_STAR register accessor: an alias for `Reg<SDMMC_STAR_SPEC>`"]
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STAR_SPEC>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sdmmc_star;
#[doc = "SDMMC_ICR register accessor: an alias for `Reg<SDMMC_ICR_SPEC>`"]
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICR_SPEC>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod sdmmc_icr;
#[doc = "SDMMC_MASKR register accessor: an alias for `Reg<SDMMC_MASKR_SPEC>`"]
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKR_SPEC>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod sdmmc_maskr;
#[doc = "SDMMC_ACKTIMER register accessor: an alias for `Reg<SDMMC_ACKTIMER_SPEC>`"]
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMER_SPEC>;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod sdmmc_acktimer;
#[doc = "SDMMC_IDMACTRLR register accessor: an alias for `Reg<SDMMC_IDMACTRLR_SPEC>`"]
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLR_SPEC>;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod sdmmc_idmactrlr;
#[doc = "SDMMC_IDMABSIZER register accessor: an alias for `Reg<SDMMC_IDMABSIZER_SPEC>`"]
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZER_SPEC>;
#[doc = "The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
pub mod sdmmc_idmabsizer;
#[doc = "SDMMC_IDMABASER register accessor: an alias for `Reg<SDMMC_IDMABASER_SPEC>`"]
pub type SDMMC_IDMABASER = crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASER_SPEC>;
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
pub mod sdmmc_idmabaser;
#[doc = "SDMMC_IDMALAR register accessor: an alias for `Reg<SDMMC_IDMALAR_SPEC>`"]
pub type SDMMC_IDMALAR = crate::Reg<sdmmc_idmalar::SDMMC_IDMALAR_SPEC>;
#[doc = "SDMMC IDMA linked list address register"]
pub mod sdmmc_idmalar;
#[doc = "SDMMC_IDMABAR register accessor: an alias for `Reg<SDMMC_IDMABAR_SPEC>`"]
pub type SDMMC_IDMABAR = crate::Reg<sdmmc_idmabar::SDMMC_IDMABAR_SPEC>;
#[doc = "SDMMC IDMA linked list memory base register"]
pub mod sdmmc_idmabar;
#[doc = "SDMMC_FIFOR0 register accessor: an alias for `Reg<SDMMC_FIFOR0_SPEC>`"]
pub type SDMMC_FIFOR0 = crate::Reg<sdmmc_fifor0::SDMMC_FIFOR0_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor0;
#[doc = "SDMMC_FIFOR1 register accessor: an alias for `Reg<SDMMC_FIFOR1_SPEC>`"]
pub type SDMMC_FIFOR1 = crate::Reg<sdmmc_fifor1::SDMMC_FIFOR1_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor1;
#[doc = "SDMMC_FIFOR2 register accessor: an alias for `Reg<SDMMC_FIFOR2_SPEC>`"]
pub type SDMMC_FIFOR2 = crate::Reg<sdmmc_fifor2::SDMMC_FIFOR2_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor2;
#[doc = "SDMMC_FIFOR3 register accessor: an alias for `Reg<SDMMC_FIFOR3_SPEC>`"]
pub type SDMMC_FIFOR3 = crate::Reg<sdmmc_fifor3::SDMMC_FIFOR3_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor3;
#[doc = "SDMMC_FIFOR4 register accessor: an alias for `Reg<SDMMC_FIFOR4_SPEC>`"]
pub type SDMMC_FIFOR4 = crate::Reg<sdmmc_fifor4::SDMMC_FIFOR4_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor4;
#[doc = "SDMMC_FIFOR5 register accessor: an alias for `Reg<SDMMC_FIFOR5_SPEC>`"]
pub type SDMMC_FIFOR5 = crate::Reg<sdmmc_fifor5::SDMMC_FIFOR5_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor5;
#[doc = "SDMMC_FIFOR6 register accessor: an alias for `Reg<SDMMC_FIFOR6_SPEC>`"]
pub type SDMMC_FIFOR6 = crate::Reg<sdmmc_fifor6::SDMMC_FIFOR6_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor6;
#[doc = "SDMMC_FIFOR7 register accessor: an alias for `Reg<SDMMC_FIFOR7_SPEC>`"]
pub type SDMMC_FIFOR7 = crate::Reg<sdmmc_fifor7::SDMMC_FIFOR7_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor7;
#[doc = "SDMMC_FIFOR8 register accessor: an alias for `Reg<SDMMC_FIFOR8_SPEC>`"]
pub type SDMMC_FIFOR8 = crate::Reg<sdmmc_fifor8::SDMMC_FIFOR8_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor8;
#[doc = "SDMMC_FIFOR9 register accessor: an alias for `Reg<SDMMC_FIFOR9_SPEC>`"]
pub type SDMMC_FIFOR9 = crate::Reg<sdmmc_fifor9::SDMMC_FIFOR9_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor9;
#[doc = "SDMMC_FIFOR10 register accessor: an alias for `Reg<SDMMC_FIFOR10_SPEC>`"]
pub type SDMMC_FIFOR10 = crate::Reg<sdmmc_fifor10::SDMMC_FIFOR10_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor10;
#[doc = "SDMMC_FIFOR11 register accessor: an alias for `Reg<SDMMC_FIFOR11_SPEC>`"]
pub type SDMMC_FIFOR11 = crate::Reg<sdmmc_fifor11::SDMMC_FIFOR11_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor11;
#[doc = "SDMMC_FIFOR12 register accessor: an alias for `Reg<SDMMC_FIFOR12_SPEC>`"]
pub type SDMMC_FIFOR12 = crate::Reg<sdmmc_fifor12::SDMMC_FIFOR12_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor12;
#[doc = "SDMMC_FIFOR13 register accessor: an alias for `Reg<SDMMC_FIFOR13_SPEC>`"]
pub type SDMMC_FIFOR13 = crate::Reg<sdmmc_fifor13::SDMMC_FIFOR13_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor13;
#[doc = "SDMMC_FIFOR14 register accessor: an alias for `Reg<SDMMC_FIFOR14_SPEC>`"]
pub type SDMMC_FIFOR14 = crate::Reg<sdmmc_fifor14::SDMMC_FIFOR14_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor14;
#[doc = "SDMMC_FIFOR15 register accessor: an alias for `Reg<SDMMC_FIFOR15_SPEC>`"]
pub type SDMMC_FIFOR15 = crate::Reg<sdmmc_fifor15::SDMMC_FIFOR15_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor15;
#[doc = "SDMMC_VERR register accessor: an alias for `Reg<SDMMC_VERR_SPEC>`"]
pub type SDMMC_VERR = crate::Reg<sdmmc_verr::SDMMC_VERR_SPEC>;
#[doc = "SDMMC version register"]
pub mod sdmmc_verr;
#[doc = "SDMMC_IPIDR register accessor: an alias for `Reg<SDMMC_IPIDR_SPEC>`"]
pub type SDMMC_IPIDR = crate::Reg<sdmmc_ipidr::SDMMC_IPIDR_SPEC>;
#[doc = "SDMMC identification register"]
pub mod sdmmc_ipidr;
#[doc = "SDMMC_SIDR register accessor: an alias for `Reg<SDMMC_SIDR_SPEC>`"]
pub type SDMMC_SIDR = crate::Reg<sdmmc_sidr::SDMMC_SIDR_SPEC>;
#[doc = "SDMMC size ID register"]
pub mod sdmmc_sidr;
