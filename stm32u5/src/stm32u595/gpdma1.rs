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
    c4lbar: C4LBAR,
    _reserved45: [u8; 0x08],
    c4fcr: C4FCR,
    c4sr: C4SR,
    c4cr: C4CR,
    _reserved48: [u8; 0x2c],
    c4tr2: C4TR2,
    c4br1: C4BR1,
    c4sar: C4SAR,
    c4dar: C4DAR,
    _reserved52: [u8; 0x28],
    c4llr: C4LLR,
    c5lbar: C5LBAR,
    _reserved54: [u8; 0x08],
    c5fcr: C5FCR,
    c5sr: C5SR,
    c5cr: C5CR,
    _reserved57: [u8; 0x2c],
    c5tr2: C5TR2,
    c5br1: C5BR1,
    c5sar: C5SAR,
    c5dar: C5DAR,
    _reserved61: [u8; 0x28],
    c5llr: C5LLR,
    c6lbar: C6LBAR,
    _reserved63: [u8; 0x08],
    c6fcr: C6FCR,
    c6sr: C6SR,
    c6cr: C6CR,
    _reserved66: [u8; 0x2c],
    c6tr2: C6TR2,
    c6br1: C6BR1,
    c6sar: C6SAR,
    c6dar: C6DAR,
    _reserved70: [u8; 0x28],
    c6llr: C6LLR,
    c7lbar: C7LBAR,
    _reserved72: [u8; 0x08],
    c7fcr: C7FCR,
    c7sr: C7SR,
    c7cr: C7CR,
    _reserved75: [u8; 0x2c],
    c7tr2: C7TR2,
    c7br1: C7BR1,
    c7sar: C7SAR,
    c7dar: C7DAR,
    _reserved79: [u8; 0x28],
    c7llr: C7LLR,
    c8lbar: C8LBAR,
    _reserved81: [u8; 0x08],
    c8fcr: C8FCR,
    c8sr: C8SR,
    c8cr: C8CR,
    _reserved84: [u8; 0x2c],
    c8tr2: C8TR2,
    c8br1: C8BR1,
    c8sar: C8SAR,
    c8dar: C8DAR,
    _reserved88: [u8; 0x28],
    c8llr: C8LLR,
    c9lbar: C9LBAR,
    _reserved90: [u8; 0x08],
    c9fcr: C9FCR,
    c9sr: C9SR,
    c9cr: C9CR,
    _reserved93: [u8; 0x2c],
    c9tr2: C9TR2,
    c9br1: C9BR1,
    c9sar: C9SAR,
    c9dar: C9DAR,
    _reserved97: [u8; 0x28],
    c9llr: C9LLR,
    c10lbar: C10LBAR,
    _reserved99: [u8; 0x08],
    c10fcr: C10FCR,
    c10sr: C10SR,
    c10cr: C10CR,
    _reserved102: [u8; 0x2c],
    c10tr2: C10TR2,
    c10br1: C10BR1,
    c10sar: C10SAR,
    c10dar: C10DAR,
    _reserved106: [u8; 0x28],
    c10llr: C10LLR,
    c11lbar: C11LBAR,
    _reserved108: [u8; 0x08],
    c11fcr: C11FCR,
    c11sr: C11SR,
    c11cr: C11CR,
    _reserved111: [u8; 0x2c],
    c11tr2: C11TR2,
    c11br1: C11BR1,
    c11sar: C11SAR,
    c11dar: C11DAR,
    _reserved115: [u8; 0x28],
    c11llr: C11LLR,
    c12lbar: C12LBAR,
    _reserved117: [u8; 0x08],
    c12fcr: C12FCR,
    c12sr: C12SR,
    c12cr: C12CR,
    _reserved120: [u8; 0x2c],
    c12tr2: C12TR2,
    c12br1: C12BR1,
    c12sar: C12SAR,
    c12dar: C12DAR,
    c12tr3: C12TR3,
    c12br2: C12BR2,
    _reserved126: [u8; 0x20],
    c12llr: C12LLR,
    c13lbar: C13LBAR,
    _reserved128: [u8; 0x08],
    c13fcr: C13FCR,
    c13sr: C13SR,
    c13cr: C13CR,
    _reserved131: [u8; 0x2c],
    c13tr2: C13TR2,
    c13br1: C13BR1,
    c13sar: C13SAR,
    c13dar: C13DAR,
    c13tr3: C13TR3,
    c13br2: C13BR2,
    _reserved137: [u8; 0x20],
    c13llr: C13LLR,
    c14lbar: C14LBAR,
    _reserved139: [u8; 0x08],
    c14fcr: C14FCR,
    c14sr: C14SR,
    c14cr: C14CR,
    _reserved142: [u8; 0x2c],
    c14tr2: C14TR2,
    c14br1: C14BR1,
    c14sar: C14SAR,
    c14dar: C14DAR,
    c14tr3: C14TR3,
    c14br2: C14BR2,
    _reserved148: [u8; 0x20],
    c14llr: C14LLR,
    c15lbar: C15LBAR,
    _reserved150: [u8; 0x08],
    c15fcr: C15FCR,
    c15sr: C15SR,
    c15cr: C15CR,
    _reserved153: [u8; 0x2c],
    c15tr2: C15TR2,
    c15br1: C15BR1,
    c15sar: C15SAR,
    c15dar: C15DAR,
    c15tr3: C15TR3,
    c15br2: C15BR2,
    _reserved159: [u8; 0x20],
    c15llr: C15LLR,
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
    ///0x0c - non-secure masked interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x10 - secure masked interrupt status register
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    ///0x50 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c0lbar(&self) -> &C0LBAR {
        &self.c0lbar
    }
    ///0x5c - GPDMA channel x flag clear register
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
    ///0x90 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c0tr1(&self) -> &C0TR1 {
        &self.c0tr1
    }
    ///0x94 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c0tr2(&self) -> &C0TR2 {
        &self.c0tr2
    }
    ///0x98 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c0br1(&self) -> &C0BR1 {
        &self.c0br1
    }
    ///0x9c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c0sar(&self) -> &C0SAR {
        &self.c0sar
    }
    ///0xa0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c0dar(&self) -> &C0DAR {
        &self.c0dar
    }
    ///0xcc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c0llr(&self) -> &C0LLR {
        &self.c0llr
    }
    ///0xd0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c1lbar(&self) -> &C1LBAR {
        &self.c1lbar
    }
    ///0xdc - GPDMA channel x flag clear register
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
    ///0x110 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c1tr1(&self) -> &C1TR1 {
        &self.c1tr1
    }
    ///0x114 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c1tr2(&self) -> &C1TR2 {
        &self.c1tr2
    }
    ///0x118 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c1br1(&self) -> &C1BR1 {
        &self.c1br1
    }
    ///0x11c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c1sar(&self) -> &C1SAR {
        &self.c1sar
    }
    ///0x120 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c1dar(&self) -> &C1DAR {
        &self.c1dar
    }
    ///0x14c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c1llr(&self) -> &C1LLR {
        &self.c1llr
    }
    ///0x150 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c2lbar(&self) -> &C2LBAR {
        &self.c2lbar
    }
    ///0x15c - GPDMA channel x flag clear register
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
    ///0x190 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c2tr1(&self) -> &C2TR1 {
        &self.c2tr1
    }
    ///0x194 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c2tr2(&self) -> &C2TR2 {
        &self.c2tr2
    }
    ///0x198 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c2br1(&self) -> &C2BR1 {
        &self.c2br1
    }
    ///0x19c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c2sar(&self) -> &C2SAR {
        &self.c2sar
    }
    ///0x1a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c2dar(&self) -> &C2DAR {
        &self.c2dar
    }
    ///0x1cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c2llr(&self) -> &C2LLR {
        &self.c2llr
    }
    ///0x1d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c3lbar(&self) -> &C3LBAR {
        &self.c3lbar
    }
    ///0x1dc - GPDMA channel x flag clear register
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
    ///0x210 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn c3tr1(&self) -> &C3TR1 {
        &self.c3tr1
    }
    ///0x214 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c3tr2(&self) -> &C3TR2 {
        &self.c3tr2
    }
    ///0x218 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c3br1(&self) -> &C3BR1 {
        &self.c3br1
    }
    ///0x21c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c3sar(&self) -> &C3SAR {
        &self.c3sar
    }
    ///0x220 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c3dar(&self) -> &C3DAR {
        &self.c3dar
    }
    ///0x24c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c3llr(&self) -> &C3LLR {
        &self.c3llr
    }
    ///0x250 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c4lbar(&self) -> &C4LBAR {
        &self.c4lbar
    }
    ///0x25c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c4fcr(&self) -> &C4FCR {
        &self.c4fcr
    }
    ///0x260 - channel x status register
    #[inline(always)]
    pub const fn c4sr(&self) -> &C4SR {
        &self.c4sr
    }
    ///0x264 - channel x control register
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4CR {
        &self.c4cr
    }
    ///0x294 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c4tr2(&self) -> &C4TR2 {
        &self.c4tr2
    }
    ///0x298 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c4br1(&self) -> &C4BR1 {
        &self.c4br1
    }
    ///0x29c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c4sar(&self) -> &C4SAR {
        &self.c4sar
    }
    ///0x2a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c4dar(&self) -> &C4DAR {
        &self.c4dar
    }
    ///0x2cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c4llr(&self) -> &C4LLR {
        &self.c4llr
    }
    ///0x2d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c5lbar(&self) -> &C5LBAR {
        &self.c5lbar
    }
    ///0x2dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c5fcr(&self) -> &C5FCR {
        &self.c5fcr
    }
    ///0x2e0 - channel x status register
    #[inline(always)]
    pub const fn c5sr(&self) -> &C5SR {
        &self.c5sr
    }
    ///0x2e4 - channel x control register
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5CR {
        &self.c5cr
    }
    ///0x314 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c5tr2(&self) -> &C5TR2 {
        &self.c5tr2
    }
    ///0x318 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c5br1(&self) -> &C5BR1 {
        &self.c5br1
    }
    ///0x31c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c5sar(&self) -> &C5SAR {
        &self.c5sar
    }
    ///0x320 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c5dar(&self) -> &C5DAR {
        &self.c5dar
    }
    ///0x34c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c5llr(&self) -> &C5LLR {
        &self.c5llr
    }
    ///0x350 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c6lbar(&self) -> &C6LBAR {
        &self.c6lbar
    }
    ///0x35c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c6fcr(&self) -> &C6FCR {
        &self.c6fcr
    }
    ///0x360 - channel x status register
    #[inline(always)]
    pub const fn c6sr(&self) -> &C6SR {
        &self.c6sr
    }
    ///0x364 - channel x control register
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6CR {
        &self.c6cr
    }
    ///0x394 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c6tr2(&self) -> &C6TR2 {
        &self.c6tr2
    }
    ///0x398 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c6br1(&self) -> &C6BR1 {
        &self.c6br1
    }
    ///0x39c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c6sar(&self) -> &C6SAR {
        &self.c6sar
    }
    ///0x3a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c6dar(&self) -> &C6DAR {
        &self.c6dar
    }
    ///0x3cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c6llr(&self) -> &C6LLR {
        &self.c6llr
    }
    ///0x3d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c7lbar(&self) -> &C7LBAR {
        &self.c7lbar
    }
    ///0x3dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c7fcr(&self) -> &C7FCR {
        &self.c7fcr
    }
    ///0x3e0 - channel x status register
    #[inline(always)]
    pub const fn c7sr(&self) -> &C7SR {
        &self.c7sr
    }
    ///0x3e4 - channel x control register
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7CR {
        &self.c7cr
    }
    ///0x414 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c7tr2(&self) -> &C7TR2 {
        &self.c7tr2
    }
    ///0x418 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c7br1(&self) -> &C7BR1 {
        &self.c7br1
    }
    ///0x41c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c7sar(&self) -> &C7SAR {
        &self.c7sar
    }
    ///0x420 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c7dar(&self) -> &C7DAR {
        &self.c7dar
    }
    ///0x44c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c7llr(&self) -> &C7LLR {
        &self.c7llr
    }
    ///0x450 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c8lbar(&self) -> &C8LBAR {
        &self.c8lbar
    }
    ///0x45c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c8fcr(&self) -> &C8FCR {
        &self.c8fcr
    }
    ///0x460 - channel x status register
    #[inline(always)]
    pub const fn c8sr(&self) -> &C8SR {
        &self.c8sr
    }
    ///0x464 - channel x control register
    #[inline(always)]
    pub const fn c8cr(&self) -> &C8CR {
        &self.c8cr
    }
    ///0x494 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c8tr2(&self) -> &C8TR2 {
        &self.c8tr2
    }
    ///0x498 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c8br1(&self) -> &C8BR1 {
        &self.c8br1
    }
    ///0x49c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c8sar(&self) -> &C8SAR {
        &self.c8sar
    }
    ///0x4a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c8dar(&self) -> &C8DAR {
        &self.c8dar
    }
    ///0x4cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c8llr(&self) -> &C8LLR {
        &self.c8llr
    }
    ///0x4d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c9lbar(&self) -> &C9LBAR {
        &self.c9lbar
    }
    ///0x4dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c9fcr(&self) -> &C9FCR {
        &self.c9fcr
    }
    ///0x4e0 - channel x status register
    #[inline(always)]
    pub const fn c9sr(&self) -> &C9SR {
        &self.c9sr
    }
    ///0x4e4 - channel x control register
    #[inline(always)]
    pub const fn c9cr(&self) -> &C9CR {
        &self.c9cr
    }
    ///0x514 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c9tr2(&self) -> &C9TR2 {
        &self.c9tr2
    }
    ///0x518 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c9br1(&self) -> &C9BR1 {
        &self.c9br1
    }
    ///0x51c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c9sar(&self) -> &C9SAR {
        &self.c9sar
    }
    ///0x520 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c9dar(&self) -> &C9DAR {
        &self.c9dar
    }
    ///0x54c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c9llr(&self) -> &C9LLR {
        &self.c9llr
    }
    ///0x550 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c10lbar(&self) -> &C10LBAR {
        &self.c10lbar
    }
    ///0x55c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c10fcr(&self) -> &C10FCR {
        &self.c10fcr
    }
    ///0x560 - channel x status register
    #[inline(always)]
    pub const fn c10sr(&self) -> &C10SR {
        &self.c10sr
    }
    ///0x564 - channel x control register
    #[inline(always)]
    pub const fn c10cr(&self) -> &C10CR {
        &self.c10cr
    }
    ///0x594 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c10tr2(&self) -> &C10TR2 {
        &self.c10tr2
    }
    ///0x598 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c10br1(&self) -> &C10BR1 {
        &self.c10br1
    }
    ///0x59c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c10sar(&self) -> &C10SAR {
        &self.c10sar
    }
    ///0x5a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c10dar(&self) -> &C10DAR {
        &self.c10dar
    }
    ///0x5cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c10llr(&self) -> &C10LLR {
        &self.c10llr
    }
    ///0x5d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c11lbar(&self) -> &C11LBAR {
        &self.c11lbar
    }
    ///0x5dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c11fcr(&self) -> &C11FCR {
        &self.c11fcr
    }
    ///0x5e0 - channel x status register
    #[inline(always)]
    pub const fn c11sr(&self) -> &C11SR {
        &self.c11sr
    }
    ///0x5e4 - channel x control register
    #[inline(always)]
    pub const fn c11cr(&self) -> &C11CR {
        &self.c11cr
    }
    ///0x614 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c11tr2(&self) -> &C11TR2 {
        &self.c11tr2
    }
    ///0x618 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c11br1(&self) -> &C11BR1 {
        &self.c11br1
    }
    ///0x61c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c11sar(&self) -> &C11SAR {
        &self.c11sar
    }
    ///0x620 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c11dar(&self) -> &C11DAR {
        &self.c11dar
    }
    ///0x64c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c11llr(&self) -> &C11LLR {
        &self.c11llr
    }
    ///0x650 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c12lbar(&self) -> &C12LBAR {
        &self.c12lbar
    }
    ///0x65c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c12fcr(&self) -> &C12FCR {
        &self.c12fcr
    }
    ///0x660 - channel x status register
    #[inline(always)]
    pub const fn c12sr(&self) -> &C12SR {
        &self.c12sr
    }
    ///0x664 - channel x control register
    #[inline(always)]
    pub const fn c12cr(&self) -> &C12CR {
        &self.c12cr
    }
    ///0x694 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c12tr2(&self) -> &C12TR2 {
        &self.c12tr2
    }
    ///0x698 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c12br1(&self) -> &C12BR1 {
        &self.c12br1
    }
    ///0x69c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c12sar(&self) -> &C12SAR {
        &self.c12sar
    }
    ///0x6a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c12dar(&self) -> &C12DAR {
        &self.c12dar
    }
    ///0x6a4 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn c12tr3(&self) -> &C12TR3 {
        &self.c12tr3
    }
    ///0x6a8 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn c12br2(&self) -> &C12BR2 {
        &self.c12br2
    }
    ///0x6cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c12llr(&self) -> &C12LLR {
        &self.c12llr
    }
    ///0x6d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c13lbar(&self) -> &C13LBAR {
        &self.c13lbar
    }
    ///0x6dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c13fcr(&self) -> &C13FCR {
        &self.c13fcr
    }
    ///0x6e0 - channel x status register
    #[inline(always)]
    pub const fn c13sr(&self) -> &C13SR {
        &self.c13sr
    }
    ///0x6e4 - channel x control register
    #[inline(always)]
    pub const fn c13cr(&self) -> &C13CR {
        &self.c13cr
    }
    ///0x714 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c13tr2(&self) -> &C13TR2 {
        &self.c13tr2
    }
    ///0x718 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c13br1(&self) -> &C13BR1 {
        &self.c13br1
    }
    ///0x71c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c13sar(&self) -> &C13SAR {
        &self.c13sar
    }
    ///0x720 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c13dar(&self) -> &C13DAR {
        &self.c13dar
    }
    ///0x724 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn c13tr3(&self) -> &C13TR3 {
        &self.c13tr3
    }
    ///0x728 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn c13br2(&self) -> &C13BR2 {
        &self.c13br2
    }
    ///0x74c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c13llr(&self) -> &C13LLR {
        &self.c13llr
    }
    ///0x750 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c14lbar(&self) -> &C14LBAR {
        &self.c14lbar
    }
    ///0x75c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c14fcr(&self) -> &C14FCR {
        &self.c14fcr
    }
    ///0x760 - channel x status register
    #[inline(always)]
    pub const fn c14sr(&self) -> &C14SR {
        &self.c14sr
    }
    ///0x764 - channel x control register
    #[inline(always)]
    pub const fn c14cr(&self) -> &C14CR {
        &self.c14cr
    }
    ///0x794 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c14tr2(&self) -> &C14TR2 {
        &self.c14tr2
    }
    ///0x798 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c14br1(&self) -> &C14BR1 {
        &self.c14br1
    }
    ///0x79c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c14sar(&self) -> &C14SAR {
        &self.c14sar
    }
    ///0x7a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c14dar(&self) -> &C14DAR {
        &self.c14dar
    }
    ///0x7a4 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn c14tr3(&self) -> &C14TR3 {
        &self.c14tr3
    }
    ///0x7a8 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn c14br2(&self) -> &C14BR2 {
        &self.c14br2
    }
    ///0x7cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c14llr(&self) -> &C14LLR {
        &self.c14llr
    }
    ///0x7d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn c15lbar(&self) -> &C15LBAR {
        &self.c15lbar
    }
    ///0x7dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn c15fcr(&self) -> &C15FCR {
        &self.c15fcr
    }
    ///0x7e0 - channel x status register
    #[inline(always)]
    pub const fn c15sr(&self) -> &C15SR {
        &self.c15sr
    }
    ///0x7e4 - channel x control register
    #[inline(always)]
    pub const fn c15cr(&self) -> &C15CR {
        &self.c15cr
    }
    ///0x814 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn c15tr2(&self) -> &C15TR2 {
        &self.c15tr2
    }
    ///0x818 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn c15br1(&self) -> &C15BR1 {
        &self.c15br1
    }
    ///0x81c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn c15sar(&self) -> &C15SAR {
        &self.c15sar
    }
    ///0x820 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn c15dar(&self) -> &C15DAR {
        &self.c15dar
    }
    ///0x824 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn c15tr3(&self) -> &C15TR3 {
        &self.c15tr3
    }
    ///0x828 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn c15br2(&self) -> &C15BR2 {
        &self.c15br2
    }
    ///0x84c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn c15llr(&self) -> &C15LLR {
        &self.c15llr
    }
}
/**SECCFGR (rw) register accessor: GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:SECCFGR)

For information about available fields see [`mod@seccfgr`]
module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///GPDMA secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:PRIVCFGR)

For information about available fields see [`mod@privcfgr`]
module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod privcfgr;
/**MISR (r) register accessor: non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:SMISR)

For information about available fields see [`mod@smisr`]
module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///secure masked interrupt status register
pub mod smisr;
/**C0LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c0lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0LBAR)

For information about available fields see [`mod@c0lbar`]
module*/
pub type C0LBAR = crate::Reg<c0lbar::C0LBARrs>;
///channel x linked-list base address register
pub mod c0lbar;
/**C0FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0FCR)

For information about available fields see [`mod@c0fcr`]
module*/
pub type C0FCR = crate::Reg<c0fcr::C0FCRrs>;
///GPDMA channel x flag clear register
pub mod c0fcr;
/**C0SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0SR)

For information about available fields see [`mod@c0sr`]
module*/
pub type C0SR = crate::Reg<c0sr::C0SRrs>;
///channel x status register
pub mod c0sr;
/**C0CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0CR)

For information about available fields see [`mod@c0cr`]
module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///channel x control register
pub mod c0cr;
/**C0TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c0tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0TR1)

For information about available fields see [`mod@c0tr1`]
module*/
pub type C0TR1 = crate::Reg<c0tr1::C0TR1rs>;
///GPDMA channel x transfer register 1
pub mod c0tr1;
/**C0TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c0tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0TR2)

For information about available fields see [`mod@c0tr2`]
module*/
pub type C0TR2 = crate::Reg<c0tr2::C0TR2rs>;
///GPDMA channel x transfer register 2
pub mod c0tr2;
/**C0BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0BR1)

For information about available fields see [`mod@c0br1`]
module*/
pub type C0BR1 = crate::Reg<c0br1::C0BR1rs>;
///GPDMA channel x block register 1
pub mod c0br1;
/**C0SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0SAR)

For information about available fields see [`mod@c0sar`]
module*/
pub type C0SAR = crate::Reg<c0sar::C0SARrs>;
///GPDMA channel x source address register
pub mod c0sar;
/**C0DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0DAR)

For information about available fields see [`mod@c0dar`]
module*/
pub type C0DAR = crate::Reg<c0dar::C0DARrs>;
///GPDMA channel x destination address register
pub mod c0dar;
/**C0LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c0llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C0LLR)

For information about available fields see [`mod@c0llr`]
module*/
pub type C0LLR = crate::Reg<c0llr::C0LLRrs>;
///GPDMA channel x linked-list address register
pub mod c0llr;
/**C1LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c1lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1LBAR)

For information about available fields see [`mod@c1lbar`]
module*/
pub type C1LBAR = crate::Reg<c1lbar::C1LBARrs>;
///channel x linked-list base address register
pub mod c1lbar;
/**C1FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1FCR)

For information about available fields see [`mod@c1fcr`]
module*/
pub type C1FCR = crate::Reg<c1fcr::C1FCRrs>;
///GPDMA channel x flag clear register
pub mod c1fcr;
/**C1SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1SR)

For information about available fields see [`mod@c1sr`]
module*/
pub type C1SR = crate::Reg<c1sr::C1SRrs>;
///channel x status register
pub mod c1sr;
/**C1CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1CR)

For information about available fields see [`mod@c1cr`]
module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///channel x control register
pub mod c1cr;
/**C1TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c1tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1TR1)

For information about available fields see [`mod@c1tr1`]
module*/
pub type C1TR1 = crate::Reg<c1tr1::C1TR1rs>;
///GPDMA channel x transfer register 1
pub mod c1tr1;
/**C1TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c1tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1TR2)

For information about available fields see [`mod@c1tr2`]
module*/
pub type C1TR2 = crate::Reg<c1tr2::C1TR2rs>;
///GPDMA channel x transfer register 2
pub mod c1tr2;
/**C1BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1BR1)

For information about available fields see [`mod@c1br1`]
module*/
pub type C1BR1 = crate::Reg<c1br1::C1BR1rs>;
///GPDMA channel x block register 1
pub mod c1br1;
/**C1SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1SAR)

For information about available fields see [`mod@c1sar`]
module*/
pub type C1SAR = crate::Reg<c1sar::C1SARrs>;
///GPDMA channel x source address register
pub mod c1sar;
/**C1DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1DAR)

For information about available fields see [`mod@c1dar`]
module*/
pub type C1DAR = crate::Reg<c1dar::C1DARrs>;
///GPDMA channel x destination address register
pub mod c1dar;
/**C1LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c1llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C1LLR)

For information about available fields see [`mod@c1llr`]
module*/
pub type C1LLR = crate::Reg<c1llr::C1LLRrs>;
///GPDMA channel x linked-list address register
pub mod c1llr;
/**C2LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c2lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2LBAR)

For information about available fields see [`mod@c2lbar`]
module*/
pub type C2LBAR = crate::Reg<c2lbar::C2LBARrs>;
///channel x linked-list base address register
pub mod c2lbar;
/**C2FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2FCR)

For information about available fields see [`mod@c2fcr`]
module*/
pub type C2FCR = crate::Reg<c2fcr::C2FCRrs>;
///GPDMA channel x flag clear register
pub mod c2fcr;
/**C2SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2SR)

For information about available fields see [`mod@c2sr`]
module*/
pub type C2SR = crate::Reg<c2sr::C2SRrs>;
///channel x status register
pub mod c2sr;
/**C2CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2CR)

For information about available fields see [`mod@c2cr`]
module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///channel x control register
pub mod c2cr;
/**C2TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c2tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2TR1)

For information about available fields see [`mod@c2tr1`]
module*/
pub type C2TR1 = crate::Reg<c2tr1::C2TR1rs>;
///GPDMA channel x transfer register 1
pub mod c2tr1;
/**C2TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c2tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2TR2)

For information about available fields see [`mod@c2tr2`]
module*/
pub type C2TR2 = crate::Reg<c2tr2::C2TR2rs>;
///GPDMA channel x transfer register 2
pub mod c2tr2;
/**C2BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c2br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2BR1)

For information about available fields see [`mod@c2br1`]
module*/
pub type C2BR1 = crate::Reg<c2br1::C2BR1rs>;
///GPDMA channel x block register 1
pub mod c2br1;
/**C2SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2SAR)

For information about available fields see [`mod@c2sar`]
module*/
pub type C2SAR = crate::Reg<c2sar::C2SARrs>;
///GPDMA channel x source address register
pub mod c2sar;
/**C2DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2DAR)

For information about available fields see [`mod@c2dar`]
module*/
pub type C2DAR = crate::Reg<c2dar::C2DARrs>;
///GPDMA channel x destination address register
pub mod c2dar;
/**C2LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c2llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C2LLR)

For information about available fields see [`mod@c2llr`]
module*/
pub type C2LLR = crate::Reg<c2llr::C2LLRrs>;
///GPDMA channel x linked-list address register
pub mod c2llr;
/**C3LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c3lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3LBAR)

For information about available fields see [`mod@c3lbar`]
module*/
pub type C3LBAR = crate::Reg<c3lbar::C3LBARrs>;
///channel x linked-list base address register
pub mod c3lbar;
/**C3FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3FCR)

For information about available fields see [`mod@c3fcr`]
module*/
pub type C3FCR = crate::Reg<c3fcr::C3FCRrs>;
///GPDMA channel x flag clear register
pub mod c3fcr;
/**C3SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3SR)

For information about available fields see [`mod@c3sr`]
module*/
pub type C3SR = crate::Reg<c3sr::C3SRrs>;
///channel x status register
pub mod c3sr;
/**C3CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3CR)

For information about available fields see [`mod@c3cr`]
module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///channel x control register
pub mod c3cr;
/**C3TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`c3tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3TR1)

For information about available fields see [`mod@c3tr1`]
module*/
pub type C3TR1 = crate::Reg<c3tr1::C3TR1rs>;
///GPDMA channel x transfer register 1
pub mod c3tr1;
/**C3TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c3tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3TR2)

For information about available fields see [`mod@c3tr2`]
module*/
pub type C3TR2 = crate::Reg<c3tr2::C3TR2rs>;
///GPDMA channel x transfer register 2
pub mod c3tr2;
/**C4TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c4tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4TR2)

For information about available fields see [`mod@c4tr2`]
module*/
pub type C4TR2 = crate::Reg<c4tr2::C4TR2rs>;
///GPDMA channel x transfer register 2
pub mod c4tr2;
/**C5TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c5tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5TR2)

For information about available fields see [`mod@c5tr2`]
module*/
pub type C5TR2 = crate::Reg<c5tr2::C5TR2rs>;
///GPDMA channel x transfer register 2
pub mod c5tr2;
/**C6TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c6tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6TR2)

For information about available fields see [`mod@c6tr2`]
module*/
pub type C6TR2 = crate::Reg<c6tr2::C6TR2rs>;
///GPDMA channel x transfer register 2
pub mod c6tr2;
/**C7TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c7tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7TR2)

For information about available fields see [`mod@c7tr2`]
module*/
pub type C7TR2 = crate::Reg<c7tr2::C7TR2rs>;
///GPDMA channel x transfer register 2
pub mod c7tr2;
/**C8TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c8tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8TR2)

For information about available fields see [`mod@c8tr2`]
module*/
pub type C8TR2 = crate::Reg<c8tr2::C8TR2rs>;
///GPDMA channel x transfer register 2
pub mod c8tr2;
/**C9TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c9tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9TR2)

For information about available fields see [`mod@c9tr2`]
module*/
pub type C9TR2 = crate::Reg<c9tr2::C9TR2rs>;
///GPDMA channel x transfer register 2
pub mod c9tr2;
/**C10TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c10tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10TR2)

For information about available fields see [`mod@c10tr2`]
module*/
pub type C10TR2 = crate::Reg<c10tr2::C10TR2rs>;
///GPDMA channel x transfer register 2
pub mod c10tr2;
/**C11TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c11tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11TR2)

For information about available fields see [`mod@c11tr2`]
module*/
pub type C11TR2 = crate::Reg<c11tr2::C11TR2rs>;
///GPDMA channel x transfer register 2
pub mod c11tr2;
/**C12TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c12tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12TR2)

For information about available fields see [`mod@c12tr2`]
module*/
pub type C12TR2 = crate::Reg<c12tr2::C12TR2rs>;
///GPDMA channel x transfer register 2
pub mod c12tr2;
/**C13TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c13tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13TR2)

For information about available fields see [`mod@c13tr2`]
module*/
pub type C13TR2 = crate::Reg<c13tr2::C13TR2rs>;
///GPDMA channel x transfer register 2
pub mod c13tr2;
/**C14TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c14tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14TR2)

For information about available fields see [`mod@c14tr2`]
module*/
pub type C14TR2 = crate::Reg<c14tr2::C14TR2rs>;
///GPDMA channel x transfer register 2
pub mod c14tr2;
/**C15TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c15tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15TR2)

For information about available fields see [`mod@c15tr2`]
module*/
pub type C15TR2 = crate::Reg<c15tr2::C15TR2rs>;
///GPDMA channel x transfer register 2
pub mod c15tr2;
/**C3BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c3br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3BR1)

For information about available fields see [`mod@c3br1`]
module*/
pub type C3BR1 = crate::Reg<c3br1::C3BR1rs>;
///GPDMA channel x block register 1
pub mod c3br1;
/**C4BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c4br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4BR1)

For information about available fields see [`mod@c4br1`]
module*/
pub type C4BR1 = crate::Reg<c4br1::C4BR1rs>;
///GPDMA channel x block register 1
pub mod c4br1;
/**C5BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c5br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5BR1)

For information about available fields see [`mod@c5br1`]
module*/
pub type C5BR1 = crate::Reg<c5br1::C5BR1rs>;
///GPDMA channel x block register 1
pub mod c5br1;
/**C6BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c6br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6BR1)

For information about available fields see [`mod@c6br1`]
module*/
pub type C6BR1 = crate::Reg<c6br1::C6BR1rs>;
///GPDMA channel x block register 1
pub mod c6br1;
/**C7BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c7br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7BR1)

For information about available fields see [`mod@c7br1`]
module*/
pub type C7BR1 = crate::Reg<c7br1::C7BR1rs>;
///GPDMA channel x block register 1
pub mod c7br1;
/**C8BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c8br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8BR1)

For information about available fields see [`mod@c8br1`]
module*/
pub type C8BR1 = crate::Reg<c8br1::C8BR1rs>;
///GPDMA channel x block register 1
pub mod c8br1;
/**C9BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c9br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9BR1)

For information about available fields see [`mod@c9br1`]
module*/
pub type C9BR1 = crate::Reg<c9br1::C9BR1rs>;
///GPDMA channel x block register 1
pub mod c9br1;
/**C10BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c10br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10BR1)

For information about available fields see [`mod@c10br1`]
module*/
pub type C10BR1 = crate::Reg<c10br1::C10BR1rs>;
///GPDMA channel x block register 1
pub mod c10br1;
/**C11BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c11br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11BR1)

For information about available fields see [`mod@c11br1`]
module*/
pub type C11BR1 = crate::Reg<c11br1::C11BR1rs>;
///GPDMA channel x block register 1
pub mod c11br1;
/**C12BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c12br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12BR1)

For information about available fields see [`mod@c12br1`]
module*/
pub type C12BR1 = crate::Reg<c12br1::C12BR1rs>;
///GPDMA channel x block register 1
pub mod c12br1;
/**C13BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c13br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13BR1)

For information about available fields see [`mod@c13br1`]
module*/
pub type C13BR1 = crate::Reg<c13br1::C13BR1rs>;
///GPDMA channel x block register 1
pub mod c13br1;
/**C14BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c14br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14BR1)

For information about available fields see [`mod@c14br1`]
module*/
pub type C14BR1 = crate::Reg<c14br1::C14BR1rs>;
///GPDMA channel x block register 1
pub mod c14br1;
/**C15BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c15br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15BR1)

For information about available fields see [`mod@c15br1`]
module*/
pub type C15BR1 = crate::Reg<c15br1::C15BR1rs>;
///GPDMA channel x block register 1
pub mod c15br1;
/**C3SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3SAR)

For information about available fields see [`mod@c3sar`]
module*/
pub type C3SAR = crate::Reg<c3sar::C3SARrs>;
///GPDMA channel x source address register
pub mod c3sar;
/**C4SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4SAR)

For information about available fields see [`mod@c4sar`]
module*/
pub type C4SAR = crate::Reg<c4sar::C4SARrs>;
///GPDMA channel x source address register
pub mod c4sar;
/**C5SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5SAR)

For information about available fields see [`mod@c5sar`]
module*/
pub type C5SAR = crate::Reg<c5sar::C5SARrs>;
///GPDMA channel x source address register
pub mod c5sar;
/**C6SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6SAR)

For information about available fields see [`mod@c6sar`]
module*/
pub type C6SAR = crate::Reg<c6sar::C6SARrs>;
///GPDMA channel x source address register
pub mod c6sar;
/**C7SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7SAR)

For information about available fields see [`mod@c7sar`]
module*/
pub type C7SAR = crate::Reg<c7sar::C7SARrs>;
///GPDMA channel x source address register
pub mod c7sar;
/**C8SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c8sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8SAR)

For information about available fields see [`mod@c8sar`]
module*/
pub type C8SAR = crate::Reg<c8sar::C8SARrs>;
///GPDMA channel x source address register
pub mod c8sar;
/**C9SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c9sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9SAR)

For information about available fields see [`mod@c9sar`]
module*/
pub type C9SAR = crate::Reg<c9sar::C9SARrs>;
///GPDMA channel x source address register
pub mod c9sar;
/**C10SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c10sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10SAR)

For information about available fields see [`mod@c10sar`]
module*/
pub type C10SAR = crate::Reg<c10sar::C10SARrs>;
///GPDMA channel x source address register
pub mod c10sar;
/**C11SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c11sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11SAR)

For information about available fields see [`mod@c11sar`]
module*/
pub type C11SAR = crate::Reg<c11sar::C11SARrs>;
///GPDMA channel x source address register
pub mod c11sar;
/**C12SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c12sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12SAR)

For information about available fields see [`mod@c12sar`]
module*/
pub type C12SAR = crate::Reg<c12sar::C12SARrs>;
///GPDMA channel x source address register
pub mod c12sar;
/**C13SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c13sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13SAR)

For information about available fields see [`mod@c13sar`]
module*/
pub type C13SAR = crate::Reg<c13sar::C13SARrs>;
///GPDMA channel x source address register
pub mod c13sar;
/**C14SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c14sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14SAR)

For information about available fields see [`mod@c14sar`]
module*/
pub type C14SAR = crate::Reg<c14sar::C14SARrs>;
///GPDMA channel x source address register
pub mod c14sar;
/**C15SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c15sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15SAR)

For information about available fields see [`mod@c15sar`]
module*/
pub type C15SAR = crate::Reg<c15sar::C15SARrs>;
///GPDMA channel x source address register
pub mod c15sar;
/**C3DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3DAR)

For information about available fields see [`mod@c3dar`]
module*/
pub type C3DAR = crate::Reg<c3dar::C3DARrs>;
///GPDMA channel x destination address register
pub mod c3dar;
/**C4DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4DAR)

For information about available fields see [`mod@c4dar`]
module*/
pub type C4DAR = crate::Reg<c4dar::C4DARrs>;
///GPDMA channel x destination address register
pub mod c4dar;
/**C5DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5DAR)

For information about available fields see [`mod@c5dar`]
module*/
pub type C5DAR = crate::Reg<c5dar::C5DARrs>;
///GPDMA channel x destination address register
pub mod c5dar;
/**C6DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6DAR)

For information about available fields see [`mod@c6dar`]
module*/
pub type C6DAR = crate::Reg<c6dar::C6DARrs>;
///GPDMA channel x destination address register
pub mod c6dar;
/**C7DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7DAR)

For information about available fields see [`mod@c7dar`]
module*/
pub type C7DAR = crate::Reg<c7dar::C7DARrs>;
///GPDMA channel x destination address register
pub mod c7dar;
/**C8DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c8dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8DAR)

For information about available fields see [`mod@c8dar`]
module*/
pub type C8DAR = crate::Reg<c8dar::C8DARrs>;
///GPDMA channel x destination address register
pub mod c8dar;
/**C9DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c9dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9DAR)

For information about available fields see [`mod@c9dar`]
module*/
pub type C9DAR = crate::Reg<c9dar::C9DARrs>;
///GPDMA channel x destination address register
pub mod c9dar;
/**C10DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c10dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10DAR)

For information about available fields see [`mod@c10dar`]
module*/
pub type C10DAR = crate::Reg<c10dar::C10DARrs>;
///GPDMA channel x destination address register
pub mod c10dar;
/**C11DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c11dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11DAR)

For information about available fields see [`mod@c11dar`]
module*/
pub type C11DAR = crate::Reg<c11dar::C11DARrs>;
///GPDMA channel x destination address register
pub mod c11dar;
/**C12DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c12dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12DAR)

For information about available fields see [`mod@c12dar`]
module*/
pub type C12DAR = crate::Reg<c12dar::C12DARrs>;
///GPDMA channel x destination address register
pub mod c12dar;
/**C13DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c13dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13DAR)

For information about available fields see [`mod@c13dar`]
module*/
pub type C13DAR = crate::Reg<c13dar::C13DARrs>;
///GPDMA channel x destination address register
pub mod c13dar;
/**C14DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c14dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14DAR)

For information about available fields see [`mod@c14dar`]
module*/
pub type C14DAR = crate::Reg<c14dar::C14DARrs>;
///GPDMA channel x destination address register
pub mod c14dar;
/**C15DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`c15dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15DAR)

For information about available fields see [`mod@c15dar`]
module*/
pub type C15DAR = crate::Reg<c15dar::C15DARrs>;
///GPDMA channel x destination address register
pub mod c15dar;
/**C3LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c3llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C3LLR)

For information about available fields see [`mod@c3llr`]
module*/
pub type C3LLR = crate::Reg<c3llr::C3LLRrs>;
///GPDMA channel x linked-list address register
pub mod c3llr;
/**C4LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c4llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4LLR)

For information about available fields see [`mod@c4llr`]
module*/
pub type C4LLR = crate::Reg<c4llr::C4LLRrs>;
///GPDMA channel x linked-list address register
pub mod c4llr;
/**C5LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c5llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5LLR)

For information about available fields see [`mod@c5llr`]
module*/
pub type C5LLR = crate::Reg<c5llr::C5LLRrs>;
///GPDMA channel x linked-list address register
pub mod c5llr;
/**C6LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c6llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6LLR)

For information about available fields see [`mod@c6llr`]
module*/
pub type C6LLR = crate::Reg<c6llr::C6LLRrs>;
///GPDMA channel x linked-list address register
pub mod c6llr;
/**C7LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c7llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7LLR)

For information about available fields see [`mod@c7llr`]
module*/
pub type C7LLR = crate::Reg<c7llr::C7LLRrs>;
///GPDMA channel x linked-list address register
pub mod c7llr;
/**C8LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c8llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8LLR)

For information about available fields see [`mod@c8llr`]
module*/
pub type C8LLR = crate::Reg<c8llr::C8LLRrs>;
///GPDMA channel x linked-list address register
pub mod c8llr;
/**C9LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c9llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9LLR)

For information about available fields see [`mod@c9llr`]
module*/
pub type C9LLR = crate::Reg<c9llr::C9LLRrs>;
///GPDMA channel x linked-list address register
pub mod c9llr;
/**C10LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c10llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10LLR)

For information about available fields see [`mod@c10llr`]
module*/
pub type C10LLR = crate::Reg<c10llr::C10LLRrs>;
///GPDMA channel x linked-list address register
pub mod c10llr;
/**C11LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c11llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11LLR)

For information about available fields see [`mod@c11llr`]
module*/
pub type C11LLR = crate::Reg<c11llr::C11LLRrs>;
///GPDMA channel x linked-list address register
pub mod c11llr;
/**C12LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c12llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12LLR)

For information about available fields see [`mod@c12llr`]
module*/
pub type C12LLR = crate::Reg<c12llr::C12LLRrs>;
///GPDMA channel x linked-list address register
pub mod c12llr;
/**C13LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c13llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13LLR)

For information about available fields see [`mod@c13llr`]
module*/
pub type C13LLR = crate::Reg<c13llr::C13LLRrs>;
///GPDMA channel x linked-list address register
pub mod c13llr;
/**C14LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c14llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14LLR)

For information about available fields see [`mod@c14llr`]
module*/
pub type C14LLR = crate::Reg<c14llr::C14LLRrs>;
///GPDMA channel x linked-list address register
pub mod c14llr;
/**C15LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c15llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15LLR)

For information about available fields see [`mod@c15llr`]
module*/
pub type C15LLR = crate::Reg<c15llr::C15LLRrs>;
///GPDMA channel x linked-list address register
pub mod c15llr;
/**C4LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c4lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4LBAR)

For information about available fields see [`mod@c4lbar`]
module*/
pub type C4LBAR = crate::Reg<c4lbar::C4LBARrs>;
///channel x linked-list base address register
pub mod c4lbar;
/**C5LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c5lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5LBAR)

For information about available fields see [`mod@c5lbar`]
module*/
pub type C5LBAR = crate::Reg<c5lbar::C5LBARrs>;
///channel x linked-list base address register
pub mod c5lbar;
/**C6LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c6lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6LBAR)

For information about available fields see [`mod@c6lbar`]
module*/
pub type C6LBAR = crate::Reg<c6lbar::C6LBARrs>;
///channel x linked-list base address register
pub mod c6lbar;
/**C7LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c7lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7LBAR)

For information about available fields see [`mod@c7lbar`]
module*/
pub type C7LBAR = crate::Reg<c7lbar::C7LBARrs>;
///channel x linked-list base address register
pub mod c7lbar;
/**C8LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c8lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8LBAR)

For information about available fields see [`mod@c8lbar`]
module*/
pub type C8LBAR = crate::Reg<c8lbar::C8LBARrs>;
///channel x linked-list base address register
pub mod c8lbar;
/**C9LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c9lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9LBAR)

For information about available fields see [`mod@c9lbar`]
module*/
pub type C9LBAR = crate::Reg<c9lbar::C9LBARrs>;
///channel x linked-list base address register
pub mod c9lbar;
/**C10LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c10lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10LBAR)

For information about available fields see [`mod@c10lbar`]
module*/
pub type C10LBAR = crate::Reg<c10lbar::C10LBARrs>;
///channel x linked-list base address register
pub mod c10lbar;
/**C11LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c11lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11LBAR)

For information about available fields see [`mod@c11lbar`]
module*/
pub type C11LBAR = crate::Reg<c11lbar::C11LBARrs>;
///channel x linked-list base address register
pub mod c11lbar;
/**C12LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c12lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12LBAR)

For information about available fields see [`mod@c12lbar`]
module*/
pub type C12LBAR = crate::Reg<c12lbar::C12LBARrs>;
///channel x linked-list base address register
pub mod c12lbar;
/**C13LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c13lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13LBAR)

For information about available fields see [`mod@c13lbar`]
module*/
pub type C13LBAR = crate::Reg<c13lbar::C13LBARrs>;
///channel x linked-list base address register
pub mod c13lbar;
/**C14LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c14lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14LBAR)

For information about available fields see [`mod@c14lbar`]
module*/
pub type C14LBAR = crate::Reg<c14lbar::C14LBARrs>;
///channel x linked-list base address register
pub mod c14lbar;
/**C15LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c15lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15LBAR)

For information about available fields see [`mod@c15lbar`]
module*/
pub type C15LBAR = crate::Reg<c15lbar::C15LBARrs>;
///channel x linked-list base address register
pub mod c15lbar;
/**C4FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4FCR)

For information about available fields see [`mod@c4fcr`]
module*/
pub type C4FCR = crate::Reg<c4fcr::C4FCRrs>;
///GPDMA channel x flag clear register
pub mod c4fcr;
/**C5FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5FCR)

For information about available fields see [`mod@c5fcr`]
module*/
pub type C5FCR = crate::Reg<c5fcr::C5FCRrs>;
///GPDMA channel x flag clear register
pub mod c5fcr;
/**C6FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6FCR)

For information about available fields see [`mod@c6fcr`]
module*/
pub type C6FCR = crate::Reg<c6fcr::C6FCRrs>;
///GPDMA channel x flag clear register
pub mod c6fcr;
/**C7FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7FCR)

For information about available fields see [`mod@c7fcr`]
module*/
pub type C7FCR = crate::Reg<c7fcr::C7FCRrs>;
///GPDMA channel x flag clear register
pub mod c7fcr;
/**C8FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8FCR)

For information about available fields see [`mod@c8fcr`]
module*/
pub type C8FCR = crate::Reg<c8fcr::C8FCRrs>;
///GPDMA channel x flag clear register
pub mod c8fcr;
/**C9FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9FCR)

For information about available fields see [`mod@c9fcr`]
module*/
pub type C9FCR = crate::Reg<c9fcr::C9FCRrs>;
///GPDMA channel x flag clear register
pub mod c9fcr;
/**C10FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10FCR)

For information about available fields see [`mod@c10fcr`]
module*/
pub type C10FCR = crate::Reg<c10fcr::C10FCRrs>;
///GPDMA channel x flag clear register
pub mod c10fcr;
/**C11FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11FCR)

For information about available fields see [`mod@c11fcr`]
module*/
pub type C11FCR = crate::Reg<c11fcr::C11FCRrs>;
///GPDMA channel x flag clear register
pub mod c11fcr;
/**C12FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12FCR)

For information about available fields see [`mod@c12fcr`]
module*/
pub type C12FCR = crate::Reg<c12fcr::C12FCRrs>;
///GPDMA channel x flag clear register
pub mod c12fcr;
/**C13FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13FCR)

For information about available fields see [`mod@c13fcr`]
module*/
pub type C13FCR = crate::Reg<c13fcr::C13FCRrs>;
///GPDMA channel x flag clear register
pub mod c13fcr;
/**C14FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14FCR)

For information about available fields see [`mod@c14fcr`]
module*/
pub type C14FCR = crate::Reg<c14fcr::C14FCRrs>;
///GPDMA channel x flag clear register
pub mod c14fcr;
/**C15FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15FCR)

For information about available fields see [`mod@c15fcr`]
module*/
pub type C15FCR = crate::Reg<c15fcr::C15FCRrs>;
///GPDMA channel x flag clear register
pub mod c15fcr;
/**C4SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c4sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4SR)

For information about available fields see [`mod@c4sr`]
module*/
pub type C4SR = crate::Reg<c4sr::C4SRrs>;
///channel x status register
pub mod c4sr;
/**C5SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c5sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5SR)

For information about available fields see [`mod@c5sr`]
module*/
pub type C5SR = crate::Reg<c5sr::C5SRrs>;
///channel x status register
pub mod c5sr;
/**C6SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c6sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6SR)

For information about available fields see [`mod@c6sr`]
module*/
pub type C6SR = crate::Reg<c6sr::C6SRrs>;
///channel x status register
pub mod c6sr;
/**C7SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c7sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7SR)

For information about available fields see [`mod@c7sr`]
module*/
pub type C7SR = crate::Reg<c7sr::C7SRrs>;
///channel x status register
pub mod c7sr;
/**C8SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c8sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8SR)

For information about available fields see [`mod@c8sr`]
module*/
pub type C8SR = crate::Reg<c8sr::C8SRrs>;
///channel x status register
pub mod c8sr;
/**C9SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c9sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9SR)

For information about available fields see [`mod@c9sr`]
module*/
pub type C9SR = crate::Reg<c9sr::C9SRrs>;
///channel x status register
pub mod c9sr;
/**C10SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c10sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10SR)

For information about available fields see [`mod@c10sr`]
module*/
pub type C10SR = crate::Reg<c10sr::C10SRrs>;
///channel x status register
pub mod c10sr;
/**C11SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c11sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11SR)

For information about available fields see [`mod@c11sr`]
module*/
pub type C11SR = crate::Reg<c11sr::C11SRrs>;
///channel x status register
pub mod c11sr;
/**C12SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c12sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12SR)

For information about available fields see [`mod@c12sr`]
module*/
pub type C12SR = crate::Reg<c12sr::C12SRrs>;
///channel x status register
pub mod c12sr;
/**C13SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c13sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13SR)

For information about available fields see [`mod@c13sr`]
module*/
pub type C13SR = crate::Reg<c13sr::C13SRrs>;
///channel x status register
pub mod c13sr;
/**C14SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c14sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14SR)

For information about available fields see [`mod@c14sr`]
module*/
pub type C14SR = crate::Reg<c14sr::C14SRrs>;
///channel x status register
pub mod c14sr;
/**C15SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`c15sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15SR)

For information about available fields see [`mod@c15sr`]
module*/
pub type C15SR = crate::Reg<c15sr::C15SRrs>;
///channel x status register
pub mod c15sr;
/**C4CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C4CR)

For information about available fields see [`mod@c4cr`]
module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///channel x control register
pub mod c4cr;
/**C5CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C5CR)

For information about available fields see [`mod@c5cr`]
module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///channel x control register
pub mod c5cr;
/**C6CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C6CR)

For information about available fields see [`mod@c6cr`]
module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///channel x control register
pub mod c6cr;
/**C7CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C7CR)

For information about available fields see [`mod@c7cr`]
module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///channel x control register
pub mod c7cr;
/**C8CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C8CR)

For information about available fields see [`mod@c8cr`]
module*/
pub type C8CR = crate::Reg<c8cr::C8CRrs>;
///channel x control register
pub mod c8cr;
/**C9CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C9CR)

For information about available fields see [`mod@c9cr`]
module*/
pub type C9CR = crate::Reg<c9cr::C9CRrs>;
///channel x control register
pub mod c9cr;
/**C10CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C10CR)

For information about available fields see [`mod@c10cr`]
module*/
pub type C10CR = crate::Reg<c10cr::C10CRrs>;
///channel x control register
pub mod c10cr;
/**C11CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C11CR)

For information about available fields see [`mod@c11cr`]
module*/
pub type C11CR = crate::Reg<c11cr::C11CRrs>;
///channel x control register
pub mod c11cr;
/**C12CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12CR)

For information about available fields see [`mod@c12cr`]
module*/
pub type C12CR = crate::Reg<c12cr::C12CRrs>;
///channel x control register
pub mod c12cr;
/**C13CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13CR)

For information about available fields see [`mod@c13cr`]
module*/
pub type C13CR = crate::Reg<c13cr::C13CRrs>;
///channel x control register
pub mod c13cr;
/**C14CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14CR)

For information about available fields see [`mod@c14cr`]
module*/
pub type C14CR = crate::Reg<c14cr::C14CRrs>;
///channel x control register
pub mod c14cr;
/**C15CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15CR)

For information about available fields see [`mod@c15cr`]
module*/
pub type C15CR = crate::Reg<c15cr::C15CRrs>;
///channel x control register
pub mod c15cr;
/**C12TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c12tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12TR3)

For information about available fields see [`mod@c12tr3`]
module*/
pub type C12TR3 = crate::Reg<c12tr3::C12TR3rs>;
///GPDMA channel x transfer register 3
pub mod c12tr3;
/**C13TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c13tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13TR3)

For information about available fields see [`mod@c13tr3`]
module*/
pub type C13TR3 = crate::Reg<c13tr3::C13TR3rs>;
///GPDMA channel x transfer register 3
pub mod c13tr3;
/**C14TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c14tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14TR3)

For information about available fields see [`mod@c14tr3`]
module*/
pub type C14TR3 = crate::Reg<c14tr3::C14TR3rs>;
///GPDMA channel x transfer register 3
pub mod c14tr3;
/**C15TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c15tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15TR3)

For information about available fields see [`mod@c15tr3`]
module*/
pub type C15TR3 = crate::Reg<c15tr3::C15TR3rs>;
///GPDMA channel x transfer register 3
pub mod c15tr3;
/**C12BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`c12br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C12BR2)

For information about available fields see [`mod@c12br2`]
module*/
pub type C12BR2 = crate::Reg<c12br2::C12BR2rs>;
///GPDMA channel x block register 2
pub mod c12br2;
/**C13BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`c13br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C13BR2)

For information about available fields see [`mod@c13br2`]
module*/
pub type C13BR2 = crate::Reg<c13br2::C13BR2rs>;
///GPDMA channel x block register 2
pub mod c13br2;
/**C14BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`c14br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C14BR2)

For information about available fields see [`mod@c14br2`]
module*/
pub type C14BR2 = crate::Reg<c14br2::C14BR2rs>;
///GPDMA channel x block register 2
pub mod c14br2;
/**C15BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`c15br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPDMA1:C15BR2)

For information about available fields see [`mod@c15br2`]
module*/
pub type C15BR2 = crate::Reg<c15br2::C15BR2rs>;
///GPDMA channel x block register 2
pub mod c15br2;
