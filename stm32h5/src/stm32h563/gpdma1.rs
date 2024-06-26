#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    rcfglockr: RCFGLOCKR,
    misr: MISR,
    smisr: SMISR,
    _reserved5: [u8; 0x3c],
    c0lbar: C0LBAR,
    _reserved6: [u8; 0x08],
    c0fcr: C0FCR,
    c0sr: C0SR,
    c0cr: C0CR,
    _reserved9: [u8; 0x28],
    c0tr1: C0TR1,
    c0tr2: C0TR2,
    c0br1: C0BR1,
    c0sar: C0SAR,
    c0dar: C0DAR,
    _reserved14: [u8; 0x28],
    c0llr: C0LLR,
    c1lbar: C1LBAR,
    _reserved16: [u8; 0x08],
    c1fcr: C1FCR,
    c1sr: C1SR,
    c1cr: C1CR,
    _reserved19: [u8; 0x28],
    c1tr1: C1TR1,
    c1tr2: C1TR2,
    c1br1: C1BR1,
    c1sar: C1SAR,
    c1dar: C1DAR,
    _reserved24: [u8; 0x28],
    c1llr: C1LLR,
    c2lbar: C2LBAR,
    _reserved26: [u8; 0x08],
    c2fcr: C2FCR,
    c2sr: C2SR,
    c2cr: C2CR,
    _reserved29: [u8; 0x28],
    c2tr1: C2TR1,
    c2tr2: C2TR2,
    c2br1: C2BR1,
    c2sar: C2SAR,
    c2dar: C2DAR,
    _reserved34: [u8; 0x28],
    c2llr: C2LLR,
    c3lbar: C3LBAR,
    _reserved36: [u8; 0x08],
    c3fcr: C3FCR,
    c3sr: C3SR,
    c3cr: C3CR,
    _reserved39: [u8; 0x28],
    c3tr1: C3TR1,
    c3tr2: C3TR2,
    c3br1: C3BR1,
    c3sar: C3SAR,
    c3dar: C3DAR,
    _reserved44: [u8; 0x28],
    c3llr: C3LLR,
    c4lbar: C4LBAR,
    _reserved46: [u8; 0x08],
    c4fcr: C4FCR,
    c4sr: C4SR,
    c4cr: C4CR,
    _reserved49: [u8; 0x28],
    c4tr1: C4TR1,
    c4tr2: C4TR2,
    c4br1: C4BR1,
    c4sar: C4SAR,
    c4dar: C4DAR,
    _reserved54: [u8; 0x28],
    c4llr: C4LLR,
    c5lbar: C5LBAR,
    _reserved56: [u8; 0x08],
    c5fcr: C5FCR,
    c5sr: C5SR,
    c5cr: C5CR,
    _reserved59: [u8; 0x28],
    c5tr1: C5TR1,
    c5tr2: C5TR2,
    c5br1: C5BR1,
    c5sar: C5SAR,
    c5dar: C5DAR,
    _reserved64: [u8; 0x28],
    c5llr: C5LLR,
    c6lbar: C6LBAR,
    _reserved66: [u8; 0x08],
    c6fcr: C6FCR,
    c6sr: C6SR,
    c6cr: C6CR,
    _reserved69: [u8; 0x28],
    c6tr1: C6TR1,
    c6tr2: C6TR2,
    c6br1: C6BR1,
    c6sar: C6SAR,
    c6dar: C6DAR,
    c6tr3: C6TR3,
    c6br2: C6BR2,
    _reserved76: [u8; 0x20],
    c6llr: C6LLR,
    c7lbar: C7LBAR,
    _reserved78: [u8; 0x08],
    c7fcr: C7FCR,
    c7sr: C7SR,
    c7cr: C7CR,
    _reserved81: [u8; 0x28],
    c7tr1: C7TR1,
    c7tr2: C7TR2,
    c7br1: C7BR1,
    c7sar: C7SAR,
    c7dar: C7DAR,
    c7tr3: C7TR3,
    c7br2: C7BR2,
    _reserved88: [u8; 0x20],
    c7llr: C7LLR,
}
impl RegisterBlock {
    ///0x00 - GPDMA secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - GPDMA privileged configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x08 - GPDMA configuration lock register
    #[inline(always)]
    pub const fn rcfglockr(&self) -> &RCFGLOCKR {
        &self.rcfglockr
    }
    ///0x0c - GPDMA non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x10 - GPDMA secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x50 - GPDMA channel 0 linked-list base address register
    #[inline(always)]
    pub const fn c0lbar(&self) -> &C0LBAR {
        &self.c0lbar
    }
    ///0x5c - GPDMA channel 0 flag clear register
    #[inline(always)]
    pub const fn c0fcr(&self) -> &C0FCR {
        &self.c0fcr
    }
    ///0x60 - GPDMA channel 0 status register
    #[inline(always)]
    pub const fn c0sr(&self) -> &C0SR {
        &self.c0sr
    }
    ///0x64 - GPDMA channel 0 control register
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0CR {
        &self.c0cr
    }
    ///0x90 - GPDMA channel 0 transfer register 1
    #[inline(always)]
    pub const fn c0tr1(&self) -> &C0TR1 {
        &self.c0tr1
    }
    ///0x94 - GPDMA channel 0 transfer register 2
    #[inline(always)]
    pub const fn c0tr2(&self) -> &C0TR2 {
        &self.c0tr2
    }
    ///0x98 - GPDMA channel 0 block register 1
    #[inline(always)]
    pub const fn c0br1(&self) -> &C0BR1 {
        &self.c0br1
    }
    ///0x9c - GPDMA channel 0 source address register
    #[inline(always)]
    pub const fn c0sar(&self) -> &C0SAR {
        &self.c0sar
    }
    ///0xa0 - GPDMA channel 0 destination address register
    #[inline(always)]
    pub const fn c0dar(&self) -> &C0DAR {
        &self.c0dar
    }
    ///0xcc - GPDMA channel 0 linked-list address register
    #[inline(always)]
    pub const fn c0llr(&self) -> &C0LLR {
        &self.c0llr
    }
    ///0xd0 - GPDMA channel 1 linked-list base address register
    #[inline(always)]
    pub const fn c1lbar(&self) -> &C1LBAR {
        &self.c1lbar
    }
    ///0xdc - GPDMA channel 1 flag clear register
    #[inline(always)]
    pub const fn c1fcr(&self) -> &C1FCR {
        &self.c1fcr
    }
    ///0xe0 - GPDMA channel 1 status register
    #[inline(always)]
    pub const fn c1sr(&self) -> &C1SR {
        &self.c1sr
    }
    ///0xe4 - GPDMA channel 1 control register
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x110 - GPDMA channel 1 transfer register 1
    #[inline(always)]
    pub const fn c1tr1(&self) -> &C1TR1 {
        &self.c1tr1
    }
    ///0x114 - GPDMA channel 1 transfer register 2
    #[inline(always)]
    pub const fn c1tr2(&self) -> &C1TR2 {
        &self.c1tr2
    }
    ///0x118 - GPDMA channel 1 block register 1
    #[inline(always)]
    pub const fn c1br1(&self) -> &C1BR1 {
        &self.c1br1
    }
    ///0x11c - GPDMA channel 1 source address register
    #[inline(always)]
    pub const fn c1sar(&self) -> &C1SAR {
        &self.c1sar
    }
    ///0x120 - GPDMA channel 1 destination address register
    #[inline(always)]
    pub const fn c1dar(&self) -> &C1DAR {
        &self.c1dar
    }
    ///0x14c - GPDMA channel 1 linked-list address register
    #[inline(always)]
    pub const fn c1llr(&self) -> &C1LLR {
        &self.c1llr
    }
    ///0x150 - GPDMA channel 2 linked-list base address register
    #[inline(always)]
    pub const fn c2lbar(&self) -> &C2LBAR {
        &self.c2lbar
    }
    ///0x15c - GPDMA channel 2 flag clear register
    #[inline(always)]
    pub const fn c2fcr(&self) -> &C2FCR {
        &self.c2fcr
    }
    ///0x160 - GPDMA channel 2 status register
    #[inline(always)]
    pub const fn c2sr(&self) -> &C2SR {
        &self.c2sr
    }
    ///0x164 - GPDMA channel 2 control register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x190 - GPDMA channel 2 transfer register 1
    #[inline(always)]
    pub const fn c2tr1(&self) -> &C2TR1 {
        &self.c2tr1
    }
    ///0x194 - GPDMA channel 2 transfer register 2
    #[inline(always)]
    pub const fn c2tr2(&self) -> &C2TR2 {
        &self.c2tr2
    }
    ///0x198 - GPDMA channel 2 block register 1
    #[inline(always)]
    pub const fn c2br1(&self) -> &C2BR1 {
        &self.c2br1
    }
    ///0x19c - GPDMA channel 2 source address register
    #[inline(always)]
    pub const fn c2sar(&self) -> &C2SAR {
        &self.c2sar
    }
    ///0x1a0 - GPDMA channel 2 destination address register
    #[inline(always)]
    pub const fn c2dar(&self) -> &C2DAR {
        &self.c2dar
    }
    ///0x1cc - GPDMA channel 2 linked-list address register
    #[inline(always)]
    pub const fn c2llr(&self) -> &C2LLR {
        &self.c2llr
    }
    ///0x1d0 - GPDMA channel 3 linked-list base address register
    #[inline(always)]
    pub const fn c3lbar(&self) -> &C3LBAR {
        &self.c3lbar
    }
    ///0x1dc - GPDMA channel 3 flag clear register
    #[inline(always)]
    pub const fn c3fcr(&self) -> &C3FCR {
        &self.c3fcr
    }
    ///0x1e0 - GPDMA channel 3 status register
    #[inline(always)]
    pub const fn c3sr(&self) -> &C3SR {
        &self.c3sr
    }
    ///0x1e4 - GPDMA channel 3 control register
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3CR {
        &self.c3cr
    }
    ///0x210 - GPDMA channel 3 transfer register 1
    #[inline(always)]
    pub const fn c3tr1(&self) -> &C3TR1 {
        &self.c3tr1
    }
    ///0x214 - GPDMA channel 3 transfer register 2
    #[inline(always)]
    pub const fn c3tr2(&self) -> &C3TR2 {
        &self.c3tr2
    }
    ///0x218 - GPDMA channel 3 block register 1
    #[inline(always)]
    pub const fn c3br1(&self) -> &C3BR1 {
        &self.c3br1
    }
    ///0x21c - GPDMA channel 3 source address register
    #[inline(always)]
    pub const fn c3sar(&self) -> &C3SAR {
        &self.c3sar
    }
    ///0x220 - GPDMA channel 3 destination address register
    #[inline(always)]
    pub const fn c3dar(&self) -> &C3DAR {
        &self.c3dar
    }
    ///0x24c - GPDMA channel 3 linked-list address register
    #[inline(always)]
    pub const fn c3llr(&self) -> &C3LLR {
        &self.c3llr
    }
    ///0x250 - GPDMA channel 4 linked-list base address register
    #[inline(always)]
    pub const fn c4lbar(&self) -> &C4LBAR {
        &self.c4lbar
    }
    ///0x25c - GPDMA channel 4 flag clear register
    #[inline(always)]
    pub const fn c4fcr(&self) -> &C4FCR {
        &self.c4fcr
    }
    ///0x260 - GPDMA channel 4 status register
    #[inline(always)]
    pub const fn c4sr(&self) -> &C4SR {
        &self.c4sr
    }
    ///0x264 - GPDMA channel 4 control register
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4CR {
        &self.c4cr
    }
    ///0x290 - GPDMA channel 4 transfer register 1
    #[inline(always)]
    pub const fn c4tr1(&self) -> &C4TR1 {
        &self.c4tr1
    }
    ///0x294 - GPDMA channel 4 transfer register 2
    #[inline(always)]
    pub const fn c4tr2(&self) -> &C4TR2 {
        &self.c4tr2
    }
    ///0x298 - GPDMA channel 4 block register 1
    #[inline(always)]
    pub const fn c4br1(&self) -> &C4BR1 {
        &self.c4br1
    }
    ///0x29c - GPDMA channel 4 source address register
    #[inline(always)]
    pub const fn c4sar(&self) -> &C4SAR {
        &self.c4sar
    }
    ///0x2a0 - GPDMA channel 4 destination address register
    #[inline(always)]
    pub const fn c4dar(&self) -> &C4DAR {
        &self.c4dar
    }
    ///0x2cc - GPDMA channel 4 linked-list address register
    #[inline(always)]
    pub const fn c4llr(&self) -> &C4LLR {
        &self.c4llr
    }
    ///0x2d0 - GPDMA channel 5 linked-list base address register
    #[inline(always)]
    pub const fn c5lbar(&self) -> &C5LBAR {
        &self.c5lbar
    }
    ///0x2dc - GPDMA channel 5 flag clear register
    #[inline(always)]
    pub const fn c5fcr(&self) -> &C5FCR {
        &self.c5fcr
    }
    ///0x2e0 - GPDMA channel 5 status register
    #[inline(always)]
    pub const fn c5sr(&self) -> &C5SR {
        &self.c5sr
    }
    ///0x2e4 - GPDMA channel 5 control register
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5CR {
        &self.c5cr
    }
    ///0x310 - GPDMA channel 5 transfer register 1
    #[inline(always)]
    pub const fn c5tr1(&self) -> &C5TR1 {
        &self.c5tr1
    }
    ///0x314 - GPDMA channel 5 transfer register 2
    #[inline(always)]
    pub const fn c5tr2(&self) -> &C5TR2 {
        &self.c5tr2
    }
    ///0x318 - GPDMA channel 5 block register 1
    #[inline(always)]
    pub const fn c5br1(&self) -> &C5BR1 {
        &self.c5br1
    }
    ///0x31c - GPDMA channel 5 source address register
    #[inline(always)]
    pub const fn c5sar(&self) -> &C5SAR {
        &self.c5sar
    }
    ///0x320 - GPDMA channel 5 destination address register
    #[inline(always)]
    pub const fn c5dar(&self) -> &C5DAR {
        &self.c5dar
    }
    ///0x34c - GPDMA channel 5 linked-list address register
    #[inline(always)]
    pub const fn c5llr(&self) -> &C5LLR {
        &self.c5llr
    }
    ///0x350 - GPDMA channel 6 linked-list base address register
    #[inline(always)]
    pub const fn c6lbar(&self) -> &C6LBAR {
        &self.c6lbar
    }
    ///0x35c - GPDMA channel 6 flag clear register
    #[inline(always)]
    pub const fn c6fcr(&self) -> &C6FCR {
        &self.c6fcr
    }
    ///0x360 - GPDMA channel 6 status register
    #[inline(always)]
    pub const fn c6sr(&self) -> &C6SR {
        &self.c6sr
    }
    ///0x364 - GPDMA channel 6 control register
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6CR {
        &self.c6cr
    }
    ///0x390 - GPDMA channel 6 transfer register 1
    #[inline(always)]
    pub const fn c6tr1(&self) -> &C6TR1 {
        &self.c6tr1
    }
    ///0x394 - GPDMA channel 6 transfer register 2
    #[inline(always)]
    pub const fn c6tr2(&self) -> &C6TR2 {
        &self.c6tr2
    }
    ///0x398 - GPDMA channel 6 alternate block register 1
    #[inline(always)]
    pub const fn c6br1(&self) -> &C6BR1 {
        &self.c6br1
    }
    ///0x39c - GPDMA channel 6 source address register
    #[inline(always)]
    pub const fn c6sar(&self) -> &C6SAR {
        &self.c6sar
    }
    ///0x3a0 - GPDMA channel 6 destination address register
    #[inline(always)]
    pub const fn c6dar(&self) -> &C6DAR {
        &self.c6dar
    }
    ///0x3a4 - GPDMA channel 6 transfer register 3
    #[inline(always)]
    pub const fn c6tr3(&self) -> &C6TR3 {
        &self.c6tr3
    }
    ///0x3a8 - GPDMA channel 6 block register 2
    #[inline(always)]
    pub const fn c6br2(&self) -> &C6BR2 {
        &self.c6br2
    }
    ///0x3cc - GPDMA channel 6 alternate linked-list address register
    #[inline(always)]
    pub const fn c6llr(&self) -> &C6LLR {
        &self.c6llr
    }
    ///0x3d0 - GPDMA channel 7 linked-list base address register
    #[inline(always)]
    pub const fn c7lbar(&self) -> &C7LBAR {
        &self.c7lbar
    }
    ///0x3dc - GPDMA channel 7 flag clear register
    #[inline(always)]
    pub const fn c7fcr(&self) -> &C7FCR {
        &self.c7fcr
    }
    ///0x3e0 - GPDMA channel 7 status register
    #[inline(always)]
    pub const fn c7sr(&self) -> &C7SR {
        &self.c7sr
    }
    ///0x3e4 - GPDMA channel 7 control register
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7CR {
        &self.c7cr
    }
    ///0x410 - GPDMA channel 7 transfer register 1
    #[inline(always)]
    pub const fn c7tr1(&self) -> &C7TR1 {
        &self.c7tr1
    }
    ///0x414 - GPDMA channel 7 transfer register 2
    #[inline(always)]
    pub const fn c7tr2(&self) -> &C7TR2 {
        &self.c7tr2
    }
    ///0x418 - GPDMA channel 7 alternate block register 1
    #[inline(always)]
    pub const fn c7br1(&self) -> &C7BR1 {
        &self.c7br1
    }
    ///0x41c - GPDMA channel 7 source address register
    #[inline(always)]
    pub const fn c7sar(&self) -> &C7SAR {
        &self.c7sar
    }
    ///0x420 - GPDMA channel 7 destination address register
    #[inline(always)]
    pub const fn c7dar(&self) -> &C7DAR {
        &self.c7dar
    }
    ///0x424 - GPDMA channel 7 transfer register 3
    #[inline(always)]
    pub const fn c7tr3(&self) -> &C7TR3 {
        &self.c7tr3
    }
    ///0x428 - GPDMA channel 7 block register 2
    #[inline(always)]
    pub const fn c7br2(&self) -> &C7BR2 {
        &self.c7br2
    }
    ///0x44c - GPDMA channel 7 alternate linked-list address register
    #[inline(always)]
    pub const fn c7llr(&self) -> &C7LLR {
        &self.c7llr
    }
}
/**SECCFGR (rw) register accessor: GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:SECCFGR)

For information about available fields see [`mod@seccfgr`]
module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPDMA secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`]
module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod privcfgr;
/**RCFGLOCKR (rw) register accessor: GPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:RCFGLOCKR)

For information about available fields see [`mod@rcfglockr`]
module*/
pub type RCFGLOCKR = crate::Reg<rcfglockr::RCFGLOCKRrs>;
///GPDMA configuration lock register
pub mod rcfglockr;
/**MISR (r) register accessor: GPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///GPDMA non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: GPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:SMISR)

For information about available fields see [`mod@smisr`]
module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///GPDMA secure masked interrupt status register
pub mod smisr;
/**C0LBAR (rw) register accessor: GPDMA channel 0 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c0lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0LBAR)

For information about available fields see [`mod@c0lbar`]
module*/
pub type C0LBAR = crate::Reg<c0lbar::C0LBARrs>;
///GPDMA channel 0 linked-list base address register
pub mod c0lbar;
/**C0FCR (w) register accessor: GPDMA channel 0 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0FCR)

For information about available fields see [`mod@c0fcr`]
module*/
pub type C0FCR = crate::Reg<c0fcr::C0FCRrs>;
///GPDMA channel 0 flag clear register
pub mod c0fcr;
/**C0SR (r) register accessor: GPDMA channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0SR)

For information about available fields see [`mod@c0sr`]
module*/
pub type C0SR = crate::Reg<c0sr::C0SRrs>;
///GPDMA channel 0 status register
pub mod c0sr;
/**C0CR (rw) register accessor: GPDMA channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0CR)

For information about available fields see [`mod@c0cr`]
module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///GPDMA channel 0 control register
pub mod c0cr;
/**C0TR1 (rw) register accessor: GPDMA channel 0 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c0tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0TR1)

For information about available fields see [`mod@c0tr1`]
module*/
pub type C0TR1 = crate::Reg<c0tr1::C0TR1rs>;
///GPDMA channel 0 transfer register 1
pub mod c0tr1;
/**C0TR2 (rw) register accessor: GPDMA channel 0 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c0tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0TR2)

For information about available fields see [`mod@c0tr2`]
module*/
pub type C0TR2 = crate::Reg<c0tr2::C0TR2rs>;
///GPDMA channel 0 transfer register 2
pub mod c0tr2;
/**C0BR1 (rw) register accessor: GPDMA channel 0 block register 1

You can [`read`](crate::Reg::read) this register and get [`c0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0BR1)

For information about available fields see [`mod@c0br1`]
module*/
pub type C0BR1 = crate::Reg<c0br1::C0BR1rs>;
///GPDMA channel 0 block register 1
pub mod c0br1;
/**C0SAR (rw) register accessor: GPDMA channel 0 source address register

You can [`read`](crate::Reg::read) this register and get [`c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0SAR)

For information about available fields see [`mod@c0sar`]
module*/
pub type C0SAR = crate::Reg<c0sar::C0SARrs>;
///GPDMA channel 0 source address register
pub mod c0sar;
/**C0DAR (rw) register accessor: GPDMA channel 0 destination address register

You can [`read`](crate::Reg::read) this register and get [`c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0DAR)

For information about available fields see [`mod@c0dar`]
module*/
pub type C0DAR = crate::Reg<c0dar::C0DARrs>;
///GPDMA channel 0 destination address register
pub mod c0dar;
/**C0LLR (rw) register accessor: GPDMA channel 0 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c0llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C0LLR)

For information about available fields see [`mod@c0llr`]
module*/
pub type C0LLR = crate::Reg<c0llr::C0LLRrs>;
///GPDMA channel 0 linked-list address register
pub mod c0llr;
/**C1LBAR (rw) register accessor: GPDMA channel 1 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c1lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1LBAR)

For information about available fields see [`mod@c1lbar`]
module*/
pub type C1LBAR = crate::Reg<c1lbar::C1LBARrs>;
///GPDMA channel 1 linked-list base address register
pub mod c1lbar;
/**C1FCR (w) register accessor: GPDMA channel 1 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1FCR)

For information about available fields see [`mod@c1fcr`]
module*/
pub type C1FCR = crate::Reg<c1fcr::C1FCRrs>;
///GPDMA channel 1 flag clear register
pub mod c1fcr;
/**C1SR (r) register accessor: GPDMA channel 1 status register

You can [`read`](crate::Reg::read) this register and get [`c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1SR)

For information about available fields see [`mod@c1sr`]
module*/
pub type C1SR = crate::Reg<c1sr::C1SRrs>;
///GPDMA channel 1 status register
pub mod c1sr;
/**C1CR (rw) register accessor: GPDMA channel 1 control register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1CR)

For information about available fields see [`mod@c1cr`]
module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///GPDMA channel 1 control register
pub mod c1cr;
/**C1TR1 (rw) register accessor: GPDMA channel 1 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c1tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1TR1)

For information about available fields see [`mod@c1tr1`]
module*/
pub type C1TR1 = crate::Reg<c1tr1::C1TR1rs>;
///GPDMA channel 1 transfer register 1
pub mod c1tr1;
/**C1TR2 (rw) register accessor: GPDMA channel 1 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c1tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1TR2)

For information about available fields see [`mod@c1tr2`]
module*/
pub type C1TR2 = crate::Reg<c1tr2::C1TR2rs>;
///GPDMA channel 1 transfer register 2
pub mod c1tr2;
/**C1BR1 (rw) register accessor: GPDMA channel 1 block register 1

You can [`read`](crate::Reg::read) this register and get [`c1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1BR1)

For information about available fields see [`mod@c1br1`]
module*/
pub type C1BR1 = crate::Reg<c1br1::C1BR1rs>;
///GPDMA channel 1 block register 1
pub mod c1br1;
/**C1SAR (rw) register accessor: GPDMA channel 1 source address register

You can [`read`](crate::Reg::read) this register and get [`c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1SAR)

For information about available fields see [`mod@c1sar`]
module*/
pub type C1SAR = crate::Reg<c1sar::C1SARrs>;
///GPDMA channel 1 source address register
pub mod c1sar;
/**C1DAR (rw) register accessor: GPDMA channel 1 destination address register

You can [`read`](crate::Reg::read) this register and get [`c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1DAR)

For information about available fields see [`mod@c1dar`]
module*/
pub type C1DAR = crate::Reg<c1dar::C1DARrs>;
///GPDMA channel 1 destination address register
pub mod c1dar;
/**C1LLR (rw) register accessor: GPDMA channel 1 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c1llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C1LLR)

For information about available fields see [`mod@c1llr`]
module*/
pub type C1LLR = crate::Reg<c1llr::C1LLRrs>;
///GPDMA channel 1 linked-list address register
pub mod c1llr;
/**C2LBAR (rw) register accessor: GPDMA channel 2 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c2lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2LBAR)

For information about available fields see [`mod@c2lbar`]
module*/
pub type C2LBAR = crate::Reg<c2lbar::C2LBARrs>;
///GPDMA channel 2 linked-list base address register
pub mod c2lbar;
/**C2FCR (w) register accessor: GPDMA channel 2 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2FCR)

For information about available fields see [`mod@c2fcr`]
module*/
pub type C2FCR = crate::Reg<c2fcr::C2FCRrs>;
///GPDMA channel 2 flag clear register
pub mod c2fcr;
/**C2SR (r) register accessor: GPDMA channel 2 status register

You can [`read`](crate::Reg::read) this register and get [`c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2SR)

For information about available fields see [`mod@c2sr`]
module*/
pub type C2SR = crate::Reg<c2sr::C2SRrs>;
///GPDMA channel 2 status register
pub mod c2sr;
/**C2CR (rw) register accessor: GPDMA channel 2 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2CR)

For information about available fields see [`mod@c2cr`]
module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///GPDMA channel 2 control register
pub mod c2cr;
/**C2TR1 (rw) register accessor: GPDMA channel 2 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c2tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2TR1)

For information about available fields see [`mod@c2tr1`]
module*/
pub type C2TR1 = crate::Reg<c2tr1::C2TR1rs>;
///GPDMA channel 2 transfer register 1
pub mod c2tr1;
/**C2TR2 (rw) register accessor: GPDMA channel 2 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c2tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2TR2)

For information about available fields see [`mod@c2tr2`]
module*/
pub type C2TR2 = crate::Reg<c2tr2::C2TR2rs>;
///GPDMA channel 2 transfer register 2
pub mod c2tr2;
/**C2BR1 (rw) register accessor: GPDMA channel 2 block register 1

You can [`read`](crate::Reg::read) this register and get [`c2br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2BR1)

For information about available fields see [`mod@c2br1`]
module*/
pub type C2BR1 = crate::Reg<c2br1::C2BR1rs>;
///GPDMA channel 2 block register 1
pub mod c2br1;
/**C2SAR (rw) register accessor: GPDMA channel 2 source address register

You can [`read`](crate::Reg::read) this register and get [`c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2SAR)

For information about available fields see [`mod@c2sar`]
module*/
pub type C2SAR = crate::Reg<c2sar::C2SARrs>;
///GPDMA channel 2 source address register
pub mod c2sar;
/**C2DAR (rw) register accessor: GPDMA channel 2 destination address register

You can [`read`](crate::Reg::read) this register and get [`c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2DAR)

For information about available fields see [`mod@c2dar`]
module*/
pub type C2DAR = crate::Reg<c2dar::C2DARrs>;
///GPDMA channel 2 destination address register
pub mod c2dar;
/**C2LLR (rw) register accessor: GPDMA channel 2 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c2llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C2LLR)

For information about available fields see [`mod@c2llr`]
module*/
pub type C2LLR = crate::Reg<c2llr::C2LLRrs>;
///GPDMA channel 2 linked-list address register
pub mod c2llr;
/**C3LBAR (rw) register accessor: GPDMA channel 3 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c3lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3LBAR)

For information about available fields see [`mod@c3lbar`]
module*/
pub type C3LBAR = crate::Reg<c3lbar::C3LBARrs>;
///GPDMA channel 3 linked-list base address register
pub mod c3lbar;
/**C3FCR (w) register accessor: GPDMA channel 3 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3FCR)

For information about available fields see [`mod@c3fcr`]
module*/
pub type C3FCR = crate::Reg<c3fcr::C3FCRrs>;
///GPDMA channel 3 flag clear register
pub mod c3fcr;
/**C3SR (r) register accessor: GPDMA channel 3 status register

You can [`read`](crate::Reg::read) this register and get [`c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3SR)

For information about available fields see [`mod@c3sr`]
module*/
pub type C3SR = crate::Reg<c3sr::C3SRrs>;
///GPDMA channel 3 status register
pub mod c3sr;
/**C3CR (rw) register accessor: GPDMA channel 3 control register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3CR)

For information about available fields see [`mod@c3cr`]
module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///GPDMA channel 3 control register
pub mod c3cr;
/**C3TR1 (rw) register accessor: GPDMA channel 3 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c3tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3TR1)

For information about available fields see [`mod@c3tr1`]
module*/
pub type C3TR1 = crate::Reg<c3tr1::C3TR1rs>;
///GPDMA channel 3 transfer register 1
pub mod c3tr1;
/**C3TR2 (rw) register accessor: GPDMA channel 3 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c3tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3TR2)

For information about available fields see [`mod@c3tr2`]
module*/
pub type C3TR2 = crate::Reg<c3tr2::C3TR2rs>;
///GPDMA channel 3 transfer register 2
pub mod c3tr2;
/**C3BR1 (rw) register accessor: GPDMA channel 3 block register 1

You can [`read`](crate::Reg::read) this register and get [`c3br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3BR1)

For information about available fields see [`mod@c3br1`]
module*/
pub type C3BR1 = crate::Reg<c3br1::C3BR1rs>;
///GPDMA channel 3 block register 1
pub mod c3br1;
/**C3SAR (rw) register accessor: GPDMA channel 3 source address register

You can [`read`](crate::Reg::read) this register and get [`c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3SAR)

For information about available fields see [`mod@c3sar`]
module*/
pub type C3SAR = crate::Reg<c3sar::C3SARrs>;
///GPDMA channel 3 source address register
pub mod c3sar;
/**C3DAR (rw) register accessor: GPDMA channel 3 destination address register

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3DAR)

For information about available fields see [`mod@c3dar`]
module*/
pub type C3DAR = crate::Reg<c3dar::C3DARrs>;
///GPDMA channel 3 destination address register
pub mod c3dar;
/**C3LLR (rw) register accessor: GPDMA channel 3 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c3llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C3LLR)

For information about available fields see [`mod@c3llr`]
module*/
pub type C3LLR = crate::Reg<c3llr::C3LLRrs>;
///GPDMA channel 3 linked-list address register
pub mod c3llr;
/**C4LBAR (rw) register accessor: GPDMA channel 4 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c4lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4LBAR)

For information about available fields see [`mod@c4lbar`]
module*/
pub type C4LBAR = crate::Reg<c4lbar::C4LBARrs>;
///GPDMA channel 4 linked-list base address register
pub mod c4lbar;
/**C4FCR (w) register accessor: GPDMA channel 4 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4FCR)

For information about available fields see [`mod@c4fcr`]
module*/
pub type C4FCR = crate::Reg<c4fcr::C4FCRrs>;
///GPDMA channel 4 flag clear register
pub mod c4fcr;
/**C4SR (r) register accessor: GPDMA channel 4 status register

You can [`read`](crate::Reg::read) this register and get [`c4sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4SR)

For information about available fields see [`mod@c4sr`]
module*/
pub type C4SR = crate::Reg<c4sr::C4SRrs>;
///GPDMA channel 4 status register
pub mod c4sr;
/**C4CR (rw) register accessor: GPDMA channel 4 control register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4CR)

For information about available fields see [`mod@c4cr`]
module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///GPDMA channel 4 control register
pub mod c4cr;
/**C4TR1 (rw) register accessor: GPDMA channel 4 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c4tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4TR1)

For information about available fields see [`mod@c4tr1`]
module*/
pub type C4TR1 = crate::Reg<c4tr1::C4TR1rs>;
///GPDMA channel 4 transfer register 1
pub mod c4tr1;
/**C4TR2 (rw) register accessor: GPDMA channel 4 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c4tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4TR2)

For information about available fields see [`mod@c4tr2`]
module*/
pub type C4TR2 = crate::Reg<c4tr2::C4TR2rs>;
///GPDMA channel 4 transfer register 2
pub mod c4tr2;
/**C4BR1 (rw) register accessor: GPDMA channel 4 block register 1

You can [`read`](crate::Reg::read) this register and get [`c4br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4BR1)

For information about available fields see [`mod@c4br1`]
module*/
pub type C4BR1 = crate::Reg<c4br1::C4BR1rs>;
///GPDMA channel 4 block register 1
pub mod c4br1;
/**C4SAR (rw) register accessor: GPDMA channel 4 source address register

You can [`read`](crate::Reg::read) this register and get [`c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4SAR)

For information about available fields see [`mod@c4sar`]
module*/
pub type C4SAR = crate::Reg<c4sar::C4SARrs>;
///GPDMA channel 4 source address register
pub mod c4sar;
/**C4DAR (rw) register accessor: GPDMA channel 4 destination address register

You can [`read`](crate::Reg::read) this register and get [`c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4DAR)

For information about available fields see [`mod@c4dar`]
module*/
pub type C4DAR = crate::Reg<c4dar::C4DARrs>;
///GPDMA channel 4 destination address register
pub mod c4dar;
/**C4LLR (rw) register accessor: GPDMA channel 4 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c4llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C4LLR)

For information about available fields see [`mod@c4llr`]
module*/
pub type C4LLR = crate::Reg<c4llr::C4LLRrs>;
///GPDMA channel 4 linked-list address register
pub mod c4llr;
/**C5LBAR (rw) register accessor: GPDMA channel 5 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c5lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5LBAR)

For information about available fields see [`mod@c5lbar`]
module*/
pub type C5LBAR = crate::Reg<c5lbar::C5LBARrs>;
///GPDMA channel 5 linked-list base address register
pub mod c5lbar;
/**C5FCR (w) register accessor: GPDMA channel 5 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5FCR)

For information about available fields see [`mod@c5fcr`]
module*/
pub type C5FCR = crate::Reg<c5fcr::C5FCRrs>;
///GPDMA channel 5 flag clear register
pub mod c5fcr;
/**C5SR (r) register accessor: GPDMA channel 5 status register

You can [`read`](crate::Reg::read) this register and get [`c5sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5SR)

For information about available fields see [`mod@c5sr`]
module*/
pub type C5SR = crate::Reg<c5sr::C5SRrs>;
///GPDMA channel 5 status register
pub mod c5sr;
/**C5CR (rw) register accessor: GPDMA channel 5 control register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5CR)

For information about available fields see [`mod@c5cr`]
module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///GPDMA channel 5 control register
pub mod c5cr;
/**C5TR1 (rw) register accessor: GPDMA channel 5 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c5tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5TR1)

For information about available fields see [`mod@c5tr1`]
module*/
pub type C5TR1 = crate::Reg<c5tr1::C5TR1rs>;
///GPDMA channel 5 transfer register 1
pub mod c5tr1;
/**C5TR2 (rw) register accessor: GPDMA channel 5 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c5tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5TR2)

For information about available fields see [`mod@c5tr2`]
module*/
pub type C5TR2 = crate::Reg<c5tr2::C5TR2rs>;
///GPDMA channel 5 transfer register 2
pub mod c5tr2;
/**C5BR1 (rw) register accessor: GPDMA channel 5 block register 1

You can [`read`](crate::Reg::read) this register and get [`c5br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5BR1)

For information about available fields see [`mod@c5br1`]
module*/
pub type C5BR1 = crate::Reg<c5br1::C5BR1rs>;
///GPDMA channel 5 block register 1
pub mod c5br1;
/**C5SAR (rw) register accessor: GPDMA channel 5 source address register

You can [`read`](crate::Reg::read) this register and get [`c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5SAR)

For information about available fields see [`mod@c5sar`]
module*/
pub type C5SAR = crate::Reg<c5sar::C5SARrs>;
///GPDMA channel 5 source address register
pub mod c5sar;
/**C5DAR (rw) register accessor: GPDMA channel 5 destination address register

You can [`read`](crate::Reg::read) this register and get [`c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5DAR)

For information about available fields see [`mod@c5dar`]
module*/
pub type C5DAR = crate::Reg<c5dar::C5DARrs>;
///GPDMA channel 5 destination address register
pub mod c5dar;
/**C5LLR (rw) register accessor: GPDMA channel 5 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c5llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C5LLR)

For information about available fields see [`mod@c5llr`]
module*/
pub type C5LLR = crate::Reg<c5llr::C5LLRrs>;
///GPDMA channel 5 linked-list address register
pub mod c5llr;
/**C6LBAR (rw) register accessor: GPDMA channel 6 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c6lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6LBAR)

For information about available fields see [`mod@c6lbar`]
module*/
pub type C6LBAR = crate::Reg<c6lbar::C6LBARrs>;
///GPDMA channel 6 linked-list base address register
pub mod c6lbar;
/**C6FCR (w) register accessor: GPDMA channel 6 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6FCR)

For information about available fields see [`mod@c6fcr`]
module*/
pub type C6FCR = crate::Reg<c6fcr::C6FCRrs>;
///GPDMA channel 6 flag clear register
pub mod c6fcr;
/**C6SR (r) register accessor: GPDMA channel 6 status register

You can [`read`](crate::Reg::read) this register and get [`c6sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6SR)

For information about available fields see [`mod@c6sr`]
module*/
pub type C6SR = crate::Reg<c6sr::C6SRrs>;
///GPDMA channel 6 status register
pub mod c6sr;
/**C6CR (rw) register accessor: GPDMA channel 6 control register

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6CR)

For information about available fields see [`mod@c6cr`]
module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///GPDMA channel 6 control register
pub mod c6cr;
/**C6TR1 (rw) register accessor: GPDMA channel 6 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c6tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6TR1)

For information about available fields see [`mod@c6tr1`]
module*/
pub type C6TR1 = crate::Reg<c6tr1::C6TR1rs>;
///GPDMA channel 6 transfer register 1
pub mod c6tr1;
/**C6TR2 (rw) register accessor: GPDMA channel 6 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c6tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6TR2)

For information about available fields see [`mod@c6tr2`]
module*/
pub type C6TR2 = crate::Reg<c6tr2::C6TR2rs>;
///GPDMA channel 6 transfer register 2
pub mod c6tr2;
/**C6BR1 (rw) register accessor: GPDMA channel 6 alternate block register 1

You can [`read`](crate::Reg::read) this register and get [`c6br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6BR1)

For information about available fields see [`mod@c6br1`]
module*/
pub type C6BR1 = crate::Reg<c6br1::C6BR1rs>;
///GPDMA channel 6 alternate block register 1
pub mod c6br1;
/**C6SAR (rw) register accessor: GPDMA channel 6 source address register

You can [`read`](crate::Reg::read) this register and get [`c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6SAR)

For information about available fields see [`mod@c6sar`]
module*/
pub type C6SAR = crate::Reg<c6sar::C6SARrs>;
///GPDMA channel 6 source address register
pub mod c6sar;
/**C6DAR (rw) register accessor: GPDMA channel 6 destination address register

You can [`read`](crate::Reg::read) this register and get [`c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6DAR)

For information about available fields see [`mod@c6dar`]
module*/
pub type C6DAR = crate::Reg<c6dar::C6DARrs>;
///GPDMA channel 6 destination address register
pub mod c6dar;
/**C6TR3 (rw) register accessor: GPDMA channel 6 transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c6tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6TR3)

For information about available fields see [`mod@c6tr3`]
module*/
pub type C6TR3 = crate::Reg<c6tr3::C6TR3rs>;
///GPDMA channel 6 transfer register 3
pub mod c6tr3;
/**C6BR2 (rw) register accessor: GPDMA channel 6 block register 2

You can [`read`](crate::Reg::read) this register and get [`c6br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6BR2)

For information about available fields see [`mod@c6br2`]
module*/
pub type C6BR2 = crate::Reg<c6br2::C6BR2rs>;
///GPDMA channel 6 block register 2
pub mod c6br2;
/**C6LLR (rw) register accessor: GPDMA channel 6 alternate linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c6llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C6LLR)

For information about available fields see [`mod@c6llr`]
module*/
pub type C6LLR = crate::Reg<c6llr::C6LLRrs>;
///GPDMA channel 6 alternate linked-list address register
pub mod c6llr;
/**C7LBAR (rw) register accessor: GPDMA channel 7 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c7lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7LBAR)

For information about available fields see [`mod@c7lbar`]
module*/
pub type C7LBAR = crate::Reg<c7lbar::C7LBARrs>;
///GPDMA channel 7 linked-list base address register
pub mod c7lbar;
/**C7FCR (w) register accessor: GPDMA channel 7 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7FCR)

For information about available fields see [`mod@c7fcr`]
module*/
pub type C7FCR = crate::Reg<c7fcr::C7FCRrs>;
///GPDMA channel 7 flag clear register
pub mod c7fcr;
/**C7SR (r) register accessor: GPDMA channel 7 status register

You can [`read`](crate::Reg::read) this register and get [`c7sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7SR)

For information about available fields see [`mod@c7sr`]
module*/
pub type C7SR = crate::Reg<c7sr::C7SRrs>;
///GPDMA channel 7 status register
pub mod c7sr;
/**C7CR (rw) register accessor: GPDMA channel 7 control register

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7CR)

For information about available fields see [`mod@c7cr`]
module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///GPDMA channel 7 control register
pub mod c7cr;
/**C7TR1 (rw) register accessor: GPDMA channel 7 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c7tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7TR1)

For information about available fields see [`mod@c7tr1`]
module*/
pub type C7TR1 = crate::Reg<c7tr1::C7TR1rs>;
///GPDMA channel 7 transfer register 1
pub mod c7tr1;
/**C7TR2 (rw) register accessor: GPDMA channel 7 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c7tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7TR2)

For information about available fields see [`mod@c7tr2`]
module*/
pub type C7TR2 = crate::Reg<c7tr2::C7TR2rs>;
///GPDMA channel 7 transfer register 2
pub mod c7tr2;
/**C7BR1 (rw) register accessor: GPDMA channel 7 alternate block register 1

You can [`read`](crate::Reg::read) this register and get [`c7br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7BR1)

For information about available fields see [`mod@c7br1`]
module*/
pub type C7BR1 = crate::Reg<c7br1::C7BR1rs>;
///GPDMA channel 7 alternate block register 1
pub mod c7br1;
/**C7SAR (rw) register accessor: GPDMA channel 7 source address register

You can [`read`](crate::Reg::read) this register and get [`c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7SAR)

For information about available fields see [`mod@c7sar`]
module*/
pub type C7SAR = crate::Reg<c7sar::C7SARrs>;
///GPDMA channel 7 source address register
pub mod c7sar;
/**C7DAR (rw) register accessor: GPDMA channel 7 destination address register

You can [`read`](crate::Reg::read) this register and get [`c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7DAR)

For information about available fields see [`mod@c7dar`]
module*/
pub type C7DAR = crate::Reg<c7dar::C7DARrs>;
///GPDMA channel 7 destination address register
pub mod c7dar;
/**C7TR3 (rw) register accessor: GPDMA channel 7 transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c7tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7TR3)

For information about available fields see [`mod@c7tr3`]
module*/
pub type C7TR3 = crate::Reg<c7tr3::C7TR3rs>;
///GPDMA channel 7 transfer register 3
pub mod c7tr3;
/**C7BR2 (rw) register accessor: GPDMA channel 7 block register 2

You can [`read`](crate::Reg::read) this register and get [`c7br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7BR2)

For information about available fields see [`mod@c7br2`]
module*/
pub type C7BR2 = crate::Reg<c7br2::C7BR2rs>;
///GPDMA channel 7 block register 2
pub mod c7br2;
/**C7LLR (rw) register accessor: GPDMA channel 7 alternate linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c7llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:C7LLR)

For information about available fields see [`mod@c7llr`]
module*/
pub type C7LLR = crate::Reg<c7llr::C7LLRrs>;
///GPDMA channel 7 alternate linked-list address register
pub mod c7llr;
