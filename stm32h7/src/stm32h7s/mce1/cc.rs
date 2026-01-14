#[repr(C)]
#[derive(Debug)]
///Cipher context cluster
pub struct CC {
    cccfgr: CCCFGR,
    ccnr0: CCNR0,
    ccnr1: CCNR1,
    cckeyr0: CCKEYR0,
    cckeyr1: CCKEYR1,
    cckeyr2: CCKEYR2,
    cckeyr3: CCKEYR3,
    _reserved_end: [u8; 0x14],
}
impl CC {
    ///0x00 - MCE cipher context 1 configuration register
    #[inline(always)]
    pub const fn cccfgr(&self) -> &CCCFGR {
        &self.cccfgr
    }
    ///0x04 - MCE cipher context 1 nonce register 0
    #[inline(always)]
    pub const fn ccnr0(&self) -> &CCNR0 {
        &self.ccnr0
    }
    ///0x08 - MCE cipher context 1 nonce register 1
    #[inline(always)]
    pub const fn ccnr1(&self) -> &CCNR1 {
        &self.ccnr1
    }
    ///0x0c - MCE cipher context 1 key register 0
    #[inline(always)]
    pub const fn cckeyr0(&self) -> &CCKEYR0 {
        &self.cckeyr0
    }
    ///0x10 - MCE cipher context 1 key register 1
    #[inline(always)]
    pub const fn cckeyr1(&self) -> &CCKEYR1 {
        &self.cckeyr1
    }
    ///0x14 - MCE cipher context 1 key register 2
    #[inline(always)]
    pub const fn cckeyr2(&self) -> &CCKEYR2 {
        &self.cckeyr2
    }
    ///0x18 - MCE cipher context 1 key register 3
    #[inline(always)]
    pub const fn cckeyr3(&self) -> &CCKEYR3 {
        &self.cckeyr3
    }
}
/**CCCFGR (rw) register accessor: MCE cipher context 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`cccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cccfgr`] module*/
pub type CCCFGR = crate::Reg<cccfgr::CCCFGRrs>;
///MCE cipher context 1 configuration register
pub mod cccfgr;
/**CCNR0 (rw) register accessor: MCE cipher context 1 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`ccnr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccnr0`] module*/
pub type CCNR0 = crate::Reg<ccnr0::CCNR0rs>;
///MCE cipher context 1 nonce register 0
pub mod ccnr0;
/**CCNR1 (rw) register accessor: MCE cipher context 1 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`ccnr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccnr1`] module*/
pub type CCNR1 = crate::Reg<ccnr1::CCNR1rs>;
///MCE cipher context 1 nonce register 1
pub mod ccnr1;
/**CCKEYR0 (w) register accessor: MCE cipher context 1 key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cckeyr0`] module*/
pub type CCKEYR0 = crate::Reg<cckeyr0::CCKEYR0rs>;
///MCE cipher context 1 key register 0
pub mod cckeyr0;
/**CCKEYR1 (w) register accessor: MCE cipher context 1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cckeyr1`] module*/
pub type CCKEYR1 = crate::Reg<cckeyr1::CCKEYR1rs>;
///MCE cipher context 1 key register 1
pub mod cckeyr1;
/**CCKEYR2 (w) register accessor: MCE cipher context 1 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cckeyr2`] module*/
pub type CCKEYR2 = crate::Reg<cckeyr2::CCKEYR2rs>;
///MCE cipher context 1 key register 2
pub mod cckeyr2;
/**CCKEYR3 (w) register accessor: MCE cipher context 1 key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cckeyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cckeyr3`] module*/
pub type CCKEYR3 = crate::Reg<cckeyr3::CCKEYR3rs>;
///MCE cipher context 1 key register 3
pub mod cckeyr3;
