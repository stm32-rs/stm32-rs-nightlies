#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    csr1: CSR1,
    cr2: CR2,
    csr2: CSR2,
}
impl RegisterBlock {
    #[doc = "0x00 - power control register"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - power control/status register"]
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    #[doc = "0x08 - power control register"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x0c - power control/status register"]
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
}
#[doc = "CR1 (rw) register accessor: power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "power control register"]
pub mod cr1;
#[doc = "CSR1 (rw) register accessor: power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
#[doc = "power control/status register"]
pub mod csr1;
#[doc = "CR2 (rw) register accessor: power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "power control register"]
pub mod cr2;
#[doc = "CSR2 (rw) register accessor: power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
pub type CSR2 = crate::Reg<csr2::CSR2rs>;
#[doc = "power control/status register"]
pub mod csr2;
