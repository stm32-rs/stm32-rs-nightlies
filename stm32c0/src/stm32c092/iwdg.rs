#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    iwdg_kr: IWDG_KR,
    iwdg_pr: IWDG_PR,
    iwdg_rlr: IWDG_RLR,
    iwdg_sr: IWDG_SR,
    iwdg_winr: IWDG_WINR,
}
impl RegisterBlock {
    ///0x00 - IWDG key register
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IWDG_KR {
        &self.iwdg_kr
    }
    ///0x04 - IWDG prescaler register
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IWDG_PR {
        &self.iwdg_pr
    }
    ///0x08 - IWDG reload register
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IWDG_RLR {
        &self.iwdg_rlr
    }
    ///0x0c - IWDG status register
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IWDG_SR {
        &self.iwdg_sr
    }
    ///0x10 - IWDG window register
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IWDG_WINR {
        &self.iwdg_winr
    }
}
/**IWDG_KR (w) register accessor: IWDG key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#IWDG:IWDG_KR)

For information about available fields see [`mod@iwdg_kr`] module*/
pub type IWDG_KR = crate::Reg<iwdg_kr::IWDG_KRrs>;
///IWDG key register
pub mod iwdg_kr;
/**IWDG_PR (rw) register accessor: IWDG prescaler register

You can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#IWDG:IWDG_PR)

For information about available fields see [`mod@iwdg_pr`] module*/
pub type IWDG_PR = crate::Reg<iwdg_pr::IWDG_PRrs>;
///IWDG prescaler register
pub mod iwdg_pr;
/**IWDG_RLR (rw) register accessor: IWDG reload register

You can [`read`](crate::Reg::read) this register and get [`iwdg_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#IWDG:IWDG_RLR)

For information about available fields see [`mod@iwdg_rlr`] module*/
pub type IWDG_RLR = crate::Reg<iwdg_rlr::IWDG_RLRrs>;
///IWDG reload register
pub mod iwdg_rlr;
/**IWDG_SR (r) register accessor: IWDG status register

You can [`read`](crate::Reg::read) this register and get [`iwdg_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#IWDG:IWDG_SR)

For information about available fields see [`mod@iwdg_sr`] module*/
pub type IWDG_SR = crate::Reg<iwdg_sr::IWDG_SRrs>;
///IWDG status register
pub mod iwdg_sr;
/**IWDG_WINR (rw) register accessor: IWDG window register

You can [`read`](crate::Reg::read) this register and get [`iwdg_winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#IWDG:IWDG_WINR)

For information about available fields see [`mod@iwdg_winr`] module*/
pub type IWDG_WINR = crate::Reg<iwdg_winr::IWDG_WINRrs>;
///IWDG window register
pub mod iwdg_winr;
