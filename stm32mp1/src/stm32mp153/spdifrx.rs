#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spdifrx_cr: SPDIFRX_CR,
    spdifrx_imr: SPDIFRX_IMR,
    spdifrx_sr: SPDIFRX_SR,
    spdifrx_ifcr: SPDIFRX_IFCR,
    spdifrx_fmt0_dr: SPDIFRX_FMT0_DR,
    spdifrx_csr: SPDIFRX_CSR,
    spdifrx_dir: SPDIFRX_DIR,
    _reserved7: [u8; 0x03d8],
    spdifrx_verr: SPDIFRX_VERR,
    spdifrx_ipidr: SPDIFRX_IPIDR,
    spdifrx_sidr: SPDIFRX_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn spdifrx_cr(&self) -> &SPDIFRX_CR {
        &self.spdifrx_cr
    }
    #[doc = "0x04 - Interrupt mask register"]
    #[inline(always)]
    pub const fn spdifrx_imr(&self) -> &SPDIFRX_IMR {
        &self.spdifrx_imr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn spdifrx_sr(&self) -> &SPDIFRX_SR {
        &self.spdifrx_sr
    }
    #[doc = "0x0c - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn spdifrx_ifcr(&self) -> &SPDIFRX_IFCR {
        &self.spdifrx_ifcr
    }
    #[doc = "0x10 - This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
    #[inline(always)]
    pub const fn spdifrx_fmt0_dr(&self) -> &SPDIFRX_FMT0_DR {
        &self.spdifrx_fmt0_dr
    }
    #[doc = "0x14 - Channel status register"]
    #[inline(always)]
    pub const fn spdifrx_csr(&self) -> &SPDIFRX_CSR {
        &self.spdifrx_csr
    }
    #[doc = "0x18 - Debug information register"]
    #[inline(always)]
    pub const fn spdifrx_dir(&self) -> &SPDIFRX_DIR {
        &self.spdifrx_dir
    }
    #[doc = "0x3f4 - SPDIFRX version register"]
    #[inline(always)]
    pub const fn spdifrx_verr(&self) -> &SPDIFRX_VERR {
        &self.spdifrx_verr
    }
    #[doc = "0x3f8 - SPDIFRX identification register"]
    #[inline(always)]
    pub const fn spdifrx_ipidr(&self) -> &SPDIFRX_IPIDR {
        &self.spdifrx_ipidr
    }
    #[doc = "0x3fc - SPDIFRX size identification register"]
    #[inline(always)]
    pub const fn spdifrx_sidr(&self) -> &SPDIFRX_SIDR {
        &self.spdifrx_sidr
    }
}
#[doc = "SPDIFRX_CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_cr`]
module"]
pub type SPDIFRX_CR = crate::Reg<spdifrx_cr::SPDIFRX_CRrs>;
#[doc = "Control register"]
pub mod spdifrx_cr;
#[doc = "SPDIFRX_IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_imr`]
module"]
pub type SPDIFRX_IMR = crate::Reg<spdifrx_imr::SPDIFRX_IMRrs>;
#[doc = "Interrupt mask register"]
pub mod spdifrx_imr;
#[doc = "SPDIFRX_SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_sr`]
module"]
pub type SPDIFRX_SR = crate::Reg<spdifrx_sr::SPDIFRX_SRrs>;
#[doc = "Status register"]
pub mod spdifrx_sr;
#[doc = "SPDIFRX_IFCR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_ifcr`]
module"]
pub type SPDIFRX_IFCR = crate::Reg<spdifrx_ifcr::SPDIFRX_IFCRrs>;
#[doc = "Interrupt flag clear register"]
pub mod spdifrx_ifcr;
#[doc = "SPDIFRX_FMT0_DR (r) register accessor: This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_fmt0_dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_fmt0_dr`]
module"]
pub type SPDIFRX_FMT0_DR = crate::Reg<spdifrx_fmt0_dr::SPDIFRX_FMT0_DRrs>;
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
pub mod spdifrx_fmt0_dr;
#[doc = "SPDIFRX_CSR (r) register accessor: Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_csr`]
module"]
pub type SPDIFRX_CSR = crate::Reg<spdifrx_csr::SPDIFRX_CSRrs>;
#[doc = "Channel status register"]
pub mod spdifrx_csr;
#[doc = "SPDIFRX_DIR (r) register accessor: Debug information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_dir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_dir`]
module"]
pub type SPDIFRX_DIR = crate::Reg<spdifrx_dir::SPDIFRX_DIRrs>;
#[doc = "Debug information register"]
pub mod spdifrx_dir;
#[doc = "SPDIFRX_VERR (r) register accessor: SPDIFRX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_verr`]
module"]
pub type SPDIFRX_VERR = crate::Reg<spdifrx_verr::SPDIFRX_VERRrs>;
#[doc = "SPDIFRX version register"]
pub mod spdifrx_verr;
#[doc = "SPDIFRX_IPIDR (r) register accessor: SPDIFRX identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_ipidr`]
module"]
pub type SPDIFRX_IPIDR = crate::Reg<spdifrx_ipidr::SPDIFRX_IPIDRrs>;
#[doc = "SPDIFRX identification register"]
pub mod spdifrx_ipidr;
#[doc = "SPDIFRX_SIDR (r) register accessor: SPDIFRX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdifrx_sidr`]
module"]
pub type SPDIFRX_SIDR = crate::Reg<spdifrx_sidr::SPDIFRX_SIDRrs>;
#[doc = "SPDIFRX size identification register"]
pub mod spdifrx_sidr;
