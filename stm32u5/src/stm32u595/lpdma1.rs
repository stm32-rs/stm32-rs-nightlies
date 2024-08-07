#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    _reserved2: [u8; 0x04],
    misr: MISR,
    smisr: SMISR,
    _reserved4: [u8; 0x3c],
    c0lbar: C0LBAR,
    _reserved5: [u8; 0x08],
    c0fcr: C0FCR,
    c0sr: C0SR,
    c0cr: C0CR,
    _reserved8: [u8; 0x28],
    c0tr1: C0TR1,
    c0tr2: C0TR2,
    c0br1: C0BR1,
    c0sar: C0SAR,
    c0dar: C0DAR,
    _reserved13: [u8; 0x28],
    c0llr: C0LLR,
    c1lbar: C1LBAR,
    _reserved15: [u8; 0x08],
    c1fcr: C1FCR,
    c1sr: C1SR,
    c1cr: C1CR,
    _reserved18: [u8; 0x28],
    c1tr1: C1TR1,
    c1tr2: C1TR2,
    c1br1: C1BR1,
    c1sar: C1SAR,
    c1dar: C1DAR,
    _reserved23: [u8; 0x28],
    c1llr: C1LLR,
    c2lbar: C2LBAR,
    _reserved25: [u8; 0x08],
    c2fcr: C2FCR,
    c2sr: C2SR,
    c2cr: C2CR,
    _reserved28: [u8; 0x28],
    c2tr1: C2TR1,
    c2tr2: C2TR2,
    c2br1: C2BR1,
    c2sar: C2SAR,
    c2dar: C2DAR,
    _reserved33: [u8; 0x28],
    c2llr: C2LLR,
    c3lbar: C3LBAR,
    _reserved35: [u8; 0x08],
    c3fcr: C3FCR,
    c3sr: C3SR,
    c3cr: C3CR,
    _reserved38: [u8; 0x28],
    c3tr1: C3TR1,
    c3tr2: C3TR2,
    c3br1: C3BR1,
    c3sar: C3SAR,
    c3dar: C3DAR,
    _reserved43: [u8; 0x28],
    c3llr: C3LLR,
}
impl RegisterBlock {
    ///0x00 - LPDMA secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - LPDMA privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x0c - LPDMA non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x10 - LPDMA secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x50 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c0lbar(&self) -> &C0LBAR {
        &self.c0lbar
    }
    ///0x5c - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c0fcr(&self) -> &C0FCR {
        &self.c0fcr
    }
    ///0x60 - channel x status register
    #[inline(always)]
    pub const fn c0sr(&self) -> &C0SR {
        &self.c0sr
    }
    ///0x64 - channel x control register
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0CR {
        &self.c0cr
    }
    ///0x90 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c0tr1(&self) -> &C0TR1 {
        &self.c0tr1
    }
    ///0x94 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c0tr2(&self) -> &C0TR2 {
        &self.c0tr2
    }
    ///0x98 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn c0br1(&self) -> &C0BR1 {
        &self.c0br1
    }
    ///0x9c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn c0sar(&self) -> &C0SAR {
        &self.c0sar
    }
    ///0xa0 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn c0dar(&self) -> &C0DAR {
        &self.c0dar
    }
    ///0xcc - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c0llr(&self) -> &C0LLR {
        &self.c0llr
    }
    ///0xd0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c1lbar(&self) -> &C1LBAR {
        &self.c1lbar
    }
    ///0xdc - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c1fcr(&self) -> &C1FCR {
        &self.c1fcr
    }
    ///0xe0 - channel x status register
    #[inline(always)]
    pub const fn c1sr(&self) -> &C1SR {
        &self.c1sr
    }
    ///0xe4 - channel x control register
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x110 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c1tr1(&self) -> &C1TR1 {
        &self.c1tr1
    }
    ///0x114 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c1tr2(&self) -> &C1TR2 {
        &self.c1tr2
    }
    ///0x118 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn c1br1(&self) -> &C1BR1 {
        &self.c1br1
    }
    ///0x11c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn c1sar(&self) -> &C1SAR {
        &self.c1sar
    }
    ///0x120 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn c1dar(&self) -> &C1DAR {
        &self.c1dar
    }
    ///0x14c - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c1llr(&self) -> &C1LLR {
        &self.c1llr
    }
    ///0x150 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c2lbar(&self) -> &C2LBAR {
        &self.c2lbar
    }
    ///0x15c - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c2fcr(&self) -> &C2FCR {
        &self.c2fcr
    }
    ///0x160 - channel x status register
    #[inline(always)]
    pub const fn c2sr(&self) -> &C2SR {
        &self.c2sr
    }
    ///0x164 - channel x control register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x190 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c2tr1(&self) -> &C2TR1 {
        &self.c2tr1
    }
    ///0x194 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c2tr2(&self) -> &C2TR2 {
        &self.c2tr2
    }
    ///0x198 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn c2br1(&self) -> &C2BR1 {
        &self.c2br1
    }
    ///0x19c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn c2sar(&self) -> &C2SAR {
        &self.c2sar
    }
    ///0x1a0 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn c2dar(&self) -> &C2DAR {
        &self.c2dar
    }
    ///0x1cc - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c2llr(&self) -> &C2LLR {
        &self.c2llr
    }
    ///0x1d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c3lbar(&self) -> &C3LBAR {
        &self.c3lbar
    }
    ///0x1dc - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c3fcr(&self) -> &C3FCR {
        &self.c3fcr
    }
    ///0x1e0 - channel x status register
    #[inline(always)]
    pub const fn c3sr(&self) -> &C3SR {
        &self.c3sr
    }
    ///0x1e4 - channel x control register
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3CR {
        &self.c3cr
    }
    ///0x210 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c3tr1(&self) -> &C3TR1 {
        &self.c3tr1
    }
    ///0x214 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c3tr2(&self) -> &C3TR2 {
        &self.c3tr2
    }
    ///0x218 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn c3br1(&self) -> &C3BR1 {
        &self.c3br1
    }
    ///0x21c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn c3sar(&self) -> &C3SAR {
        &self.c3sar
    }
    ///0x220 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn c3dar(&self) -> &C3DAR {
        &self.c3dar
    }
    ///0x24c - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c3llr(&self) -> &C3LLR {
        &self.c3llr
    }
}
/**SECCFGR (rw) register accessor: LPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:SECCFGR)

For information about available fields see [`mod@seccfgr`]
module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///LPDMA secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: LPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`]
module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///LPDMA privileged configuration register
pub mod privcfgr;
/**MISR (r) register accessor: LPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///LPDMA non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: LPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:SMISR)

For information about available fields see [`mod@smisr`]
module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///LPDMA secure masked interrupt status register
pub mod smisr;
/**C0LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c0lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0LBAR)

For information about available fields see [`mod@c0lbar`]
module*/
pub type C0LBAR = crate::Reg<c0lbar::C0LBARrs>;
///channel x linked-list base address register
pub mod c0lbar;
/**C0FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0FCR)

For information about available fields see [`mod@c0fcr`]
module*/
pub type C0FCR = crate::Reg<c0fcr::C0FCRrs>;
///LPDMA channel x flag clear register
pub mod c0fcr;
/**C0SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0SR)

For information about available fields see [`mod@c0sr`]
module*/
pub type C0SR = crate::Reg<c0sr::C0SRrs>;
///channel x status register
pub mod c0sr;
/**C0CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0CR)

For information about available fields see [`mod@c0cr`]
module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///channel x control register
pub mod c0cr;
/**C0TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c0tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0TR1)

For information about available fields see [`mod@c0tr1`]
module*/
pub type C0TR1 = crate::Reg<c0tr1::C0TR1rs>;
///LPDMA channel x transfer register 1
pub mod c0tr1;
/**C0TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c0tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0TR2)

For information about available fields see [`mod@c0tr2`]
module*/
pub type C0TR2 = crate::Reg<c0tr2::C0TR2rs>;
///LPDMA channel x transfer register 2
pub mod c0tr2;
/**C0BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0BR1)

For information about available fields see [`mod@c0br1`]
module*/
pub type C0BR1 = crate::Reg<c0br1::C0BR1rs>;
///LPDMA channel x block register 1
pub mod c0br1;
/**C0SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0SAR)

For information about available fields see [`mod@c0sar`]
module*/
pub type C0SAR = crate::Reg<c0sar::C0SARrs>;
///LPDMA channel x source address register
pub mod c0sar;
/**C0DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0DAR)

For information about available fields see [`mod@c0dar`]
module*/
pub type C0DAR = crate::Reg<c0dar::C0DARrs>;
///LPDMA channel x destination address register
pub mod c0dar;
/**C0LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c0llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C0LLR)

For information about available fields see [`mod@c0llr`]
module*/
pub type C0LLR = crate::Reg<c0llr::C0LLRrs>;
///LPDMA channel x linked-list address register
pub mod c0llr;
/**C1LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c1lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1LBAR)

For information about available fields see [`mod@c1lbar`]
module*/
pub type C1LBAR = crate::Reg<c1lbar::C1LBARrs>;
///channel x linked-list base address register
pub mod c1lbar;
/**C1FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1FCR)

For information about available fields see [`mod@c1fcr`]
module*/
pub type C1FCR = crate::Reg<c1fcr::C1FCRrs>;
///LPDMA channel x flag clear register
pub mod c1fcr;
/**C1SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1SR)

For information about available fields see [`mod@c1sr`]
module*/
pub type C1SR = crate::Reg<c1sr::C1SRrs>;
///channel x status register
pub mod c1sr;
/**C1CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1CR)

For information about available fields see [`mod@c1cr`]
module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///channel x control register
pub mod c1cr;
/**C1TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c1tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1TR1)

For information about available fields see [`mod@c1tr1`]
module*/
pub type C1TR1 = crate::Reg<c1tr1::C1TR1rs>;
///LPDMA channel x transfer register 1
pub mod c1tr1;
/**C1TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c1tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1TR2)

For information about available fields see [`mod@c1tr2`]
module*/
pub type C1TR2 = crate::Reg<c1tr2::C1TR2rs>;
///LPDMA channel x transfer register 2
pub mod c1tr2;
/**C1BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1BR1)

For information about available fields see [`mod@c1br1`]
module*/
pub type C1BR1 = crate::Reg<c1br1::C1BR1rs>;
///LPDMA channel x block register 1
pub mod c1br1;
/**C1SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1SAR)

For information about available fields see [`mod@c1sar`]
module*/
pub type C1SAR = crate::Reg<c1sar::C1SARrs>;
///LPDMA channel x source address register
pub mod c1sar;
/**C1DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1DAR)

For information about available fields see [`mod@c1dar`]
module*/
pub type C1DAR = crate::Reg<c1dar::C1DARrs>;
///LPDMA channel x destination address register
pub mod c1dar;
/**C1LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c1llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C1LLR)

For information about available fields see [`mod@c1llr`]
module*/
pub type C1LLR = crate::Reg<c1llr::C1LLRrs>;
///LPDMA channel x linked-list address register
pub mod c1llr;
/**C2LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c2lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2LBAR)

For information about available fields see [`mod@c2lbar`]
module*/
pub type C2LBAR = crate::Reg<c2lbar::C2LBARrs>;
///channel x linked-list base address register
pub mod c2lbar;
/**C2FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2FCR)

For information about available fields see [`mod@c2fcr`]
module*/
pub type C2FCR = crate::Reg<c2fcr::C2FCRrs>;
///LPDMA channel x flag clear register
pub mod c2fcr;
/**C2SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2SR)

For information about available fields see [`mod@c2sr`]
module*/
pub type C2SR = crate::Reg<c2sr::C2SRrs>;
///channel x status register
pub mod c2sr;
/**C2CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2CR)

For information about available fields see [`mod@c2cr`]
module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///channel x control register
pub mod c2cr;
/**C2TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c2tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2TR1)

For information about available fields see [`mod@c2tr1`]
module*/
pub type C2TR1 = crate::Reg<c2tr1::C2TR1rs>;
///LPDMA channel x transfer register 1
pub mod c2tr1;
/**C2TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c2tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2TR2)

For information about available fields see [`mod@c2tr2`]
module*/
pub type C2TR2 = crate::Reg<c2tr2::C2TR2rs>;
///LPDMA channel x transfer register 2
pub mod c2tr2;
/**C2BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c2br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2BR1)

For information about available fields see [`mod@c2br1`]
module*/
pub type C2BR1 = crate::Reg<c2br1::C2BR1rs>;
///LPDMA channel x block register 1
pub mod c2br1;
/**C2SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2SAR)

For information about available fields see [`mod@c2sar`]
module*/
pub type C2SAR = crate::Reg<c2sar::C2SARrs>;
///LPDMA channel x source address register
pub mod c2sar;
/**C2DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2DAR)

For information about available fields see [`mod@c2dar`]
module*/
pub type C2DAR = crate::Reg<c2dar::C2DARrs>;
///LPDMA channel x destination address register
pub mod c2dar;
/**C2LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c2llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C2LLR)

For information about available fields see [`mod@c2llr`]
module*/
pub type C2LLR = crate::Reg<c2llr::C2LLRrs>;
///LPDMA channel x linked-list address register
pub mod c2llr;
/**C3LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c3lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3LBAR)

For information about available fields see [`mod@c3lbar`]
module*/
pub type C3LBAR = crate::Reg<c3lbar::C3LBARrs>;
///channel x linked-list base address register
pub mod c3lbar;
/**C3FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3FCR)

For information about available fields see [`mod@c3fcr`]
module*/
pub type C3FCR = crate::Reg<c3fcr::C3FCRrs>;
///LPDMA channel x flag clear register
pub mod c3fcr;
/**C3SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3SR)

For information about available fields see [`mod@c3sr`]
module*/
pub type C3SR = crate::Reg<c3sr::C3SRrs>;
///channel x status register
pub mod c3sr;
/**C3CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3CR)

For information about available fields see [`mod@c3cr`]
module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///channel x control register
pub mod c3cr;
/**C3TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c3tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3TR1)

For information about available fields see [`mod@c3tr1`]
module*/
pub type C3TR1 = crate::Reg<c3tr1::C3TR1rs>;
///LPDMA channel x transfer register 1
pub mod c3tr1;
/**C3TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c3tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3TR2)

For information about available fields see [`mod@c3tr2`]
module*/
pub type C3TR2 = crate::Reg<c3tr2::C3TR2rs>;
///LPDMA channel x transfer register 2
pub mod c3tr2;
/**C3BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c3br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3BR1)

For information about available fields see [`mod@c3br1`]
module*/
pub type C3BR1 = crate::Reg<c3br1::C3BR1rs>;
///LPDMA channel x block register 1
pub mod c3br1;
/**C3SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3SAR)

For information about available fields see [`mod@c3sar`]
module*/
pub type C3SAR = crate::Reg<c3sar::C3SARrs>;
///LPDMA channel x source address register
pub mod c3sar;
/**C3DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3DAR)

For information about available fields see [`mod@c3dar`]
module*/
pub type C3DAR = crate::Reg<c3dar::C3DARrs>;
///LPDMA channel x destination address register
pub mod c3dar;
/**C3LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c3llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3LLR)

For information about available fields see [`mod@c3llr`]
module*/
pub type C3LLR = crate::Reg<c3llr::C3LLRrs>;
///LPDMA channel x linked-list address register
pub mod c3llr;
