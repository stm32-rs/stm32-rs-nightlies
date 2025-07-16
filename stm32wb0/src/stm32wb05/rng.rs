#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    val: VAL,
}
impl RegisterBlock {
    ///0x00 - RNG_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RNG_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - RNG_VAL register
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
}
/**CR (rw) register accessor: RNG_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RNG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RNG_CR register
pub mod cr;
/**SR (rw) register accessor: RNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RNG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RNG_SR register
pub mod sr;
/**VAL (r) register accessor: RNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RNG:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///RNG_VAL register
pub mod val;
