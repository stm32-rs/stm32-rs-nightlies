#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdmmc_power: SDMMC_POWER,
    sdmmc_clkcr: SDMMC_CLKCR,
    sdmmc_argr: SDMMC_ARGR,
    sdmmc_cmdr: SDMMC_CMDR,
    sdmmc_respcmdr: SDMMC_RESPCMDR,
    sdmmc_resp1r: SDMMC_RESP1R,
    sdmmc_resp2r: SDMMC_RESP2R,
    sdmmc_resp3r: SDMMC_RESP3R,
    sdmmc_resp4r: SDMMC_RESP4R,
    sdmmc_dtimer: SDMMC_DTIMER,
    sdmmc_dlenr: SDMMC_DLENR,
    sdmmc_dctrl: SDMMC_DCTRL,
    sdmmc_dcntr: SDMMC_DCNTR,
    sdmmc_star: SDMMC_STAR,
    sdmmc_icr: SDMMC_ICR,
    sdmmc_maskr: SDMMC_MASKR,
    sdmmc_acktimer: SDMMC_ACKTIMER,
    _reserved17: [u8; 0x0c],
    sdmmc_idmactrlr: SDMMC_IDMACTRLR,
    sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    sdmmc_idmabaser: SDMMC_IDMABASER,
    _reserved20: [u8; 0x08],
    sdmmc_idmalar: SDMMC_IDMALAR,
    sdmmc_idmabar: SDMMC_IDMABAR,
    _reserved22: [u8; 0x14],
    sdmmc_fifor0: SDMMC_FIFOR0,
    sdmmc_fifor1: SDMMC_FIFOR1,
    sdmmc_fifor2: SDMMC_FIFOR2,
    sdmmc_fifor3: SDMMC_FIFOR3,
    sdmmc_fifor4: SDMMC_FIFOR4,
    sdmmc_fifor5: SDMMC_FIFOR5,
    sdmmc_fifor6: SDMMC_FIFOR6,
    sdmmc_fifor7: SDMMC_FIFOR7,
    sdmmc_fifor8: SDMMC_FIFOR8,
    sdmmc_fifor9: SDMMC_FIFOR9,
    sdmmc_fifor10: SDMMC_FIFOR10,
    sdmmc_fifor11: SDMMC_FIFOR11,
    sdmmc_fifor12: SDMMC_FIFOR12,
    sdmmc_fifor13: SDMMC_FIFOR13,
    sdmmc_fifor14: SDMMC_FIFOR14,
    sdmmc_fifor15: SDMMC_FIFOR15,
    _reserved38: [u8; 0x0334],
    sdmmc_verr: SDMMC_VERR,
    sdmmc_ipidr: SDMMC_IPIDR,
    sdmmc_sidr: SDMMC_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    #[inline(always)]
    pub const fn sdmmc_power(&self) -> &SDMMC_POWER {
        &self.sdmmc_power
    }
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
    #[inline(always)]
    pub const fn sdmmc_clkcr(&self) -> &SDMMC_CLKCR {
        &self.sdmmc_clkcr
    }
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    #[inline(always)]
    pub const fn sdmmc_argr(&self) -> &SDMMC_ARGR {
        &self.sdmmc_argr
    }
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    #[inline(always)]
    pub const fn sdmmc_cmdr(&self) -> &SDMMC_CMDR {
        &self.sdmmc_cmdr
    }
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
    #[inline(always)]
    pub const fn sdmmc_respcmdr(&self) -> &SDMMC_RESPCMDR {
        &self.sdmmc_respcmdr
    }
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn sdmmc_resp1r(&self) -> &SDMMC_RESP1R {
        &self.sdmmc_resp1r
    }
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn sdmmc_resp2r(&self) -> &SDMMC_RESP2R {
        &self.sdmmc_resp2r
    }
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn sdmmc_resp3r(&self) -> &SDMMC_RESP3R {
        &self.sdmmc_resp3r
    }
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn sdmmc_resp4r(&self) -> &SDMMC_RESP4R {
        &self.sdmmc_resp4r
    }
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    #[inline(always)]
    pub const fn sdmmc_dtimer(&self) -> &SDMMC_DTIMER {
        &self.sdmmc_dtimer
    }
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    #[inline(always)]
    pub const fn sdmmc_dlenr(&self) -> &SDMMC_DLENR {
        &self.sdmmc_dlenr
    }
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    #[inline(always)]
    pub const fn sdmmc_dctrl(&self) -> &SDMMC_DCTRL {
        &self.sdmmc_dctrl
    }
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    #[inline(always)]
    pub const fn sdmmc_dcntr(&self) -> &SDMMC_DCNTR {
        &self.sdmmc_dcntr
    }
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    #[inline(always)]
    pub const fn sdmmc_star(&self) -> &SDMMC_STAR {
        &self.sdmmc_star
    }
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    #[inline(always)]
    pub const fn sdmmc_icr(&self) -> &SDMMC_ICR {
        &self.sdmmc_icr
    }
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    #[inline(always)]
    pub const fn sdmmc_maskr(&self) -> &SDMMC_MASKR {
        &self.sdmmc_maskr
    }
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    #[inline(always)]
    pub const fn sdmmc_acktimer(&self) -> &SDMMC_ACKTIMER {
        &self.sdmmc_acktimer
    }
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    #[inline(always)]
    pub const fn sdmmc_idmactrlr(&self) -> &SDMMC_IDMACTRLR {
        &self.sdmmc_idmactrlr
    }
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
    #[inline(always)]
    pub const fn sdmmc_idmabsizer(&self) -> &SDMMC_IDMABSIZER {
        &self.sdmmc_idmabsizer
    }
    #[doc = "0x58 - The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
    #[inline(always)]
    pub const fn sdmmc_idmabaser(&self) -> &SDMMC_IDMABASER {
        &self.sdmmc_idmabaser
    }
    #[doc = "0x64 - SDMMC IDMA linked list address register"]
    #[inline(always)]
    pub const fn sdmmc_idmalar(&self) -> &SDMMC_IDMALAR {
        &self.sdmmc_idmalar
    }
    #[doc = "0x68 - SDMMC IDMA linked list memory base register"]
    #[inline(always)]
    pub const fn sdmmc_idmabar(&self) -> &SDMMC_IDMABAR {
        &self.sdmmc_idmabar
    }
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor0(&self) -> &SDMMC_FIFOR0 {
        &self.sdmmc_fifor0
    }
    #[doc = "0x84 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor1(&self) -> &SDMMC_FIFOR1 {
        &self.sdmmc_fifor1
    }
    #[doc = "0x88 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor2(&self) -> &SDMMC_FIFOR2 {
        &self.sdmmc_fifor2
    }
    #[doc = "0x8c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor3(&self) -> &SDMMC_FIFOR3 {
        &self.sdmmc_fifor3
    }
    #[doc = "0x90 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor4(&self) -> &SDMMC_FIFOR4 {
        &self.sdmmc_fifor4
    }
    #[doc = "0x94 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor5(&self) -> &SDMMC_FIFOR5 {
        &self.sdmmc_fifor5
    }
    #[doc = "0x98 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor6(&self) -> &SDMMC_FIFOR6 {
        &self.sdmmc_fifor6
    }
    #[doc = "0x9c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor7(&self) -> &SDMMC_FIFOR7 {
        &self.sdmmc_fifor7
    }
    #[doc = "0xa0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor8(&self) -> &SDMMC_FIFOR8 {
        &self.sdmmc_fifor8
    }
    #[doc = "0xa4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor9(&self) -> &SDMMC_FIFOR9 {
        &self.sdmmc_fifor9
    }
    #[doc = "0xa8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor10(&self) -> &SDMMC_FIFOR10 {
        &self.sdmmc_fifor10
    }
    #[doc = "0xac - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor11(&self) -> &SDMMC_FIFOR11 {
        &self.sdmmc_fifor11
    }
    #[doc = "0xb0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor12(&self) -> &SDMMC_FIFOR12 {
        &self.sdmmc_fifor12
    }
    #[doc = "0xb4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor13(&self) -> &SDMMC_FIFOR13 {
        &self.sdmmc_fifor13
    }
    #[doc = "0xb8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor14(&self) -> &SDMMC_FIFOR14 {
        &self.sdmmc_fifor14
    }
    #[doc = "0xbc - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn sdmmc_fifor15(&self) -> &SDMMC_FIFOR15 {
        &self.sdmmc_fifor15
    }
    #[doc = "0x3f4 - SDMMC version register"]
    #[inline(always)]
    pub const fn sdmmc_verr(&self) -> &SDMMC_VERR {
        &self.sdmmc_verr
    }
    #[doc = "0x3f8 - SDMMC identification register"]
    #[inline(always)]
    pub const fn sdmmc_ipidr(&self) -> &SDMMC_IPIDR {
        &self.sdmmc_ipidr
    }
    #[doc = "0x3fc - SDMMC size ID register"]
    #[inline(always)]
    pub const fn sdmmc_sidr(&self) -> &SDMMC_SIDR {
        &self.sdmmc_sidr
    }
}
#[doc = "SDMMC_POWER (rw) register accessor: SDMMC power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_power`]
module"]
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWERrs>;
#[doc = "SDMMC power control register"]
pub mod sdmmc_power;
#[doc = "SDMMC_CLKCR (rw) register accessor: The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_clkcr`]
module"]
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCRrs>;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width."]
pub mod sdmmc_clkcr;
#[doc = "SDMMC_ARGR (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_argr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_argr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_argr`]
module"]
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGRrs>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod sdmmc_argr;
#[doc = "SDMMC_CMDR (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_cmdr`]
module"]
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDRrs>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod sdmmc_cmdr;
#[doc = "SDMMC_RESPCMDR (r) register accessor: The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_respcmdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_respcmdr`]
module"]
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDRrs>;
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod sdmmc_respcmdr;
#[doc = "SDMMC_RESP1R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp1r`]
module"]
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp1r;
#[doc = "SDMMC_RESP2R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp2r`]
module"]
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp2r;
#[doc = "SDMMC_RESP3R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp3r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp3r`]
module"]
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp3r;
#[doc = "SDMMC_RESP4R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp4r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp4r`]
module"]
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp4r;
#[doc = "SDMMC_DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dtimer`]
module"]
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMERrs>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod sdmmc_dtimer;
#[doc = "SDMMC_DLENR (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dlenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dlenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dlenr`]
module"]
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENRrs>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod sdmmc_dlenr;
#[doc = "SDMMC_DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dctrl`]
module"]
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRLrs>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod sdmmc_dctrl;
#[doc = "SDMMC_DCNTR (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dcntr`]
module"]
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTRrs>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod sdmmc_dcntr;
#[doc = "SDMMC_STAR (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_star::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_star`]
module"]
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STARrs>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \\[28, 21, 11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sdmmc_star;
#[doc = "SDMMC_ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_icr`]
module"]
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICRrs>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod sdmmc_icr;
#[doc = "SDMMC_MASKR (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_maskr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_maskr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_maskr`]
module"]
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKRrs>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod sdmmc_maskr;
#[doc = "SDMMC_ACKTIMER (rw) register accessor: The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_acktimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_acktimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_acktimer`]
module"]
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMERrs>;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod sdmmc_acktimer;
#[doc = "SDMMC_IDMACTRLR (rw) register accessor: The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmactrlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmactrlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idmactrlr`]
module"]
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLRrs>;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod sdmmc_idmactrlr;
#[doc = "SDMMC_IDMABSIZER (rw) register accessor: The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabsizer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabsizer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idmabsizer`]
module"]
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZERrs>;
#[doc = "The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration."]
pub mod sdmmc_idmabsizer;
#[doc = "SDMMC_IDMABASER (rw) register accessor: The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabaser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabaser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idmabaser`]
module"]
pub type SDMMC_IDMABASER = crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASERrs>;
#[doc = "The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration."]
pub mod sdmmc_idmabaser;
#[doc = "SDMMC_IDMALAR (rw) register accessor: SDMMC IDMA linked list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmalar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmalar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idmalar`]
module"]
pub type SDMMC_IDMALAR = crate::Reg<sdmmc_idmalar::SDMMC_IDMALARrs>;
#[doc = "SDMMC IDMA linked list address register"]
pub mod sdmmc_idmalar;
#[doc = "SDMMC_IDMABAR (rw) register accessor: SDMMC IDMA linked list memory base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idmabar`]
module"]
pub type SDMMC_IDMABAR = crate::Reg<sdmmc_idmabar::SDMMC_IDMABARrs>;
#[doc = "SDMMC IDMA linked list memory base register"]
pub mod sdmmc_idmabar;
#[doc = "SDMMC_FIFOR0 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor0`]
module"]
pub type SDMMC_FIFOR0 = crate::Reg<sdmmc_fifor0::SDMMC_FIFOR0rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor0;
#[doc = "SDMMC_FIFOR1 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor1`]
module"]
pub type SDMMC_FIFOR1 = crate::Reg<sdmmc_fifor1::SDMMC_FIFOR1rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor1;
#[doc = "SDMMC_FIFOR2 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor2`]
module"]
pub type SDMMC_FIFOR2 = crate::Reg<sdmmc_fifor2::SDMMC_FIFOR2rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor2;
#[doc = "SDMMC_FIFOR3 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor3`]
module"]
pub type SDMMC_FIFOR3 = crate::Reg<sdmmc_fifor3::SDMMC_FIFOR3rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor3;
#[doc = "SDMMC_FIFOR4 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor4`]
module"]
pub type SDMMC_FIFOR4 = crate::Reg<sdmmc_fifor4::SDMMC_FIFOR4rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor4;
#[doc = "SDMMC_FIFOR5 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor5`]
module"]
pub type SDMMC_FIFOR5 = crate::Reg<sdmmc_fifor5::SDMMC_FIFOR5rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor5;
#[doc = "SDMMC_FIFOR6 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor6`]
module"]
pub type SDMMC_FIFOR6 = crate::Reg<sdmmc_fifor6::SDMMC_FIFOR6rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor6;
#[doc = "SDMMC_FIFOR7 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor7`]
module"]
pub type SDMMC_FIFOR7 = crate::Reg<sdmmc_fifor7::SDMMC_FIFOR7rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor7;
#[doc = "SDMMC_FIFOR8 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor8`]
module"]
pub type SDMMC_FIFOR8 = crate::Reg<sdmmc_fifor8::SDMMC_FIFOR8rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor8;
#[doc = "SDMMC_FIFOR9 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor9`]
module"]
pub type SDMMC_FIFOR9 = crate::Reg<sdmmc_fifor9::SDMMC_FIFOR9rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor9;
#[doc = "SDMMC_FIFOR10 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor10`]
module"]
pub type SDMMC_FIFOR10 = crate::Reg<sdmmc_fifor10::SDMMC_FIFOR10rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor10;
#[doc = "SDMMC_FIFOR11 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor11`]
module"]
pub type SDMMC_FIFOR11 = crate::Reg<sdmmc_fifor11::SDMMC_FIFOR11rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor11;
#[doc = "SDMMC_FIFOR12 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor12`]
module"]
pub type SDMMC_FIFOR12 = crate::Reg<sdmmc_fifor12::SDMMC_FIFOR12rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor12;
#[doc = "SDMMC_FIFOR13 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor13`]
module"]
pub type SDMMC_FIFOR13 = crate::Reg<sdmmc_fifor13::SDMMC_FIFOR13rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor13;
#[doc = "SDMMC_FIFOR14 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor14`]
module"]
pub type SDMMC_FIFOR14 = crate::Reg<sdmmc_fifor14::SDMMC_FIFOR14rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor14;
#[doc = "SDMMC_FIFOR15 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifor15`]
module"]
pub type SDMMC_FIFOR15 = crate::Reg<sdmmc_fifor15::SDMMC_FIFOR15rs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor15;
#[doc = "SDMMC_VERR (r) register accessor: SDMMC version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_verr`]
module"]
pub type SDMMC_VERR = crate::Reg<sdmmc_verr::SDMMC_VERRrs>;
#[doc = "SDMMC version register"]
pub mod sdmmc_verr;
#[doc = "SDMMC_IPIDR (r) register accessor: SDMMC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_ipidr`]
module"]
pub type SDMMC_IPIDR = crate::Reg<sdmmc_ipidr::SDMMC_IPIDRrs>;
#[doc = "SDMMC identification register"]
pub mod sdmmc_ipidr;
#[doc = "SDMMC_SIDR (r) register accessor: SDMMC size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_sidr`]
module"]
pub type SDMMC_SIDR = crate::Reg<sdmmc_sidr::SDMMC_SIDRrs>;
#[doc = "SDMMC size ID register"]
pub mod sdmmc_sidr;
