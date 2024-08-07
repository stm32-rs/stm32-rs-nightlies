#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - SDMMC power control register
    #[inline(always)]
    pub const fn sdmmc_power(&self) -> &SDMMC_POWER {
        &self.sdmmc_power
    }
    ///0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.
    #[inline(always)]
    pub const fn sdmmc_clkcr(&self) -> &SDMMC_CLKCR {
        &self.sdmmc_clkcr
    }
    ///0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
    #[inline(always)]
    pub const fn sdmmc_argr(&self) -> &SDMMC_ARGR {
        &self.sdmmc_argr
    }
    ///0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
    #[inline(always)]
    pub const fn sdmmc_cmdr(&self) -> &SDMMC_CMDR {
        &self.sdmmc_cmdr
    }
    ///0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
    #[inline(always)]
    pub const fn sdmmc_respcmdr(&self) -> &SDMMC_RESPCMDR {
        &self.sdmmc_respcmdr
    }
    ///0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn sdmmc_resp1r(&self) -> &SDMMC_RESP1R {
        &self.sdmmc_resp1r
    }
    ///0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn sdmmc_resp2r(&self) -> &SDMMC_RESP2R {
        &self.sdmmc_resp2r
    }
    ///0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn sdmmc_resp3r(&self) -> &SDMMC_RESP3R {
        &self.sdmmc_resp3r
    }
    ///0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn sdmmc_resp4r(&self) -> &SDMMC_RESP4R {
        &self.sdmmc_resp4r
    }
    ///0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
    #[inline(always)]
    pub const fn sdmmc_dtimer(&self) -> &SDMMC_DTIMER {
        &self.sdmmc_dtimer
    }
    ///0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
    #[inline(always)]
    pub const fn sdmmc_dlenr(&self) -> &SDMMC_DLENR {
        &self.sdmmc_dlenr
    }
    ///0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM).
    #[inline(always)]
    pub const fn sdmmc_dctrl(&self) -> &SDMMC_DCTRL {
        &self.sdmmc_dctrl
    }
    ///0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
    #[inline(always)]
    pub const fn sdmmc_dcntr(&self) -> &SDMMC_DCNTR {
        &self.sdmmc_dcntr
    }
    ///0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
    #[inline(always)]
    pub const fn sdmmc_star(&self) -> &SDMMC_STAR {
        &self.sdmmc_star
    }
    ///0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
    #[inline(always)]
    pub const fn sdmmc_icr(&self) -> &SDMMC_ICR {
        &self.sdmmc_icr
    }
    ///0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    #[inline(always)]
    pub const fn sdmmc_maskr(&self) -> &SDMMC_MASKR {
        &self.sdmmc_maskr
    }
    ///0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.
    #[inline(always)]
    pub const fn sdmmc_acktimer(&self) -> &SDMMC_ACKTIMER {
        &self.sdmmc_acktimer
    }
    ///0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
    #[inline(always)]
    pub const fn sdmmc_idmactrlr(&self) -> &SDMMC_IDMACTRLR {
        &self.sdmmc_idmactrlr
    }
    ///0x54 - The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.
    #[inline(always)]
    pub const fn sdmmc_idmabsizer(&self) -> &SDMMC_IDMABSIZER {
        &self.sdmmc_idmabsizer
    }
    ///0x58 - The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.
    #[inline(always)]
    pub const fn sdmmc_idmabaser(&self) -> &SDMMC_IDMABASER {
        &self.sdmmc_idmabaser
    }
    ///0x64 - SDMMC IDMA linked list address register
    #[inline(always)]
    pub const fn sdmmc_idmalar(&self) -> &SDMMC_IDMALAR {
        &self.sdmmc_idmalar
    }
    ///0x68 - SDMMC IDMA linked list memory base register
    #[inline(always)]
    pub const fn sdmmc_idmabar(&self) -> &SDMMC_IDMABAR {
        &self.sdmmc_idmabar
    }
    ///0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor0(&self) -> &SDMMC_FIFOR0 {
        &self.sdmmc_fifor0
    }
    ///0x84 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor1(&self) -> &SDMMC_FIFOR1 {
        &self.sdmmc_fifor1
    }
    ///0x88 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor2(&self) -> &SDMMC_FIFOR2 {
        &self.sdmmc_fifor2
    }
    ///0x8c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor3(&self) -> &SDMMC_FIFOR3 {
        &self.sdmmc_fifor3
    }
    ///0x90 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor4(&self) -> &SDMMC_FIFOR4 {
        &self.sdmmc_fifor4
    }
    ///0x94 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor5(&self) -> &SDMMC_FIFOR5 {
        &self.sdmmc_fifor5
    }
    ///0x98 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor6(&self) -> &SDMMC_FIFOR6 {
        &self.sdmmc_fifor6
    }
    ///0x9c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor7(&self) -> &SDMMC_FIFOR7 {
        &self.sdmmc_fifor7
    }
    ///0xa0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor8(&self) -> &SDMMC_FIFOR8 {
        &self.sdmmc_fifor8
    }
    ///0xa4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor9(&self) -> &SDMMC_FIFOR9 {
        &self.sdmmc_fifor9
    }
    ///0xa8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor10(&self) -> &SDMMC_FIFOR10 {
        &self.sdmmc_fifor10
    }
    ///0xac - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor11(&self) -> &SDMMC_FIFOR11 {
        &self.sdmmc_fifor11
    }
    ///0xb0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor12(&self) -> &SDMMC_FIFOR12 {
        &self.sdmmc_fifor12
    }
    ///0xb4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor13(&self) -> &SDMMC_FIFOR13 {
        &self.sdmmc_fifor13
    }
    ///0xb8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor14(&self) -> &SDMMC_FIFOR14 {
        &self.sdmmc_fifor14
    }
    ///0xbc - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor15(&self) -> &SDMMC_FIFOR15 {
        &self.sdmmc_fifor15
    }
    ///0x3f4 - SDMMC version register
    #[inline(always)]
    pub const fn sdmmc_verr(&self) -> &SDMMC_VERR {
        &self.sdmmc_verr
    }
    ///0x3f8 - SDMMC identification register
    #[inline(always)]
    pub const fn sdmmc_ipidr(&self) -> &SDMMC_IPIDR {
        &self.sdmmc_ipidr
    }
    ///0x3fc - SDMMC size ID register
    #[inline(always)]
    pub const fn sdmmc_sidr(&self) -> &SDMMC_SIDR {
        &self.sdmmc_sidr
    }
}
/**SDMMC_POWER (rw) register accessor: SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_POWER)

For information about available fields see [`mod@sdmmc_power`]
module*/
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWERrs>;
///SDMMC power control register
pub mod sdmmc_power;
/**SDMMC_CLKCR (rw) register accessor: The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_CLKCR)

For information about available fields see [`mod@sdmmc_clkcr`]
module*/
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCRrs>;
///The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.
pub mod sdmmc_clkcr;
/**SDMMC_ARGR (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_ARGR)

For information about available fields see [`mod@sdmmc_argr`]
module*/
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGRrs>;
///The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
pub mod sdmmc_argr;
/**SDMMC_CMDR (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).

You can [`read`](crate::Reg::read) this register and get [`sdmmc_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_CMDR)

For information about available fields see [`mod@sdmmc_cmdr`]
module*/
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDRrs>;
///The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
pub mod sdmmc_cmdr;
/**SDMMC_RESPCMDR (r) register accessor: The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).

You can [`read`](crate::Reg::read) this register and get [`sdmmc_respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_RESPCMDR)

For information about available fields see [`mod@sdmmc_respcmdr`]
module*/
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDRrs>;
///The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
pub mod sdmmc_respcmdr;
/**SDMMC_RESP1R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_RESP1R)

For information about available fields see [`mod@sdmmc_resp1r`]
module*/
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp1r;
/**SDMMC_RESP2R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_RESP2R)

For information about available fields see [`mod@sdmmc_resp2r`]
module*/
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp2r;
/**SDMMC_RESP3R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_RESP3R)

For information about available fields see [`mod@sdmmc_resp3r`]
module*/
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp3r;
/**SDMMC_RESP4R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp4r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_RESP4R)

For information about available fields see [`mod@sdmmc_resp4r`]
module*/
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp4r;
/**SDMMC_DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_DTIMER)

For information about available fields see [`mod@sdmmc_dtimer`]
module*/
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMERrs>;
///The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
pub mod sdmmc_dtimer;
/**SDMMC_DLENR (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_DLENR)

For information about available fields see [`mod@sdmmc_dlenr`]
module*/
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENRrs>;
///The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
pub mod sdmmc_dlenr;
/**SDMMC_DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_DCTRL)

For information about available fields see [`mod@sdmmc_dctrl`]
module*/
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRLrs>;
///The SDMMC_DCTRL register control the data path state machine (DPSM).
pub mod sdmmc_dctrl;
/**SDMMC_DCNTR (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_DCNTR)

For information about available fields see [`mod@sdmmc_dcntr`]
module*/
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTRrs>;
///The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
pub mod sdmmc_dcntr;
/**SDMMC_STAR (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)

You can [`read`](crate::Reg::read) this register and get [`sdmmc_star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_STAR)

For information about available fields see [`mod@sdmmc_star`]
module*/
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STARrs>;
///The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
pub mod sdmmc_star;
/**SDMMC_ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_ICR)

For information about available fields see [`mod@sdmmc_icr`]
module*/
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICRrs>;
///The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
pub mod sdmmc_icr;
/**SDMMC_MASKR (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_MASKR)

For information about available fields see [`mod@sdmmc_maskr`]
module*/
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKRrs>;
///The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod sdmmc_maskr;
/**SDMMC_ACKTIMER (rw) register accessor: The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_ACKTIMER)

For information about available fields see [`mod@sdmmc_acktimer`]
module*/
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMERrs>;
///The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.
pub mod sdmmc_acktimer;
/**SDMMC_IDMACTRLR (rw) register accessor: The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IDMACTRLR)

For information about available fields see [`mod@sdmmc_idmactrlr`]
module*/
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLRrs>;
///The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
pub mod sdmmc_idmactrlr;
/**SDMMC_IDMABSIZER (rw) register accessor: The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IDMABSIZER)

For information about available fields see [`mod@sdmmc_idmabsizer`]
module*/
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZERrs>;
///The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.
pub mod sdmmc_idmabsizer;
/**SDMMC_IDMABASER (rw) register accessor: The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabaser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabaser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IDMABASER)

For information about available fields see [`mod@sdmmc_idmabaser`]
module*/
pub type SDMMC_IDMABASER = crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASERrs>;
///The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.
pub mod sdmmc_idmabaser;
/**SDMMC_IDMALAR (rw) register accessor: SDMMC IDMA linked list address register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmalar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmalar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IDMALAR)

For information about available fields see [`mod@sdmmc_idmalar`]
module*/
pub type SDMMC_IDMALAR = crate::Reg<sdmmc_idmalar::SDMMC_IDMALARrs>;
///SDMMC IDMA linked list address register
pub mod sdmmc_idmalar;
/**SDMMC_IDMABAR (rw) register accessor: SDMMC IDMA linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IDMABAR)

For information about available fields see [`mod@sdmmc_idmabar`]
module*/
pub type SDMMC_IDMABAR = crate::Reg<sdmmc_idmabar::SDMMC_IDMABARrs>;
///SDMMC IDMA linked list memory base register
pub mod sdmmc_idmabar;
/**SDMMC_FIFOR0 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR0)

For information about available fields see [`mod@sdmmc_fifor0`]
module*/
pub type SDMMC_FIFOR0 = crate::Reg<sdmmc_fifor0::SDMMC_FIFOR0rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor0;
/**SDMMC_FIFOR1 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR1)

For information about available fields see [`mod@sdmmc_fifor1`]
module*/
pub type SDMMC_FIFOR1 = crate::Reg<sdmmc_fifor1::SDMMC_FIFOR1rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor1;
/**SDMMC_FIFOR2 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR2)

For information about available fields see [`mod@sdmmc_fifor2`]
module*/
pub type SDMMC_FIFOR2 = crate::Reg<sdmmc_fifor2::SDMMC_FIFOR2rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor2;
/**SDMMC_FIFOR3 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR3)

For information about available fields see [`mod@sdmmc_fifor3`]
module*/
pub type SDMMC_FIFOR3 = crate::Reg<sdmmc_fifor3::SDMMC_FIFOR3rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor3;
/**SDMMC_FIFOR4 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR4)

For information about available fields see [`mod@sdmmc_fifor4`]
module*/
pub type SDMMC_FIFOR4 = crate::Reg<sdmmc_fifor4::SDMMC_FIFOR4rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor4;
/**SDMMC_FIFOR5 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR5)

For information about available fields see [`mod@sdmmc_fifor5`]
module*/
pub type SDMMC_FIFOR5 = crate::Reg<sdmmc_fifor5::SDMMC_FIFOR5rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor5;
/**SDMMC_FIFOR6 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR6)

For information about available fields see [`mod@sdmmc_fifor6`]
module*/
pub type SDMMC_FIFOR6 = crate::Reg<sdmmc_fifor6::SDMMC_FIFOR6rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor6;
/**SDMMC_FIFOR7 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR7)

For information about available fields see [`mod@sdmmc_fifor7`]
module*/
pub type SDMMC_FIFOR7 = crate::Reg<sdmmc_fifor7::SDMMC_FIFOR7rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor7;
/**SDMMC_FIFOR8 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR8)

For information about available fields see [`mod@sdmmc_fifor8`]
module*/
pub type SDMMC_FIFOR8 = crate::Reg<sdmmc_fifor8::SDMMC_FIFOR8rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor8;
/**SDMMC_FIFOR9 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR9)

For information about available fields see [`mod@sdmmc_fifor9`]
module*/
pub type SDMMC_FIFOR9 = crate::Reg<sdmmc_fifor9::SDMMC_FIFOR9rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor9;
/**SDMMC_FIFOR10 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR10)

For information about available fields see [`mod@sdmmc_fifor10`]
module*/
pub type SDMMC_FIFOR10 = crate::Reg<sdmmc_fifor10::SDMMC_FIFOR10rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor10;
/**SDMMC_FIFOR11 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR11)

For information about available fields see [`mod@sdmmc_fifor11`]
module*/
pub type SDMMC_FIFOR11 = crate::Reg<sdmmc_fifor11::SDMMC_FIFOR11rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor11;
/**SDMMC_FIFOR12 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR12)

For information about available fields see [`mod@sdmmc_fifor12`]
module*/
pub type SDMMC_FIFOR12 = crate::Reg<sdmmc_fifor12::SDMMC_FIFOR12rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor12;
/**SDMMC_FIFOR13 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR13)

For information about available fields see [`mod@sdmmc_fifor13`]
module*/
pub type SDMMC_FIFOR13 = crate::Reg<sdmmc_fifor13::SDMMC_FIFOR13rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor13;
/**SDMMC_FIFOR14 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR14)

For information about available fields see [`mod@sdmmc_fifor14`]
module*/
pub type SDMMC_FIFOR14 = crate::Reg<sdmmc_fifor14::SDMMC_FIFOR14rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor14;
/**SDMMC_FIFOR15 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR15)

For information about available fields see [`mod@sdmmc_fifor15`]
module*/
pub type SDMMC_FIFOR15 = crate::Reg<sdmmc_fifor15::SDMMC_FIFOR15rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor15;
/**SDMMC_VERR (r) register accessor: SDMMC version register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_VERR)

For information about available fields see [`mod@sdmmc_verr`]
module*/
pub type SDMMC_VERR = crate::Reg<sdmmc_verr::SDMMC_VERRrs>;
///SDMMC version register
pub mod sdmmc_verr;
/**SDMMC_IPIDR (r) register accessor: SDMMC identification register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_IPIDR)

For information about available fields see [`mod@sdmmc_ipidr`]
module*/
pub type SDMMC_IPIDR = crate::Reg<sdmmc_ipidr::SDMMC_IPIDRrs>;
///SDMMC identification register
pub mod sdmmc_ipidr;
/**SDMMC_SIDR (r) register accessor: SDMMC size ID register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_SIDR)

For information about available fields see [`mod@sdmmc_sidr`]
module*/
pub type SDMMC_SIDR = crate::Reg<sdmmc_sidr::SDMMC_SIDRrs>;
///SDMMC size ID register
pub mod sdmmc_sidr;
