#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    arg: ARG,
    cmd: CMD,
    respcmd: RESPCMD,
    resp: [RESP; 4],
    dtimer: DTIMER,
    dlen: DLEN,
    dctrl: DCTRL,
    dcount: DCOUNT,
    sta: STA,
    icr: ICR,
    mask: MASK,
    _reserved13: [u8; 0x08],
    fifocnt: FIFOCNT,
    _reserved14: [u8; 0x34],
    fifo: FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - SDIO power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    #[doc = "0x04 - CLKCR register controls the SDIO_CK output clock."]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    #[inline(always)]
    pub const fn arg(&self) -> &ARG {
        &self.arg
    }
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
    #[inline(always)]
    pub const fn respcmd(&self) -> &RESPCMD {
        &self.respcmd
    }
    #[doc = "0x14..0x24 - SDIO response %s register"]
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &RESP {
        &self.resp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - SDIO response %s register"]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &RESP> {
        self.resp.iter()
    }
    #[doc = "0x14 - SDIO response 1 register"]
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP {
        self.resp(0)
    }
    #[doc = "0x18 - SDIO response 2 register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP {
        self.resp(1)
    }
    #[doc = "0x1c - SDIO response 3 register"]
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP {
        self.resp(2)
    }
    #[doc = "0x20 - SDIO response 4 register"]
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP {
        self.resp(3)
    }
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    #[inline(always)]
    pub const fn dlen(&self) -> &DLEN {
        &self.dlen
    }
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    #[inline(always)]
    pub const fn dcount(&self) -> &DCOUNT {
        &self.dcount
    }
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    #[inline(always)]
    pub const fn mask(&self) -> &MASK {
        &self.mask
    }
    #[doc = "0x48 - The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &FIFOCNT {
        &self.fifocnt
    }
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
}
#[doc = "POWER (rw) register accessor: SDIO power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWERrs>;
#[doc = "SDIO power control register"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: CLKCR register controls the SDIO_CK output clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
#[doc = "CLKCR register controls the SDIO_CK output clock."]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`]
module"]
pub type ARG = crate::Reg<arg::ARGrs>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod arg;
#[doc = "CMD (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMDrs>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod cmd;
#[doc = "RESP (r) register accessor: SDIO response %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp`]
module"]
pub type RESP = crate::Reg<resp::RESPrs>;
#[doc = "SDIO response %s register"]
pub mod resp;
#[doc = "DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod dtimer;
#[doc = "DLEN (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
pub type DLEN = crate::Reg<dlen::DLENrs>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod dlen;
#[doc = "DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod dctrl;
#[doc = "DCOUNT (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcount`]
module"]
pub type DCOUNT = crate::Reg<dcount::DCOUNTrs>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod dcount;
#[doc = "STA (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STArs>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sta;
#[doc = "ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod icr;
#[doc = "MASK (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASKrs>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod mask;
#[doc = "RESPCMD (r) register accessor: The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`]
module"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMDrs>;
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response)."]
pub mod respcmd;
#[doc = "FIFO (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFOrs>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod fifo;
#[doc = "FIFOCNT (r) register accessor: The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`]
module"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNTrs>;
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."]
pub mod fifocnt;
