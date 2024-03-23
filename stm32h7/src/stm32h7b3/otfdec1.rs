#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x1c],
    r1cfgr: R1CFGR,
    r1startaddr: R1STARTADDR,
    r1endaddr: R1ENDADDR,
    r1noncer0: R1NONCER0,
    r1noncer1: R1NONCER1,
    r1keyr0: R1KEYR0,
    r1keyr1: R1KEYR1,
    r1keyr2: R1KEYR2,
    r1keyr3: R1KEYR3,
    _reserved10: [u8; 0x0c],
    r2cfgr: R2CFGR,
    r2startaddr: R2STARTADDR,
    r2endaddr: R2ENDADDR,
    r2noncer0: R2NONCER0,
    r2noncer1: R2NONCER1,
    r2keyr0: R2KEYR0,
    r2keyr1: R2KEYR1,
    r2keyr2: R2KEYR2,
    r2keyr3: R2KEYR3,
    _reserved19: [u8; 0x0c],
    r3cfgr: R3CFGR,
    r3startaddr: R3STARTADDR,
    r3endaddr: R3ENDADDR,
    _reserved_22_r3noncer0: [u8; 0x04],
    r3noncer1: R3NONCER1,
    r3keyr0: R3KEYR0,
    r3keyr1: R3KEYR1,
    r3keyr2: R3KEYR2,
    r3keyr3: R3KEYR3,
    _reserved28: [u8; 0x0c],
    r4cfgr: R4CFGR,
    r4startaddr: R4STARTADDR,
    _reserved30: [u8; 0x04],
    r4noncer0: R4NONCER0,
    r4noncer1: R4NONCER1,
    r4keyr0: R4KEYR0,
    r4keyr1: R4KEYR1,
    r4keyr2: R4KEYR2,
    r4keyr3: R4KEYR3,
    _reserved36: [u8; 0x022c],
    isr: ISR,
    icr: ICR,
    ier: IER,
}
impl RegisterBlock {
    #[doc = "0x00 - OTFDEC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x20 - OTFDEC region x configuration register"]
    #[inline(always)]
    pub const fn r1cfgr(&self) -> &R1CFGR {
        &self.r1cfgr
    }
    #[doc = "0x24 - OTFDEC region x start address register"]
    #[inline(always)]
    pub const fn r1startaddr(&self) -> &R1STARTADDR {
        &self.r1startaddr
    }
    #[doc = "0x28 - OTFDEC region x end address register"]
    #[inline(always)]
    pub const fn r1endaddr(&self) -> &R1ENDADDR {
        &self.r1endaddr
    }
    #[doc = "0x2c - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub const fn r1noncer0(&self) -> &R1NONCER0 {
        &self.r1noncer0
    }
    #[doc = "0x30 - OTFDEC region x nonce register 1"]
    #[inline(always)]
    pub const fn r1noncer1(&self) -> &R1NONCER1 {
        &self.r1noncer1
    }
    #[doc = "0x34 - OTFDEC region x key register 0"]
    #[inline(always)]
    pub const fn r1keyr0(&self) -> &R1KEYR0 {
        &self.r1keyr0
    }
    #[doc = "0x38 - OTFDEC region x key register 1"]
    #[inline(always)]
    pub const fn r1keyr1(&self) -> &R1KEYR1 {
        &self.r1keyr1
    }
    #[doc = "0x3c - OTFDEC region x key register 2"]
    #[inline(always)]
    pub const fn r1keyr2(&self) -> &R1KEYR2 {
        &self.r1keyr2
    }
    #[doc = "0x40 - OTFDEC region x key register 3"]
    #[inline(always)]
    pub const fn r1keyr3(&self) -> &R1KEYR3 {
        &self.r1keyr3
    }
    #[doc = "0x50 - OTFDEC region x configuration register"]
    #[inline(always)]
    pub const fn r2cfgr(&self) -> &R2CFGR {
        &self.r2cfgr
    }
    #[doc = "0x54 - OTFDEC region x start address register"]
    #[inline(always)]
    pub const fn r2startaddr(&self) -> &R2STARTADDR {
        &self.r2startaddr
    }
    #[doc = "0x58 - OTFDEC region x end address register"]
    #[inline(always)]
    pub const fn r2endaddr(&self) -> &R2ENDADDR {
        &self.r2endaddr
    }
    #[doc = "0x5c - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub const fn r2noncer0(&self) -> &R2NONCER0 {
        &self.r2noncer0
    }
    #[doc = "0x60 - OTFDEC region x nonce register 1"]
    #[inline(always)]
    pub const fn r2noncer1(&self) -> &R2NONCER1 {
        &self.r2noncer1
    }
    #[doc = "0x64 - OTFDEC region x key register 0"]
    #[inline(always)]
    pub const fn r2keyr0(&self) -> &R2KEYR0 {
        &self.r2keyr0
    }
    #[doc = "0x68 - OTFDEC region x key register 1"]
    #[inline(always)]
    pub const fn r2keyr1(&self) -> &R2KEYR1 {
        &self.r2keyr1
    }
    #[doc = "0x6c - OTFDEC region x key register 2"]
    #[inline(always)]
    pub const fn r2keyr2(&self) -> &R2KEYR2 {
        &self.r2keyr2
    }
    #[doc = "0x70 - OTFDEC region x key register 3"]
    #[inline(always)]
    pub const fn r2keyr3(&self) -> &R2KEYR3 {
        &self.r2keyr3
    }
    #[doc = "0x80 - OTFDEC region x configuration register"]
    #[inline(always)]
    pub const fn r3cfgr(&self) -> &R3CFGR {
        &self.r3cfgr
    }
    #[doc = "0x84 - OTFDEC region x start address register"]
    #[inline(always)]
    pub const fn r3startaddr(&self) -> &R3STARTADDR {
        &self.r3startaddr
    }
    #[doc = "0x88 - OTFDEC region x end address register"]
    #[inline(always)]
    pub const fn r3endaddr(&self) -> &R3ENDADDR {
        &self.r3endaddr
    }
    #[doc = "0x8c - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub const fn r3noncer0(&self) -> &R3NONCER0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - OTFDEC region x end address register"]
    #[inline(always)]
    pub const fn r4endaddr(&self) -> &R4ENDADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - OTFDEC region x nonce register 1"]
    #[inline(always)]
    pub const fn r3noncer1(&self) -> &R3NONCER1 {
        &self.r3noncer1
    }
    #[doc = "0x94 - OTFDEC region x key register 0"]
    #[inline(always)]
    pub const fn r3keyr0(&self) -> &R3KEYR0 {
        &self.r3keyr0
    }
    #[doc = "0x98 - OTFDEC region x key register 1"]
    #[inline(always)]
    pub const fn r3keyr1(&self) -> &R3KEYR1 {
        &self.r3keyr1
    }
    #[doc = "0x9c - OTFDEC region x key register 2"]
    #[inline(always)]
    pub const fn r3keyr2(&self) -> &R3KEYR2 {
        &self.r3keyr2
    }
    #[doc = "0xa0 - OTFDEC region x key register 3"]
    #[inline(always)]
    pub const fn r3keyr3(&self) -> &R3KEYR3 {
        &self.r3keyr3
    }
    #[doc = "0xb0 - OTFDEC region x configuration register"]
    #[inline(always)]
    pub const fn r4cfgr(&self) -> &R4CFGR {
        &self.r4cfgr
    }
    #[doc = "0xb4 - OTFDEC region x start address register"]
    #[inline(always)]
    pub const fn r4startaddr(&self) -> &R4STARTADDR {
        &self.r4startaddr
    }
    #[doc = "0xbc - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub const fn r4noncer0(&self) -> &R4NONCER0 {
        &self.r4noncer0
    }
    #[doc = "0xc0 - OTFDEC region x nonce register 1"]
    #[inline(always)]
    pub const fn r4noncer1(&self) -> &R4NONCER1 {
        &self.r4noncer1
    }
    #[doc = "0xc4 - OTFDEC region x key register 0"]
    #[inline(always)]
    pub const fn r4keyr0(&self) -> &R4KEYR0 {
        &self.r4keyr0
    }
    #[doc = "0xc8 - OTFDEC region x key register 1"]
    #[inline(always)]
    pub const fn r4keyr1(&self) -> &R4KEYR1 {
        &self.r4keyr1
    }
    #[doc = "0xcc - OTFDEC region x key register 2"]
    #[inline(always)]
    pub const fn r4keyr2(&self) -> &R4KEYR2 {
        &self.r4keyr2
    }
    #[doc = "0xd0 - OTFDEC region x key register 3"]
    #[inline(always)]
    pub const fn r4keyr3(&self) -> &R4KEYR3 {
        &self.r4keyr3
    }
    #[doc = "0x300 - OTFDEC interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x304 - OTFDEC interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x308 - OTFDEC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
}
#[doc = "CR (rw) register accessor: OTFDEC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "OTFDEC control register"]
pub mod cr;
#[doc = "R1CFGR (rw) register accessor: OTFDEC region x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1cfgr`]
module"]
pub type R1CFGR = crate::Reg<r1cfgr::R1CFGRrs>;
#[doc = "OTFDEC region x configuration register"]
pub mod r1cfgr;
#[doc = "R2CFGR (rw) register accessor: OTFDEC region x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2cfgr`]
module"]
pub type R2CFGR = crate::Reg<r2cfgr::R2CFGRrs>;
#[doc = "OTFDEC region x configuration register"]
pub mod r2cfgr;
#[doc = "R3CFGR (rw) register accessor: OTFDEC region x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3cfgr`]
module"]
pub type R3CFGR = crate::Reg<r3cfgr::R3CFGRrs>;
#[doc = "OTFDEC region x configuration register"]
pub mod r3cfgr;
#[doc = "R4CFGR (rw) register accessor: OTFDEC region x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4cfgr`]
module"]
pub type R4CFGR = crate::Reg<r4cfgr::R4CFGRrs>;
#[doc = "OTFDEC region x configuration register"]
pub mod r4cfgr;
#[doc = "R1STARTADDR (rw) register accessor: OTFDEC region x start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1startaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1startaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1startaddr`]
module"]
pub type R1STARTADDR = crate::Reg<r1startaddr::R1STARTADDRrs>;
#[doc = "OTFDEC region x start address register"]
pub mod r1startaddr;
#[doc = "R2STARTADDR (rw) register accessor: OTFDEC region x start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2startaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2startaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2startaddr`]
module"]
pub type R2STARTADDR = crate::Reg<r2startaddr::R2STARTADDRrs>;
#[doc = "OTFDEC region x start address register"]
pub mod r2startaddr;
#[doc = "R3STARTADDR (rw) register accessor: OTFDEC region x start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3startaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3startaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3startaddr`]
module"]
pub type R3STARTADDR = crate::Reg<r3startaddr::R3STARTADDRrs>;
#[doc = "OTFDEC region x start address register"]
pub mod r3startaddr;
#[doc = "R4STARTADDR (rw) register accessor: OTFDEC region x start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4startaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4startaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4startaddr`]
module"]
pub type R4STARTADDR = crate::Reg<r4startaddr::R4STARTADDRrs>;
#[doc = "OTFDEC region x start address register"]
pub mod r4startaddr;
#[doc = "R1ENDADDR (rw) register accessor: OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1endaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1endaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1endaddr`]
module"]
pub type R1ENDADDR = crate::Reg<r1endaddr::R1ENDADDRrs>;
#[doc = "OTFDEC region x end address register"]
pub mod r1endaddr;
#[doc = "R2ENDADDR (rw) register accessor: OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2endaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2endaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2endaddr`]
module"]
pub type R2ENDADDR = crate::Reg<r2endaddr::R2ENDADDRrs>;
#[doc = "OTFDEC region x end address register"]
pub mod r2endaddr;
#[doc = "R3ENDADDR (rw) register accessor: OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3endaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3endaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3endaddr`]
module"]
pub type R3ENDADDR = crate::Reg<r3endaddr::R3ENDADDRrs>;
#[doc = "OTFDEC region x end address register"]
pub mod r3endaddr;
#[doc = "R4ENDADDR (rw) register accessor: OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4endaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4endaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4endaddr`]
module"]
pub type R4ENDADDR = crate::Reg<r4endaddr::R4ENDADDRrs>;
#[doc = "OTFDEC region x end address register"]
pub mod r4endaddr;
#[doc = "R1NONCER0 (rw) register accessor: OTFDEC region x nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1noncer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1noncer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1noncer0`]
module"]
pub type R1NONCER0 = crate::Reg<r1noncer0::R1NONCER0rs>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r1noncer0;
#[doc = "R2NONCER0 (rw) register accessor: OTFDEC region x nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2noncer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2noncer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2noncer0`]
module"]
pub type R2NONCER0 = crate::Reg<r2noncer0::R2NONCER0rs>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r2noncer0;
#[doc = "R3NONCER0 (rw) register accessor: OTFDEC region x nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3noncer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3noncer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3noncer0`]
module"]
pub type R3NONCER0 = crate::Reg<r3noncer0::R3NONCER0rs>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r3noncer0;
#[doc = "R4NONCER0 (rw) register accessor: OTFDEC region x nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4noncer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4noncer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4noncer0`]
module"]
pub type R4NONCER0 = crate::Reg<r4noncer0::R4NONCER0rs>;
#[doc = "OTFDEC region x nonce register 0"]
pub mod r4noncer0;
#[doc = "R1NONCER1 (rw) register accessor: OTFDEC region x nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1noncer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1noncer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1noncer1`]
module"]
pub type R1NONCER1 = crate::Reg<r1noncer1::R1NONCER1rs>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r1noncer1;
#[doc = "R2NONCER1 (rw) register accessor: OTFDEC region x nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2noncer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2noncer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2noncer1`]
module"]
pub type R2NONCER1 = crate::Reg<r2noncer1::R2NONCER1rs>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r2noncer1;
#[doc = "R3NONCER1 (rw) register accessor: OTFDEC region x nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3noncer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3noncer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3noncer1`]
module"]
pub type R3NONCER1 = crate::Reg<r3noncer1::R3NONCER1rs>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r3noncer1;
#[doc = "R4NONCER1 (rw) register accessor: OTFDEC region x nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4noncer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4noncer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4noncer1`]
module"]
pub type R4NONCER1 = crate::Reg<r4noncer1::R4NONCER1rs>;
#[doc = "OTFDEC region x nonce register 1"]
pub mod r4noncer1;
#[doc = "R1KEYR0 (w) register accessor: OTFDEC region x key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1keyr0`]
module"]
pub type R1KEYR0 = crate::Reg<r1keyr0::R1KEYR0rs>;
#[doc = "OTFDEC region x key register 0"]
pub mod r1keyr0;
#[doc = "R2KEYR0 (w) register accessor: OTFDEC region x key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2keyr0`]
module"]
pub type R2KEYR0 = crate::Reg<r2keyr0::R2KEYR0rs>;
#[doc = "OTFDEC region x key register 0"]
pub mod r2keyr0;
#[doc = "R3KEYR0 (w) register accessor: OTFDEC region x key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3keyr0`]
module"]
pub type R3KEYR0 = crate::Reg<r3keyr0::R3KEYR0rs>;
#[doc = "OTFDEC region x key register 0"]
pub mod r3keyr0;
#[doc = "R4KEYR0 (w) register accessor: OTFDEC region x key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4keyr0`]
module"]
pub type R4KEYR0 = crate::Reg<r4keyr0::R4KEYR0rs>;
#[doc = "OTFDEC region x key register 0"]
pub mod r4keyr0;
#[doc = "R1KEYR1 (w) register accessor: OTFDEC region x key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1keyr1`]
module"]
pub type R1KEYR1 = crate::Reg<r1keyr1::R1KEYR1rs>;
#[doc = "OTFDEC region x key register 1"]
pub mod r1keyr1;
#[doc = "R2KEYR1 (w) register accessor: OTFDEC region x key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2keyr1`]
module"]
pub type R2KEYR1 = crate::Reg<r2keyr1::R2KEYR1rs>;
#[doc = "OTFDEC region x key register 1"]
pub mod r2keyr1;
#[doc = "R3KEYR1 (w) register accessor: OTFDEC region x key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3keyr1`]
module"]
pub type R3KEYR1 = crate::Reg<r3keyr1::R3KEYR1rs>;
#[doc = "OTFDEC region x key register 1"]
pub mod r3keyr1;
#[doc = "R4KEYR1 (w) register accessor: OTFDEC region x key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4keyr1`]
module"]
pub type R4KEYR1 = crate::Reg<r4keyr1::R4KEYR1rs>;
#[doc = "OTFDEC region x key register 1"]
pub mod r4keyr1;
#[doc = "R1KEYR2 (w) register accessor: OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1keyr2`]
module"]
pub type R1KEYR2 = crate::Reg<r1keyr2::R1KEYR2rs>;
#[doc = "OTFDEC region x key register 2"]
pub mod r1keyr2;
#[doc = "R2KEYR2 (w) register accessor: OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2keyr2`]
module"]
pub type R2KEYR2 = crate::Reg<r2keyr2::R2KEYR2rs>;
#[doc = "OTFDEC region x key register 2"]
pub mod r2keyr2;
#[doc = "R3KEYR2 (w) register accessor: OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3keyr2`]
module"]
pub type R3KEYR2 = crate::Reg<r3keyr2::R3KEYR2rs>;
#[doc = "OTFDEC region x key register 2"]
pub mod r3keyr2;
#[doc = "R4KEYR2 (w) register accessor: OTFDEC region x key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4keyr2`]
module"]
pub type R4KEYR2 = crate::Reg<r4keyr2::R4KEYR2rs>;
#[doc = "OTFDEC region x key register 2"]
pub mod r4keyr2;
#[doc = "R1KEYR3 (w) register accessor: OTFDEC region x key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r1keyr3`]
module"]
pub type R1KEYR3 = crate::Reg<r1keyr3::R1KEYR3rs>;
#[doc = "OTFDEC region x key register 3"]
pub mod r1keyr3;
#[doc = "R2KEYR3 (w) register accessor: OTFDEC region x key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2keyr3`]
module"]
pub type R2KEYR3 = crate::Reg<r2keyr3::R2KEYR3rs>;
#[doc = "OTFDEC region x key register 3"]
pub mod r2keyr3;
#[doc = "R3KEYR3 (w) register accessor: OTFDEC region x key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r3keyr3`]
module"]
pub type R3KEYR3 = crate::Reg<r3keyr3::R3KEYR3rs>;
#[doc = "OTFDEC region x key register 3"]
pub mod r3keyr3;
#[doc = "R4KEYR3 (w) register accessor: OTFDEC region x key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r4keyr3`]
module"]
pub type R4KEYR3 = crate::Reg<r4keyr3::R4KEYR3rs>;
#[doc = "OTFDEC region x key register 3"]
pub mod r4keyr3;
#[doc = "ISR (r) register accessor: OTFDEC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "OTFDEC interrupt status register"]
pub mod isr;
#[doc = "ICR (r) register accessor: OTFDEC interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "OTFDEC interrupt clear register"]
pub mod icr;
#[doc = "IER (rw) register accessor: OTFDEC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "OTFDEC interrupt enable register"]
pub mod ier;
