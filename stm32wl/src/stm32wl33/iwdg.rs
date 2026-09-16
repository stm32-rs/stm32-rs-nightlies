#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    kr: KR,
    pr: PR,
    rlr: RLR,
    sr: SR,
    winr: WINR,
}
impl RegisterBlock {
    ///0x00 - IWDG_KR register
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    ///0x04 - IWDG_PR register
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    ///0x08 - IWDG_RLR register
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
    ///0x0c - IWDG_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x10 - IWDG_WINR register
    #[inline(always)]
    pub const fn winr(&self) -> &WINR {
        &self.winr
    }
}
/**KR (rw) register accessor: IWDG_KR register

You can [`read`](crate::Reg::read) this register and get [`kr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:KR)

For information about available fields see [`mod@kr`] module*/
pub type KR = crate::Reg<kr::KRrs>;
///IWDG_KR register
pub mod kr;
/**PR (rw) register accessor: IWDG_PR register

You can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:PR)

For information about available fields see [`mod@pr`] module*/
pub type PR = crate::Reg<pr::PRrs>;
///IWDG_PR register
pub mod pr;
/**RLR (rw) register accessor: IWDG_RLR register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:RLR)

For information about available fields see [`mod@rlr`] module*/
pub type RLR = crate::Reg<rlr::RLRrs>;
///IWDG_RLR register
pub mod rlr;
/**SR (r) register accessor: IWDG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///IWDG_SR register
pub mod sr;
/**WINR (rw) register accessor: IWDG_WINR register

You can [`read`](crate::Reg::read) this register and get [`winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:WINR)

For information about available fields see [`mod@winr`] module*/
pub type WINR = crate::Reg<winr::WINRrs>;
///IWDG_WINR register
pub mod winr;
