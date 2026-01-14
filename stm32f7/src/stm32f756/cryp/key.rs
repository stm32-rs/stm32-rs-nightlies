#[repr(C)]
#[derive(Debug)]
///Cluster KEY%s, containing K?LR, K?RR
pub struct KEY {
    klr: KLR,
    krr: KRR,
}
impl KEY {
    ///0x00 - key registers
    #[inline(always)]
    pub const fn klr(&self) -> &KLR {
        &self.klr
    }
    ///0x04 - key registers
    #[inline(always)]
    pub const fn krr(&self) -> &KRR {
        &self.krr
    }
}
/**KLR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`klr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@klr`] module*/
pub type KLR = crate::Reg<klr::KLRrs>;
///key registers
pub mod klr;
/**KRR (w) register accessor: key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@krr`] module*/
pub type KRR = crate::Reg<krr::KRRrs>;
///key registers
pub mod krr;
