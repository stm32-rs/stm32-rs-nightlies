#[repr(C)]
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub struct KEY {
    klr: KLR,
    krr: KRR,
}
impl KEY {
    #[doc = "0x00 - key registers"]
    #[inline(always)]
    pub const fn klr(&self) -> &KLR {
        &self.klr
    }
    #[doc = "0x04 - key registers"]
    #[inline(always)]
    pub const fn krr(&self) -> &KRR {
        &self.krr
    }
}
#[doc = "KLR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`klr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@klr`]
module"]
pub type KLR = crate::Reg<klr::KLRrs>;
#[doc = "key registers"]
pub mod klr;
#[doc = "KRR (w) register accessor: key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@krr`]
module"]
pub type KRR = crate::Reg<krr::KRRrs>;
#[doc = "key registers"]
pub mod krr;
