#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    argr: ARGR,
    cmdr: CMDR,
    respcmdr: RESPCMDR,
    resp1r: RESP1R,
    resp2r: RESP2R,
    resp3r: RESP3R,
    resp4r: RESP4R,
    dtimer: DTIMER,
    dlenr: DLENR,
    dctrl: DCTRL,
    dcntr: DCNTR,
    star: STAR,
    icr: ICR,
    maskr: MASKR,
    acktimer: ACKTIMER,
    _reserved17: [u8; 0x0c],
    idmactrlr: IDMACTRLR,
    idmabsizer: IDMABSIZER,
    idmabase0r: IDMABASE0R,
    idmabase1r: IDMABASE1R,
    _reserved21: [u8; 0x20],
    fifor: FIFOR,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    #[doc = "0x10 - SDMMC command response register"]
    #[inline(always)]
    pub const fn respcmdr(&self) -> &RESPCMDR {
        &self.respcmdr
    }
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn resp1r(&self) -> &RESP1R {
        &self.resp1r
    }
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn resp2r(&self) -> &RESP2R {
        &self.resp2r
    }
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn resp3r(&self) -> &RESP3R {
        &self.resp3r
    }
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    #[inline(always)]
    pub const fn resp4r(&self) -> &RESP4R {
        &self.resp4r
    }
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    #[inline(always)]
    pub const fn dlenr(&self) -> &DLENR {
        &self.dlenr
    }
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    #[inline(always)]
    pub const fn dcntr(&self) -> &DCNTR {
        &self.dcntr
    }
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    #[inline(always)]
    pub const fn star(&self) -> &STAR {
        &self.star
    }
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    #[inline(always)]
    pub const fn maskr(&self) -> &MASKR {
        &self.maskr
    }
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    #[inline(always)]
    pub const fn acktimer(&self) -> &ACKTIMER {
        &self.acktimer
    }
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    #[inline(always)]
    pub const fn idmactrlr(&self) -> &IDMACTRLR {
        &self.idmactrlr
    }
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    #[inline(always)]
    pub const fn idmabsizer(&self) -> &IDMABSIZER {
        &self.idmabsizer
    }
    #[doc = "0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    #[inline(always)]
    pub const fn idmabase0r(&self) -> &IDMABASE0R {
        &self.idmabase0r
    }
    #[doc = "0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    #[inline(always)]
    pub const fn idmabase1r(&self) -> &IDMABASE1R {
        &self.idmabase1r
    }
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn fifor(&self) -> &FIFOR {
        &self.fifor
    }
}
#[doc = "POWER (rw) register accessor: SDMMC power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWERrs>;
#[doc = "SDMMC power control register"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
pub mod clkcr;
#[doc = "ARGR (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argr`]
module"]
pub type ARGR = crate::Reg<argr::ARGRrs>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod argr;
#[doc = "CMDR (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`]
module"]
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmdr;
#[doc = "RESP1R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1r`]
module"]
pub type RESP1R = crate::Reg<resp1r::RESP1Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp1r;
#[doc = "RESP2R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2r`]
module"]
pub type RESP2R = crate::Reg<resp2r::RESP2Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp2r;
#[doc = "RESP3R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3r`]
module"]
pub type RESP3R = crate::Reg<resp3r::RESP3Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp3r;
#[doc = "RESP4R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4r`]
module"]
pub type RESP4R = crate::Reg<resp4r::RESP4Rrs>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod resp4r;
#[doc = "DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "DLENR (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlenr`]
module"]
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlenr;
#[doc = "DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "DCNTR (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcntr`]
module"]
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcntr;
#[doc = "STAR (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`star::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@star`]
module"]
pub type STAR = crate::Reg<star::STARrs>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod star;
#[doc = "ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "MASKR (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskr`]
module"]
pub type MASKR = crate::Reg<maskr::MASKRrs>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod maskr;
#[doc = "ACKTIMER (rw) register accessor: The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acktimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acktimer`]
module"]
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod acktimer;
#[doc = "IDMACTRLR (rw) register accessor: The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmactrlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmactrlr`]
module"]
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod idmactrlr;
#[doc = "IDMABSIZER (rw) register accessor: The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabsizer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabsizer`]
module"]
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
pub mod idmabsizer;
#[doc = "IDMABASE0R (rw) register accessor: The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabase0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabase0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabase0r`]
module"]
pub type IDMABASE0R = crate::Reg<idmabase0r::IDMABASE0Rrs>;
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
pub mod idmabase0r;
#[doc = "IDMABASE1R (rw) register accessor: The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabase1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabase1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmabase1r`]
module"]
pub type IDMABASE1R = crate::Reg<idmabase1r::IDMABASE1Rrs>;
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
pub mod idmabase1r;
#[doc = "FIFOR (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifor`]
module"]
pub type FIFOR = crate::Reg<fifor::FIFORrs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifor;
#[doc = "RESPCMDR (r) register accessor: SDMMC command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmdr`]
module"]
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
#[doc = "SDMMC command response register"]
pub mod respcmdr;
