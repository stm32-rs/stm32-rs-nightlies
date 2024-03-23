#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmamux_c0cr: DMAMUX_C0CR,
    dmamux_c1cr: DMAMUX_C1CR,
    dmamux_c2cr: DMAMUX_C2CR,
    dmamux_c3cr: DMAMUX_C3CR,
    dmamux_c4cr: DMAMUX_C4CR,
    dmamux_c5cr: DMAMUX_C5CR,
    dmamux_c6cr: DMAMUX_C6CR,
    dmamux_c7cr: DMAMUX_C7CR,
    dmamux_c8cr: DMAMUX_C8CR,
    dmamux_c9cr: DMAMUX_C9CR,
    dmamux_c10cr: DMAMUX_C10CR,
    dmamux_c11cr: DMAMUX_C11CR,
    dmamux_c12cr: DMAMUX_C12CR,
    dmamux_c13cr: DMAMUX_C13CR,
    dmamux_c14cr: DMAMUX_C14CR,
    dmamux_c15cr: DMAMUX_C15CR,
    _reserved16: [u8; 0x40],
    dmamux_csr: DMAMUX_CSR,
    dmamux_cfr: DMAMUX_CFR,
    _reserved18: [u8; 0x78],
    dmamux_rg0cr: DMAMUX_RG0CR,
    dmamux_rg1cr: DMAMUX_RG1CR,
    dmamux_rg2cr: DMAMUX_RG2CR,
    dmamux_rg3cr: DMAMUX_RG3CR,
    dmamux_rg4cr: DMAMUX_RG4CR,
    dmamux_rg5cr: DMAMUX_RG5CR,
    dmamux_rg6cr: DMAMUX_RG6CR,
    dmamux_rg7cr: DMAMUX_RG7CR,
    _reserved26: [u8; 0x20],
    dmamux_rgsr: DMAMUX_RGSR,
    dmamux_rgcfr: DMAMUX_RGCFR,
    _reserved28: [u8; 0x02a4],
    dmamux_hwcfgr2: DMAMUX_HWCFGR2,
    dmamux_hwcfgr1: DMAMUX_HWCFGR1,
    dmamux_verr: DMAMUX_VERR,
    dmamux_ipidr: DMAMUX_IPIDR,
    dmamux_sidr: DMAMUX_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel 0 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c0cr(&self) -> &DMAMUX_C0CR {
        &self.dmamux_c0cr
    }
    #[doc = "0x04 - DMAMUX request line multiplexer channel 1 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c1cr(&self) -> &DMAMUX_C1CR {
        &self.dmamux_c1cr
    }
    #[doc = "0x08 - DMAMUX request line multiplexer channel 2 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c2cr(&self) -> &DMAMUX_C2CR {
        &self.dmamux_c2cr
    }
    #[doc = "0x0c - DMAMUX request line multiplexer channel 3 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c3cr(&self) -> &DMAMUX_C3CR {
        &self.dmamux_c3cr
    }
    #[doc = "0x10 - DMAMUX request line multiplexer channel 4 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c4cr(&self) -> &DMAMUX_C4CR {
        &self.dmamux_c4cr
    }
    #[doc = "0x14 - DMAMUX request line multiplexer channel 5 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c5cr(&self) -> &DMAMUX_C5CR {
        &self.dmamux_c5cr
    }
    #[doc = "0x18 - DMAMUX request line multiplexer channel 6 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c6cr(&self) -> &DMAMUX_C6CR {
        &self.dmamux_c6cr
    }
    #[doc = "0x1c - DMAMUX request line multiplexer channel 7 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c7cr(&self) -> &DMAMUX_C7CR {
        &self.dmamux_c7cr
    }
    #[doc = "0x20 - DMAMUX request line multiplexer channel 8 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c8cr(&self) -> &DMAMUX_C8CR {
        &self.dmamux_c8cr
    }
    #[doc = "0x24 - DMAMUX request line multiplexer channel 9 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c9cr(&self) -> &DMAMUX_C9CR {
        &self.dmamux_c9cr
    }
    #[doc = "0x28 - DMAMUX request line multiplexer channel 10 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c10cr(&self) -> &DMAMUX_C10CR {
        &self.dmamux_c10cr
    }
    #[doc = "0x2c - DMAMUX request line multiplexer channel 11 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c11cr(&self) -> &DMAMUX_C11CR {
        &self.dmamux_c11cr
    }
    #[doc = "0x30 - DMAMUX request line multiplexer channel 12 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c12cr(&self) -> &DMAMUX_C12CR {
        &self.dmamux_c12cr
    }
    #[doc = "0x34 - DMAMUX request line multiplexer channel 13 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c13cr(&self) -> &DMAMUX_C13CR {
        &self.dmamux_c13cr
    }
    #[doc = "0x38 - DMAMUX request line multiplexer channel 14 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c14cr(&self) -> &DMAMUX_C14CR {
        &self.dmamux_c14cr
    }
    #[doc = "0x3c - DMAMUX request line multiplexer channel 15 configuration register"]
    #[inline(always)]
    pub const fn dmamux_c15cr(&self) -> &DMAMUX_C15CR {
        &self.dmamux_c15cr
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn dmamux_csr(&self) -> &DMAMUX_CSR {
        &self.dmamux_csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_cfr(&self) -> &DMAMUX_CFR {
        &self.dmamux_cfr
    }
    #[doc = "0x100 - DMAMUX request generator channel 0 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg0cr(&self) -> &DMAMUX_RG0CR {
        &self.dmamux_rg0cr
    }
    #[doc = "0x104 - DMAMUX request generator channel 1 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg1cr(&self) -> &DMAMUX_RG1CR {
        &self.dmamux_rg1cr
    }
    #[doc = "0x108 - DMAMUX request generator channel 2 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg2cr(&self) -> &DMAMUX_RG2CR {
        &self.dmamux_rg2cr
    }
    #[doc = "0x10c - DMAMUX request generator channel 3 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg3cr(&self) -> &DMAMUX_RG3CR {
        &self.dmamux_rg3cr
    }
    #[doc = "0x110 - DMAMUX request generator channel 4 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg4cr(&self) -> &DMAMUX_RG4CR {
        &self.dmamux_rg4cr
    }
    #[doc = "0x114 - DMAMUX request generator channel 5 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg5cr(&self) -> &DMAMUX_RG5CR {
        &self.dmamux_rg5cr
    }
    #[doc = "0x118 - DMAMUX request generator channel 6 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg6cr(&self) -> &DMAMUX_RG6CR {
        &self.dmamux_rg6cr
    }
    #[doc = "0x11c - DMAMUX request generator channel 7 configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg7cr(&self) -> &DMAMUX_RG7CR {
        &self.dmamux_rg7cr
    }
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    #[inline(always)]
    pub const fn dmamux_rgsr(&self) -> &DMAMUX_RGSR {
        &self.dmamux_rgsr
    }
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_rgcfr(&self) -> &DMAMUX_RGCFR {
        &self.dmamux_rgcfr
    }
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    #[inline(always)]
    pub const fn dmamux_hwcfgr2(&self) -> &DMAMUX_HWCFGR2 {
        &self.dmamux_hwcfgr2
    }
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    #[inline(always)]
    pub const fn dmamux_hwcfgr1(&self) -> &DMAMUX_HWCFGR1 {
        &self.dmamux_hwcfgr1
    }
    #[doc = "0x3f4 - This register identifies the IP version."]
    #[inline(always)]
    pub const fn dmamux_verr(&self) -> &DMAMUX_VERR {
        &self.dmamux_verr
    }
    #[doc = "0x3f8 - This register identifies the IP."]
    #[inline(always)]
    pub const fn dmamux_ipidr(&self) -> &DMAMUX_IPIDR {
        &self.dmamux_ipidr
    }
    #[doc = "0x3fc - DMAMUX size identification register"]
    #[inline(always)]
    pub const fn dmamux_sidr(&self) -> &DMAMUX_SIDR {
        &self.dmamux_sidr
    }
}
#[doc = "DMAMUX_C0CR (rw) register accessor: DMAMUX request line multiplexer channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c0cr`]
module"]
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CRrs>;
#[doc = "DMAMUX request line multiplexer channel 0 configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX_C1CR (rw) register accessor: DMAMUX request line multiplexer channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c1cr`]
module"]
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CRrs>;
#[doc = "DMAMUX request line multiplexer channel 1 configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX_C2CR (rw) register accessor: DMAMUX request line multiplexer channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c2cr`]
module"]
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CRrs>;
#[doc = "DMAMUX request line multiplexer channel 2 configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX_C3CR (rw) register accessor: DMAMUX request line multiplexer channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c3cr`]
module"]
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CRrs>;
#[doc = "DMAMUX request line multiplexer channel 3 configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX_C4CR (rw) register accessor: DMAMUX request line multiplexer channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c4cr`]
module"]
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CRrs>;
#[doc = "DMAMUX request line multiplexer channel 4 configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX_C5CR (rw) register accessor: DMAMUX request line multiplexer channel 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c5cr`]
module"]
pub type DMAMUX_C5CR = crate::Reg<dmamux_c5cr::DMAMUX_C5CRrs>;
#[doc = "DMAMUX request line multiplexer channel 5 configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX_C6CR (rw) register accessor: DMAMUX request line multiplexer channel 6 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c6cr`]
module"]
pub type DMAMUX_C6CR = crate::Reg<dmamux_c6cr::DMAMUX_C6CRrs>;
#[doc = "DMAMUX request line multiplexer channel 6 configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX_C7CR (rw) register accessor: DMAMUX request line multiplexer channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c7cr`]
module"]
pub type DMAMUX_C7CR = crate::Reg<dmamux_c7cr::DMAMUX_C7CRrs>;
#[doc = "DMAMUX request line multiplexer channel 7 configuration register"]
pub mod dmamux_c7cr;
#[doc = "DMAMUX_C8CR (rw) register accessor: DMAMUX request line multiplexer channel 8 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c8cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c8cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c8cr`]
module"]
pub type DMAMUX_C8CR = crate::Reg<dmamux_c8cr::DMAMUX_C8CRrs>;
#[doc = "DMAMUX request line multiplexer channel 8 configuration register"]
pub mod dmamux_c8cr;
#[doc = "DMAMUX_C9CR (rw) register accessor: DMAMUX request line multiplexer channel 9 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c9cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c9cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c9cr`]
module"]
pub type DMAMUX_C9CR = crate::Reg<dmamux_c9cr::DMAMUX_C9CRrs>;
#[doc = "DMAMUX request line multiplexer channel 9 configuration register"]
pub mod dmamux_c9cr;
#[doc = "DMAMUX_C10CR (rw) register accessor: DMAMUX request line multiplexer channel 10 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c10cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c10cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c10cr`]
module"]
pub type DMAMUX_C10CR = crate::Reg<dmamux_c10cr::DMAMUX_C10CRrs>;
#[doc = "DMAMUX request line multiplexer channel 10 configuration register"]
pub mod dmamux_c10cr;
#[doc = "DMAMUX_C11CR (rw) register accessor: DMAMUX request line multiplexer channel 11 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c11cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c11cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c11cr`]
module"]
pub type DMAMUX_C11CR = crate::Reg<dmamux_c11cr::DMAMUX_C11CRrs>;
#[doc = "DMAMUX request line multiplexer channel 11 configuration register"]
pub mod dmamux_c11cr;
#[doc = "DMAMUX_C12CR (rw) register accessor: DMAMUX request line multiplexer channel 12 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c12cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c12cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c12cr`]
module"]
pub type DMAMUX_C12CR = crate::Reg<dmamux_c12cr::DMAMUX_C12CRrs>;
#[doc = "DMAMUX request line multiplexer channel 12 configuration register"]
pub mod dmamux_c12cr;
#[doc = "DMAMUX_C13CR (rw) register accessor: DMAMUX request line multiplexer channel 13 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c13cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c13cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c13cr`]
module"]
pub type DMAMUX_C13CR = crate::Reg<dmamux_c13cr::DMAMUX_C13CRrs>;
#[doc = "DMAMUX request line multiplexer channel 13 configuration register"]
pub mod dmamux_c13cr;
#[doc = "DMAMUX_C14CR (rw) register accessor: DMAMUX request line multiplexer channel 14 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c14cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c14cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c14cr`]
module"]
pub type DMAMUX_C14CR = crate::Reg<dmamux_c14cr::DMAMUX_C14CRrs>;
#[doc = "DMAMUX request line multiplexer channel 14 configuration register"]
pub mod dmamux_c14cr;
#[doc = "DMAMUX_C15CR (rw) register accessor: DMAMUX request line multiplexer channel 15 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c15cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c15cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c15cr`]
module"]
pub type DMAMUX_C15CR = crate::Reg<dmamux_c15cr::DMAMUX_C15CRrs>;
#[doc = "DMAMUX request line multiplexer channel 15 configuration register"]
pub mod dmamux_c15cr;
#[doc = "DMAMUX_CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_csr`]
module"]
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSRrs>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX_CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_cfr`]
module"]
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFRrs>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX_RG0CR (rw) register accessor: DMAMUX request generator channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg0cr`]
module"]
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CRrs>;
#[doc = "DMAMUX request generator channel 0 configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX_RG1CR (rw) register accessor: DMAMUX request generator channel 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg1cr`]
module"]
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CRrs>;
#[doc = "DMAMUX request generator channel 1 configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX_RG2CR (rw) register accessor: DMAMUX request generator channel 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg2cr`]
module"]
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CRrs>;
#[doc = "DMAMUX request generator channel 2 configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX_RG3CR (rw) register accessor: DMAMUX request generator channel 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg3cr`]
module"]
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CRrs>;
#[doc = "DMAMUX request generator channel 3 configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX_RG4CR (rw) register accessor: DMAMUX request generator channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg4cr`]
module"]
pub type DMAMUX_RG4CR = crate::Reg<dmamux_rg4cr::DMAMUX_RG4CRrs>;
#[doc = "DMAMUX request generator channel 4 configuration register"]
pub mod dmamux_rg4cr;
#[doc = "DMAMUX_RG5CR (rw) register accessor: DMAMUX request generator channel 5 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg5cr`]
module"]
pub type DMAMUX_RG5CR = crate::Reg<dmamux_rg5cr::DMAMUX_RG5CRrs>;
#[doc = "DMAMUX request generator channel 5 configuration register"]
pub mod dmamux_rg5cr;
#[doc = "DMAMUX_RG6CR (rw) register accessor: DMAMUX request generator channel 6 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg6cr`]
module"]
pub type DMAMUX_RG6CR = crate::Reg<dmamux_rg6cr::DMAMUX_RG6CRrs>;
#[doc = "DMAMUX request generator channel 6 configuration register"]
pub mod dmamux_rg6cr;
#[doc = "DMAMUX_RG7CR (rw) register accessor: DMAMUX request generator channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg7cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg7cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg7cr`]
module"]
pub type DMAMUX_RG7CR = crate::Reg<dmamux_rg7cr::DMAMUX_RG7CRrs>;
#[doc = "DMAMUX request generator channel 7 configuration register"]
pub mod dmamux_rg7cr;
#[doc = "DMAMUX_RGSR (r) register accessor: DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgsr`]
module"]
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSRrs>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX_RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgcfr`]
module"]
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFRrs>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
#[doc = "DMAMUX_HWCFGR2 (r) register accessor: DMAMUX hardware configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_hwcfgr2`]
module"]
pub type DMAMUX_HWCFGR2 = crate::Reg<dmamux_hwcfgr2::DMAMUX_HWCFGR2rs>;
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod dmamux_hwcfgr2;
#[doc = "DMAMUX_HWCFGR1 (r) register accessor: DMAMUX hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_hwcfgr1`]
module"]
pub type DMAMUX_HWCFGR1 = crate::Reg<dmamux_hwcfgr1::DMAMUX_HWCFGR1rs>;
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod dmamux_hwcfgr1;
#[doc = "DMAMUX_VERR (r) register accessor: This register identifies the IP version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_verr`]
module"]
pub type DMAMUX_VERR = crate::Reg<dmamux_verr::DMAMUX_VERRrs>;
#[doc = "This register identifies the IP version."]
pub mod dmamux_verr;
#[doc = "DMAMUX_IPIDR (r) register accessor: This register identifies the IP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_ipidr`]
module"]
pub type DMAMUX_IPIDR = crate::Reg<dmamux_ipidr::DMAMUX_IPIDRrs>;
#[doc = "This register identifies the IP."]
pub mod dmamux_ipidr;
#[doc = "DMAMUX_SIDR (r) register accessor: DMAMUX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_sidr`]
module"]
pub type DMAMUX_SIDR = crate::Reg<dmamux_sidr::DMAMUX_SIDRrs>;
#[doc = "DMAMUX size identification register"]
pub mod dmamux_sidr;
