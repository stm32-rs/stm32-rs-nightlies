#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub sdmmc_power: SDMMC_POWER,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    pub sdmmc_clkcr: SDMMC_CLKCR,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub sdmmc_argr: SDMMC_ARGR,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub sdmmc_cmdr: SDMMC_CMDR,
    #[doc = "0x10 - SDMMC command response register"]
    pub sdmmc_respcmdr: SDMMC_RESPCMDR,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp1r: SDMMC_RESP1R,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp2r: SDMMC_RESP2R,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp3r: SDMMC_RESP3R,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp4r: SDMMC_RESP4R,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub sdmmc_dtimer: SDMMC_DTIMER,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub sdmmc_dlenr: SDMMC_DLENR,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub sdmmc_dctrl: SDMMC_DCTRL,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub sdmmc_dcntr: SDMMC_DCNTR,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub sdmmc_star: SDMMC_STAR,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub sdmmc_icr: SDMMC_ICR,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub sdmmc_maskr: SDMMC_MASKR,
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    pub sdmmc_acktimer: SDMMC_ACKTIMER,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    pub sdmmc_idmactrlr: SDMMC_IDMACTRLR,
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    pub sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    #[doc = "0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    pub sdmmc_idmabase0r: SDMMC_IDMABASE0R,
    #[doc = "0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    pub sdmmc_idmabase1r: SDMMC_IDMABASE1R,
    _reserved21: [u8; 32usize],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor: SDMMC_FIFOR,
    _reserved22: [u8; 880usize],
    #[doc = "0x3f4 - SDMMC IP version register"]
    pub sdmmc_ver: SDMMC_VER,
    #[doc = "0x3f8 - SDMMC IP identification register"]
    pub sdmmc_id: SDMMC_ID,
}
#[doc = "SDMMC power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_power](sdmmc_power) module"]
pub type SDMMC_POWER = crate::Reg<u32, _SDMMC_POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_POWER;
#[doc = "`read()` method returns [sdmmc_power::R](sdmmc_power::R) reader structure"]
impl crate::Readable for SDMMC_POWER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_power::W](sdmmc_power::W) writer structure"]
impl crate::Writable for SDMMC_POWER {}
#[doc = "SDMMC power control register"]
pub mod sdmmc_power;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_clkcr](sdmmc_clkcr) module"]
pub type SDMMC_CLKCR = crate::Reg<u32, _SDMMC_CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_CLKCR;
#[doc = "`read()` method returns [sdmmc_clkcr::R](sdmmc_clkcr::R) reader structure"]
impl crate::Readable for SDMMC_CLKCR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_clkcr::W](sdmmc_clkcr::W) writer structure"]
impl crate::Writable for SDMMC_CLKCR {}
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
pub mod sdmmc_clkcr;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_argr](sdmmc_argr) module"]
pub type SDMMC_ARGR = crate::Reg<u32, _SDMMC_ARGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_ARGR;
#[doc = "`read()` method returns [sdmmc_argr::R](sdmmc_argr::R) reader structure"]
impl crate::Readable for SDMMC_ARGR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_argr::W](sdmmc_argr::W) writer structure"]
impl crate::Writable for SDMMC_ARGR {}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod sdmmc_argr;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_cmdr](sdmmc_cmdr) module"]
pub type SDMMC_CMDR = crate::Reg<u32, _SDMMC_CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_CMDR;
#[doc = "`read()` method returns [sdmmc_cmdr::R](sdmmc_cmdr::R) reader structure"]
impl crate::Readable for SDMMC_CMDR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_cmdr::W](sdmmc_cmdr::W) writer structure"]
impl crate::Writable for SDMMC_CMDR {}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod sdmmc_cmdr;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp1r](sdmmc_resp1r) module"]
pub type SDMMC_RESP1R = crate::Reg<u32, _SDMMC_RESP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESP1R;
#[doc = "`read()` method returns [sdmmc_resp1r::R](sdmmc_resp1r::R) reader structure"]
impl crate::Readable for SDMMC_RESP1R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp1r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp2r](sdmmc_resp2r) module"]
pub type SDMMC_RESP2R = crate::Reg<u32, _SDMMC_RESP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESP2R;
#[doc = "`read()` method returns [sdmmc_resp2r::R](sdmmc_resp2r::R) reader structure"]
impl crate::Readable for SDMMC_RESP2R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp2r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp3r](sdmmc_resp3r) module"]
pub type SDMMC_RESP3R = crate::Reg<u32, _SDMMC_RESP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESP3R;
#[doc = "`read()` method returns [sdmmc_resp3r::R](sdmmc_resp3r::R) reader structure"]
impl crate::Readable for SDMMC_RESP3R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp3r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_resp4r](sdmmc_resp4r) module"]
pub type SDMMC_RESP4R = crate::Reg<u32, _SDMMC_RESP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESP4R;
#[doc = "`read()` method returns [sdmmc_resp4r::R](sdmmc_resp4r::R) reader structure"]
impl crate::Readable for SDMMC_RESP4R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp4r;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dtimer](sdmmc_dtimer) module"]
pub type SDMMC_DTIMER = crate::Reg<u32, _SDMMC_DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_DTIMER;
#[doc = "`read()` method returns [sdmmc_dtimer::R](sdmmc_dtimer::R) reader structure"]
impl crate::Readable for SDMMC_DTIMER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_dtimer::W](sdmmc_dtimer::W) writer structure"]
impl crate::Writable for SDMMC_DTIMER {}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod sdmmc_dtimer;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dlenr](sdmmc_dlenr) module"]
pub type SDMMC_DLENR = crate::Reg<u32, _SDMMC_DLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_DLENR;
#[doc = "`read()` method returns [sdmmc_dlenr::R](sdmmc_dlenr::R) reader structure"]
impl crate::Readable for SDMMC_DLENR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_dlenr::W](sdmmc_dlenr::W) writer structure"]
impl crate::Writable for SDMMC_DLENR {}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod sdmmc_dlenr;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dctrl](sdmmc_dctrl) module"]
pub type SDMMC_DCTRL = crate::Reg<u32, _SDMMC_DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_DCTRL;
#[doc = "`read()` method returns [sdmmc_dctrl::R](sdmmc_dctrl::R) reader structure"]
impl crate::Readable for SDMMC_DCTRL {}
#[doc = "`write(|w| ..)` method takes [sdmmc_dctrl::W](sdmmc_dctrl::W) writer structure"]
impl crate::Writable for SDMMC_DCTRL {}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod sdmmc_dctrl;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dcntr](sdmmc_dcntr) module"]
pub type SDMMC_DCNTR = crate::Reg<u32, _SDMMC_DCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_DCNTR;
#[doc = "`read()` method returns [sdmmc_dcntr::R](sdmmc_dcntr::R) reader structure"]
impl crate::Readable for SDMMC_DCNTR {}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod sdmmc_dcntr;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_star](sdmmc_star) module"]
pub type SDMMC_STAR = crate::Reg<u32, _SDMMC_STAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_STAR;
#[doc = "`read()` method returns [sdmmc_star::R](sdmmc_star::R) reader structure"]
impl crate::Readable for SDMMC_STAR {}
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sdmmc_star;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_icr](sdmmc_icr) module"]
pub type SDMMC_ICR = crate::Reg<u32, _SDMMC_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_ICR;
#[doc = "`read()` method returns [sdmmc_icr::R](sdmmc_icr::R) reader structure"]
impl crate::Readable for SDMMC_ICR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_icr::W](sdmmc_icr::W) writer structure"]
impl crate::Writable for SDMMC_ICR {}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod sdmmc_icr;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_maskr](sdmmc_maskr) module"]
pub type SDMMC_MASKR = crate::Reg<u32, _SDMMC_MASKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_MASKR;
#[doc = "`read()` method returns [sdmmc_maskr::R](sdmmc_maskr::R) reader structure"]
impl crate::Readable for SDMMC_MASKR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_maskr::W](sdmmc_maskr::W) writer structure"]
impl crate::Writable for SDMMC_MASKR {}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod sdmmc_maskr;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_acktimer](sdmmc_acktimer) module"]
pub type SDMMC_ACKTIMER = crate::Reg<u32, _SDMMC_ACKTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_ACKTIMER;
#[doc = "`read()` method returns [sdmmc_acktimer::R](sdmmc_acktimer::R) reader structure"]
impl crate::Readable for SDMMC_ACKTIMER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_acktimer::W](sdmmc_acktimer::W) writer structure"]
impl crate::Writable for SDMMC_ACKTIMER {}
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod sdmmc_acktimer;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmactrlr](sdmmc_idmactrlr) module"]
pub type SDMMC_IDMACTRLR = crate::Reg<u32, _SDMMC_IDMACTRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMACTRLR;
#[doc = "`read()` method returns [sdmmc_idmactrlr::R](sdmmc_idmactrlr::R) reader structure"]
impl crate::Readable for SDMMC_IDMACTRLR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmactrlr::W](sdmmc_idmactrlr::W) writer structure"]
impl crate::Writable for SDMMC_IDMACTRLR {}
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod sdmmc_idmactrlr;
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabsizer](sdmmc_idmabsizer) module"]
pub type SDMMC_IDMABSIZER = crate::Reg<u32, _SDMMC_IDMABSIZER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABSIZER;
#[doc = "`read()` method returns [sdmmc_idmabsizer::R](sdmmc_idmabsizer::R) reader structure"]
impl crate::Readable for SDMMC_IDMABSIZER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabsizer::W](sdmmc_idmabsizer::W) writer structure"]
impl crate::Writable for SDMMC_IDMABSIZER {}
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
pub mod sdmmc_idmabsizer;
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabase0r](sdmmc_idmabase0r) module"]
pub type SDMMC_IDMABASE0R = crate::Reg<u32, _SDMMC_IDMABASE0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABASE0R;
#[doc = "`read()` method returns [sdmmc_idmabase0r::R](sdmmc_idmabase0r::R) reader structure"]
impl crate::Readable for SDMMC_IDMABASE0R {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabase0r::W](sdmmc_idmabase0r::W) writer structure"]
impl crate::Writable for SDMMC_IDMABASE0R {}
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
pub mod sdmmc_idmabase0r;
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabase1r](sdmmc_idmabase1r) module"]
pub type SDMMC_IDMABASE1R = crate::Reg<u32, _SDMMC_IDMABASE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABASE1R;
#[doc = "`read()` method returns [sdmmc_idmabase1r::R](sdmmc_idmabase1r::R) reader structure"]
impl crate::Readable for SDMMC_IDMABASE1R {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabase1r::W](sdmmc_idmabase1r::W) writer structure"]
impl crate::Writable for SDMMC_IDMABASE1R {}
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
pub mod sdmmc_idmabase1r;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor](sdmmc_fifor) module"]
pub type SDMMC_FIFOR = crate::Reg<u32, _SDMMC_FIFOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR;
#[doc = "`read()` method returns [sdmmc_fifor::R](sdmmc_fifor::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor::W](sdmmc_fifor::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor;
#[doc = "SDMMC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_ver](sdmmc_ver) module"]
pub type SDMMC_VER = crate::Reg<u32, _SDMMC_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_VER;
#[doc = "`read()` method returns [sdmmc_ver::R](sdmmc_ver::R) reader structure"]
impl crate::Readable for SDMMC_VER {}
#[doc = "SDMMC IP version register"]
pub mod sdmmc_ver;
#[doc = "SDMMC IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_id](sdmmc_id) module"]
pub type SDMMC_ID = crate::Reg<u32, _SDMMC_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_ID;
#[doc = "`read()` method returns [sdmmc_id::R](sdmmc_id::R) reader structure"]
impl crate::Readable for SDMMC_ID {}
#[doc = "SDMMC IP identification register"]
pub mod sdmmc_id;
#[doc = "SDMMC command response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_respcmdr](sdmmc_respcmdr) module"]
pub type SDMMC_RESPCMDR = crate::Reg<u32, _SDMMC_RESPCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESPCMDR;
#[doc = "`read()` method returns [sdmmc_respcmdr::R](sdmmc_respcmdr::R) reader structure"]
impl crate::Readable for SDMMC_RESPCMDR {}
#[doc = "SDMMC command response register"]
pub mod sdmmc_respcmdr;
