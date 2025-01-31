#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    otfdec_r1cfgr: OTFDEC_R1CFGR,
    otfdec_r1startaddr: OTFDEC_R1STARTADDR,
    otfdec_r1endaddr: OTFDEC_R1ENDADDR,
    otfdec_r1noncer0: OTFDEC_R1NONCER0,
    otfdec_r1noncer1: OTFDEC_R1NONCER1,
    otfdec_r1keyr0: OTFDEC_R1KEYR0,
    otfdec_r1keyr1: OTFDEC_R1KEYR1,
    otfdec_r1keyr2: OTFDEC_R1KEYR2,
    otfdec_r1keyr3: OTFDEC_R1KEYR3,
    _reserved9: [u8; 0x0c],
    otfdec_r2cfgr: OTFDEC_R2CFGR,
    otfdec_r2startaddr: OTFDEC_R2STARTADDR,
    otfdec_r2endaddr: OTFDEC_R2ENDADDR,
    otfdec_r2noncer0: OTFDEC_R2NONCER0,
    otfdec_r2noncer1: OTFDEC_R2NONCER1,
    otfdec_r2keyr0: OTFDEC_R2KEYR0,
    otfdec_r2keyr1: OTFDEC_R2KEYR1,
    otfdec_r2keyr2: OTFDEC_R2KEYR2,
    otfdec_r2keyr3: OTFDEC_R2KEYR3,
    _reserved18: [u8; 0x0c],
    otfdec_r3cfgr: OTFDEC_R3CFGR,
    otfdec_r3startaddr: OTFDEC_R3STARTADDR,
    otfdec_r3endaddr: OTFDEC_R3ENDADDR,
    otfdec_r3noncer0: OTFDEC_R3NONCER0,
    otfdec_r3noncer1: OTFDEC_R3NONCER1,
    otfdec_r3keyr0: OTFDEC_R3KEYR0,
    otfdec_r3keyr1: OTFDEC_R3KEYR1,
    otfdec_r3keyr2: OTFDEC_R3KEYR2,
    otfdec_r3keyr3: OTFDEC_R3KEYR3,
    _reserved27: [u8; 0x0c],
    otfdec_r4cfgr: OTFDEC_R4CFGR,
    otfdec_r4startaddr: OTFDEC_R4STARTADDR,
    otfdec_r4endaddr: OTFDEC_R4ENDADDR,
    otfdec_r4noncer0: OTFDEC_R4NONCER0,
    otfdec_r4noncer1: OTFDEC_R4NONCER1,
    otfdec_r4keyr0: OTFDEC_R4KEYR0,
    otfdec_r4keyr1: OTFDEC_R4KEYR1,
    otfdec_r4keyr2: OTFDEC_R4KEYR2,
    otfdec_r4keyr3: OTFDEC_R4KEYR3,
    _reserved36: [u8; 0x022c],
    otfdec_isr: OTFDEC_ISR,
    otfdec_icr: OTFDEC_ICR,
    otfdec_ier: OTFDEC_IER,
}
impl RegisterBlock {
    ///0x20 - OTFDEC region 1 configuration register
    #[inline(always)]
    pub const fn otfdec_r1cfgr(&self) -> &OTFDEC_R1CFGR {
        &self.otfdec_r1cfgr
    }
    ///0x24 - OTFDEC region 1 start address register
    #[inline(always)]
    pub const fn otfdec_r1startaddr(&self) -> &OTFDEC_R1STARTADDR {
        &self.otfdec_r1startaddr
    }
    ///0x28 - OTFDEC region 1 end address register
    #[inline(always)]
    pub const fn otfdec_r1endaddr(&self) -> &OTFDEC_R1ENDADDR {
        &self.otfdec_r1endaddr
    }
    ///0x2c - OTFDEC region 1 nonce register 0
    #[inline(always)]
    pub const fn otfdec_r1noncer0(&self) -> &OTFDEC_R1NONCER0 {
        &self.otfdec_r1noncer0
    }
    ///0x30 - OTFDEC region 1 nonce register 1
    #[inline(always)]
    pub const fn otfdec_r1noncer1(&self) -> &OTFDEC_R1NONCER1 {
        &self.otfdec_r1noncer1
    }
    ///0x34 - OTFDEC region 1 key register 0
    #[inline(always)]
    pub const fn otfdec_r1keyr0(&self) -> &OTFDEC_R1KEYR0 {
        &self.otfdec_r1keyr0
    }
    ///0x38 - OTFDEC region 1 key register 1
    #[inline(always)]
    pub const fn otfdec_r1keyr1(&self) -> &OTFDEC_R1KEYR1 {
        &self.otfdec_r1keyr1
    }
    ///0x3c - OTFDEC region 1 key register 2
    #[inline(always)]
    pub const fn otfdec_r1keyr2(&self) -> &OTFDEC_R1KEYR2 {
        &self.otfdec_r1keyr2
    }
    ///0x40 - OTFDEC region 1 key register 3
    #[inline(always)]
    pub const fn otfdec_r1keyr3(&self) -> &OTFDEC_R1KEYR3 {
        &self.otfdec_r1keyr3
    }
    ///0x50 - OTFDEC region 2 configuration register
    #[inline(always)]
    pub const fn otfdec_r2cfgr(&self) -> &OTFDEC_R2CFGR {
        &self.otfdec_r2cfgr
    }
    ///0x54 - OTFDEC region 2 start address register
    #[inline(always)]
    pub const fn otfdec_r2startaddr(&self) -> &OTFDEC_R2STARTADDR {
        &self.otfdec_r2startaddr
    }
    ///0x58 - OTFDEC region 2 end address register
    #[inline(always)]
    pub const fn otfdec_r2endaddr(&self) -> &OTFDEC_R2ENDADDR {
        &self.otfdec_r2endaddr
    }
    ///0x5c - OTFDEC region 2 nonce register 0
    #[inline(always)]
    pub const fn otfdec_r2noncer0(&self) -> &OTFDEC_R2NONCER0 {
        &self.otfdec_r2noncer0
    }
    ///0x60 - OTFDEC region 2 nonce register 1
    #[inline(always)]
    pub const fn otfdec_r2noncer1(&self) -> &OTFDEC_R2NONCER1 {
        &self.otfdec_r2noncer1
    }
    ///0x64 - OTFDEC region 2 key register 0
    #[inline(always)]
    pub const fn otfdec_r2keyr0(&self) -> &OTFDEC_R2KEYR0 {
        &self.otfdec_r2keyr0
    }
    ///0x68 - OTFDEC region 2 key register 1
    #[inline(always)]
    pub const fn otfdec_r2keyr1(&self) -> &OTFDEC_R2KEYR1 {
        &self.otfdec_r2keyr1
    }
    ///0x6c - OTFDEC region 2 key register 2
    #[inline(always)]
    pub const fn otfdec_r2keyr2(&self) -> &OTFDEC_R2KEYR2 {
        &self.otfdec_r2keyr2
    }
    ///0x70 - OTFDEC region 2 key register 3
    #[inline(always)]
    pub const fn otfdec_r2keyr3(&self) -> &OTFDEC_R2KEYR3 {
        &self.otfdec_r2keyr3
    }
    ///0x80 - OTFDEC region 3 configuration register
    #[inline(always)]
    pub const fn otfdec_r3cfgr(&self) -> &OTFDEC_R3CFGR {
        &self.otfdec_r3cfgr
    }
    ///0x84 - OTFDEC region 3 start address register
    #[inline(always)]
    pub const fn otfdec_r3startaddr(&self) -> &OTFDEC_R3STARTADDR {
        &self.otfdec_r3startaddr
    }
    ///0x88 - OTFDEC region 3 end address register
    #[inline(always)]
    pub const fn otfdec_r3endaddr(&self) -> &OTFDEC_R3ENDADDR {
        &self.otfdec_r3endaddr
    }
    ///0x8c - OTFDEC region 3 nonce register 0
    #[inline(always)]
    pub const fn otfdec_r3noncer0(&self) -> &OTFDEC_R3NONCER0 {
        &self.otfdec_r3noncer0
    }
    ///0x90 - OTFDEC region 3 nonce register 1
    #[inline(always)]
    pub const fn otfdec_r3noncer1(&self) -> &OTFDEC_R3NONCER1 {
        &self.otfdec_r3noncer1
    }
    ///0x94 - OTFDEC region 3 key register 0
    #[inline(always)]
    pub const fn otfdec_r3keyr0(&self) -> &OTFDEC_R3KEYR0 {
        &self.otfdec_r3keyr0
    }
    ///0x98 - OTFDEC region 3 key register 1
    #[inline(always)]
    pub const fn otfdec_r3keyr1(&self) -> &OTFDEC_R3KEYR1 {
        &self.otfdec_r3keyr1
    }
    ///0x9c - OTFDEC region 3 key register 2
    #[inline(always)]
    pub const fn otfdec_r3keyr2(&self) -> &OTFDEC_R3KEYR2 {
        &self.otfdec_r3keyr2
    }
    ///0xa0 - OTFDEC region 3 key register 3
    #[inline(always)]
    pub const fn otfdec_r3keyr3(&self) -> &OTFDEC_R3KEYR3 {
        &self.otfdec_r3keyr3
    }
    ///0xb0 - OTFDEC region 4 configuration register
    #[inline(always)]
    pub const fn otfdec_r4cfgr(&self) -> &OTFDEC_R4CFGR {
        &self.otfdec_r4cfgr
    }
    ///0xb4 - OTFDEC region 4 start address register
    #[inline(always)]
    pub const fn otfdec_r4startaddr(&self) -> &OTFDEC_R4STARTADDR {
        &self.otfdec_r4startaddr
    }
    ///0xb8 - OTFDEC region 4 end address register
    #[inline(always)]
    pub const fn otfdec_r4endaddr(&self) -> &OTFDEC_R4ENDADDR {
        &self.otfdec_r4endaddr
    }
    ///0xbc - OTFDEC region 4 nonce register 0
    #[inline(always)]
    pub const fn otfdec_r4noncer0(&self) -> &OTFDEC_R4NONCER0 {
        &self.otfdec_r4noncer0
    }
    ///0xc0 - OTFDEC region 4 nonce register 1
    #[inline(always)]
    pub const fn otfdec_r4noncer1(&self) -> &OTFDEC_R4NONCER1 {
        &self.otfdec_r4noncer1
    }
    ///0xc4 - OTFDEC region 4 key register 0
    #[inline(always)]
    pub const fn otfdec_r4keyr0(&self) -> &OTFDEC_R4KEYR0 {
        &self.otfdec_r4keyr0
    }
    ///0xc8 - OTFDEC region 4 key register 1
    #[inline(always)]
    pub const fn otfdec_r4keyr1(&self) -> &OTFDEC_R4KEYR1 {
        &self.otfdec_r4keyr1
    }
    ///0xcc - OTFDEC region 4 key register 2
    #[inline(always)]
    pub const fn otfdec_r4keyr2(&self) -> &OTFDEC_R4KEYR2 {
        &self.otfdec_r4keyr2
    }
    ///0xd0 - OTFDEC region 4 key register 3
    #[inline(always)]
    pub const fn otfdec_r4keyr3(&self) -> &OTFDEC_R4KEYR3 {
        &self.otfdec_r4keyr3
    }
    ///0x300 - OTFDEC interrupt status register
    #[inline(always)]
    pub const fn otfdec_isr(&self) -> &OTFDEC_ISR {
        &self.otfdec_isr
    }
    ///0x304 - OTFDEC interrupt clear register
    #[inline(always)]
    pub const fn otfdec_icr(&self) -> &OTFDEC_ICR {
        &self.otfdec_icr
    }
    ///0x308 - OTFDEC interrupt enable register
    #[inline(always)]
    pub const fn otfdec_ier(&self) -> &OTFDEC_IER {
        &self.otfdec_ier
    }
}
/**OTFDEC_R1CFGR (rw) register accessor: OTFDEC region 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1CFGR)

For information about available fields see [`mod@otfdec_r1cfgr`]
module*/
pub type OTFDEC_R1CFGR = crate::Reg<otfdec_r1cfgr::OTFDEC_R1CFGRrs>;
///OTFDEC region 1 configuration register
pub mod otfdec_r1cfgr;
/**OTFDEC_R1STARTADDR (rw) register accessor: OTFDEC region 1 start address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1STARTADDR)

For information about available fields see [`mod@otfdec_r1startaddr`]
module*/
pub type OTFDEC_R1STARTADDR = crate::Reg<otfdec_r1startaddr::OTFDEC_R1STARTADDRrs>;
///OTFDEC region 1 start address register
pub mod otfdec_r1startaddr;
/**OTFDEC_R1ENDADDR (rw) register accessor: OTFDEC region 1 end address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1endaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1endaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1ENDADDR)

For information about available fields see [`mod@otfdec_r1endaddr`]
module*/
pub type OTFDEC_R1ENDADDR = crate::Reg<otfdec_r1endaddr::OTFDEC_R1ENDADDRrs>;
///OTFDEC region 1 end address register
pub mod otfdec_r1endaddr;
/**OTFDEC_R1NONCER0 (rw) register accessor: OTFDEC region 1 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1noncer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1noncer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1NONCER0)

For information about available fields see [`mod@otfdec_r1noncer0`]
module*/
pub type OTFDEC_R1NONCER0 = crate::Reg<otfdec_r1noncer0::OTFDEC_R1NONCER0rs>;
///OTFDEC region 1 nonce register 0
pub mod otfdec_r1noncer0;
/**OTFDEC_R1NONCER1 (rw) register accessor: OTFDEC region 1 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1noncer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1noncer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1NONCER1)

For information about available fields see [`mod@otfdec_r1noncer1`]
module*/
pub type OTFDEC_R1NONCER1 = crate::Reg<otfdec_r1noncer1::OTFDEC_R1NONCER1rs>;
///OTFDEC region 1 nonce register 1
pub mod otfdec_r1noncer1;
/**OTFDEC_R1KEYR0 (w) register accessor: OTFDEC region 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1KEYR0)

For information about available fields see [`mod@otfdec_r1keyr0`]
module*/
pub type OTFDEC_R1KEYR0 = crate::Reg<otfdec_r1keyr0::OTFDEC_R1KEYR0rs>;
///OTFDEC region 1 key register 0
pub mod otfdec_r1keyr0;
/**OTFDEC_R1KEYR1 (w) register accessor: OTFDEC region 1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1KEYR1)

For information about available fields see [`mod@otfdec_r1keyr1`]
module*/
pub type OTFDEC_R1KEYR1 = crate::Reg<otfdec_r1keyr1::OTFDEC_R1KEYR1rs>;
///OTFDEC region 1 key register 1
pub mod otfdec_r1keyr1;
/**OTFDEC_R1KEYR2 (w) register accessor: OTFDEC region 1 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1KEYR2)

For information about available fields see [`mod@otfdec_r1keyr2`]
module*/
pub type OTFDEC_R1KEYR2 = crate::Reg<otfdec_r1keyr2::OTFDEC_R1KEYR2rs>;
///OTFDEC region 1 key register 2
pub mod otfdec_r1keyr2;
/**OTFDEC_R1KEYR3 (w) register accessor: OTFDEC region 1 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R1KEYR3)

For information about available fields see [`mod@otfdec_r1keyr3`]
module*/
pub type OTFDEC_R1KEYR3 = crate::Reg<otfdec_r1keyr3::OTFDEC_R1KEYR3rs>;
///OTFDEC region 1 key register 3
pub mod otfdec_r1keyr3;
/**OTFDEC_R2CFGR (rw) register accessor: OTFDEC region 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2CFGR)

For information about available fields see [`mod@otfdec_r2cfgr`]
module*/
pub type OTFDEC_R2CFGR = crate::Reg<otfdec_r2cfgr::OTFDEC_R2CFGRrs>;
///OTFDEC region 2 configuration register
pub mod otfdec_r2cfgr;
/**OTFDEC_R2STARTADDR (rw) register accessor: OTFDEC region 2 start address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r2startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2STARTADDR)

For information about available fields see [`mod@otfdec_r2startaddr`]
module*/
pub type OTFDEC_R2STARTADDR = crate::Reg<otfdec_r2startaddr::OTFDEC_R2STARTADDRrs>;
///OTFDEC region 2 start address register
pub mod otfdec_r2startaddr;
/**OTFDEC_R2ENDADDR (rw) register accessor: OTFDEC region 2 end address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r2endaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2endaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2ENDADDR)

For information about available fields see [`mod@otfdec_r2endaddr`]
module*/
pub type OTFDEC_R2ENDADDR = crate::Reg<otfdec_r2endaddr::OTFDEC_R2ENDADDRrs>;
///OTFDEC region 2 end address register
pub mod otfdec_r2endaddr;
/**OTFDEC_R2NONCER0 (rw) register accessor: OTFDEC region 2 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`otfdec_r2noncer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2noncer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2NONCER0)

For information about available fields see [`mod@otfdec_r2noncer0`]
module*/
pub type OTFDEC_R2NONCER0 = crate::Reg<otfdec_r2noncer0::OTFDEC_R2NONCER0rs>;
///OTFDEC region 2 nonce register 0
pub mod otfdec_r2noncer0;
/**OTFDEC_R2NONCER1 (rw) register accessor: OTFDEC region 2 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`otfdec_r2noncer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2noncer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2NONCER1)

For information about available fields see [`mod@otfdec_r2noncer1`]
module*/
pub type OTFDEC_R2NONCER1 = crate::Reg<otfdec_r2noncer1::OTFDEC_R2NONCER1rs>;
///OTFDEC region 2 nonce register 1
pub mod otfdec_r2noncer1;
/**OTFDEC_R2KEYR0 (w) register accessor: OTFDEC region 2 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2KEYR0)

For information about available fields see [`mod@otfdec_r2keyr0`]
module*/
pub type OTFDEC_R2KEYR0 = crate::Reg<otfdec_r2keyr0::OTFDEC_R2KEYR0rs>;
///OTFDEC region 2 key register 0
pub mod otfdec_r2keyr0;
/**OTFDEC_R2KEYR1 (w) register accessor: OTFDEC region 2 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2KEYR1)

For information about available fields see [`mod@otfdec_r2keyr1`]
module*/
pub type OTFDEC_R2KEYR1 = crate::Reg<otfdec_r2keyr1::OTFDEC_R2KEYR1rs>;
///OTFDEC region 2 key register 1
pub mod otfdec_r2keyr1;
/**OTFDEC_R2KEYR2 (w) register accessor: OTFDEC region 2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2KEYR2)

For information about available fields see [`mod@otfdec_r2keyr2`]
module*/
pub type OTFDEC_R2KEYR2 = crate::Reg<otfdec_r2keyr2::OTFDEC_R2KEYR2rs>;
///OTFDEC region 2 key register 2
pub mod otfdec_r2keyr2;
/**OTFDEC_R2KEYR3 (w) register accessor: OTFDEC region 2 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r2keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R2KEYR3)

For information about available fields see [`mod@otfdec_r2keyr3`]
module*/
pub type OTFDEC_R2KEYR3 = crate::Reg<otfdec_r2keyr3::OTFDEC_R2KEYR3rs>;
///OTFDEC region 2 key register 3
pub mod otfdec_r2keyr3;
/**OTFDEC_R3CFGR (rw) register accessor: OTFDEC region 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3CFGR)

For information about available fields see [`mod@otfdec_r3cfgr`]
module*/
pub type OTFDEC_R3CFGR = crate::Reg<otfdec_r3cfgr::OTFDEC_R3CFGRrs>;
///OTFDEC region 3 configuration register
pub mod otfdec_r3cfgr;
/**OTFDEC_R3STARTADDR (rw) register accessor: OTFDEC region 3 start address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r3startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3STARTADDR)

For information about available fields see [`mod@otfdec_r3startaddr`]
module*/
pub type OTFDEC_R3STARTADDR = crate::Reg<otfdec_r3startaddr::OTFDEC_R3STARTADDRrs>;
///OTFDEC region 3 start address register
pub mod otfdec_r3startaddr;
/**OTFDEC_R3ENDADDR (rw) register accessor: OTFDEC region 3 end address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r3endaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3endaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3ENDADDR)

For information about available fields see [`mod@otfdec_r3endaddr`]
module*/
pub type OTFDEC_R3ENDADDR = crate::Reg<otfdec_r3endaddr::OTFDEC_R3ENDADDRrs>;
///OTFDEC region 3 end address register
pub mod otfdec_r3endaddr;
/**OTFDEC_R3NONCER0 (rw) register accessor: OTFDEC region 3 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`otfdec_r3noncer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3noncer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3NONCER0)

For information about available fields see [`mod@otfdec_r3noncer0`]
module*/
pub type OTFDEC_R3NONCER0 = crate::Reg<otfdec_r3noncer0::OTFDEC_R3NONCER0rs>;
///OTFDEC region 3 nonce register 0
pub mod otfdec_r3noncer0;
/**OTFDEC_R3NONCER1 (rw) register accessor: OTFDEC region 3 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`otfdec_r3noncer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3noncer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3NONCER1)

For information about available fields see [`mod@otfdec_r3noncer1`]
module*/
pub type OTFDEC_R3NONCER1 = crate::Reg<otfdec_r3noncer1::OTFDEC_R3NONCER1rs>;
///OTFDEC region 3 nonce register 1
pub mod otfdec_r3noncer1;
/**OTFDEC_R3KEYR0 (w) register accessor: OTFDEC region 3 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3KEYR0)

For information about available fields see [`mod@otfdec_r3keyr0`]
module*/
pub type OTFDEC_R3KEYR0 = crate::Reg<otfdec_r3keyr0::OTFDEC_R3KEYR0rs>;
///OTFDEC region 3 key register 0
pub mod otfdec_r3keyr0;
/**OTFDEC_R3KEYR1 (w) register accessor: OTFDEC region 3 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3KEYR1)

For information about available fields see [`mod@otfdec_r3keyr1`]
module*/
pub type OTFDEC_R3KEYR1 = crate::Reg<otfdec_r3keyr1::OTFDEC_R3KEYR1rs>;
///OTFDEC region 3 key register 1
pub mod otfdec_r3keyr1;
/**OTFDEC_R3KEYR2 (w) register accessor: OTFDEC region 3 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3KEYR2)

For information about available fields see [`mod@otfdec_r3keyr2`]
module*/
pub type OTFDEC_R3KEYR2 = crate::Reg<otfdec_r3keyr2::OTFDEC_R3KEYR2rs>;
///OTFDEC region 3 key register 2
pub mod otfdec_r3keyr2;
/**OTFDEC_R3KEYR3 (w) register accessor: OTFDEC region 3 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R3KEYR3)

For information about available fields see [`mod@otfdec_r3keyr3`]
module*/
pub type OTFDEC_R3KEYR3 = crate::Reg<otfdec_r3keyr3::OTFDEC_R3KEYR3rs>;
///OTFDEC region 3 key register 3
pub mod otfdec_r3keyr3;
/**OTFDEC_R4CFGR (rw) register accessor: OTFDEC region 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r4cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4CFGR)

For information about available fields see [`mod@otfdec_r4cfgr`]
module*/
pub type OTFDEC_R4CFGR = crate::Reg<otfdec_r4cfgr::OTFDEC_R4CFGRrs>;
///OTFDEC region 4 configuration register
pub mod otfdec_r4cfgr;
/**OTFDEC_R4STARTADDR (rw) register accessor: OTFDEC region 4 start address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r4startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4STARTADDR)

For information about available fields see [`mod@otfdec_r4startaddr`]
module*/
pub type OTFDEC_R4STARTADDR = crate::Reg<otfdec_r4startaddr::OTFDEC_R4STARTADDRrs>;
///OTFDEC region 4 start address register
pub mod otfdec_r4startaddr;
/**OTFDEC_R4ENDADDR (rw) register accessor: OTFDEC region 4 end address register

You can [`read`](crate::Reg::read) this register and get [`otfdec_r4endaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4endaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4ENDADDR)

For information about available fields see [`mod@otfdec_r4endaddr`]
module*/
pub type OTFDEC_R4ENDADDR = crate::Reg<otfdec_r4endaddr::OTFDEC_R4ENDADDRrs>;
///OTFDEC region 4 end address register
pub mod otfdec_r4endaddr;
/**OTFDEC_R4NONCER0 (rw) register accessor: OTFDEC region 4 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`otfdec_r4noncer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4noncer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4NONCER0)

For information about available fields see [`mod@otfdec_r4noncer0`]
module*/
pub type OTFDEC_R4NONCER0 = crate::Reg<otfdec_r4noncer0::OTFDEC_R4NONCER0rs>;
///OTFDEC region 4 nonce register 0
pub mod otfdec_r4noncer0;
/**OTFDEC_R4NONCER1 (rw) register accessor: OTFDEC region 4 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`otfdec_r4noncer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4noncer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4NONCER1)

For information about available fields see [`mod@otfdec_r4noncer1`]
module*/
pub type OTFDEC_R4NONCER1 = crate::Reg<otfdec_r4noncer1::OTFDEC_R4NONCER1rs>;
///OTFDEC region 4 nonce register 1
pub mod otfdec_r4noncer1;
/**OTFDEC_R4KEYR0 (w) register accessor: OTFDEC region 4 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4KEYR0)

For information about available fields see [`mod@otfdec_r4keyr0`]
module*/
pub type OTFDEC_R4KEYR0 = crate::Reg<otfdec_r4keyr0::OTFDEC_R4KEYR0rs>;
///OTFDEC region 4 key register 0
pub mod otfdec_r4keyr0;
/**OTFDEC_R4KEYR1 (w) register accessor: OTFDEC region 4 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4KEYR1)

For information about available fields see [`mod@otfdec_r4keyr1`]
module*/
pub type OTFDEC_R4KEYR1 = crate::Reg<otfdec_r4keyr1::OTFDEC_R4KEYR1rs>;
///OTFDEC region 4 key register 1
pub mod otfdec_r4keyr1;
/**OTFDEC_R4KEYR2 (w) register accessor: OTFDEC region 4 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4KEYR2)

For information about available fields see [`mod@otfdec_r4keyr2`]
module*/
pub type OTFDEC_R4KEYR2 = crate::Reg<otfdec_r4keyr2::OTFDEC_R4KEYR2rs>;
///OTFDEC region 4 key register 2
pub mod otfdec_r4keyr2;
/**OTFDEC_R4KEYR3 (w) register accessor: OTFDEC region 4 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r4keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_R4KEYR3)

For information about available fields see [`mod@otfdec_r4keyr3`]
module*/
pub type OTFDEC_R4KEYR3 = crate::Reg<otfdec_r4keyr3::OTFDEC_R4KEYR3rs>;
///OTFDEC region 4 key register 3
pub mod otfdec_r4keyr3;
/**OTFDEC_ISR (r) register accessor: OTFDEC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`otfdec_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_ISR)

For information about available fields see [`mod@otfdec_isr`]
module*/
pub type OTFDEC_ISR = crate::Reg<otfdec_isr::OTFDEC_ISRrs>;
///OTFDEC interrupt status register
pub mod otfdec_isr;
/**OTFDEC_ICR (w) register accessor: OTFDEC interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_ICR)

For information about available fields see [`mod@otfdec_icr`]
module*/
pub type OTFDEC_ICR = crate::Reg<otfdec_icr::OTFDEC_ICRrs>;
///OTFDEC interrupt clear register
pub mod otfdec_icr;
/**OTFDEC_IER (rw) register accessor: OTFDEC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`otfdec_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:OTFDEC_IER)

For information about available fields see [`mod@otfdec_ier`]
module*/
pub type OTFDEC_IER = crate::Reg<otfdec_ier::OTFDEC_IERrs>;
///OTFDEC interrupt enable register
pub mod otfdec_ier;
