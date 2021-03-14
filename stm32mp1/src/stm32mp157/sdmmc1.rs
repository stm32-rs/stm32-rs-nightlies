#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub sdmmc_power: SDMMC_POWER,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
    pub sdmmc_clkcr: SDMMC_CLKCR,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub sdmmc_argr: SDMMC_ARGR,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub sdmmc_cmdr: SDMMC_CMDR,
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
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
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
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
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
    pub sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    #[doc = "0x58 - The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
    pub sdmmc_idmabaser: SDMMC_IDMABASER,
    _reserved20: [u8; 8usize],
    #[doc = "0x64 - SDMMC IDMA linked list address register"]
    pub sdmmc_idmalar: SDMMC_IDMALAR,
    #[doc = "0x68 - SDMMC IDMA linked list memory base register"]
    pub sdmmc_idmabar: SDMMC_IDMABAR,
    _reserved22: [u8; 20usize],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor0: SDMMC_FIFOR0,
    #[doc = "0x84 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor1: SDMMC_FIFOR1,
    #[doc = "0x88 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor2: SDMMC_FIFOR2,
    #[doc = "0x8c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor3: SDMMC_FIFOR3,
    #[doc = "0x90 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor4: SDMMC_FIFOR4,
    #[doc = "0x94 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor5: SDMMC_FIFOR5,
    #[doc = "0x98 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor6: SDMMC_FIFOR6,
    #[doc = "0x9c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor7: SDMMC_FIFOR7,
    #[doc = "0xa0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor8: SDMMC_FIFOR8,
    #[doc = "0xa4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor9: SDMMC_FIFOR9,
    #[doc = "0xa8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor10: SDMMC_FIFOR10,
    #[doc = "0xac - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor11: SDMMC_FIFOR11,
    #[doc = "0xb0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor12: SDMMC_FIFOR12,
    #[doc = "0xb4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor13: SDMMC_FIFOR13,
    #[doc = "0xb8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor14: SDMMC_FIFOR14,
    #[doc = "0xbc - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor15: SDMMC_FIFOR15,
    _reserved38: [u8; 820usize],
    #[doc = "0x3f4 - SDMMC version register"]
    pub sdmmc_verr: SDMMC_VERR,
    #[doc = "0x3f8 - SDMMC identification register"]
    pub sdmmc_ipidr: SDMMC_IPIDR,
    #[doc = "0x3fc - SDMMC size ID register"]
    pub sdmmc_sidr: SDMMC_SIDR,
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
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_clkcr](sdmmc_clkcr) module"]
pub type SDMMC_CLKCR = crate::Reg<u32, _SDMMC_CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_CLKCR;
#[doc = "`read()` method returns [sdmmc_clkcr::R](sdmmc_clkcr::R) reader structure"]
impl crate::Readable for SDMMC_CLKCR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_clkcr::W](sdmmc_clkcr::W) writer structure"]
impl crate::Writable for SDMMC_CLKCR {}
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
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
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_respcmdr](sdmmc_respcmdr) module"]
pub type SDMMC_RESPCMDR = crate::Reg<u32, _SDMMC_RESPCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_RESPCMDR;
#[doc = "`read()` method returns [sdmmc_respcmdr::R](sdmmc_respcmdr::R) reader structure"]
impl crate::Readable for SDMMC_RESPCMDR {}
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod sdmmc_respcmdr;
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
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_star](sdmmc_star) module"]
pub type SDMMC_STAR = crate::Reg<u32, _SDMMC_STAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_STAR;
#[doc = "`read()` method returns [sdmmc_star::R](sdmmc_star::R) reader structure"]
impl crate::Readable for SDMMC_STAR {}
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
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
#[doc = "The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabsizer](sdmmc_idmabsizer) module"]
pub type SDMMC_IDMABSIZER = crate::Reg<u32, _SDMMC_IDMABSIZER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABSIZER;
#[doc = "`read()` method returns [sdmmc_idmabsizer::R](sdmmc_idmabsizer::R) reader structure"]
impl crate::Readable for SDMMC_IDMABSIZER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabsizer::W](sdmmc_idmabsizer::W) writer structure"]
impl crate::Writable for SDMMC_IDMABSIZER {}
#[doc = "The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
pub mod sdmmc_idmabsizer;
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabaser](sdmmc_idmabaser) module"]
pub type SDMMC_IDMABASER = crate::Reg<u32, _SDMMC_IDMABASER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABASER;
#[doc = "`read()` method returns [sdmmc_idmabaser::R](sdmmc_idmabaser::R) reader structure"]
impl crate::Readable for SDMMC_IDMABASER {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabaser::W](sdmmc_idmabaser::W) writer structure"]
impl crate::Writable for SDMMC_IDMABASER {}
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
pub mod sdmmc_idmabaser;
#[doc = "SDMMC IDMA linked list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmalar](sdmmc_idmalar) module"]
pub type SDMMC_IDMALAR = crate::Reg<u32, _SDMMC_IDMALAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMALAR;
#[doc = "`read()` method returns [sdmmc_idmalar::R](sdmmc_idmalar::R) reader structure"]
impl crate::Readable for SDMMC_IDMALAR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmalar::W](sdmmc_idmalar::W) writer structure"]
impl crate::Writable for SDMMC_IDMALAR {}
#[doc = "SDMMC IDMA linked list address register"]
pub mod sdmmc_idmalar;
#[doc = "SDMMC IDMA linked list memory base register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmabar](sdmmc_idmabar) module"]
pub type SDMMC_IDMABAR = crate::Reg<u32, _SDMMC_IDMABAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IDMABAR;
#[doc = "`read()` method returns [sdmmc_idmabar::R](sdmmc_idmabar::R) reader structure"]
impl crate::Readable for SDMMC_IDMABAR {}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmabar::W](sdmmc_idmabar::W) writer structure"]
impl crate::Writable for SDMMC_IDMABAR {}
#[doc = "SDMMC IDMA linked list memory base register"]
pub mod sdmmc_idmabar;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor0](sdmmc_fifor0) module"]
pub type SDMMC_FIFOR0 = crate::Reg<u32, _SDMMC_FIFOR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR0;
#[doc = "`read()` method returns [sdmmc_fifor0::R](sdmmc_fifor0::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR0 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor0::W](sdmmc_fifor0::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR0 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor0;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor1](sdmmc_fifor1) module"]
pub type SDMMC_FIFOR1 = crate::Reg<u32, _SDMMC_FIFOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR1;
#[doc = "`read()` method returns [sdmmc_fifor1::R](sdmmc_fifor1::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR1 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor1::W](sdmmc_fifor1::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR1 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor1;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor2](sdmmc_fifor2) module"]
pub type SDMMC_FIFOR2 = crate::Reg<u32, _SDMMC_FIFOR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR2;
#[doc = "`read()` method returns [sdmmc_fifor2::R](sdmmc_fifor2::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR2 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor2::W](sdmmc_fifor2::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR2 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor2;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor3](sdmmc_fifor3) module"]
pub type SDMMC_FIFOR3 = crate::Reg<u32, _SDMMC_FIFOR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR3;
#[doc = "`read()` method returns [sdmmc_fifor3::R](sdmmc_fifor3::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR3 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor3::W](sdmmc_fifor3::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR3 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor3;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor4](sdmmc_fifor4) module"]
pub type SDMMC_FIFOR4 = crate::Reg<u32, _SDMMC_FIFOR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR4;
#[doc = "`read()` method returns [sdmmc_fifor4::R](sdmmc_fifor4::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR4 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor4::W](sdmmc_fifor4::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR4 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor4;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor5](sdmmc_fifor5) module"]
pub type SDMMC_FIFOR5 = crate::Reg<u32, _SDMMC_FIFOR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR5;
#[doc = "`read()` method returns [sdmmc_fifor5::R](sdmmc_fifor5::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR5 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor5::W](sdmmc_fifor5::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR5 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor5;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor6](sdmmc_fifor6) module"]
pub type SDMMC_FIFOR6 = crate::Reg<u32, _SDMMC_FIFOR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR6;
#[doc = "`read()` method returns [sdmmc_fifor6::R](sdmmc_fifor6::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR6 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor6::W](sdmmc_fifor6::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR6 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor6;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor7](sdmmc_fifor7) module"]
pub type SDMMC_FIFOR7 = crate::Reg<u32, _SDMMC_FIFOR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR7;
#[doc = "`read()` method returns [sdmmc_fifor7::R](sdmmc_fifor7::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR7 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor7::W](sdmmc_fifor7::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR7 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor7;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor8](sdmmc_fifor8) module"]
pub type SDMMC_FIFOR8 = crate::Reg<u32, _SDMMC_FIFOR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR8;
#[doc = "`read()` method returns [sdmmc_fifor8::R](sdmmc_fifor8::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR8 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor8::W](sdmmc_fifor8::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR8 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor8;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor9](sdmmc_fifor9) module"]
pub type SDMMC_FIFOR9 = crate::Reg<u32, _SDMMC_FIFOR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR9;
#[doc = "`read()` method returns [sdmmc_fifor9::R](sdmmc_fifor9::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR9 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor9::W](sdmmc_fifor9::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR9 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor9;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor10](sdmmc_fifor10) module"]
pub type SDMMC_FIFOR10 = crate::Reg<u32, _SDMMC_FIFOR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR10;
#[doc = "`read()` method returns [sdmmc_fifor10::R](sdmmc_fifor10::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR10 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor10::W](sdmmc_fifor10::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR10 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor10;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor11](sdmmc_fifor11) module"]
pub type SDMMC_FIFOR11 = crate::Reg<u32, _SDMMC_FIFOR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR11;
#[doc = "`read()` method returns [sdmmc_fifor11::R](sdmmc_fifor11::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR11 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor11::W](sdmmc_fifor11::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR11 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor11;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor12](sdmmc_fifor12) module"]
pub type SDMMC_FIFOR12 = crate::Reg<u32, _SDMMC_FIFOR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR12;
#[doc = "`read()` method returns [sdmmc_fifor12::R](sdmmc_fifor12::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR12 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor12::W](sdmmc_fifor12::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR12 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor12;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor13](sdmmc_fifor13) module"]
pub type SDMMC_FIFOR13 = crate::Reg<u32, _SDMMC_FIFOR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR13;
#[doc = "`read()` method returns [sdmmc_fifor13::R](sdmmc_fifor13::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR13 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor13::W](sdmmc_fifor13::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR13 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor13;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor14](sdmmc_fifor14) module"]
pub type SDMMC_FIFOR14 = crate::Reg<u32, _SDMMC_FIFOR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR14;
#[doc = "`read()` method returns [sdmmc_fifor14::R](sdmmc_fifor14::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR14 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor14::W](sdmmc_fifor14::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR14 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor14;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_fifor15](sdmmc_fifor15) module"]
pub type SDMMC_FIFOR15 = crate::Reg<u32, _SDMMC_FIFOR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_FIFOR15;
#[doc = "`read()` method returns [sdmmc_fifor15::R](sdmmc_fifor15::R) reader structure"]
impl crate::Readable for SDMMC_FIFOR15 {}
#[doc = "`write(|w| ..)` method takes [sdmmc_fifor15::W](sdmmc_fifor15::W) writer structure"]
impl crate::Writable for SDMMC_FIFOR15 {}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor15;
#[doc = "SDMMC version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_verr](sdmmc_verr) module"]
pub type SDMMC_VERR = crate::Reg<u32, _SDMMC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_VERR;
#[doc = "`read()` method returns [sdmmc_verr::R](sdmmc_verr::R) reader structure"]
impl crate::Readable for SDMMC_VERR {}
#[doc = "SDMMC version register"]
pub mod sdmmc_verr;
#[doc = "SDMMC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_ipidr](sdmmc_ipidr) module"]
pub type SDMMC_IPIDR = crate::Reg<u32, _SDMMC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_IPIDR;
#[doc = "`read()` method returns [sdmmc_ipidr::R](sdmmc_ipidr::R) reader structure"]
impl crate::Readable for SDMMC_IPIDR {}
#[doc = "SDMMC identification register"]
pub mod sdmmc_ipidr;
#[doc = "SDMMC size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_sidr](sdmmc_sidr) module"]
pub type SDMMC_SIDR = crate::Reg<u32, _SDMMC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMMC_SIDR;
#[doc = "`read()` method returns [sdmmc_sidr::R](sdmmc_sidr::R) reader structure"]
impl crate::Readable for SDMMC_SIDR {}
#[doc = "SDMMC size ID register"]
pub mod sdmmc_sidr;
