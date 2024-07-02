#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gpdma_seccfgr: GPDMA_SECCFGR,
    gpdma_privcfgr: GPDMA_PRIVCFGR,
    _reserved2: [u8; 0x04],
    misr: MISR,
    smisr: SMISR,
    _reserved4: [u8; 0x3c],
    gpdma_c0lbar: GPDMA_C0LBAR,
    _reserved5: [u8; 0x08],
    gpdma_c0fcr: GPDMA_C0FCR,
    gpdma_c0sr: GPDMA_C0SR,
    gpdma_c0cr: GPDMA_C0CR,
    _reserved8: [u8; 0x28],
    gpdma_c0tr1: GPDMA_C0TR1,
    gpdma_c0tr2: GPDMA_C0TR2,
    gpdma_c0br1: GPDMA_C0BR1,
    gpdma_c0sar: GPDMA_C0SAR,
    gpdma_c0dar: GPDMA_C0DAR,
    _reserved13: [u8; 0x28],
    gpdma_c0llr: GPDMA_C0LLR,
    gpdma_c1lbar: GPDMA_C1LBAR,
    _reserved15: [u8; 0x08],
    gpdma_c1fcr: GPDMA_C1FCR,
    gpdma_c1sr: GPDMA_C1SR,
    gpdma_c1cr: GPDMA_C1CR,
    _reserved18: [u8; 0x28],
    gpdma_c1tr1: GPDMA_C1TR1,
    gpdma_c1tr2: GPDMA_C1TR2,
    gpdma_c1br1: GPDMA_C1BR1,
    gpdma_c1sar: GPDMA_C1SAR,
    gpdma_c1dar: GPDMA_C1DAR,
    _reserved23: [u8; 0x28],
    gpdma_c1llr: GPDMA_C1LLR,
    gpdma_c2lbar: GPDMA_C2LBAR,
    _reserved25: [u8; 0x08],
    gpdma_c2fcr: GPDMA_C2FCR,
    gpdma_c2sr: GPDMA_C2SR,
    gpdma_c2cr: GPDMA_C2CR,
    _reserved28: [u8; 0x28],
    gpdma_c2tr1: GPDMA_C2TR1,
    gpdma_c2tr2: GPDMA_C2TR2,
    gpdma_c2br1: GPDMA_C2BR1,
    gpdma_c2sar: GPDMA_C2SAR,
    gpdma_c2dar: GPDMA_C2DAR,
    _reserved33: [u8; 0x28],
    gpdma_c2llr: GPDMA_C2LLR,
    gpdma_c3lbar: GPDMA_C3LBAR,
    _reserved35: [u8; 0x08],
    gpdma_c3fcr: GPDMA_C3FCR,
    gpdma_c3sr: GPDMA_C3SR,
    gpdma_c3cr: GPDMA_C3CR,
    _reserved38: [u8; 0x28],
    gpdma_c3tr1: GPDMA_C3TR1,
    gpdma_c3tr2: GPDMA_C3TR2,
    gpdma_c3br1: GPDMA_C3BR1,
    gpdma_c3sar: GPDMA_C3SAR,
    gpdma_c3dar: GPDMA_C3DAR,
    _reserved43: [u8; 0x28],
    gpdma_c3llr: GPDMA_C3LLR,
    gpdma_c4lbar: GPDMA_C4LBAR,
    _reserved45: [u8; 0x08],
    gpdma_c4fcr: GPDMA_C4FCR,
    gpdma_c4sr: GPDMA_C4SR,
    gpdma_c4cr: GPDMA_C4CR,
    _reserved48: [u8; 0x2c],
    gpdma_c4tr2: GPDMA_C4TR2,
    gpdma_c4br1: GPDMA_C4BR1,
    gpdma_c4sar: GPDMA_C4SAR,
    gpdma_c4dar: GPDMA_C4DAR,
    _reserved52: [u8; 0x28],
    gpdma_c4llr: GPDMA_C4LLR,
    gpdma_c5lbar: GPDMA_C5LBAR,
    _reserved54: [u8; 0x08],
    gpdma_c5fcr: GPDMA_C5FCR,
    gpdma_c5sr: GPDMA_C5SR,
    gpdma_c5cr: GPDMA_C5CR,
    _reserved57: [u8; 0x2c],
    gpdma_c5tr2: GPDMA_C5TR2,
    gpdma_c5br1: GPDMA_C5BR1,
    gpdma_c5sar: GPDMA_C5SAR,
    gpdma_c5dar: GPDMA_C5DAR,
    _reserved61: [u8; 0x28],
    gpdma_c5llr: GPDMA_C5LLR,
    gpdma_c6lbar: GPDMA_C6LBAR,
    _reserved63: [u8; 0x08],
    gpdma_c6fcr: GPDMA_C6FCR,
    gpdma_c6sr: GPDMA_C6SR,
    gpdma_c6cr: GPDMA_C6CR,
    _reserved66: [u8; 0x2c],
    gpdma_c6tr2: GPDMA_C6TR2,
    gpdma_c6br1: GPDMA_C6BR1,
    gpdma_c6sar: GPDMA_C6SAR,
    gpdma_c6dar: GPDMA_C6DAR,
    _reserved70: [u8; 0x28],
    gpdma_c6llr: GPDMA_C6LLR,
    gpdma_c7lbar: GPDMA_C7LBAR,
    _reserved72: [u8; 0x08],
    gpdma_c7fcr: GPDMA_C7FCR,
    gpdma_c7sr: GPDMA_C7SR,
    gpdma_c7cr: GPDMA_C7CR,
    _reserved75: [u8; 0x2c],
    gpdma_c7tr2: GPDMA_C7TR2,
    gpdma_c7br1: GPDMA_C7BR1,
    gpdma_c7sar: GPDMA_C7SAR,
    gpdma_c7dar: GPDMA_C7DAR,
    _reserved79: [u8; 0x28],
    gpdma_c7llr: GPDMA_C7LLR,
    gpdma_c8lbar: GPDMA_C8LBAR,
    _reserved81: [u8; 0x08],
    gpdma_c8fcr: GPDMA_C8FCR,
    gpdma_c8sr: GPDMA_C8SR,
    gpdma_c8cr: GPDMA_C8CR,
    _reserved84: [u8; 0x2c],
    gpdma_c8tr2: GPDMA_C8TR2,
    gpdma_c8br1: GPDMA_C8BR1,
    gpdma_c8sar: GPDMA_C8SAR,
    gpdma_c8dar: GPDMA_C8DAR,
    _reserved88: [u8; 0x28],
    gpdma_c8llr: GPDMA_C8LLR,
    gpdma_c9lbar: GPDMA_C9LBAR,
    _reserved90: [u8; 0x08],
    gpdma_c9fcr: GPDMA_C9FCR,
    gpdma_c9sr: GPDMA_C9SR,
    gpdma_c9cr: GPDMA_C9CR,
    _reserved93: [u8; 0x2c],
    gpdma_c9tr2: GPDMA_C9TR2,
    gpdma_c9br1: GPDMA_C9BR1,
    gpdma_c9sar: GPDMA_C9SAR,
    gpdma_c9dar: GPDMA_C9DAR,
    _reserved97: [u8; 0x28],
    gpdma_c9llr: GPDMA_C9LLR,
    gpdma_c10lbar: GPDMA_C10LBAR,
    _reserved99: [u8; 0x08],
    gpdma_c10fcr: GPDMA_C10FCR,
    gpdma_c10sr: GPDMA_C10SR,
    gpdma_c10cr: GPDMA_C10CR,
    _reserved102: [u8; 0x2c],
    gpdma_c10tr2: GPDMA_C10TR2,
    gpdma_c10br1: GPDMA_C10BR1,
    gpdma_c10sar: GPDMA_C10SAR,
    gpdma_c10dar: GPDMA_C10DAR,
    _reserved106: [u8; 0x28],
    gpdma_c10llr: GPDMA_C10LLR,
    gpdma_c11lbar: GPDMA_C11LBAR,
    _reserved108: [u8; 0x08],
    gpdma_c11fcr: GPDMA_C11FCR,
    gpdma_c11sr: GPDMA_C11SR,
    gpdma_c11cr: GPDMA_C11CR,
    _reserved111: [u8; 0x2c],
    gpdma_c11tr2: GPDMA_C11TR2,
    gpdma_c11br1: GPDMA_C11BR1,
    gpdma_c11sar: GPDMA_C11SAR,
    gpdma_c11dar: GPDMA_C11DAR,
    _reserved115: [u8; 0x28],
    gpdma_c11llr: GPDMA_C11LLR,
    gpdma_c12lbar: GPDMA_C12LBAR,
    _reserved117: [u8; 0x08],
    gpdma_c12fcr: GPDMA_C12FCR,
    gpdma_c12sr: GPDMA_C12SR,
    gpdma_c12cr: GPDMA_C12CR,
    _reserved120: [u8; 0x2c],
    gpdma_c12tr2: GPDMA_C12TR2,
    gpdma_c12br1: GPDMA_C12BR1,
    gpdma_c12sar: GPDMA_C12SAR,
    gpdma_c12dar: GPDMA_C12DAR,
    gpdma_c12tr3: GPDMA_C12TR3,
    gpdma_c12br2: GPDMA_C12BR2,
    _reserved126: [u8; 0x20],
    gpdma_c12llr: GPDMA_C12LLR,
    gpdma_c13lbar: GPDMA_C13LBAR,
    _reserved128: [u8; 0x08],
    gpdma_c13fcr: GPDMA_C13FCR,
    gpdma_c13sr: GPDMA_C13SR,
    gpdma_c13cr: GPDMA_C13CR,
    _reserved131: [u8; 0x2c],
    gpdma_c13tr2: GPDMA_C13TR2,
    gpdma_c13br1: GPDMA_C13BR1,
    gpdma_c13sar: GPDMA_C13SAR,
    gpdma_c13dar: GPDMA_C13DAR,
    gpdma_c13tr3: GPDMA_C13TR3,
    gpdma_c13br2: GPDMA_C13BR2,
    _reserved137: [u8; 0x20],
    gpdma_c13llr: GPDMA_C13LLR,
    gpdma_c14lbar: GPDMA_C14LBAR,
    _reserved139: [u8; 0x08],
    gpdma_c14fcr: GPDMA_C14FCR,
    gpdma_c14sr: GPDMA_C14SR,
    gpdma_c14cr: GPDMA_C14CR,
    _reserved142: [u8; 0x2c],
    gpdma_c14tr2: GPDMA_C14TR2,
    gpdma_c14br1: GPDMA_C14BR1,
    gpdma_c14sar: GPDMA_C14SAR,
    gpdma_c14dar: GPDMA_C14DAR,
    gpdma_c14tr3: GPDMA_C14TR3,
    gpdma_c14br2: GPDMA_C14BR2,
    _reserved148: [u8; 0x20],
    gpdma_c14llr: GPDMA_C14LLR,
    gpdma_c15lbar: GPDMA_C15LBAR,
    _reserved150: [u8; 0x08],
    gpdma_c15fcr: GPDMA_C15FCR,
    gpdma_c15sr: GPDMA_C15SR,
    gpdma_c15cr: GPDMA_C15CR,
    _reserved153: [u8; 0x2c],
    gpdma_c15tr2: GPDMA_C15TR2,
    gpdma_c15br1: GPDMA_C15BR1,
    gpdma_c15sar: GPDMA_C15SAR,
    gpdma_c15dar: GPDMA_C15DAR,
    gpdma_c15tr3: GPDMA_C15TR3,
    gpdma_c15br2: GPDMA_C15BR2,
    _reserved159: [u8; 0x20],
    gpdma_c15llr: GPDMA_C15LLR,
}
impl RegisterBlock {
    ///0x00 - GPDMA secure configuration register
    #[inline(always)]
    pub const fn gpdma_seccfgr(&self) -> &GPDMA_SECCFGR {
        &self.gpdma_seccfgr
    }
    ///0x04 - GPDMA privileged configuration register
    #[inline(always)]
    pub const fn gpdma_privcfgr(&self) -> &GPDMA_PRIVCFGR {
        &self.gpdma_privcfgr
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
    pub const fn gpdma_c0lbar(&self) -> &GPDMA_C0LBAR {
        &self.gpdma_c0lbar
    }
    ///0x5c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c0fcr(&self) -> &GPDMA_C0FCR {
        &self.gpdma_c0fcr
    }
    ///0x60 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c0sr(&self) -> &GPDMA_C0SR {
        &self.gpdma_c0sr
    }
    ///0x64 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c0cr(&self) -> &GPDMA_C0CR {
        &self.gpdma_c0cr
    }
    ///0x90 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn gpdma_c0tr1(&self) -> &GPDMA_C0TR1 {
        &self.gpdma_c0tr1
    }
    ///0x94 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c0tr2(&self) -> &GPDMA_C0TR2 {
        &self.gpdma_c0tr2
    }
    ///0x98 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c0br1(&self) -> &GPDMA_C0BR1 {
        &self.gpdma_c0br1
    }
    ///0x9c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c0sar(&self) -> &GPDMA_C0SAR {
        &self.gpdma_c0sar
    }
    ///0xa0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c0dar(&self) -> &GPDMA_C0DAR {
        &self.gpdma_c0dar
    }
    ///0xcc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c0llr(&self) -> &GPDMA_C0LLR {
        &self.gpdma_c0llr
    }
    ///0xd0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c1lbar(&self) -> &GPDMA_C1LBAR {
        &self.gpdma_c1lbar
    }
    ///0xdc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c1fcr(&self) -> &GPDMA_C1FCR {
        &self.gpdma_c1fcr
    }
    ///0xe0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c1sr(&self) -> &GPDMA_C1SR {
        &self.gpdma_c1sr
    }
    ///0xe4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c1cr(&self) -> &GPDMA_C1CR {
        &self.gpdma_c1cr
    }
    ///0x110 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn gpdma_c1tr1(&self) -> &GPDMA_C1TR1 {
        &self.gpdma_c1tr1
    }
    ///0x114 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c1tr2(&self) -> &GPDMA_C1TR2 {
        &self.gpdma_c1tr2
    }
    ///0x118 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c1br1(&self) -> &GPDMA_C1BR1 {
        &self.gpdma_c1br1
    }
    ///0x11c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c1sar(&self) -> &GPDMA_C1SAR {
        &self.gpdma_c1sar
    }
    ///0x120 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c1dar(&self) -> &GPDMA_C1DAR {
        &self.gpdma_c1dar
    }
    ///0x14c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c1llr(&self) -> &GPDMA_C1LLR {
        &self.gpdma_c1llr
    }
    ///0x150 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c2lbar(&self) -> &GPDMA_C2LBAR {
        &self.gpdma_c2lbar
    }
    ///0x15c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c2fcr(&self) -> &GPDMA_C2FCR {
        &self.gpdma_c2fcr
    }
    ///0x160 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c2sr(&self) -> &GPDMA_C2SR {
        &self.gpdma_c2sr
    }
    ///0x164 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c2cr(&self) -> &GPDMA_C2CR {
        &self.gpdma_c2cr
    }
    ///0x190 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn gpdma_c2tr1(&self) -> &GPDMA_C2TR1 {
        &self.gpdma_c2tr1
    }
    ///0x194 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c2tr2(&self) -> &GPDMA_C2TR2 {
        &self.gpdma_c2tr2
    }
    ///0x198 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c2br1(&self) -> &GPDMA_C2BR1 {
        &self.gpdma_c2br1
    }
    ///0x19c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c2sar(&self) -> &GPDMA_C2SAR {
        &self.gpdma_c2sar
    }
    ///0x1a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c2dar(&self) -> &GPDMA_C2DAR {
        &self.gpdma_c2dar
    }
    ///0x1cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c2llr(&self) -> &GPDMA_C2LLR {
        &self.gpdma_c2llr
    }
    ///0x1d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c3lbar(&self) -> &GPDMA_C3LBAR {
        &self.gpdma_c3lbar
    }
    ///0x1dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c3fcr(&self) -> &GPDMA_C3FCR {
        &self.gpdma_c3fcr
    }
    ///0x1e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c3sr(&self) -> &GPDMA_C3SR {
        &self.gpdma_c3sr
    }
    ///0x1e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c3cr(&self) -> &GPDMA_C3CR {
        &self.gpdma_c3cr
    }
    ///0x210 - GPDMA channel x transfer register 1
    #[inline(always)]
    pub const fn gpdma_c3tr1(&self) -> &GPDMA_C3TR1 {
        &self.gpdma_c3tr1
    }
    ///0x214 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c3tr2(&self) -> &GPDMA_C3TR2 {
        &self.gpdma_c3tr2
    }
    ///0x218 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c3br1(&self) -> &GPDMA_C3BR1 {
        &self.gpdma_c3br1
    }
    ///0x21c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c3sar(&self) -> &GPDMA_C3SAR {
        &self.gpdma_c3sar
    }
    ///0x220 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c3dar(&self) -> &GPDMA_C3DAR {
        &self.gpdma_c3dar
    }
    ///0x24c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c3llr(&self) -> &GPDMA_C3LLR {
        &self.gpdma_c3llr
    }
    ///0x250 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c4lbar(&self) -> &GPDMA_C4LBAR {
        &self.gpdma_c4lbar
    }
    ///0x25c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c4fcr(&self) -> &GPDMA_C4FCR {
        &self.gpdma_c4fcr
    }
    ///0x260 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c4sr(&self) -> &GPDMA_C4SR {
        &self.gpdma_c4sr
    }
    ///0x264 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c4cr(&self) -> &GPDMA_C4CR {
        &self.gpdma_c4cr
    }
    ///0x294 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c4tr2(&self) -> &GPDMA_C4TR2 {
        &self.gpdma_c4tr2
    }
    ///0x298 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c4br1(&self) -> &GPDMA_C4BR1 {
        &self.gpdma_c4br1
    }
    ///0x29c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c4sar(&self) -> &GPDMA_C4SAR {
        &self.gpdma_c4sar
    }
    ///0x2a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c4dar(&self) -> &GPDMA_C4DAR {
        &self.gpdma_c4dar
    }
    ///0x2cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c4llr(&self) -> &GPDMA_C4LLR {
        &self.gpdma_c4llr
    }
    ///0x2d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c5lbar(&self) -> &GPDMA_C5LBAR {
        &self.gpdma_c5lbar
    }
    ///0x2dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c5fcr(&self) -> &GPDMA_C5FCR {
        &self.gpdma_c5fcr
    }
    ///0x2e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c5sr(&self) -> &GPDMA_C5SR {
        &self.gpdma_c5sr
    }
    ///0x2e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c5cr(&self) -> &GPDMA_C5CR {
        &self.gpdma_c5cr
    }
    ///0x314 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c5tr2(&self) -> &GPDMA_C5TR2 {
        &self.gpdma_c5tr2
    }
    ///0x318 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c5br1(&self) -> &GPDMA_C5BR1 {
        &self.gpdma_c5br1
    }
    ///0x31c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c5sar(&self) -> &GPDMA_C5SAR {
        &self.gpdma_c5sar
    }
    ///0x320 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c5dar(&self) -> &GPDMA_C5DAR {
        &self.gpdma_c5dar
    }
    ///0x34c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c5llr(&self) -> &GPDMA_C5LLR {
        &self.gpdma_c5llr
    }
    ///0x350 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c6lbar(&self) -> &GPDMA_C6LBAR {
        &self.gpdma_c6lbar
    }
    ///0x35c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c6fcr(&self) -> &GPDMA_C6FCR {
        &self.gpdma_c6fcr
    }
    ///0x360 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c6sr(&self) -> &GPDMA_C6SR {
        &self.gpdma_c6sr
    }
    ///0x364 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c6cr(&self) -> &GPDMA_C6CR {
        &self.gpdma_c6cr
    }
    ///0x394 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c6tr2(&self) -> &GPDMA_C6TR2 {
        &self.gpdma_c6tr2
    }
    ///0x398 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c6br1(&self) -> &GPDMA_C6BR1 {
        &self.gpdma_c6br1
    }
    ///0x39c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c6sar(&self) -> &GPDMA_C6SAR {
        &self.gpdma_c6sar
    }
    ///0x3a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c6dar(&self) -> &GPDMA_C6DAR {
        &self.gpdma_c6dar
    }
    ///0x3cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c6llr(&self) -> &GPDMA_C6LLR {
        &self.gpdma_c6llr
    }
    ///0x3d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c7lbar(&self) -> &GPDMA_C7LBAR {
        &self.gpdma_c7lbar
    }
    ///0x3dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c7fcr(&self) -> &GPDMA_C7FCR {
        &self.gpdma_c7fcr
    }
    ///0x3e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c7sr(&self) -> &GPDMA_C7SR {
        &self.gpdma_c7sr
    }
    ///0x3e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c7cr(&self) -> &GPDMA_C7CR {
        &self.gpdma_c7cr
    }
    ///0x414 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c7tr2(&self) -> &GPDMA_C7TR2 {
        &self.gpdma_c7tr2
    }
    ///0x418 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c7br1(&self) -> &GPDMA_C7BR1 {
        &self.gpdma_c7br1
    }
    ///0x41c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c7sar(&self) -> &GPDMA_C7SAR {
        &self.gpdma_c7sar
    }
    ///0x420 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c7dar(&self) -> &GPDMA_C7DAR {
        &self.gpdma_c7dar
    }
    ///0x44c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c7llr(&self) -> &GPDMA_C7LLR {
        &self.gpdma_c7llr
    }
    ///0x450 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c8lbar(&self) -> &GPDMA_C8LBAR {
        &self.gpdma_c8lbar
    }
    ///0x45c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c8fcr(&self) -> &GPDMA_C8FCR {
        &self.gpdma_c8fcr
    }
    ///0x460 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c8sr(&self) -> &GPDMA_C8SR {
        &self.gpdma_c8sr
    }
    ///0x464 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c8cr(&self) -> &GPDMA_C8CR {
        &self.gpdma_c8cr
    }
    ///0x494 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c8tr2(&self) -> &GPDMA_C8TR2 {
        &self.gpdma_c8tr2
    }
    ///0x498 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c8br1(&self) -> &GPDMA_C8BR1 {
        &self.gpdma_c8br1
    }
    ///0x49c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c8sar(&self) -> &GPDMA_C8SAR {
        &self.gpdma_c8sar
    }
    ///0x4a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c8dar(&self) -> &GPDMA_C8DAR {
        &self.gpdma_c8dar
    }
    ///0x4cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c8llr(&self) -> &GPDMA_C8LLR {
        &self.gpdma_c8llr
    }
    ///0x4d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c9lbar(&self) -> &GPDMA_C9LBAR {
        &self.gpdma_c9lbar
    }
    ///0x4dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c9fcr(&self) -> &GPDMA_C9FCR {
        &self.gpdma_c9fcr
    }
    ///0x4e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c9sr(&self) -> &GPDMA_C9SR {
        &self.gpdma_c9sr
    }
    ///0x4e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c9cr(&self) -> &GPDMA_C9CR {
        &self.gpdma_c9cr
    }
    ///0x514 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c9tr2(&self) -> &GPDMA_C9TR2 {
        &self.gpdma_c9tr2
    }
    ///0x518 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c9br1(&self) -> &GPDMA_C9BR1 {
        &self.gpdma_c9br1
    }
    ///0x51c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c9sar(&self) -> &GPDMA_C9SAR {
        &self.gpdma_c9sar
    }
    ///0x520 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c9dar(&self) -> &GPDMA_C9DAR {
        &self.gpdma_c9dar
    }
    ///0x54c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c9llr(&self) -> &GPDMA_C9LLR {
        &self.gpdma_c9llr
    }
    ///0x550 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c10lbar(&self) -> &GPDMA_C10LBAR {
        &self.gpdma_c10lbar
    }
    ///0x55c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c10fcr(&self) -> &GPDMA_C10FCR {
        &self.gpdma_c10fcr
    }
    ///0x560 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c10sr(&self) -> &GPDMA_C10SR {
        &self.gpdma_c10sr
    }
    ///0x564 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c10cr(&self) -> &GPDMA_C10CR {
        &self.gpdma_c10cr
    }
    ///0x594 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c10tr2(&self) -> &GPDMA_C10TR2 {
        &self.gpdma_c10tr2
    }
    ///0x598 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c10br1(&self) -> &GPDMA_C10BR1 {
        &self.gpdma_c10br1
    }
    ///0x59c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c10sar(&self) -> &GPDMA_C10SAR {
        &self.gpdma_c10sar
    }
    ///0x5a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c10dar(&self) -> &GPDMA_C10DAR {
        &self.gpdma_c10dar
    }
    ///0x5cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c10llr(&self) -> &GPDMA_C10LLR {
        &self.gpdma_c10llr
    }
    ///0x5d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c11lbar(&self) -> &GPDMA_C11LBAR {
        &self.gpdma_c11lbar
    }
    ///0x5dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c11fcr(&self) -> &GPDMA_C11FCR {
        &self.gpdma_c11fcr
    }
    ///0x5e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c11sr(&self) -> &GPDMA_C11SR {
        &self.gpdma_c11sr
    }
    ///0x5e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c11cr(&self) -> &GPDMA_C11CR {
        &self.gpdma_c11cr
    }
    ///0x614 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c11tr2(&self) -> &GPDMA_C11TR2 {
        &self.gpdma_c11tr2
    }
    ///0x618 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c11br1(&self) -> &GPDMA_C11BR1 {
        &self.gpdma_c11br1
    }
    ///0x61c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c11sar(&self) -> &GPDMA_C11SAR {
        &self.gpdma_c11sar
    }
    ///0x620 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c11dar(&self) -> &GPDMA_C11DAR {
        &self.gpdma_c11dar
    }
    ///0x64c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c11llr(&self) -> &GPDMA_C11LLR {
        &self.gpdma_c11llr
    }
    ///0x650 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c12lbar(&self) -> &GPDMA_C12LBAR {
        &self.gpdma_c12lbar
    }
    ///0x65c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c12fcr(&self) -> &GPDMA_C12FCR {
        &self.gpdma_c12fcr
    }
    ///0x660 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c12sr(&self) -> &GPDMA_C12SR {
        &self.gpdma_c12sr
    }
    ///0x664 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c12cr(&self) -> &GPDMA_C12CR {
        &self.gpdma_c12cr
    }
    ///0x694 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c12tr2(&self) -> &GPDMA_C12TR2 {
        &self.gpdma_c12tr2
    }
    ///0x698 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c12br1(&self) -> &GPDMA_C12BR1 {
        &self.gpdma_c12br1
    }
    ///0x69c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c12sar(&self) -> &GPDMA_C12SAR {
        &self.gpdma_c12sar
    }
    ///0x6a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c12dar(&self) -> &GPDMA_C12DAR {
        &self.gpdma_c12dar
    }
    ///0x6a4 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn gpdma_c12tr3(&self) -> &GPDMA_C12TR3 {
        &self.gpdma_c12tr3
    }
    ///0x6a8 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn gpdma_c12br2(&self) -> &GPDMA_C12BR2 {
        &self.gpdma_c12br2
    }
    ///0x6cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c12llr(&self) -> &GPDMA_C12LLR {
        &self.gpdma_c12llr
    }
    ///0x6d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c13lbar(&self) -> &GPDMA_C13LBAR {
        &self.gpdma_c13lbar
    }
    ///0x6dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c13fcr(&self) -> &GPDMA_C13FCR {
        &self.gpdma_c13fcr
    }
    ///0x6e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c13sr(&self) -> &GPDMA_C13SR {
        &self.gpdma_c13sr
    }
    ///0x6e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c13cr(&self) -> &GPDMA_C13CR {
        &self.gpdma_c13cr
    }
    ///0x714 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c13tr2(&self) -> &GPDMA_C13TR2 {
        &self.gpdma_c13tr2
    }
    ///0x718 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c13br1(&self) -> &GPDMA_C13BR1 {
        &self.gpdma_c13br1
    }
    ///0x71c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c13sar(&self) -> &GPDMA_C13SAR {
        &self.gpdma_c13sar
    }
    ///0x720 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c13dar(&self) -> &GPDMA_C13DAR {
        &self.gpdma_c13dar
    }
    ///0x724 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn gpdma_c13tr3(&self) -> &GPDMA_C13TR3 {
        &self.gpdma_c13tr3
    }
    ///0x728 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn gpdma_c13br2(&self) -> &GPDMA_C13BR2 {
        &self.gpdma_c13br2
    }
    ///0x74c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c13llr(&self) -> &GPDMA_C13LLR {
        &self.gpdma_c13llr
    }
    ///0x750 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c14lbar(&self) -> &GPDMA_C14LBAR {
        &self.gpdma_c14lbar
    }
    ///0x75c - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c14fcr(&self) -> &GPDMA_C14FCR {
        &self.gpdma_c14fcr
    }
    ///0x760 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c14sr(&self) -> &GPDMA_C14SR {
        &self.gpdma_c14sr
    }
    ///0x764 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c14cr(&self) -> &GPDMA_C14CR {
        &self.gpdma_c14cr
    }
    ///0x794 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c14tr2(&self) -> &GPDMA_C14TR2 {
        &self.gpdma_c14tr2
    }
    ///0x798 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c14br1(&self) -> &GPDMA_C14BR1 {
        &self.gpdma_c14br1
    }
    ///0x79c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c14sar(&self) -> &GPDMA_C14SAR {
        &self.gpdma_c14sar
    }
    ///0x7a0 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c14dar(&self) -> &GPDMA_C14DAR {
        &self.gpdma_c14dar
    }
    ///0x7a4 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn gpdma_c14tr3(&self) -> &GPDMA_C14TR3 {
        &self.gpdma_c14tr3
    }
    ///0x7a8 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn gpdma_c14br2(&self) -> &GPDMA_C14BR2 {
        &self.gpdma_c14br2
    }
    ///0x7cc - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c14llr(&self) -> &GPDMA_C14LLR {
        &self.gpdma_c14llr
    }
    ///0x7d0 - channel x linked-list base address register
    #[inline(always)]
    pub const fn gpdma_c15lbar(&self) -> &GPDMA_C15LBAR {
        &self.gpdma_c15lbar
    }
    ///0x7dc - GPDMA channel x flag clear register
    #[inline(always)]
    pub const fn gpdma_c15fcr(&self) -> &GPDMA_C15FCR {
        &self.gpdma_c15fcr
    }
    ///0x7e0 - channel x status register
    #[inline(always)]
    pub const fn gpdma_c15sr(&self) -> &GPDMA_C15SR {
        &self.gpdma_c15sr
    }
    ///0x7e4 - channel x control register
    #[inline(always)]
    pub const fn gpdma_c15cr(&self) -> &GPDMA_C15CR {
        &self.gpdma_c15cr
    }
    ///0x814 - GPDMA channel x transfer register 2
    #[inline(always)]
    pub const fn gpdma_c15tr2(&self) -> &GPDMA_C15TR2 {
        &self.gpdma_c15tr2
    }
    ///0x818 - GPDMA channel x block register 1
    #[inline(always)]
    pub const fn gpdma_c15br1(&self) -> &GPDMA_C15BR1 {
        &self.gpdma_c15br1
    }
    ///0x81c - GPDMA channel x source address register
    #[inline(always)]
    pub const fn gpdma_c15sar(&self) -> &GPDMA_C15SAR {
        &self.gpdma_c15sar
    }
    ///0x820 - GPDMA channel x destination address register
    #[inline(always)]
    pub const fn gpdma_c15dar(&self) -> &GPDMA_C15DAR {
        &self.gpdma_c15dar
    }
    ///0x824 - GPDMA channel x transfer register 3
    #[inline(always)]
    pub const fn gpdma_c15tr3(&self) -> &GPDMA_C15TR3 {
        &self.gpdma_c15tr3
    }
    ///0x828 - GPDMA channel x block register 2
    #[inline(always)]
    pub const fn gpdma_c15br2(&self) -> &GPDMA_C15BR2 {
        &self.gpdma_c15br2
    }
    ///0x84c - GPDMA channel x linked-list address register
    #[inline(always)]
    pub const fn gpdma_c15llr(&self) -> &GPDMA_C15LLR {
        &self.gpdma_c15llr
    }
}
/**GPDMA_SECCFGR (rw) register accessor: GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`gpdma_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_SECCFGR)

For information about available fields see [`mod@gpdma_seccfgr`]
module*/
pub type GPDMA_SECCFGR = crate::Reg<gpdma_seccfgr::GPDMA_SECCFGRrs>;
///GPDMA secure configuration register
pub mod gpdma_seccfgr;
/**GPDMA_PRIVCFGR (rw) register accessor: GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`gpdma_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_PRIVCFGR)

For information about available fields see [`mod@gpdma_privcfgr`]
module*/
pub type GPDMA_PRIVCFGR = crate::Reg<gpdma_privcfgr::GPDMA_PRIVCFGRrs>;
///GPDMA privileged configuration register
pub mod gpdma_privcfgr;
/**MISR (r) register accessor: non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:MISR)

For information about available fields see [`mod@misr`]
module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///non-secure masked interrupt status register
pub mod misr;
/**SMISR (r) register accessor: secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:SMISR)

For information about available fields see [`mod@smisr`]
module*/
pub type SMISR = crate::Reg<smisr::SMISRrs>;
///secure masked interrupt status register
pub mod smisr;
/**GPDMA_C0LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0LBAR)

For information about available fields see [`mod@gpdma_c0lbar`]
module*/
pub type GPDMA_C0LBAR = crate::Reg<gpdma_c0lbar::GPDMA_C0LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c0lbar;
/**GPDMA_C0FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0FCR)

For information about available fields see [`mod@gpdma_c0fcr`]
module*/
pub type GPDMA_C0FCR = crate::Reg<gpdma_c0fcr::GPDMA_C0FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c0fcr;
/**GPDMA_C0SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0SR)

For information about available fields see [`mod@gpdma_c0sr`]
module*/
pub type GPDMA_C0SR = crate::Reg<gpdma_c0sr::GPDMA_C0SRrs>;
///channel x status register
pub mod gpdma_c0sr;
/**GPDMA_C0CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0CR)

For information about available fields see [`mod@gpdma_c0cr`]
module*/
pub type GPDMA_C0CR = crate::Reg<gpdma_c0cr::GPDMA_C0CRrs>;
///channel x control register
pub mod gpdma_c0cr;
/**GPDMA_C0TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0TR1)

For information about available fields see [`mod@gpdma_c0tr1`]
module*/
pub type GPDMA_C0TR1 = crate::Reg<gpdma_c0tr1::GPDMA_C0TR1rs>;
///GPDMA channel x transfer register 1
pub mod gpdma_c0tr1;
/**GPDMA_C0TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0TR2)

For information about available fields see [`mod@gpdma_c0tr2`]
module*/
pub type GPDMA_C0TR2 = crate::Reg<gpdma_c0tr2::GPDMA_C0TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c0tr2;
/**GPDMA_C0BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0BR1)

For information about available fields see [`mod@gpdma_c0br1`]
module*/
pub type GPDMA_C0BR1 = crate::Reg<gpdma_c0br1::GPDMA_C0BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c0br1;
/**GPDMA_C0SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0SAR)

For information about available fields see [`mod@gpdma_c0sar`]
module*/
pub type GPDMA_C0SAR = crate::Reg<gpdma_c0sar::GPDMA_C0SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c0sar;
/**GPDMA_C0DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0DAR)

For information about available fields see [`mod@gpdma_c0dar`]
module*/
pub type GPDMA_C0DAR = crate::Reg<gpdma_c0dar::GPDMA_C0DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c0dar;
/**GPDMA_C0LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c0llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c0llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C0LLR)

For information about available fields see [`mod@gpdma_c0llr`]
module*/
pub type GPDMA_C0LLR = crate::Reg<gpdma_c0llr::GPDMA_C0LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c0llr;
/**GPDMA_C1LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1LBAR)

For information about available fields see [`mod@gpdma_c1lbar`]
module*/
pub type GPDMA_C1LBAR = crate::Reg<gpdma_c1lbar::GPDMA_C1LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c1lbar;
/**GPDMA_C1FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1FCR)

For information about available fields see [`mod@gpdma_c1fcr`]
module*/
pub type GPDMA_C1FCR = crate::Reg<gpdma_c1fcr::GPDMA_C1FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c1fcr;
/**GPDMA_C1SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1SR)

For information about available fields see [`mod@gpdma_c1sr`]
module*/
pub type GPDMA_C1SR = crate::Reg<gpdma_c1sr::GPDMA_C1SRrs>;
///channel x status register
pub mod gpdma_c1sr;
/**GPDMA_C1CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1CR)

For information about available fields see [`mod@gpdma_c1cr`]
module*/
pub type GPDMA_C1CR = crate::Reg<gpdma_c1cr::GPDMA_C1CRrs>;
///channel x control register
pub mod gpdma_c1cr;
/**GPDMA_C1TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1TR1)

For information about available fields see [`mod@gpdma_c1tr1`]
module*/
pub type GPDMA_C1TR1 = crate::Reg<gpdma_c1tr1::GPDMA_C1TR1rs>;
///GPDMA channel x transfer register 1
pub mod gpdma_c1tr1;
/**GPDMA_C1TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1TR2)

For information about available fields see [`mod@gpdma_c1tr2`]
module*/
pub type GPDMA_C1TR2 = crate::Reg<gpdma_c1tr2::GPDMA_C1TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c1tr2;
/**GPDMA_C1BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1BR1)

For information about available fields see [`mod@gpdma_c1br1`]
module*/
pub type GPDMA_C1BR1 = crate::Reg<gpdma_c1br1::GPDMA_C1BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c1br1;
/**GPDMA_C1SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1SAR)

For information about available fields see [`mod@gpdma_c1sar`]
module*/
pub type GPDMA_C1SAR = crate::Reg<gpdma_c1sar::GPDMA_C1SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c1sar;
/**GPDMA_C1DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1DAR)

For information about available fields see [`mod@gpdma_c1dar`]
module*/
pub type GPDMA_C1DAR = crate::Reg<gpdma_c1dar::GPDMA_C1DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c1dar;
/**GPDMA_C1LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c1llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c1llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C1LLR)

For information about available fields see [`mod@gpdma_c1llr`]
module*/
pub type GPDMA_C1LLR = crate::Reg<gpdma_c1llr::GPDMA_C1LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c1llr;
/**GPDMA_C2LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2LBAR)

For information about available fields see [`mod@gpdma_c2lbar`]
module*/
pub type GPDMA_C2LBAR = crate::Reg<gpdma_c2lbar::GPDMA_C2LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c2lbar;
/**GPDMA_C2FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2FCR)

For information about available fields see [`mod@gpdma_c2fcr`]
module*/
pub type GPDMA_C2FCR = crate::Reg<gpdma_c2fcr::GPDMA_C2FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c2fcr;
/**GPDMA_C2SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2SR)

For information about available fields see [`mod@gpdma_c2sr`]
module*/
pub type GPDMA_C2SR = crate::Reg<gpdma_c2sr::GPDMA_C2SRrs>;
///channel x status register
pub mod gpdma_c2sr;
/**GPDMA_C2CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2CR)

For information about available fields see [`mod@gpdma_c2cr`]
module*/
pub type GPDMA_C2CR = crate::Reg<gpdma_c2cr::GPDMA_C2CRrs>;
///channel x control register
pub mod gpdma_c2cr;
/**GPDMA_C2TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2TR1)

For information about available fields see [`mod@gpdma_c2tr1`]
module*/
pub type GPDMA_C2TR1 = crate::Reg<gpdma_c2tr1::GPDMA_C2TR1rs>;
///GPDMA channel x transfer register 1
pub mod gpdma_c2tr1;
/**GPDMA_C2TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2TR2)

For information about available fields see [`mod@gpdma_c2tr2`]
module*/
pub type GPDMA_C2TR2 = crate::Reg<gpdma_c2tr2::GPDMA_C2TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c2tr2;
/**GPDMA_C2BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2BR1)

For information about available fields see [`mod@gpdma_c2br1`]
module*/
pub type GPDMA_C2BR1 = crate::Reg<gpdma_c2br1::GPDMA_C2BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c2br1;
/**GPDMA_C2SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2SAR)

For information about available fields see [`mod@gpdma_c2sar`]
module*/
pub type GPDMA_C2SAR = crate::Reg<gpdma_c2sar::GPDMA_C2SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c2sar;
/**GPDMA_C2DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2DAR)

For information about available fields see [`mod@gpdma_c2dar`]
module*/
pub type GPDMA_C2DAR = crate::Reg<gpdma_c2dar::GPDMA_C2DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c2dar;
/**GPDMA_C2LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c2llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c2llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C2LLR)

For information about available fields see [`mod@gpdma_c2llr`]
module*/
pub type GPDMA_C2LLR = crate::Reg<gpdma_c2llr::GPDMA_C2LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c2llr;
/**GPDMA_C3LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3LBAR)

For information about available fields see [`mod@gpdma_c3lbar`]
module*/
pub type GPDMA_C3LBAR = crate::Reg<gpdma_c3lbar::GPDMA_C3LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c3lbar;
/**GPDMA_C3FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3FCR)

For information about available fields see [`mod@gpdma_c3fcr`]
module*/
pub type GPDMA_C3FCR = crate::Reg<gpdma_c3fcr::GPDMA_C3FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c3fcr;
/**GPDMA_C3SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3SR)

For information about available fields see [`mod@gpdma_c3sr`]
module*/
pub type GPDMA_C3SR = crate::Reg<gpdma_c3sr::GPDMA_C3SRrs>;
///channel x status register
pub mod gpdma_c3sr;
/**GPDMA_C3CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3CR)

For information about available fields see [`mod@gpdma_c3cr`]
module*/
pub type GPDMA_C3CR = crate::Reg<gpdma_c3cr::GPDMA_C3CRrs>;
///channel x control register
pub mod gpdma_c3cr;
/**GPDMA_C3TR1 (rw) register accessor: GPDMA channel x transfer register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3TR1)

For information about available fields see [`mod@gpdma_c3tr1`]
module*/
pub type GPDMA_C3TR1 = crate::Reg<gpdma_c3tr1::GPDMA_C3TR1rs>;
///GPDMA channel x transfer register 1
pub mod gpdma_c3tr1;
/**GPDMA_C3TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3TR2)

For information about available fields see [`mod@gpdma_c3tr2`]
module*/
pub type GPDMA_C3TR2 = crate::Reg<gpdma_c3tr2::GPDMA_C3TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c3tr2;
/**GPDMA_C4TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4TR2)

For information about available fields see [`mod@gpdma_c4tr2`]
module*/
pub type GPDMA_C4TR2 = crate::Reg<gpdma_c4tr2::GPDMA_C4TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c4tr2;
/**GPDMA_C5TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5TR2)

For information about available fields see [`mod@gpdma_c5tr2`]
module*/
pub type GPDMA_C5TR2 = crate::Reg<gpdma_c5tr2::GPDMA_C5TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c5tr2;
/**GPDMA_C6TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6TR2)

For information about available fields see [`mod@gpdma_c6tr2`]
module*/
pub type GPDMA_C6TR2 = crate::Reg<gpdma_c6tr2::GPDMA_C6TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c6tr2;
/**GPDMA_C7TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7TR2)

For information about available fields see [`mod@gpdma_c7tr2`]
module*/
pub type GPDMA_C7TR2 = crate::Reg<gpdma_c7tr2::GPDMA_C7TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c7tr2;
/**GPDMA_C8TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8TR2)

For information about available fields see [`mod@gpdma_c8tr2`]
module*/
pub type GPDMA_C8TR2 = crate::Reg<gpdma_c8tr2::GPDMA_C8TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c8tr2;
/**GPDMA_C9TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9TR2)

For information about available fields see [`mod@gpdma_c9tr2`]
module*/
pub type GPDMA_C9TR2 = crate::Reg<gpdma_c9tr2::GPDMA_C9TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c9tr2;
/**GPDMA_C10TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10TR2)

For information about available fields see [`mod@gpdma_c10tr2`]
module*/
pub type GPDMA_C10TR2 = crate::Reg<gpdma_c10tr2::GPDMA_C10TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c10tr2;
/**GPDMA_C11TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11TR2)

For information about available fields see [`mod@gpdma_c11tr2`]
module*/
pub type GPDMA_C11TR2 = crate::Reg<gpdma_c11tr2::GPDMA_C11TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c11tr2;
/**GPDMA_C12TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12TR2)

For information about available fields see [`mod@gpdma_c12tr2`]
module*/
pub type GPDMA_C12TR2 = crate::Reg<gpdma_c12tr2::GPDMA_C12TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c12tr2;
/**GPDMA_C13TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13TR2)

For information about available fields see [`mod@gpdma_c13tr2`]
module*/
pub type GPDMA_C13TR2 = crate::Reg<gpdma_c13tr2::GPDMA_C13TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c13tr2;
/**GPDMA_C14TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14TR2)

For information about available fields see [`mod@gpdma_c14tr2`]
module*/
pub type GPDMA_C14TR2 = crate::Reg<gpdma_c14tr2::GPDMA_C14TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c14tr2;
/**GPDMA_C15TR2 (rw) register accessor: GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15TR2)

For information about available fields see [`mod@gpdma_c15tr2`]
module*/
pub type GPDMA_C15TR2 = crate::Reg<gpdma_c15tr2::GPDMA_C15TR2rs>;
///GPDMA channel x transfer register 2
pub mod gpdma_c15tr2;
/**GPDMA_C3BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3BR1)

For information about available fields see [`mod@gpdma_c3br1`]
module*/
pub type GPDMA_C3BR1 = crate::Reg<gpdma_c3br1::GPDMA_C3BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c3br1;
/**GPDMA_C4BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4BR1)

For information about available fields see [`mod@gpdma_c4br1`]
module*/
pub type GPDMA_C4BR1 = crate::Reg<gpdma_c4br1::GPDMA_C4BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c4br1;
/**GPDMA_C5BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5BR1)

For information about available fields see [`mod@gpdma_c5br1`]
module*/
pub type GPDMA_C5BR1 = crate::Reg<gpdma_c5br1::GPDMA_C5BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c5br1;
/**GPDMA_C6BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6BR1)

For information about available fields see [`mod@gpdma_c6br1`]
module*/
pub type GPDMA_C6BR1 = crate::Reg<gpdma_c6br1::GPDMA_C6BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c6br1;
/**GPDMA_C7BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7BR1)

For information about available fields see [`mod@gpdma_c7br1`]
module*/
pub type GPDMA_C7BR1 = crate::Reg<gpdma_c7br1::GPDMA_C7BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c7br1;
/**GPDMA_C8BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8BR1)

For information about available fields see [`mod@gpdma_c8br1`]
module*/
pub type GPDMA_C8BR1 = crate::Reg<gpdma_c8br1::GPDMA_C8BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c8br1;
/**GPDMA_C9BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9BR1)

For information about available fields see [`mod@gpdma_c9br1`]
module*/
pub type GPDMA_C9BR1 = crate::Reg<gpdma_c9br1::GPDMA_C9BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c9br1;
/**GPDMA_C10BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10BR1)

For information about available fields see [`mod@gpdma_c10br1`]
module*/
pub type GPDMA_C10BR1 = crate::Reg<gpdma_c10br1::GPDMA_C10BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c10br1;
/**GPDMA_C11BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11BR1)

For information about available fields see [`mod@gpdma_c11br1`]
module*/
pub type GPDMA_C11BR1 = crate::Reg<gpdma_c11br1::GPDMA_C11BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c11br1;
/**GPDMA_C12BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12BR1)

For information about available fields see [`mod@gpdma_c12br1`]
module*/
pub type GPDMA_C12BR1 = crate::Reg<gpdma_c12br1::GPDMA_C12BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c12br1;
/**GPDMA_C13BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13BR1)

For information about available fields see [`mod@gpdma_c13br1`]
module*/
pub type GPDMA_C13BR1 = crate::Reg<gpdma_c13br1::GPDMA_C13BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c13br1;
/**GPDMA_C14BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14BR1)

For information about available fields see [`mod@gpdma_c14br1`]
module*/
pub type GPDMA_C14BR1 = crate::Reg<gpdma_c14br1::GPDMA_C14BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c14br1;
/**GPDMA_C15BR1 (rw) register accessor: GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15BR1)

For information about available fields see [`mod@gpdma_c15br1`]
module*/
pub type GPDMA_C15BR1 = crate::Reg<gpdma_c15br1::GPDMA_C15BR1rs>;
///GPDMA channel x block register 1
pub mod gpdma_c15br1;
/**GPDMA_C3SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3SAR)

For information about available fields see [`mod@gpdma_c3sar`]
module*/
pub type GPDMA_C3SAR = crate::Reg<gpdma_c3sar::GPDMA_C3SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c3sar;
/**GPDMA_C4SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4SAR)

For information about available fields see [`mod@gpdma_c4sar`]
module*/
pub type GPDMA_C4SAR = crate::Reg<gpdma_c4sar::GPDMA_C4SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c4sar;
/**GPDMA_C5SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5SAR)

For information about available fields see [`mod@gpdma_c5sar`]
module*/
pub type GPDMA_C5SAR = crate::Reg<gpdma_c5sar::GPDMA_C5SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c5sar;
/**GPDMA_C6SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6SAR)

For information about available fields see [`mod@gpdma_c6sar`]
module*/
pub type GPDMA_C6SAR = crate::Reg<gpdma_c6sar::GPDMA_C6SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c6sar;
/**GPDMA_C7SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7SAR)

For information about available fields see [`mod@gpdma_c7sar`]
module*/
pub type GPDMA_C7SAR = crate::Reg<gpdma_c7sar::GPDMA_C7SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c7sar;
/**GPDMA_C8SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8SAR)

For information about available fields see [`mod@gpdma_c8sar`]
module*/
pub type GPDMA_C8SAR = crate::Reg<gpdma_c8sar::GPDMA_C8SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c8sar;
/**GPDMA_C9SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9SAR)

For information about available fields see [`mod@gpdma_c9sar`]
module*/
pub type GPDMA_C9SAR = crate::Reg<gpdma_c9sar::GPDMA_C9SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c9sar;
/**GPDMA_C10SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10SAR)

For information about available fields see [`mod@gpdma_c10sar`]
module*/
pub type GPDMA_C10SAR = crate::Reg<gpdma_c10sar::GPDMA_C10SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c10sar;
/**GPDMA_C11SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11SAR)

For information about available fields see [`mod@gpdma_c11sar`]
module*/
pub type GPDMA_C11SAR = crate::Reg<gpdma_c11sar::GPDMA_C11SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c11sar;
/**GPDMA_C12SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12SAR)

For information about available fields see [`mod@gpdma_c12sar`]
module*/
pub type GPDMA_C12SAR = crate::Reg<gpdma_c12sar::GPDMA_C12SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c12sar;
/**GPDMA_C13SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13SAR)

For information about available fields see [`mod@gpdma_c13sar`]
module*/
pub type GPDMA_C13SAR = crate::Reg<gpdma_c13sar::GPDMA_C13SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c13sar;
/**GPDMA_C14SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14SAR)

For information about available fields see [`mod@gpdma_c14sar`]
module*/
pub type GPDMA_C14SAR = crate::Reg<gpdma_c14sar::GPDMA_C14SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c14sar;
/**GPDMA_C15SAR (rw) register accessor: GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15SAR)

For information about available fields see [`mod@gpdma_c15sar`]
module*/
pub type GPDMA_C15SAR = crate::Reg<gpdma_c15sar::GPDMA_C15SARrs>;
///GPDMA channel x source address register
pub mod gpdma_c15sar;
/**GPDMA_C3DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3DAR)

For information about available fields see [`mod@gpdma_c3dar`]
module*/
pub type GPDMA_C3DAR = crate::Reg<gpdma_c3dar::GPDMA_C3DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c3dar;
/**GPDMA_C4DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4DAR)

For information about available fields see [`mod@gpdma_c4dar`]
module*/
pub type GPDMA_C4DAR = crate::Reg<gpdma_c4dar::GPDMA_C4DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c4dar;
/**GPDMA_C5DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5DAR)

For information about available fields see [`mod@gpdma_c5dar`]
module*/
pub type GPDMA_C5DAR = crate::Reg<gpdma_c5dar::GPDMA_C5DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c5dar;
/**GPDMA_C6DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6DAR)

For information about available fields see [`mod@gpdma_c6dar`]
module*/
pub type GPDMA_C6DAR = crate::Reg<gpdma_c6dar::GPDMA_C6DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c6dar;
/**GPDMA_C7DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7DAR)

For information about available fields see [`mod@gpdma_c7dar`]
module*/
pub type GPDMA_C7DAR = crate::Reg<gpdma_c7dar::GPDMA_C7DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c7dar;
/**GPDMA_C8DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8DAR)

For information about available fields see [`mod@gpdma_c8dar`]
module*/
pub type GPDMA_C8DAR = crate::Reg<gpdma_c8dar::GPDMA_C8DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c8dar;
/**GPDMA_C9DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9DAR)

For information about available fields see [`mod@gpdma_c9dar`]
module*/
pub type GPDMA_C9DAR = crate::Reg<gpdma_c9dar::GPDMA_C9DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c9dar;
/**GPDMA_C10DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10DAR)

For information about available fields see [`mod@gpdma_c10dar`]
module*/
pub type GPDMA_C10DAR = crate::Reg<gpdma_c10dar::GPDMA_C10DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c10dar;
/**GPDMA_C11DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11DAR)

For information about available fields see [`mod@gpdma_c11dar`]
module*/
pub type GPDMA_C11DAR = crate::Reg<gpdma_c11dar::GPDMA_C11DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c11dar;
/**GPDMA_C12DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12DAR)

For information about available fields see [`mod@gpdma_c12dar`]
module*/
pub type GPDMA_C12DAR = crate::Reg<gpdma_c12dar::GPDMA_C12DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c12dar;
/**GPDMA_C13DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13DAR)

For information about available fields see [`mod@gpdma_c13dar`]
module*/
pub type GPDMA_C13DAR = crate::Reg<gpdma_c13dar::GPDMA_C13DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c13dar;
/**GPDMA_C14DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14DAR)

For information about available fields see [`mod@gpdma_c14dar`]
module*/
pub type GPDMA_C14DAR = crate::Reg<gpdma_c14dar::GPDMA_C14DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c14dar;
/**GPDMA_C15DAR (rw) register accessor: GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15DAR)

For information about available fields see [`mod@gpdma_c15dar`]
module*/
pub type GPDMA_C15DAR = crate::Reg<gpdma_c15dar::GPDMA_C15DARrs>;
///GPDMA channel x destination address register
pub mod gpdma_c15dar;
/**GPDMA_C3LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c3llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c3llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C3LLR)

For information about available fields see [`mod@gpdma_c3llr`]
module*/
pub type GPDMA_C3LLR = crate::Reg<gpdma_c3llr::GPDMA_C3LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c3llr;
/**GPDMA_C4LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4LLR)

For information about available fields see [`mod@gpdma_c4llr`]
module*/
pub type GPDMA_C4LLR = crate::Reg<gpdma_c4llr::GPDMA_C4LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c4llr;
/**GPDMA_C5LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5LLR)

For information about available fields see [`mod@gpdma_c5llr`]
module*/
pub type GPDMA_C5LLR = crate::Reg<gpdma_c5llr::GPDMA_C5LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c5llr;
/**GPDMA_C6LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6LLR)

For information about available fields see [`mod@gpdma_c6llr`]
module*/
pub type GPDMA_C6LLR = crate::Reg<gpdma_c6llr::GPDMA_C6LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c6llr;
/**GPDMA_C7LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7LLR)

For information about available fields see [`mod@gpdma_c7llr`]
module*/
pub type GPDMA_C7LLR = crate::Reg<gpdma_c7llr::GPDMA_C7LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c7llr;
/**GPDMA_C8LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8LLR)

For information about available fields see [`mod@gpdma_c8llr`]
module*/
pub type GPDMA_C8LLR = crate::Reg<gpdma_c8llr::GPDMA_C8LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c8llr;
/**GPDMA_C9LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9LLR)

For information about available fields see [`mod@gpdma_c9llr`]
module*/
pub type GPDMA_C9LLR = crate::Reg<gpdma_c9llr::GPDMA_C9LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c9llr;
/**GPDMA_C10LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10LLR)

For information about available fields see [`mod@gpdma_c10llr`]
module*/
pub type GPDMA_C10LLR = crate::Reg<gpdma_c10llr::GPDMA_C10LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c10llr;
/**GPDMA_C11LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11LLR)

For information about available fields see [`mod@gpdma_c11llr`]
module*/
pub type GPDMA_C11LLR = crate::Reg<gpdma_c11llr::GPDMA_C11LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c11llr;
/**GPDMA_C12LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12LLR)

For information about available fields see [`mod@gpdma_c12llr`]
module*/
pub type GPDMA_C12LLR = crate::Reg<gpdma_c12llr::GPDMA_C12LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c12llr;
/**GPDMA_C13LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13LLR)

For information about available fields see [`mod@gpdma_c13llr`]
module*/
pub type GPDMA_C13LLR = crate::Reg<gpdma_c13llr::GPDMA_C13LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c13llr;
/**GPDMA_C14LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14LLR)

For information about available fields see [`mod@gpdma_c14llr`]
module*/
pub type GPDMA_C14LLR = crate::Reg<gpdma_c14llr::GPDMA_C14LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c14llr;
/**GPDMA_C15LLR (rw) register accessor: GPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15LLR)

For information about available fields see [`mod@gpdma_c15llr`]
module*/
pub type GPDMA_C15LLR = crate::Reg<gpdma_c15llr::GPDMA_C15LLRrs>;
///GPDMA channel x linked-list address register
pub mod gpdma_c15llr;
/**GPDMA_C4LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4LBAR)

For information about available fields see [`mod@gpdma_c4lbar`]
module*/
pub type GPDMA_C4LBAR = crate::Reg<gpdma_c4lbar::GPDMA_C4LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c4lbar;
/**GPDMA_C5LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5LBAR)

For information about available fields see [`mod@gpdma_c5lbar`]
module*/
pub type GPDMA_C5LBAR = crate::Reg<gpdma_c5lbar::GPDMA_C5LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c5lbar;
/**GPDMA_C6LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6LBAR)

For information about available fields see [`mod@gpdma_c6lbar`]
module*/
pub type GPDMA_C6LBAR = crate::Reg<gpdma_c6lbar::GPDMA_C6LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c6lbar;
/**GPDMA_C7LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7LBAR)

For information about available fields see [`mod@gpdma_c7lbar`]
module*/
pub type GPDMA_C7LBAR = crate::Reg<gpdma_c7lbar::GPDMA_C7LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c7lbar;
/**GPDMA_C8LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8LBAR)

For information about available fields see [`mod@gpdma_c8lbar`]
module*/
pub type GPDMA_C8LBAR = crate::Reg<gpdma_c8lbar::GPDMA_C8LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c8lbar;
/**GPDMA_C9LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9LBAR)

For information about available fields see [`mod@gpdma_c9lbar`]
module*/
pub type GPDMA_C9LBAR = crate::Reg<gpdma_c9lbar::GPDMA_C9LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c9lbar;
/**GPDMA_C10LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10LBAR)

For information about available fields see [`mod@gpdma_c10lbar`]
module*/
pub type GPDMA_C10LBAR = crate::Reg<gpdma_c10lbar::GPDMA_C10LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c10lbar;
/**GPDMA_C11LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11LBAR)

For information about available fields see [`mod@gpdma_c11lbar`]
module*/
pub type GPDMA_C11LBAR = crate::Reg<gpdma_c11lbar::GPDMA_C11LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c11lbar;
/**GPDMA_C12LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12LBAR)

For information about available fields see [`mod@gpdma_c12lbar`]
module*/
pub type GPDMA_C12LBAR = crate::Reg<gpdma_c12lbar::GPDMA_C12LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c12lbar;
/**GPDMA_C13LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13LBAR)

For information about available fields see [`mod@gpdma_c13lbar`]
module*/
pub type GPDMA_C13LBAR = crate::Reg<gpdma_c13lbar::GPDMA_C13LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c13lbar;
/**GPDMA_C14LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14LBAR)

For information about available fields see [`mod@gpdma_c14lbar`]
module*/
pub type GPDMA_C14LBAR = crate::Reg<gpdma_c14lbar::GPDMA_C14LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c14lbar;
/**GPDMA_C15LBAR (rw) register accessor: channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15LBAR)

For information about available fields see [`mod@gpdma_c15lbar`]
module*/
pub type GPDMA_C15LBAR = crate::Reg<gpdma_c15lbar::GPDMA_C15LBARrs>;
///channel x linked-list base address register
pub mod gpdma_c15lbar;
/**GPDMA_C4FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4FCR)

For information about available fields see [`mod@gpdma_c4fcr`]
module*/
pub type GPDMA_C4FCR = crate::Reg<gpdma_c4fcr::GPDMA_C4FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c4fcr;
/**GPDMA_C5FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5FCR)

For information about available fields see [`mod@gpdma_c5fcr`]
module*/
pub type GPDMA_C5FCR = crate::Reg<gpdma_c5fcr::GPDMA_C5FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c5fcr;
/**GPDMA_C6FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6FCR)

For information about available fields see [`mod@gpdma_c6fcr`]
module*/
pub type GPDMA_C6FCR = crate::Reg<gpdma_c6fcr::GPDMA_C6FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c6fcr;
/**GPDMA_C7FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7FCR)

For information about available fields see [`mod@gpdma_c7fcr`]
module*/
pub type GPDMA_C7FCR = crate::Reg<gpdma_c7fcr::GPDMA_C7FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c7fcr;
/**GPDMA_C8FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8FCR)

For information about available fields see [`mod@gpdma_c8fcr`]
module*/
pub type GPDMA_C8FCR = crate::Reg<gpdma_c8fcr::GPDMA_C8FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c8fcr;
/**GPDMA_C9FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9FCR)

For information about available fields see [`mod@gpdma_c9fcr`]
module*/
pub type GPDMA_C9FCR = crate::Reg<gpdma_c9fcr::GPDMA_C9FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c9fcr;
/**GPDMA_C10FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10FCR)

For information about available fields see [`mod@gpdma_c10fcr`]
module*/
pub type GPDMA_C10FCR = crate::Reg<gpdma_c10fcr::GPDMA_C10FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c10fcr;
/**GPDMA_C11FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11FCR)

For information about available fields see [`mod@gpdma_c11fcr`]
module*/
pub type GPDMA_C11FCR = crate::Reg<gpdma_c11fcr::GPDMA_C11FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c11fcr;
/**GPDMA_C12FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12FCR)

For information about available fields see [`mod@gpdma_c12fcr`]
module*/
pub type GPDMA_C12FCR = crate::Reg<gpdma_c12fcr::GPDMA_C12FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c12fcr;
/**GPDMA_C13FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13FCR)

For information about available fields see [`mod@gpdma_c13fcr`]
module*/
pub type GPDMA_C13FCR = crate::Reg<gpdma_c13fcr::GPDMA_C13FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c13fcr;
/**GPDMA_C14FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14FCR)

For information about available fields see [`mod@gpdma_c14fcr`]
module*/
pub type GPDMA_C14FCR = crate::Reg<gpdma_c14fcr::GPDMA_C14FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c14fcr;
/**GPDMA_C15FCR (w) register accessor: GPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15FCR)

For information about available fields see [`mod@gpdma_c15fcr`]
module*/
pub type GPDMA_C15FCR = crate::Reg<gpdma_c15fcr::GPDMA_C15FCRrs>;
///GPDMA channel x flag clear register
pub mod gpdma_c15fcr;
/**GPDMA_C4SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4SR)

For information about available fields see [`mod@gpdma_c4sr`]
module*/
pub type GPDMA_C4SR = crate::Reg<gpdma_c4sr::GPDMA_C4SRrs>;
///channel x status register
pub mod gpdma_c4sr;
/**GPDMA_C5SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5SR)

For information about available fields see [`mod@gpdma_c5sr`]
module*/
pub type GPDMA_C5SR = crate::Reg<gpdma_c5sr::GPDMA_C5SRrs>;
///channel x status register
pub mod gpdma_c5sr;
/**GPDMA_C6SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6SR)

For information about available fields see [`mod@gpdma_c6sr`]
module*/
pub type GPDMA_C6SR = crate::Reg<gpdma_c6sr::GPDMA_C6SRrs>;
///channel x status register
pub mod gpdma_c6sr;
/**GPDMA_C7SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7SR)

For information about available fields see [`mod@gpdma_c7sr`]
module*/
pub type GPDMA_C7SR = crate::Reg<gpdma_c7sr::GPDMA_C7SRrs>;
///channel x status register
pub mod gpdma_c7sr;
/**GPDMA_C8SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8SR)

For information about available fields see [`mod@gpdma_c8sr`]
module*/
pub type GPDMA_C8SR = crate::Reg<gpdma_c8sr::GPDMA_C8SRrs>;
///channel x status register
pub mod gpdma_c8sr;
/**GPDMA_C9SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9SR)

For information about available fields see [`mod@gpdma_c9sr`]
module*/
pub type GPDMA_C9SR = crate::Reg<gpdma_c9sr::GPDMA_C9SRrs>;
///channel x status register
pub mod gpdma_c9sr;
/**GPDMA_C10SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10SR)

For information about available fields see [`mod@gpdma_c10sr`]
module*/
pub type GPDMA_C10SR = crate::Reg<gpdma_c10sr::GPDMA_C10SRrs>;
///channel x status register
pub mod gpdma_c10sr;
/**GPDMA_C11SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11SR)

For information about available fields see [`mod@gpdma_c11sr`]
module*/
pub type GPDMA_C11SR = crate::Reg<gpdma_c11sr::GPDMA_C11SRrs>;
///channel x status register
pub mod gpdma_c11sr;
/**GPDMA_C12SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12SR)

For information about available fields see [`mod@gpdma_c12sr`]
module*/
pub type GPDMA_C12SR = crate::Reg<gpdma_c12sr::GPDMA_C12SRrs>;
///channel x status register
pub mod gpdma_c12sr;
/**GPDMA_C13SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13SR)

For information about available fields see [`mod@gpdma_c13sr`]
module*/
pub type GPDMA_C13SR = crate::Reg<gpdma_c13sr::GPDMA_C13SRrs>;
///channel x status register
pub mod gpdma_c13sr;
/**GPDMA_C14SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14SR)

For information about available fields see [`mod@gpdma_c14sr`]
module*/
pub type GPDMA_C14SR = crate::Reg<gpdma_c14sr::GPDMA_C14SRrs>;
///channel x status register
pub mod gpdma_c14sr;
/**GPDMA_C15SR (r) register accessor: channel x status register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15SR)

For information about available fields see [`mod@gpdma_c15sr`]
module*/
pub type GPDMA_C15SR = crate::Reg<gpdma_c15sr::GPDMA_C15SRrs>;
///channel x status register
pub mod gpdma_c15sr;
/**GPDMA_C4CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C4CR)

For information about available fields see [`mod@gpdma_c4cr`]
module*/
pub type GPDMA_C4CR = crate::Reg<gpdma_c4cr::GPDMA_C4CRrs>;
///channel x control register
pub mod gpdma_c4cr;
/**GPDMA_C5CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C5CR)

For information about available fields see [`mod@gpdma_c5cr`]
module*/
pub type GPDMA_C5CR = crate::Reg<gpdma_c5cr::GPDMA_C5CRrs>;
///channel x control register
pub mod gpdma_c5cr;
/**GPDMA_C6CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C6CR)

For information about available fields see [`mod@gpdma_c6cr`]
module*/
pub type GPDMA_C6CR = crate::Reg<gpdma_c6cr::GPDMA_C6CRrs>;
///channel x control register
pub mod gpdma_c6cr;
/**GPDMA_C7CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C7CR)

For information about available fields see [`mod@gpdma_c7cr`]
module*/
pub type GPDMA_C7CR = crate::Reg<gpdma_c7cr::GPDMA_C7CRrs>;
///channel x control register
pub mod gpdma_c7cr;
/**GPDMA_C8CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C8CR)

For information about available fields see [`mod@gpdma_c8cr`]
module*/
pub type GPDMA_C8CR = crate::Reg<gpdma_c8cr::GPDMA_C8CRrs>;
///channel x control register
pub mod gpdma_c8cr;
/**GPDMA_C9CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C9CR)

For information about available fields see [`mod@gpdma_c9cr`]
module*/
pub type GPDMA_C9CR = crate::Reg<gpdma_c9cr::GPDMA_C9CRrs>;
///channel x control register
pub mod gpdma_c9cr;
/**GPDMA_C10CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C10CR)

For information about available fields see [`mod@gpdma_c10cr`]
module*/
pub type GPDMA_C10CR = crate::Reg<gpdma_c10cr::GPDMA_C10CRrs>;
///channel x control register
pub mod gpdma_c10cr;
/**GPDMA_C11CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C11CR)

For information about available fields see [`mod@gpdma_c11cr`]
module*/
pub type GPDMA_C11CR = crate::Reg<gpdma_c11cr::GPDMA_C11CRrs>;
///channel x control register
pub mod gpdma_c11cr;
/**GPDMA_C12CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12CR)

For information about available fields see [`mod@gpdma_c12cr`]
module*/
pub type GPDMA_C12CR = crate::Reg<gpdma_c12cr::GPDMA_C12CRrs>;
///channel x control register
pub mod gpdma_c12cr;
/**GPDMA_C13CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13CR)

For information about available fields see [`mod@gpdma_c13cr`]
module*/
pub type GPDMA_C13CR = crate::Reg<gpdma_c13cr::GPDMA_C13CRrs>;
///channel x control register
pub mod gpdma_c13cr;
/**GPDMA_C14CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14CR)

For information about available fields see [`mod@gpdma_c14cr`]
module*/
pub type GPDMA_C14CR = crate::Reg<gpdma_c14cr::GPDMA_C14CRrs>;
///channel x control register
pub mod gpdma_c14cr;
/**GPDMA_C15CR (rw) register accessor: channel x control register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15CR)

For information about available fields see [`mod@gpdma_c15cr`]
module*/
pub type GPDMA_C15CR = crate::Reg<gpdma_c15cr::GPDMA_C15CRrs>;
///channel x control register
pub mod gpdma_c15cr;
/**GPDMA_C12TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12TR3)

For information about available fields see [`mod@gpdma_c12tr3`]
module*/
pub type GPDMA_C12TR3 = crate::Reg<gpdma_c12tr3::GPDMA_C12TR3rs>;
///GPDMA channel x transfer register 3
pub mod gpdma_c12tr3;
/**GPDMA_C13TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13TR3)

For information about available fields see [`mod@gpdma_c13tr3`]
module*/
pub type GPDMA_C13TR3 = crate::Reg<gpdma_c13tr3::GPDMA_C13TR3rs>;
///GPDMA channel x transfer register 3
pub mod gpdma_c13tr3;
/**GPDMA_C14TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14TR3)

For information about available fields see [`mod@gpdma_c14tr3`]
module*/
pub type GPDMA_C14TR3 = crate::Reg<gpdma_c14tr3::GPDMA_C14TR3rs>;
///GPDMA channel x transfer register 3
pub mod gpdma_c14tr3;
/**GPDMA_C15TR3 (rw) register accessor: GPDMA channel x transfer register 3

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15TR3)

For information about available fields see [`mod@gpdma_c15tr3`]
module*/
pub type GPDMA_C15TR3 = crate::Reg<gpdma_c15tr3::GPDMA_C15TR3rs>;
///GPDMA channel x transfer register 3
pub mod gpdma_c15tr3;
/**GPDMA_C12BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C12BR2)

For information about available fields see [`mod@gpdma_c12br2`]
module*/
pub type GPDMA_C12BR2 = crate::Reg<gpdma_c12br2::GPDMA_C12BR2rs>;
///GPDMA channel x block register 2
pub mod gpdma_c12br2;
/**GPDMA_C13BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c13br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c13br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C13BR2)

For information about available fields see [`mod@gpdma_c13br2`]
module*/
pub type GPDMA_C13BR2 = crate::Reg<gpdma_c13br2::GPDMA_C13BR2rs>;
///GPDMA channel x block register 2
pub mod gpdma_c13br2;
/**GPDMA_C14BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c14br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c14br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C14BR2)

For information about available fields see [`mod@gpdma_c14br2`]
module*/
pub type GPDMA_C14BR2 = crate::Reg<gpdma_c14br2::GPDMA_C14BR2rs>;
///GPDMA channel x block register 2
pub mod gpdma_c14br2;
/**GPDMA_C15BR2 (rw) register accessor: GPDMA channel x block register 2

You can [`read`](crate::Reg::read) this register and get [`gpdma_c15br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c15br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GPDMA1:GPDMA_C15BR2)

For information about available fields see [`mod@gpdma_c15br2`]
module*/
pub type GPDMA_C15BR2 = crate::Reg<gpdma_c15br2::GPDMA_C15BR2rs>;
///GPDMA channel x block register 2
pub mod gpdma_c15br2;
