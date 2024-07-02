#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    lpgpio_moder: LPGPIO_MODER,
    _reserved1: [u8; 0x0c],
    lpgpio_idr: LPGPIO_IDR,
    lpgpio_odr: LPGPIO_ODR,
    lpgpio_bsrr: LPGPIO_BSRR,
    _reserved4: [u8; 0x0c],
    lpgpio_brr: LPGPIO_BRR,
}
impl RegisterBlock {
    ///0x00 - LPGPIO port mode register
    #[inline(always)]
    pub const fn lpgpio_moder(&self) -> &LPGPIO_MODER {
        &self.lpgpio_moder
    }
    ///0x10 - LPGPIO port input data register
    #[inline(always)]
    pub const fn lpgpio_idr(&self) -> &LPGPIO_IDR {
        &self.lpgpio_idr
    }
    ///0x14 - LPGPIO port output data register
    #[inline(always)]
    pub const fn lpgpio_odr(&self) -> &LPGPIO_ODR {
        &self.lpgpio_odr
    }
    ///0x18 - LPGPIO port bit set/reset register
    #[inline(always)]
    pub const fn lpgpio_bsrr(&self) -> &LPGPIO_BSRR {
        &self.lpgpio_bsrr
    }
    ///0x28 - LPGPIO port bit reset register
    #[inline(always)]
    pub const fn lpgpio_brr(&self) -> &LPGPIO_BRR {
        &self.lpgpio_brr
    }
}
/**LPGPIO_MODER (rw) register accessor: LPGPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpgpio_moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_MODER)

For information about available fields see [`mod@lpgpio_moder`]
module*/
pub type LPGPIO_MODER = crate::Reg<lpgpio_moder::LPGPIO_MODERrs>;
///LPGPIO port mode register
pub mod lpgpio_moder;
/**LPGPIO_IDR (r) register accessor: LPGPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_IDR)

For information about available fields see [`mod@lpgpio_idr`]
module*/
pub type LPGPIO_IDR = crate::Reg<lpgpio_idr::LPGPIO_IDRrs>;
///LPGPIO port input data register
pub mod lpgpio_idr;
/**LPGPIO_ODR (rw) register accessor: LPGPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpgpio_odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_ODR)

For information about available fields see [`mod@lpgpio_odr`]
module*/
pub type LPGPIO_ODR = crate::Reg<lpgpio_odr::LPGPIO_ODRrs>;
///LPGPIO port output data register
pub mod lpgpio_odr;
/**LPGPIO_BSRR (w) register accessor: LPGPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpgpio_bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_BSRR)

For information about available fields see [`mod@lpgpio_bsrr`]
module*/
pub type LPGPIO_BSRR = crate::Reg<lpgpio_bsrr::LPGPIO_BSRRrs>;
///LPGPIO port bit set/reset register
pub mod lpgpio_bsrr;
/**LPGPIO_BRR (r) register accessor: LPGPIO port bit reset register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_brr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_BRR)

For information about available fields see [`mod@lpgpio_brr`]
module*/
pub type LPGPIO_BRR = crate::Reg<lpgpio_brr::LPGPIO_BRRrs>;
///LPGPIO port bit reset register
pub mod lpgpio_brr;
