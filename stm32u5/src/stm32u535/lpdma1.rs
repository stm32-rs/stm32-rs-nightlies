#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    lpdma_seccfgr: LPDMA_SECCFGR,
    lpdma_privcfgr: LPDMA_PRIVCFGR,
    _reserved2: [u8; 0x04],
    misr: MISR,
    smisr: SMISR,
    _reserved4: [u8; 0x3c],
    lpdma_c0lbar: LPDMA_C0LBAR,
    _reserved5: [u8; 0x08],
    lpdma_c0fcr: LPDMA_C0FCR,
    lpdma_c0sr: LPDMA_C0SR,
    lpdma_c0cr: LPDMA_C0CR,
    _reserved8: [u8; 0x28],
    lpdma_c0tr1: LPDMA_C0TR1,
    lpdma_c0tr2: LPDMA_C0TR2,
    lpdma_c0br1: LPDMA_C0BR1,
    lpdma_c0sar: LPDMA_C0SAR,
    lpdma_c0dar: LPDMA_C0DAR,
    _reserved13: [u8; 0x28],
    lpdma_c0llr: LPDMA_C0LLR,
    lpdma_c1lbar: LPDMA_C1LBAR,
    _reserved15: [u8; 0x08],
    lpdma_c1fcr: LPDMA_C1FCR,
    lpdma_c1sr: LPDMA_C1SR,
    lpdma_c1cr: LPDMA_C1CR,
    _reserved18: [u8; 0x28],
    lpdma_c1tr1: LPDMA_C1TR1,
    lpdma_c1tr2: LPDMA_C1TR2,
    lpdma_c1br1: LPDMA_C1BR1,
    lpdma_c1sar: LPDMA_C1SAR,
    lpdma_c1dar: LPDMA_C1DAR,
    _reserved23: [u8; 0x28],
    lpdma_c1llr: LPDMA_C1LLR,
    lpdma_c2lbar: LPDMA_C2LBAR,
    _reserved25: [u8; 0x08],
    lpdma_c2fcr: LPDMA_C2FCR,
    lpdma_c2sr: LPDMA_C2SR,
    lpdma_c2cr: LPDMA_C2CR,
    _reserved28: [u8; 0x28],
    lpdma_c2tr1: LPDMA_C2TR1,
    lpdma_c2tr2: LPDMA_C2TR2,
    lpdma_c2br1: LPDMA_C2BR1,
    lpdma_c2sar: LPDMA_C2SAR,
    lpdma_c2dar: LPDMA_C2DAR,
    _reserved33: [u8; 0x28],
    lpdma_c2llr: LPDMA_C2LLR,
    lpdma_c3lbar: LPDMA_C3LBAR,
    _reserved35: [u8; 0x08],
    lpdma_c3fcr: LPDMA_C3FCR,
    lpdma_c3sr: LPDMA_C3SR,
    lpdma_c3cr: LPDMA_C3CR,
    _reserved38: [u8; 0x28],
    lpdma_c3tr1: LPDMA_C3TR1,
    lpdma_c3tr2: LPDMA_C3TR2,
    lpdma_c3br1: LPDMA_C3BR1,
    lpdma_c3sar: LPDMA_C3SAR,
    lpdma_c3dar: LPDMA_C3DAR,
    _reserved43: [u8; 0x28],
    lpdma_c3llr: LPDMA_C3LLR,
}
impl RegisterBlock {
    ///0x00 - LPDMA secure configuration register
    #[inline(always)]
    pub const fn lpdma_seccfgr(&self) -> &LPDMA_SECCFGR {
        &self.lpdma_seccfgr
    }
    ///0x04 - LPDMA privileged configuration register
    #[inline(always)]
    pub const fn lpdma_privcfgr(&self) -> &LPDMA_PRIVCFGR {
        &self.lpdma_privcfgr
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
    pub const fn lpdma_c0lbar(&self) -> &LPDMA_C0LBAR {
        &self.lpdma_c0lbar
    }
    ///0x5c - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn lpdma_c0fcr(&self) -> &LPDMA_C0FCR {
        &self.lpdma_c0fcr
    }
    ///0x60 - channel x status register
    #[inline(always)]
    pub const fn lpdma_c0sr(&self) -> &LPDMA_C0SR {
        &self.lpdma_c0sr
    }
    ///0x64 - channel x control register
    #[inline(always)]
    pub const fn lpdma_c0cr(&self) -> &LPDMA_C0CR {
        &self.lpdma_c0cr
    }
    ///0x90 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn lpdma_c0tr1(&self) -> &LPDMA_C0TR1 {
        &self.lpdma_c0tr1
    }
    ///0x94 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn lpdma_c0tr2(&self) -> &LPDMA_C0TR2 {
        &self.lpdma_c0tr2
    }
    ///0x98 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn lpdma_c0br1(&self) -> &LPDMA_C0BR1 {
        &self.lpdma_c0br1
    }
    ///0x9c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn lpdma_c0sar(&self) -> &LPDMA_C0SAR {
        &self.lpdma_c0sar
    }
    ///0xa0 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn lpdma_c0dar(&self) -> &LPDMA_C0DAR {
        &self.lpdma_c0dar
    }
    ///0xcc - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn lpdma_c0llr(&self) -> &LPDMA_C0LLR {
        &self.lpdma_c0llr
    }
    ///0xd0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn lpdma_c1lbar(&self) -> &LPDMA_C1LBAR {
        &self.lpdma_c1lbar
    }
    ///0xdc - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn lpdma_c1fcr(&self) -> &LPDMA_C1FCR {
        &self.lpdma_c1fcr
    }
    ///0xe0 - channel x status register
    #[inline(always)]
    pub const fn lpdma_c1sr(&self) -> &LPDMA_C1SR {
        &self.lpdma_c1sr
    }
    ///0xe4 - channel x control register
    #[inline(always)]
    pub const fn lpdma_c1cr(&self) -> &LPDMA_C1CR {
        &self.lpdma_c1cr
    }
    ///0x110 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn lpdma_c1tr1(&self) -> &LPDMA_C1TR1 {
        &self.lpdma_c1tr1
    }
    ///0x114 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn lpdma_c1tr2(&self) -> &LPDMA_C1TR2 {
        &self.lpdma_c1tr2
    }
    ///0x118 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn lpdma_c1br1(&self) -> &LPDMA_C1BR1 {
        &self.lpdma_c1br1
    }
    ///0x11c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn lpdma_c1sar(&self) -> &LPDMA_C1SAR {
        &self.lpdma_c1sar
    }
    ///0x120 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn lpdma_c1dar(&self) -> &LPDMA_C1DAR {
        &self.lpdma_c1dar
    }
    ///0x14c - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn lpdma_c1llr(&self) -> &LPDMA_C1LLR {
        &self.lpdma_c1llr
    }
    ///0x150 - channel x linked-list base address register
    #[inline(always)]
    pub const fn lpdma_c2lbar(&self) -> &LPDMA_C2LBAR {
        &self.lpdma_c2lbar
    }
    ///0x15c - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn lpdma_c2fcr(&self) -> &LPDMA_C2FCR {
        &self.lpdma_c2fcr
    }
    ///0x160 - channel x status register
    #[inline(always)]
    pub const fn lpdma_c2sr(&self) -> &LPDMA_C2SR {
        &self.lpdma_c2sr
    }
    ///0x164 - channel x control register
    #[inline(always)]
    pub const fn lpdma_c2cr(&self) -> &LPDMA_C2CR {
        &self.lpdma_c2cr
    }
    ///0x190 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn lpdma_c2tr1(&self) -> &LPDMA_C2TR1 {
        &self.lpdma_c2tr1
    }
    ///0x194 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn lpdma_c2tr2(&self) -> &LPDMA_C2TR2 {
        &self.lpdma_c2tr2
    }
    ///0x198 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn lpdma_c2br1(&self) -> &LPDMA_C2BR1 {
        &self.lpdma_c2br1
    }
    ///0x19c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn lpdma_c2sar(&self) -> &LPDMA_C2SAR {
        &self.lpdma_c2sar
    }
    ///0x1a0 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn lpdma_c2dar(&self) -> &LPDMA_C2DAR {
        &self.lpdma_c2dar
    }
    ///0x1cc - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn lpdma_c2llr(&self) -> &LPDMA_C2LLR {
        &self.lpdma_c2llr
    }
    ///0x1d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn lpdma_c3lbar(&self) -> &LPDMA_C3LBAR {
        &self.lpdma_c3lbar
    }
    ///0x1dc - LPDMA channel x flag clear register
    #[inline(always)]
    pub const fn lpdma_c3fcr(&self) -> &LPDMA_C3FCR {
        &self.lpdma_c3fcr
    }
    ///0x1e0 - channel x status register
    #[inline(always)]
    pub const fn lpdma_c3sr(&self) -> &LPDMA_C3SR {
        &self.lpdma_c3sr
    }
    ///0x1e4 - channel x control register
    #[inline(always)]
    pub const fn lpdma_c3cr(&self) -> &LPDMA_C3CR {
        &self.lpdma_c3cr
    }
    ///0x210 - LPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn lpdma_c3tr1(&self) -> &LPDMA_C3TR1 {
        &self.lpdma_c3tr1
    }
    ///0x214 - LPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn lpdma_c3tr2(&self) -> &LPDMA_C3TR2 {
        &self.lpdma_c3tr2
    }
    ///0x218 - LPDMA channel x block register 1
    #[inline(always)]
    pub const fn lpdma_c3br1(&self) -> &LPDMA_C3BR1 {
        &self.lpdma_c3br1
    }
    ///0x21c - LPDMA channel x source address register
    #[inline(always)]
    pub const fn lpdma_c3sar(&self) -> &LPDMA_C3SAR {
        &self.lpdma_c3sar
    }
    ///0x220 - LPDMA channel x destination address register
    #[inline(always)]
    pub const fn lpdma_c3dar(&self) -> &LPDMA_C3DAR {
        &self.lpdma_c3dar
    }
    ///0x24c - LPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn lpdma_c3llr(&self) -> &LPDMA_C3LLR {
        &self.lpdma_c3llr
    }
}
/**LPDMA_SECCFGR (rw) register accessor: LPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`lpdma_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_SECCFGR)

For information about available fields see [`mod@lpdma_seccfgr`]
module*/
pub type LPDMA_SECCFGR = crate::Reg<lpdma_seccfgr::LPDMA_SECCFGRrs>;
///LPDMA secure configuration register
pub mod lpdma_seccfgr;
/**LPDMA_PRIVCFGR (rw) register accessor: LPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`lpdma_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_PRIVCFGR)

For information about available fields see [`mod@lpdma_privcfgr`]
module*/
pub type LPDMA_PRIVCFGR = crate::Reg<lpdma_privcfgr::LPDMA_PRIVCFGRrs>;
///LPDMA privileged configuration register
pub mod lpdma_privcfgr;
/**MISR (r) register accessor: LPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///LPDMA non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: LPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:SMISR)

For information about available fields see [`mod@smisr`]
module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///LPDMA secure masked interrupt status register
pub mod smisr;
/**LPDMA_C0LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0LBAR)

For information about available fields see [`mod@lpdma_c0lbar`]
module*/
pub type LPDMA_C0LBAR = crate::Reg<lpdma_c0lbar::LPDMA_C0LBARrs>;
///channel x linked-list base address register
pub mod lpdma_c0lbar;
/**LPDMA_C0FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0FCR)

For information about available fields see [`mod@lpdma_c0fcr`]
module*/
pub type LPDMA_C0FCR = crate::Reg<lpdma_c0fcr::LPDMA_C0FCRrs>;
///LPDMA channel x flag clear register
pub mod lpdma_c0fcr;
/**LPDMA_C0SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0SR)

For information about available fields see [`mod@lpdma_c0sr`]
module*/
pub type LPDMA_C0SR = crate::Reg<lpdma_c0sr::LPDMA_C0SRrs>;
///channel x status register
pub mod lpdma_c0sr;
/**LPDMA_C0CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0CR)

For information about available fields see [`mod@lpdma_c0cr`]
module*/
pub type LPDMA_C0CR = crate::Reg<lpdma_c0cr::LPDMA_C0CRrs>;
///channel x control register
pub mod lpdma_c0cr;
/**LPDMA_C0TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0TR1)

For information about available fields see [`mod@lpdma_c0tr1`]
module*/
pub type LPDMA_C0TR1 = crate::Reg<lpdma_c0tr1::LPDMA_C0TR1rs>;
///LPDMA channel x transfer register 1
pub mod lpdma_c0tr1;
/**LPDMA_C0TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0TR2)

For information about available fields see [`mod@lpdma_c0tr2`]
module*/
pub type LPDMA_C0TR2 = crate::Reg<lpdma_c0tr2::LPDMA_C0TR2rs>;
///LPDMA channel x transfer register 2
pub mod lpdma_c0tr2;
/**LPDMA_C0BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0BR1)

For information about available fields see [`mod@lpdma_c0br1`]
module*/
pub type LPDMA_C0BR1 = crate::Reg<lpdma_c0br1::LPDMA_C0BR1rs>;
///LPDMA channel x block register 1
pub mod lpdma_c0br1;
/**LPDMA_C0SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0SAR)

For information about available fields see [`mod@lpdma_c0sar`]
module*/
pub type LPDMA_C0SAR = crate::Reg<lpdma_c0sar::LPDMA_C0SARrs>;
///LPDMA channel x source address register
pub mod lpdma_c0sar;
/**LPDMA_C0DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0DAR)

For information about available fields see [`mod@lpdma_c0dar`]
module*/
pub type LPDMA_C0DAR = crate::Reg<lpdma_c0dar::LPDMA_C0DARrs>;
///LPDMA channel x destination address register
pub mod lpdma_c0dar;
/**LPDMA_C0LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c0llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c0llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C0LLR)

For information about available fields see [`mod@lpdma_c0llr`]
module*/
pub type LPDMA_C0LLR = crate::Reg<lpdma_c0llr::LPDMA_C0LLRrs>;
///LPDMA channel x linked-list address register
pub mod lpdma_c0llr;
/**LPDMA_C1LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1LBAR)

For information about available fields see [`mod@lpdma_c1lbar`]
module*/
pub type LPDMA_C1LBAR = crate::Reg<lpdma_c1lbar::LPDMA_C1LBARrs>;
///channel x linked-list base address register
pub mod lpdma_c1lbar;
/**LPDMA_C1FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1FCR)

For information about available fields see [`mod@lpdma_c1fcr`]
module*/
pub type LPDMA_C1FCR = crate::Reg<lpdma_c1fcr::LPDMA_C1FCRrs>;
///LPDMA channel x flag clear register
pub mod lpdma_c1fcr;
/**LPDMA_C1SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1SR)

For information about available fields see [`mod@lpdma_c1sr`]
module*/
pub type LPDMA_C1SR = crate::Reg<lpdma_c1sr::LPDMA_C1SRrs>;
///channel x status register
pub mod lpdma_c1sr;
/**LPDMA_C1CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1CR)

For information about available fields see [`mod@lpdma_c1cr`]
module*/
pub type LPDMA_C1CR = crate::Reg<lpdma_c1cr::LPDMA_C1CRrs>;
///channel x control register
pub mod lpdma_c1cr;
/**LPDMA_C1TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1TR1)

For information about available fields see [`mod@lpdma_c1tr1`]
module*/
pub type LPDMA_C1TR1 = crate::Reg<lpdma_c1tr1::LPDMA_C1TR1rs>;
///LPDMA channel x transfer register 1
pub mod lpdma_c1tr1;
/**LPDMA_C1TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1TR2)

For information about available fields see [`mod@lpdma_c1tr2`]
module*/
pub type LPDMA_C1TR2 = crate::Reg<lpdma_c1tr2::LPDMA_C1TR2rs>;
///LPDMA channel x transfer register 2
pub mod lpdma_c1tr2;
/**LPDMA_C1BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1BR1)

For information about available fields see [`mod@lpdma_c1br1`]
module*/
pub type LPDMA_C1BR1 = crate::Reg<lpdma_c1br1::LPDMA_C1BR1rs>;
///LPDMA channel x block register 1
pub mod lpdma_c1br1;
/**LPDMA_C1SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1SAR)

For information about available fields see [`mod@lpdma_c1sar`]
module*/
pub type LPDMA_C1SAR = crate::Reg<lpdma_c1sar::LPDMA_C1SARrs>;
///LPDMA channel x source address register
pub mod lpdma_c1sar;
/**LPDMA_C1DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1DAR)

For information about available fields see [`mod@lpdma_c1dar`]
module*/
pub type LPDMA_C1DAR = crate::Reg<lpdma_c1dar::LPDMA_C1DARrs>;
///LPDMA channel x destination address register
pub mod lpdma_c1dar;
/**LPDMA_C1LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C1LLR)

For information about available fields see [`mod@lpdma_c1llr`]
module*/
pub type LPDMA_C1LLR = crate::Reg<lpdma_c1llr::LPDMA_C1LLRrs>;
///LPDMA channel x linked-list address register
pub mod lpdma_c1llr;
/**LPDMA_C2LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2LBAR)

For information about available fields see [`mod@lpdma_c2lbar`]
module*/
pub type LPDMA_C2LBAR = crate::Reg<lpdma_c2lbar::LPDMA_C2LBARrs>;
///channel x linked-list base address register
pub mod lpdma_c2lbar;
/**LPDMA_C2FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2FCR)

For information about available fields see [`mod@lpdma_c2fcr`]
module*/
pub type LPDMA_C2FCR = crate::Reg<lpdma_c2fcr::LPDMA_C2FCRrs>;
///LPDMA channel x flag clear register
pub mod lpdma_c2fcr;
/**LPDMA_C2SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2SR)

For information about available fields see [`mod@lpdma_c2sr`]
module*/
pub type LPDMA_C2SR = crate::Reg<lpdma_c2sr::LPDMA_C2SRrs>;
///channel x status register
pub mod lpdma_c2sr;
/**LPDMA_C2CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2CR)

For information about available fields see [`mod@lpdma_c2cr`]
module*/
pub type LPDMA_C2CR = crate::Reg<lpdma_c2cr::LPDMA_C2CRrs>;
///channel x control register
pub mod lpdma_c2cr;
/**LPDMA_C2TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2TR1)

For information about available fields see [`mod@lpdma_c2tr1`]
module*/
pub type LPDMA_C2TR1 = crate::Reg<lpdma_c2tr1::LPDMA_C2TR1rs>;
///LPDMA channel x transfer register 1
pub mod lpdma_c2tr1;
/**LPDMA_C2TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2TR2)

For information about available fields see [`mod@lpdma_c2tr2`]
module*/
pub type LPDMA_C2TR2 = crate::Reg<lpdma_c2tr2::LPDMA_C2TR2rs>;
///LPDMA channel x transfer register 2
pub mod lpdma_c2tr2;
/**LPDMA_C2BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2BR1)

For information about available fields see [`mod@lpdma_c2br1`]
module*/
pub type LPDMA_C2BR1 = crate::Reg<lpdma_c2br1::LPDMA_C2BR1rs>;
///LPDMA channel x block register 1
pub mod lpdma_c2br1;
/**LPDMA_C2SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2SAR)

For information about available fields see [`mod@lpdma_c2sar`]
module*/
pub type LPDMA_C2SAR = crate::Reg<lpdma_c2sar::LPDMA_C2SARrs>;
///LPDMA channel x source address register
pub mod lpdma_c2sar;
/**LPDMA_C2DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2DAR)

For information about available fields see [`mod@lpdma_c2dar`]
module*/
pub type LPDMA_C2DAR = crate::Reg<lpdma_c2dar::LPDMA_C2DARrs>;
///LPDMA channel x destination address register
pub mod lpdma_c2dar;
/**LPDMA_C2LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C2LLR)

For information about available fields see [`mod@lpdma_c2llr`]
module*/
pub type LPDMA_C2LLR = crate::Reg<lpdma_c2llr::LPDMA_C2LLRrs>;
///LPDMA channel x linked-list address register
pub mod lpdma_c2llr;
/**LPDMA_C3LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3LBAR)

For information about available fields see [`mod@lpdma_c3lbar`]
module*/
pub type LPDMA_C3LBAR = crate::Reg<lpdma_c3lbar::LPDMA_C3LBARrs>;
///channel x linked-list base address register
pub mod lpdma_c3lbar;
/**LPDMA_C3FCR (w) register accessor: LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3FCR)

For information about available fields see [`mod@lpdma_c3fcr`]
module*/
pub type LPDMA_C3FCR = crate::Reg<lpdma_c3fcr::LPDMA_C3FCRrs>;
///LPDMA channel x flag clear register
pub mod lpdma_c3fcr;
/**LPDMA_C3SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3SR)

For information about available fields see [`mod@lpdma_c3sr`]
module*/
pub type LPDMA_C3SR = crate::Reg<lpdma_c3sr::LPDMA_C3SRrs>;
///channel x status register
pub mod lpdma_c3sr;
/**LPDMA_C3CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3CR)

For information about available fields see [`mod@lpdma_c3cr`]
module*/
pub type LPDMA_C3CR = crate::Reg<lpdma_c3cr::LPDMA_C3CRrs>;
///channel x control register
pub mod lpdma_c3cr;
/**LPDMA_C3TR1 (rw) register accessor: LPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3TR1)

For information about available fields see [`mod@lpdma_c3tr1`]
module*/
pub type LPDMA_C3TR1 = crate::Reg<lpdma_c3tr1::LPDMA_C3TR1rs>;
///LPDMA channel x transfer register 1
pub mod lpdma_c3tr1;
/**LPDMA_C3TR2 (rw) register accessor: LPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3TR2)

For information about available fields see [`mod@lpdma_c3tr2`]
module*/
pub type LPDMA_C3TR2 = crate::Reg<lpdma_c3tr2::LPDMA_C3TR2rs>;
///LPDMA channel x transfer register 2
pub mod lpdma_c3tr2;
/**LPDMA_C3BR1 (rw) register accessor: LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3BR1)

For information about available fields see [`mod@lpdma_c3br1`]
module*/
pub type LPDMA_C3BR1 = crate::Reg<lpdma_c3br1::LPDMA_C3BR1rs>;
///LPDMA channel x block register 1
pub mod lpdma_c3br1;
/**LPDMA_C3SAR (rw) register accessor: LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3SAR)

For information about available fields see [`mod@lpdma_c3sar`]
module*/
pub type LPDMA_C3SAR = crate::Reg<lpdma_c3sar::LPDMA_C3SARrs>;
///LPDMA channel x source address register
pub mod lpdma_c3sar;
/**LPDMA_C3DAR (rw) register accessor: LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3DAR)

For information about available fields see [`mod@lpdma_c3dar`]
module*/
pub type LPDMA_C3DAR = crate::Reg<lpdma_c3dar::LPDMA_C3DARrs>;
///LPDMA channel x destination address register
pub mod lpdma_c3dar;
/**LPDMA_C3LLR (rw) register accessor: LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c3llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c3llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:LPDMA_C3LLR)

For information about available fields see [`mod@lpdma_c3llr`]
module*/
pub type LPDMA_C3LLR = crate::Reg<lpdma_c3llr::LPDMA_C3LLRrs>;
///LPDMA channel x linked-list address register
pub mod lpdma_c3llr;
