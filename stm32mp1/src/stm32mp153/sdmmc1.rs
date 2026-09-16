#[repr(C)]
#[derive(Debug)]
///Register block
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
    idmabaser: IDMABASER,
    _reserved20: [u8; 0x08],
    idmalar: IDMALAR,
    idmabar: IDMABAR,
    _reserved22: [u8; 0x14],
    fifor0: FIFOR0,
    fifor1: FIFOR1,
    fifor2: FIFOR2,
    fifor3: FIFOR3,
    fifor4: FIFOR4,
    fifor5: FIFOR5,
    fifor6: FIFOR6,
    fifor7: FIFOR7,
    fifor8: FIFOR8,
    fifor9: FIFOR9,
    fifor10: FIFOR10,
    fifor11: FIFOR11,
    fifor12: FIFOR12,
    fifor13: FIFOR13,
    fifor14: FIFOR14,
    fifor15: FIFOR15,
    _reserved38: [u8; 0x0334],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - SDMMC power control register
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    ///0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    ///0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    ///0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    ///0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
    #[inline(always)]
    pub const fn respcmdr(&self) -> &RESPCMDR {
        &self.respcmdr
    }
    ///0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn resp1r(&self) -> &RESP1R {
        &self.resp1r
    }
    ///0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn resp2r(&self) -> &RESP2R {
        &self.resp2r
    }
    ///0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn resp3r(&self) -> &RESP3R {
        &self.resp3r
    }
    ///0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub const fn resp4r(&self) -> &RESP4R {
        &self.resp4r
    }
    ///0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    ///0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
    #[inline(always)]
    pub const fn dlenr(&self) -> &DLENR {
        &self.dlenr
    }
    ///0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM).
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    ///0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
    #[inline(always)]
    pub const fn dcntr(&self) -> &DCNTR {
        &self.dcntr
    }
    ///0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
    #[inline(always)]
    pub const fn star(&self) -> &STAR {
        &self.star
    }
    ///0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    #[inline(always)]
    pub const fn maskr(&self) -> &MASKR {
        &self.maskr
    }
    ///0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.
    #[inline(always)]
    pub const fn acktimer(&self) -> &ACKTIMER {
        &self.acktimer
    }
    ///0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
    #[inline(always)]
    pub const fn idmactrlr(&self) -> &IDMACTRLR {
        &self.idmactrlr
    }
    ///0x54 - The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.
    #[inline(always)]
    pub const fn idmabsizer(&self) -> &IDMABSIZER {
        &self.idmabsizer
    }
    ///0x58 - The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.
    #[inline(always)]
    pub const fn idmabaser(&self) -> &IDMABASER {
        &self.idmabaser
    }
    ///0x64 - SDMMC IDMA linked list address register
    #[inline(always)]
    pub const fn idmalar(&self) -> &IDMALAR {
        &self.idmalar
    }
    ///0x68 - SDMMC IDMA linked list memory base register
    #[inline(always)]
    pub const fn idmabar(&self) -> &IDMABAR {
        &self.idmabar
    }
    ///0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor0(&self) -> &FIFOR0 {
        &self.fifor0
    }
    ///0x84 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor1(&self) -> &FIFOR1 {
        &self.fifor1
    }
    ///0x88 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor2(&self) -> &FIFOR2 {
        &self.fifor2
    }
    ///0x8c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor3(&self) -> &FIFOR3 {
        &self.fifor3
    }
    ///0x90 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor4(&self) -> &FIFOR4 {
        &self.fifor4
    }
    ///0x94 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor5(&self) -> &FIFOR5 {
        &self.fifor5
    }
    ///0x98 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor6(&self) -> &FIFOR6 {
        &self.fifor6
    }
    ///0x9c - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor7(&self) -> &FIFOR7 {
        &self.fifor7
    }
    ///0xa0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor8(&self) -> &FIFOR8 {
        &self.fifor8
    }
    ///0xa4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor9(&self) -> &FIFOR9 {
        &self.fifor9
    }
    ///0xa8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor10(&self) -> &FIFOR10 {
        &self.fifor10
    }
    ///0xac - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor11(&self) -> &FIFOR11 {
        &self.fifor11
    }
    ///0xb0 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor12(&self) -> &FIFOR12 {
        &self.fifor12
    }
    ///0xb4 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor13(&self) -> &FIFOR13 {
        &self.fifor13
    }
    ///0xb8 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor14(&self) -> &FIFOR14 {
        &self.fifor14
    }
    ///0xbc - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    #[inline(always)]
    pub const fn fifor15(&self) -> &FIFOR15 {
        &self.fifor15
    }
    ///0x3f4 - SDMMC version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - SDMMC identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - SDMMC size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**POWER (rw) register accessor: SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:POWER)

For information about available fields see [`mod@power`] module*/
pub type POWER = crate::Reg<power::POWERrs>;
///SDMMC power control register
pub mod power;
/**CLKCR (rw) register accessor: The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:CLKCR)

For information about available fields see [`mod@clkcr`] module*/
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
///The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.
pub mod clkcr;
/**ARGR (rw) register accessor: The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.

You can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:ARGR)

For information about available fields see [`mod@argr`] module*/
pub type ARGR = crate::Reg<argr::ARGRrs>;
///The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
pub mod argr;
/**CMDR (rw) register accessor: The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:CMDR)

For information about available fields see [`mod@cmdr`] module*/
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
///The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
pub mod cmdr;
/**RESPCMDR (r) register accessor: The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).

You can [`read`](crate::Reg::read) this register and get [`respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESPCMDR)

For information about available fields see [`mod@respcmdr`] module*/
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
///The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
pub mod respcmdr;
/**RESP1R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP1R)

For information about available fields see [`mod@resp1r`] module*/
pub type RESP1R = crate::Reg<resp1r::RESP1Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod resp1r;
/**RESP2R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP2R)

For information about available fields see [`mod@resp2r`] module*/
pub type RESP2R = crate::Reg<resp2r::RESP2Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod resp2r;
/**RESP3R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP3R)

For information about available fields see [`mod@resp3r`] module*/
pub type RESP3R = crate::Reg<resp3r::RESP3Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod resp3r;
/**RESP4R (r) register accessor: The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp4r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP4R)

For information about available fields see [`mod@resp4r`] module*/
pub type RESP4R = crate::Reg<resp4r::RESP4Rrs>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod resp4r;
/**DTIMER (rw) register accessor: The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:DTIMER)

For information about available fields see [`mod@dtimer`] module*/
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
///The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
pub mod dtimer;
/**DLENR (rw) register accessor: The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:DLENR)

For information about available fields see [`mod@dlenr`] module*/
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
///The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
pub mod dlenr;
/**DCTRL (rw) register accessor: The SDMMC_DCTRL register control the data path state machine (DPSM).

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:DCTRL)

For information about available fields see [`mod@dctrl`] module*/
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
///The SDMMC_DCTRL register control the data path state machine (DPSM).
pub mod dctrl;
/**DCNTR (r) register accessor: The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:DCNTR)

For information about available fields see [`mod@dcntr`] module*/
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
///The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
pub mod dcntr;
/**STAR (r) register accessor: The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)

You can [`read`](crate::Reg::read) this register and get [`star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:STAR)

For information about available fields see [`mod@star`] module*/
pub type STAR = crate::Reg<star::STARrs>;
///The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
pub mod star;
/**ICR (rw) register accessor: The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
pub mod icr;
/**MASKR (rw) register accessor: The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:MASKR)

For information about available fields see [`mod@maskr`] module*/
pub type MASKR = crate::Reg<maskr::MASKRrs>;
///The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod maskr;
/**ACKTIMER (rw) register accessor: The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:ACKTIMER)

For information about available fields see [`mod@acktimer`] module*/
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
///The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.
pub mod acktimer;
/**IDMACTRLR (rw) register accessor: The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.

You can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMACTRLR)

For information about available fields see [`mod@idmactrlr`] module*/
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
///The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.
pub mod idmactrlr;
/**IDMABSIZER (rw) register accessor: The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMABSIZER)

For information about available fields see [`mod@idmabsizer`] module*/
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
///The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.
pub mod idmabsizer;
/**IDMABASER (rw) register accessor: The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMABASER)

For information about available fields see [`mod@idmabaser`] module*/
pub type IDMABASER = crate::Reg<idmabaser::IDMABASERrs>;
///The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.
pub mod idmabaser;
/**IDMALAR (rw) register accessor: SDMMC IDMA linked list address register

You can [`read`](crate::Reg::read) this register and get [`idmalar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmalar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMALAR)

For information about available fields see [`mod@idmalar`] module*/
pub type IDMALAR = crate::Reg<idmalar::IDMALARrs>;
///SDMMC IDMA linked list address register
pub mod idmalar;
/**IDMABAR (rw) register accessor: SDMMC IDMA linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`idmabar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMABAR)

For information about available fields see [`mod@idmabar`] module*/
pub type IDMABAR = crate::Reg<idmabar::IDMABARrs>;
///SDMMC IDMA linked list memory base register
pub mod idmabar;
/**FIFOR0 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR0)

For information about available fields see [`mod@fifor0`] module*/
pub type FIFOR0 = crate::Reg<fifor0::FIFOR0rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor0;
/**FIFOR1 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR1)

For information about available fields see [`mod@fifor1`] module*/
pub type FIFOR1 = crate::Reg<fifor1::FIFOR1rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor1;
/**FIFOR2 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR2)

For information about available fields see [`mod@fifor2`] module*/
pub type FIFOR2 = crate::Reg<fifor2::FIFOR2rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor2;
/**FIFOR3 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR3)

For information about available fields see [`mod@fifor3`] module*/
pub type FIFOR3 = crate::Reg<fifor3::FIFOR3rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor3;
/**FIFOR4 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR4)

For information about available fields see [`mod@fifor4`] module*/
pub type FIFOR4 = crate::Reg<fifor4::FIFOR4rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor4;
/**FIFOR5 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR5)

For information about available fields see [`mod@fifor5`] module*/
pub type FIFOR5 = crate::Reg<fifor5::FIFOR5rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor5;
/**FIFOR6 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR6)

For information about available fields see [`mod@fifor6`] module*/
pub type FIFOR6 = crate::Reg<fifor6::FIFOR6rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor6;
/**FIFOR7 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR7)

For information about available fields see [`mod@fifor7`] module*/
pub type FIFOR7 = crate::Reg<fifor7::FIFOR7rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor7;
/**FIFOR8 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR8)

For information about available fields see [`mod@fifor8`] module*/
pub type FIFOR8 = crate::Reg<fifor8::FIFOR8rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor8;
/**FIFOR9 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR9)

For information about available fields see [`mod@fifor9`] module*/
pub type FIFOR9 = crate::Reg<fifor9::FIFOR9rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor9;
/**FIFOR10 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR10)

For information about available fields see [`mod@fifor10`] module*/
pub type FIFOR10 = crate::Reg<fifor10::FIFOR10rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor10;
/**FIFOR11 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR11)

For information about available fields see [`mod@fifor11`] module*/
pub type FIFOR11 = crate::Reg<fifor11::FIFOR11rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor11;
/**FIFOR12 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR12)

For information about available fields see [`mod@fifor12`] module*/
pub type FIFOR12 = crate::Reg<fifor12::FIFOR12rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor12;
/**FIFOR13 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR13)

For information about available fields see [`mod@fifor13`] module*/
pub type FIFOR13 = crate::Reg<fifor13::FIFOR13rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor13;
/**FIFOR14 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR14)

For information about available fields see [`mod@fifor14`] module*/
pub type FIFOR14 = crate::Reg<fifor14::FIFOR14rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor14;
/**FIFOR15 (rw) register accessor: The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:FIFOR15)

For information about available fields see [`mod@fifor15`] module*/
pub type FIFOR15 = crate::Reg<fifor15::FIFOR15rs>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifor15;
/**VERR (r) register accessor: SDMMC version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///SDMMC version register
pub mod verr;
/**IPIDR (r) register accessor: SDMMC identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///SDMMC identification register
pub mod ipidr;
/**SIDR (r) register accessor: SDMMC size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///SDMMC size ID register
pub mod sidr;
