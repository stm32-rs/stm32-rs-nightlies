#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub power: POWER,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    pub clkcr: CLKCR,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub argr: ARGR,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub cmdr: CMDR,
    #[doc = "0x10 - SDMMC command response register"]
    pub respcmdr: RESPCMDR,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp1r: RESP1R,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp2r: RESP2R,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp3r: RESP3R,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub resp4r: RESP4R,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub dtimer: DTIMER,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub dlenr: DLENR,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub dctrl: DCTRL,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub dcntr: DCNTR,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub star: STAR,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub icr: ICR,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub maskr: MASKR,
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    pub acktimer: ACKTIMER,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    pub idmactrlr: IDMACTRLR,
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    pub idmabsizer: IDMABSIZER,
    #[doc = "0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    pub idmabase0r: IDMABASE0R,
    #[doc = "0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    pub idmabase1r: IDMABASE1R,
    _reserved21: [u8; 32usize],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub fifor: FIFOR,
}
#[doc = "SDMMC power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "SDMMC power control register"]
pub mod power;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
pub mod clkcr;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argr](argr) module"]
pub type ARGR = crate::Reg<u32, _ARGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGR;
#[doc = "`read()` method returns [argr::R](argr::R) reader structure"]
impl crate::Readable for ARGR {}
#[doc = "`write(|w| ..)` method takes [argr::W](argr::W) writer structure"]
impl crate::Writable for ARGR {}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod argr;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`read()` method returns [cmdr::R](cmdr::R) reader structure"]
impl crate::Readable for CMDR {}
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmdr;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1r](resp1r) module"]
pub type RESP1R = crate::Reg<u32, _RESP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP1R;
#[doc = "`read()` method returns [resp1r::R](resp1r::R) reader structure"]
impl crate::Readable for RESP1R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp1r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2r](resp2r) module"]
pub type RESP2R = crate::Reg<u32, _RESP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2R;
#[doc = "`read()` method returns [resp2r::R](resp2r::R) reader structure"]
impl crate::Readable for RESP2R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp2r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3r](resp3r) module"]
pub type RESP3R = crate::Reg<u32, _RESP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3R;
#[doc = "`read()` method returns [resp3r::R](resp3r::R) reader structure"]
impl crate::Readable for RESP3R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp3r;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4r](resp4r) module"]
pub type RESP4R = crate::Reg<u32, _RESP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4R;
#[doc = "`read()` method returns [resp4r::R](resp4r::R) reader structure"]
impl crate::Readable for RESP4R {}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp4r;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](dtimer) module"]
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
#[doc = "`read()` method returns [dtimer::R](dtimer::R) reader structure"]
impl crate::Readable for DTIMER {}
#[doc = "`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure"]
impl crate::Writable for DTIMER {}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlenr](dlenr) module"]
pub type DLENR = crate::Reg<u32, _DLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLENR;
#[doc = "`read()` method returns [dlenr::R](dlenr::R) reader structure"]
impl crate::Readable for DLENR {}
#[doc = "`write(|w| ..)` method takes [dlenr::W](dlenr::W) writer structure"]
impl crate::Writable for DLENR {}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlenr;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](dctrl) module"]
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
#[doc = "`read()` method returns [dctrl::R](dctrl::R) reader structure"]
impl crate::Readable for DCTRL {}
#[doc = "`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure"]
impl crate::Writable for DCTRL {}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcntr](dcntr) module"]
pub type DCNTR = crate::Reg<u32, _DCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCNTR;
#[doc = "`read()` method returns [dcntr::R](dcntr::R) reader structure"]
impl crate::Readable for DCNTR {}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcntr;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](star) module"]
pub type STAR = crate::Reg<u32, _STAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAR;
#[doc = "`read()` method returns [star::R](star::R) reader structure"]
impl crate::Readable for STAR {}
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod star;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maskr](maskr) module"]
pub type MASKR = crate::Reg<u32, _MASKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKR;
#[doc = "`read()` method returns [maskr::R](maskr::R) reader structure"]
impl crate::Readable for MASKR {}
#[doc = "`write(|w| ..)` method takes [maskr::W](maskr::W) writer structure"]
impl crate::Writable for MASKR {}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod maskr;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acktimer](acktimer) module"]
pub type ACKTIMER = crate::Reg<u32, _ACKTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACKTIMER;
#[doc = "`read()` method returns [acktimer::R](acktimer::R) reader structure"]
impl crate::Readable for ACKTIMER {}
#[doc = "`write(|w| ..)` method takes [acktimer::W](acktimer::W) writer structure"]
impl crate::Writable for ACKTIMER {}
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod acktimer;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmactrlr](idmactrlr) module"]
pub type IDMACTRLR = crate::Reg<u32, _IDMACTRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMACTRLR;
#[doc = "`read()` method returns [idmactrlr::R](idmactrlr::R) reader structure"]
impl crate::Readable for IDMACTRLR {}
#[doc = "`write(|w| ..)` method takes [idmactrlr::W](idmactrlr::W) writer structure"]
impl crate::Writable for IDMACTRLR {}
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod idmactrlr;
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabsizer](idmabsizer) module"]
pub type IDMABSIZER = crate::Reg<u32, _IDMABSIZER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABSIZER;
#[doc = "`read()` method returns [idmabsizer::R](idmabsizer::R) reader structure"]
impl crate::Readable for IDMABSIZER {}
#[doc = "`write(|w| ..)` method takes [idmabsizer::W](idmabsizer::W) writer structure"]
impl crate::Writable for IDMABSIZER {}
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
pub mod idmabsizer;
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase0r](idmabase0r) module"]
pub type IDMABASE0R = crate::Reg<u32, _IDMABASE0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABASE0R;
#[doc = "`read()` method returns [idmabase0r::R](idmabase0r::R) reader structure"]
impl crate::Readable for IDMABASE0R {}
#[doc = "`write(|w| ..)` method takes [idmabase0r::W](idmabase0r::W) writer structure"]
impl crate::Writable for IDMABASE0R {}
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
pub mod idmabase0r;
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase1r](idmabase1r) module"]
pub type IDMABASE1R = crate::Reg<u32, _IDMABASE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABASE1R;
#[doc = "`read()` method returns [idmabase1r::R](idmabase1r::R) reader structure"]
impl crate::Readable for IDMABASE1R {}
#[doc = "`write(|w| ..)` method takes [idmabase1r::W](idmabase1r::W) writer structure"]
impl crate::Writable for IDMABASE1R {}
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
pub mod idmabase1r;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor](fifor) module"]
pub type FIFOR = crate::Reg<u32, _FIFOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR;
#[doc = "`read()` method returns [fifor::R](fifor::R) reader structure"]
impl crate::Readable for FIFOR {}
#[doc = "`write(|w| ..)` method takes [fifor::W](fifor::W) writer structure"]
impl crate::Writable for FIFOR {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifor;
#[doc = "SDMMC command response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmdr](respcmdr) module"]
pub type RESPCMDR = crate::Reg<u32, _RESPCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMDR;
#[doc = "`read()` method returns [respcmdr::R](respcmdr::R) reader structure"]
impl crate::Readable for RESPCMDR {}
#[doc = "SDMMC command response register"]
pub mod respcmdr;
