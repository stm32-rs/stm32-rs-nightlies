#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    p1cr: P1CR,
    p2cr: P2CR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - OCTOSPI I/O manager Port 1 configuration register"]
    #[inline(always)]
    pub const fn p1cr(&self) -> &P1CR {
        &self.p1cr
    }
    #[doc = "0x08 - OCTOSPI I/O manager Port 2 configuration register"]
    #[inline(always)]
    pub const fn p2cr(&self) -> &P2CR {
        &self.p2cr
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "P1CR (rw) register accessor: OCTOSPI I/O manager Port 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1cr`]
module"]
pub type P1CR = crate::Reg<p1cr::P1CRrs>;
#[doc = "OCTOSPI I/O manager Port 1 configuration register"]
pub mod p1cr;
#[doc = "P2CR (rw) register accessor: OCTOSPI I/O manager Port 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2cr`]
module"]
pub type P2CR = crate::Reg<p2cr::P2CRrs>;
#[doc = "OCTOSPI I/O manager Port 2 configuration register"]
pub mod p2cr;
