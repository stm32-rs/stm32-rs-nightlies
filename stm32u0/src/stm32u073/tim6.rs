#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim6_cr1: TIM6_CR1,
    _reserved1: [u8; 0x02],
    tim6_cr2: TIM6_CR2,
    _reserved2: [u8; 0x06],
    tim6_dier: TIM6_DIER,
    _reserved3: [u8; 0x02],
    tim6_sr: TIM6_SR,
    _reserved4: [u8; 0x02],
    tim6_egr: TIM6_EGR,
    _reserved5: [u8; 0x0e],
    tim6_cnt: TIM6_CNT,
    tim6_psc: TIM6_PSC,
    _reserved7: [u8; 0x02],
    tim6_arr: TIM6_ARR,
}
impl RegisterBlock {
    ///0x00 - TIM6 control register 1
    #[inline(always)]
    pub const fn tim6_cr1(&self) -> &TIM6_CR1 {
        &self.tim6_cr1
    }
    ///0x04 - TIM6 control register 2
    #[inline(always)]
    pub const fn tim6_cr2(&self) -> &TIM6_CR2 {
        &self.tim6_cr2
    }
    ///0x0c - TIM6 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn tim6_dier(&self) -> &TIM6_DIER {
        &self.tim6_dier
    }
    ///0x10 - TIM6 status register
    #[inline(always)]
    pub const fn tim6_sr(&self) -> &TIM6_SR {
        &self.tim6_sr
    }
    ///0x14 - TIM6 event generation register
    #[inline(always)]
    pub const fn tim6_egr(&self) -> &TIM6_EGR {
        &self.tim6_egr
    }
    ///0x24 - TIM6 counter
    #[inline(always)]
    pub const fn tim6_cnt(&self) -> &TIM6_CNT {
        &self.tim6_cnt
    }
    ///0x28 - TIM6 prescaler
    #[inline(always)]
    pub const fn tim6_psc(&self) -> &TIM6_PSC {
        &self.tim6_psc
    }
    ///0x2c - TIM6 auto-reload register
    #[inline(always)]
    pub const fn tim6_arr(&self) -> &TIM6_ARR {
        &self.tim6_arr
    }
}
/**TIM6_CR1 (rw) register accessor: TIM6 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim6_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_CR1)

For information about available fields see [`mod@tim6_cr1`]
module*/
pub type TIM6_CR1 = crate::Reg<tim6_cr1::TIM6_CR1rs>;
///TIM6 control register 1
pub mod tim6_cr1;
/**TIM6_CR2 (rw) register accessor: TIM6 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim6_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_CR2)

For information about available fields see [`mod@tim6_cr2`]
module*/
pub type TIM6_CR2 = crate::Reg<tim6_cr2::TIM6_CR2rs>;
///TIM6 control register 2
pub mod tim6_cr2;
/**TIM6_DIER (rw) register accessor: TIM6 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim6_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_DIER)

For information about available fields see [`mod@tim6_dier`]
module*/
pub type TIM6_DIER = crate::Reg<tim6_dier::TIM6_DIERrs>;
///TIM6 DMA/Interrupt enable register
pub mod tim6_dier;
/**TIM6_SR (rw) register accessor: TIM6 status register

You can [`read`](crate::Reg::read) this register and get [`tim6_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_SR)

For information about available fields see [`mod@tim6_sr`]
module*/
pub type TIM6_SR = crate::Reg<tim6_sr::TIM6_SRrs>;
///TIM6 status register
pub mod tim6_sr;
/**TIM6_EGR (w) register accessor: TIM6 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_EGR)

For information about available fields see [`mod@tim6_egr`]
module*/
pub type TIM6_EGR = crate::Reg<tim6_egr::TIM6_EGRrs>;
///TIM6 event generation register
pub mod tim6_egr;
/**TIM6_CNT (rw) register accessor: TIM6 counter

You can [`read`](crate::Reg::read) this register and get [`tim6_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_CNT)

For information about available fields see [`mod@tim6_cnt`]
module*/
pub type TIM6_CNT = crate::Reg<tim6_cnt::TIM6_CNTrs>;
///TIM6 counter
pub mod tim6_cnt;
/**TIM6_PSC (rw) register accessor: TIM6 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim6_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_PSC)

For information about available fields see [`mod@tim6_psc`]
module*/
pub type TIM6_PSC = crate::Reg<tim6_psc::TIM6_PSCrs>;
///TIM6 prescaler
pub mod tim6_psc;
/**TIM6_ARR (rw) register accessor: TIM6 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim6_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_ARR)

For information about available fields see [`mod@tim6_arr`]
module*/
pub type TIM6_ARR = crate::Reg<tim6_arr::TIM6_ARRrs>;
///TIM6 auto-reload register
pub mod tim6_arr;
