#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim14_cr1: TIM14_CR1,
    _reserved1: [u8; 0x0a],
    tim14_dier: TIM14_DIER,
    _reserved2: [u8; 0x02],
    tim14_sr: TIM14_SR,
    _reserved3: [u8; 0x02],
    tim14_egr: TIM14_EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_tim14_ccmr: [u8; 0x04],
    _reserved5: [u8; 0x04],
    tim14_ccer: TIM14_CCER,
    _reserved6: [u8; 0x02],
    tim14_cnt: TIM14_CNT,
    tim14_psc: TIM14_PSC,
    _reserved8: [u8; 0x02],
    tim14_arr: TIM14_ARR,
    _reserved9: [u8; 0x06],
    tim14_ccr1: TIM14_CCR1,
    _reserved10: [u8; 0x32],
    tim14_tisel: TIM14_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM14 control register 1
    #[inline(always)]
    pub const fn tim14_cr1(&self) -> &TIM14_CR1 {
        &self.tim14_cr1
    }
    ///0x0c - TIM14 Interrupt enable register
    #[inline(always)]
    pub const fn tim14_dier(&self) -> &TIM14_DIER {
        &self.tim14_dier
    }
    ///0x10 - TIM14 status register
    #[inline(always)]
    pub const fn tim14_sr(&self) -> &TIM14_SR {
        &self.tim14_sr
    }
    ///0x14 - TIM14 event generation register
    #[inline(always)]
    pub const fn tim14_egr(&self) -> &TIM14_EGR {
        &self.tim14_egr
    }
    ///0x18 - TIM14 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim14_ccmr1_alternate1(&self) -> &TIM14_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM14 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim14_ccmr1(&self) -> &TIM14_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM14 capture/compare enable register
    #[inline(always)]
    pub const fn tim14_ccer(&self) -> &TIM14_CCER {
        &self.tim14_ccer
    }
    ///0x24 - TIM14 counter
    #[inline(always)]
    pub const fn tim14_cnt(&self) -> &TIM14_CNT {
        &self.tim14_cnt
    }
    ///0x28 - TIM14 prescaler
    #[inline(always)]
    pub const fn tim14_psc(&self) -> &TIM14_PSC {
        &self.tim14_psc
    }
    ///0x2c - TIM14 auto-reload register
    #[inline(always)]
    pub const fn tim14_arr(&self) -> &TIM14_ARR {
        &self.tim14_arr
    }
    ///0x34 - TIM14 capture/compare register 1
    #[inline(always)]
    pub const fn tim14_ccr1(&self) -> &TIM14_CCR1 {
        &self.tim14_ccr1
    }
    ///0x68 - TIM14 timer input selection register
    #[inline(always)]
    pub const fn tim14_tisel(&self) -> &TIM14_TISEL {
        &self.tim14_tisel
    }
}
/**TIM14_CR1 (rw) register accessor: TIM14 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim14_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CR1)

For information about available fields see [`mod@tim14_cr1`] module*/
pub type TIM14_CR1 = crate::Reg<tim14_cr1::TIM14_CR1rs>;
///TIM14 control register 1
pub mod tim14_cr1;
/**TIM14_DIER (rw) register accessor: TIM14 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim14_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_DIER)

For information about available fields see [`mod@tim14_dier`] module*/
pub type TIM14_DIER = crate::Reg<tim14_dier::TIM14_DIERrs>;
///TIM14 Interrupt enable register
pub mod tim14_dier;
/**TIM14_SR (rw) register accessor: TIM14 status register

You can [`read`](crate::Reg::read) this register and get [`tim14_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_SR)

For information about available fields see [`mod@tim14_sr`] module*/
pub type TIM14_SR = crate::Reg<tim14_sr::TIM14_SRrs>;
///TIM14 status register
pub mod tim14_sr;
/**TIM14_EGR (w) register accessor: TIM14 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_EGR)

For information about available fields see [`mod@tim14_egr`] module*/
pub type TIM14_EGR = crate::Reg<tim14_egr::TIM14_EGRrs>;
///TIM14 event generation register
pub mod tim14_egr;
/**TIM14_CCMR1 (rw) register accessor: TIM14 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim14_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CCMR1)

For information about available fields see [`mod@tim14_ccmr1`] module*/
pub type TIM14_CCMR1 = crate::Reg<tim14_ccmr1::TIM14_CCMR1rs>;
///TIM14 capture/compare mode register 1
pub mod tim14_ccmr1;
/**TIM14_CCMR1_ALTERNATE1 (rw) register accessor: TIM14 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim14_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim14_ccmr1_alternate1`] module*/
pub type TIM14_CCMR1_ALTERNATE1 = crate::Reg<tim14_ccmr1_alternate1::TIM14_CCMR1_ALTERNATE1rs>;
///TIM14 capture/compare mode register 1
pub mod tim14_ccmr1_alternate1;
/**TIM14_CCER (rw) register accessor: TIM14 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim14_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CCER)

For information about available fields see [`mod@tim14_ccer`] module*/
pub type TIM14_CCER = crate::Reg<tim14_ccer::TIM14_CCERrs>;
///TIM14 capture/compare enable register
pub mod tim14_ccer;
/**TIM14_CNT (rw) register accessor: TIM14 counter

You can [`read`](crate::Reg::read) this register and get [`tim14_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CNT)

For information about available fields see [`mod@tim14_cnt`] module*/
pub type TIM14_CNT = crate::Reg<tim14_cnt::TIM14_CNTrs>;
///TIM14 counter
pub mod tim14_cnt;
/**TIM14_PSC (rw) register accessor: TIM14 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim14_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_PSC)

For information about available fields see [`mod@tim14_psc`] module*/
pub type TIM14_PSC = crate::Reg<tim14_psc::TIM14_PSCrs>;
///TIM14 prescaler
pub mod tim14_psc;
/**TIM14_ARR (rw) register accessor: TIM14 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim14_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_ARR)

For information about available fields see [`mod@tim14_arr`] module*/
pub type TIM14_ARR = crate::Reg<tim14_arr::TIM14_ARRrs>;
///TIM14 auto-reload register
pub mod tim14_arr;
/**TIM14_CCR1 (rw) register accessor: TIM14 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim14_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_CCR1)

For information about available fields see [`mod@tim14_ccr1`] module*/
pub type TIM14_CCR1 = crate::Reg<tim14_ccr1::TIM14_CCR1rs>;
///TIM14 capture/compare register 1
pub mod tim14_ccr1;
/**TIM14_TISEL (rw) register accessor: TIM14 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim14_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM14:TIM14_TISEL)

For information about available fields see [`mod@tim14_tisel`] module*/
pub type TIM14_TISEL = crate::Reg<tim14_tisel::TIM14_TISELrs>;
///TIM14 timer input selection register
pub mod tim14_tisel;
