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
    sdmmc_idmabase0r: SDMMC_IDMABASE0R,
    sdmmc_idmabase1r: SDMMC_IDMABASE1R,
    _reserved21: [u8; 0x20],
    sdmmc_fifor: SDMMC_FIFOR,
    _reserved22: [u8; 0x0370],
    sdmmc_ver: SDMMC_VER,
    sdmmc_id: SDMMC_ID,
}
impl RegisterBlock {
    ///0x00 - SDMMC power control register
    #[inline(always)]
    pub const fn sdmmc_power(&self) -> &SDMMC_POWER {
        &self.sdmmc_power
    }
    ///0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.
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
    ///0x10 - SDMMC command response register
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
    ///0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
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
    ///0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.
    #[inline(always)]
    pub const fn sdmmc_idmabsizer(&self) -> &SDMMC_IDMABSIZER {
        &self.sdmmc_idmabsizer
    }
    ///0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.
    #[inline(always)]
    pub const fn sdmmc_idmabase0r(&self) -> &SDMMC_IDMABASE0R {
        &self.sdmmc_idmabase0r
    }
    ///0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.
    #[inline(always)]
    pub const fn sdmmc_idmabase1r(&self) -> &SDMMC_IDMABASE1R {
        &self.sdmmc_idmabase1r
    }
    ///0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn sdmmc_fifor(&self) -> &SDMMC_FIFOR {
        &self.sdmmc_fifor
    }
    ///0x3f4 - SDMMC IP version register
    #[inline(always)]
    pub const fn sdmmc_ver(&self) -> &SDMMC_VER {
        &self.sdmmc_ver
    }
    ///0x3f8 - SDMMC IP identification register
    #[inline(always)]
    pub const fn sdmmc_id(&self) -> &SDMMC_ID {
        &self.sdmmc_id
    }
}
/**SDMMC_POWER (rw) register accessor: SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_POWER)

For information about available fields see [`mod@sdmmc_power`]
module*/
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWERrs>;
///SDMMC power control register
pub mod sdmmc_power;
/**SDMMC_CLKCR (rw) register accessor: The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_CLKCR)

For information about available fields see [`mod@sdmmc_clkcr`]
module*/
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCRrs>;
///The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.
pub mod sdmmc_clkcr;
/**SDMMC_ARGR (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_ARGR)

For information about available fields see [`mod@sdmmc_argr`]
module*/
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGRrs>;
///The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
pub mod sdmmc_argr;
/**SDMMC_CMDR (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).

You can [`read`](crate::Reg::read) this register and get [`sdmmc_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_CMDR)

For information about available fields see [`mod@sdmmc_cmdr`]
module*/
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDRrs>;
///The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
pub mod sdmmc_cmdr;
/**SDMMC_RESP1R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_RESP1R)

For information about available fields see [`mod@sdmmc_resp1r`]
module*/
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp1r;
/**SDMMC_RESP2R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_RESP2R)

For information about available fields see [`mod@sdmmc_resp2r`]
module*/
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp2r;
/**SDMMC_RESP3R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_RESP3R)

For information about available fields see [`mod@sdmmc_resp3r`]
module*/
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp3r;
/**SDMMC_RESP4R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp4r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_RESP4R)

For information about available fields see [`mod@sdmmc_resp4r`]
module*/
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod sdmmc_resp4r;
/**SDMMC_DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_DTIMER)

For information about available fields see [`mod@sdmmc_dtimer`]
module*/
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMERrs>;
///The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
pub mod sdmmc_dtimer;
/**SDMMC_DLENR (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_DLENR)

For information about available fields see [`mod@sdmmc_dlenr`]
module*/
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENRrs>;
///The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
pub mod sdmmc_dlenr;
/**SDMMC_DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_DCTRL)

For information about available fields see [`mod@sdmmc_dctrl`]
module*/
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRLrs>;
///The SDMMC_DCTRL register control the data path state machine (DPSM).
pub mod sdmmc_dctrl;
/**SDMMC_DCNTR (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_DCNTR)

For information about available fields see [`mod@sdmmc_dcntr`]
module*/
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTRrs>;
///The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
pub mod sdmmc_dcntr;
/**SDMMC_STAR (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)

You can [`read`](crate::Reg::read) this register and get [`sdmmc_star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_STAR)

For information about available fields see [`mod@sdmmc_star`]
module*/
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STARrs>;
///The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
pub mod sdmmc_star;
/**SDMMC_ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_ICR)

For information about available fields see [`mod@sdmmc_icr`]
module*/
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICRrs>;
///The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
pub mod sdmmc_icr;
/**SDMMC_MASKR (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_MASKR)

For information about available fields see [`mod@sdmmc_maskr`]
module*/
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKRrs>;
///The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod sdmmc_maskr;
/**SDMMC_ACKTIMER (rw) register accessor: The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_ACKTIMER)

For information about available fields see [`mod@sdmmc_acktimer`]
module*/
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMERrs>;
///The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.
pub mod sdmmc_acktimer;
/**SDMMC_IDMACTRLR (rw) register accessor: The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_IDMACTRLR)

For information about available fields see [`mod@sdmmc_idmactrlr`]
module*/
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLRrs>;
///The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
pub mod sdmmc_idmactrlr;
/**SDMMC_IDMABSIZER (rw) register accessor: The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_IDMABSIZER)

For information about available fields see [`mod@sdmmc_idmabsizer`]
module*/
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZERrs>;
///The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.
pub mod sdmmc_idmabsizer;
/**SDMMC_IDMABASE0R (rw) register accessor: The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabase0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabase0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_IDMABASE0R)

For information about available fields see [`mod@sdmmc_idmabase0r`]
module*/
pub type SDMMC_IDMABASE0R = crate::Reg<sdmmc_idmabase0r::SDMMC_IDMABASE0Rrs>;
///The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.
pub mod sdmmc_idmabase0r;
/**SDMMC_IDMABASE1R (rw) register accessor: The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabase1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabase1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_IDMABASE1R)

For information about available fields see [`mod@sdmmc_idmabase1r`]
module*/
pub type SDMMC_IDMABASE1R = crate::Reg<sdmmc_idmabase1r::SDMMC_IDMABASE1Rrs>;
///The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.
pub mod sdmmc_idmabase1r;
/**SDMMC_FIFOR (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_FIFOR)

For information about available fields see [`mod@sdmmc_fifor`]
module*/
pub type SDMMC_FIFOR = crate::Reg<sdmmc_fifor::SDMMC_FIFORrs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod sdmmc_fifor;
/**SDMMC_VER (r) register accessor: SDMMC IP version register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_VER)

For information about available fields see [`mod@sdmmc_ver`]
module*/
pub type SDMMC_VER = crate::Reg<sdmmc_ver::SDMMC_VERrs>;
///SDMMC IP version register
pub mod sdmmc_ver;
/**SDMMC_ID (r) register accessor: SDMMC IP identification register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_ID)

For information about available fields see [`mod@sdmmc_id`]
module*/
pub type SDMMC_ID = crate::Reg<sdmmc_id::SDMMC_IDrs>;
///SDMMC IP identification register
pub mod sdmmc_id;
/**SDMMC_RESPCMDR (r) register accessor: SDMMC command response register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_RESPCMDR)

For information about available fields see [`mod@sdmmc_respcmdr`]
module*/
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDRrs>;
///SDMMC command response register
pub mod sdmmc_respcmdr;
