#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim7_cr1: TIM7_CR1,
    _reserved1: [u8; 0x02],
    tim7_cr2: TIM7_CR2,
    _reserved2: [u8; 0x06],
    tim7_dier: TIM7_DIER,
    _reserved3: [u8; 0x02],
    tim7_sr: TIM7_SR,
    _reserved4: [u8; 0x02],
    tim7_egr: TIM7_EGR,
    _reserved5: [u8; 0x0e],
    tim7_cnt: TIM7_CNT,
    tim7_psc: TIM7_PSC,
    _reserved7: [u8; 0x02],
    tim7_arr: TIM7_ARR,
}
impl RegisterBlock {
    ///0x00 - TIM7 control register 1
    #[inline(always)]
    pub const fn tim7_cr1(&self) -> &TIM7_CR1 {
        &self.tim7_cr1
    }
    ///0x04 - TIM7 control register 2
    #[inline(always)]
    pub const fn tim7_cr2(&self) -> &TIM7_CR2 {
        &self.tim7_cr2
    }
    ///0x0c - TIM7 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn tim7_dier(&self) -> &TIM7_DIER {
        &self.tim7_dier
    }
    ///0x10 - TIM7 status register
    #[inline(always)]
    pub const fn tim7_sr(&self) -> &TIM7_SR {
        &self.tim7_sr
    }
    ///0x14 - TIM7 event generation register
    #[inline(always)]
    pub const fn tim7_egr(&self) -> &TIM7_EGR {
        &self.tim7_egr
    }
    ///0x24 - TIM7 counter
    #[inline(always)]
    pub const fn tim7_cnt(&self) -> &TIM7_CNT {
        &self.tim7_cnt
    }
    ///0x28 - TIM7 prescaler
    #[inline(always)]
    pub const fn tim7_psc(&self) -> &TIM7_PSC {
        &self.tim7_psc
    }
    ///0x2c - TIM7 auto-reload register
    #[inline(always)]
    pub const fn tim7_arr(&self) -> &TIM7_ARR {
        &self.tim7_arr
    }
}
/**TIM7_CR1 (rw) register accessor: TIM7 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim7_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_CR1)

For information about available fields see [`mod@tim7_cr1`]
module*/
pub type TIM7_CR1 = crate::Reg<tim7_cr1::TIM7_CR1rs>;
///TIM7 control register 1
pub mod tim7_cr1;
/**TIM7_CR2 (rw) register accessor: TIM7 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim7_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_CR2)

For information about available fields see [`mod@tim7_cr2`]
module*/
pub type TIM7_CR2 = crate::Reg<tim7_cr2::TIM7_CR2rs>;
///TIM7 control register 2
pub mod tim7_cr2;
/**TIM7_DIER (rw) register accessor: TIM7 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim7_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_DIER)

For information about available fields see [`mod@tim7_dier`]
module*/
pub type TIM7_DIER = crate::Reg<tim7_dier::TIM7_DIERrs>;
///TIM7 DMA/Interrupt enable register
pub mod tim7_dier;
/**TIM7_SR (rw) register accessor: TIM7 status register

You can [`read`](crate::Reg::read) this register and get [`tim7_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_SR)

For information about available fields see [`mod@tim7_sr`]
module*/
pub type TIM7_SR = crate::Reg<tim7_sr::TIM7_SRrs>;
///TIM7 status register
pub mod tim7_sr;
/**TIM7_EGR (w) register accessor: TIM7 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_EGR)

For information about available fields see [`mod@tim7_egr`]
module*/
pub type TIM7_EGR = crate::Reg<tim7_egr::TIM7_EGRrs>;
///TIM7 event generation register
pub mod tim7_egr;
/**TIM7_CNT (rw) register accessor: TIM7 counter

You can [`read`](crate::Reg::read) this register and get [`tim7_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_CNT)

For information about available fields see [`mod@tim7_cnt`]
module*/
pub type TIM7_CNT = crate::Reg<tim7_cnt::TIM7_CNTrs>;
///TIM7 counter
pub mod tim7_cnt;
/**TIM7_PSC (rw) register accessor: TIM7 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim7_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_PSC)

For information about available fields see [`mod@tim7_psc`]
module*/
pub type TIM7_PSC = crate::Reg<tim7_psc::TIM7_PSCrs>;
///TIM7 prescaler
pub mod tim7_psc;
/**TIM7_ARR (rw) register accessor: TIM7 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim7_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_ARR)

For information about available fields see [`mod@tim7_arr`]
module*/
pub type TIM7_ARR = crate::Reg<tim7_arr::TIM7_ARRrs>;
///TIM7 auto-reload register
pub mod tim7_arr;
