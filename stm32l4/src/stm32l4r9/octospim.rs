#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    p1cr: P1CR,
    p2cr: P2CR,
}
impl RegisterBlock {
    #[doc = "0x04 - OctoSPI IO Manager Port 1 Configuration Register"]
    #[inline(always)]
    pub const fn p1cr(&self) -> &P1CR {
        &self.p1cr
    }
    #[doc = "0x08 - OctoSPI IO Manager Port 2 Configuration Register"]
    #[inline(always)]
    pub const fn p2cr(&self) -> &P2CR {
        &self.p2cr
    }
}
#[doc = "P1CR (rw) register accessor: OctoSPI IO Manager Port 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1cr`]
module"]
pub type P1CR = crate::Reg<p1cr::P1CRrs>;
#[doc = "OctoSPI IO Manager Port 1 Configuration Register"]
pub mod p1cr;
#[doc = "P2CR (rw) register accessor: OctoSPI IO Manager Port 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2cr`]
module"]
pub type P2CR = crate::Reg<p2cr::P2CRrs>;
#[doc = "OctoSPI IO Manager Port 2 Configuration Register"]
pub mod p2cr;
