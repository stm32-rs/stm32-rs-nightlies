#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim13_cr1: TIM13_CR1,
    _reserved1: [u8; 0x0a],
    tim13_dier: TIM13_DIER,
    _reserved2: [u8; 0x02],
    tim13_sr: TIM13_SR,
    _reserved3: [u8; 0x02],
    tim13_egr: TIM13_EGR,
    _reserved4: [u8; 0x02],
    tim13_ccmr1: TIM13_CCMR1,
    _reserved5: [u8; 0x04],
    tim13_ccer: TIM13_CCER,
    _reserved6: [u8; 0x02],
    tim13_cnt: TIM13_CNT,
    tim13_psc: TIM13_PSC,
    _reserved8: [u8; 0x02],
    tim13_arr: TIM13_ARR,
    _reserved9: [u8; 0x06],
    tim13_ccr1: TIM13_CCR1,
    _reserved10: [u8; 0x32],
    tim13_tisel: TIM13_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM13 control register 1
    #[inline(always)]
    pub const fn tim13_cr1(&self) -> &TIM13_CR1 {
        &self.tim13_cr1
    }
    ///0x0c - TIM13 Interrupt enable register
    #[inline(always)]
    pub const fn tim13_dier(&self) -> &TIM13_DIER {
        &self.tim13_dier
    }
    ///0x10 - TIM13 status register
    #[inline(always)]
    pub const fn tim13_sr(&self) -> &TIM13_SR {
        &self.tim13_sr
    }
    ///0x14 - TIM13 event generation register
    #[inline(always)]
    pub const fn tim13_egr(&self) -> &TIM13_EGR {
        &self.tim13_egr
    }
    ///0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
    #[inline(always)]
    pub const fn tim13_ccmr1(&self) -> &TIM13_CCMR1 {
        &self.tim13_ccmr1
    }
    ///0x20 - TIM13 capture/compare enable register
    #[inline(always)]
    pub const fn tim13_ccer(&self) -> &TIM13_CCER {
        &self.tim13_ccer
    }
    ///0x24 - TIM13 counter
    #[inline(always)]
    pub const fn tim13_cnt(&self) -> &TIM13_CNT {
        &self.tim13_cnt
    }
    ///0x28 - TIM13 prescaler
    #[inline(always)]
    pub const fn tim13_psc(&self) -> &TIM13_PSC {
        &self.tim13_psc
    }
    ///0x2c - TIM13 auto-reload register
    #[inline(always)]
    pub const fn tim13_arr(&self) -> &TIM13_ARR {
        &self.tim13_arr
    }
    ///0x34 - TIM13 capture/compare register 1
    #[inline(always)]
    pub const fn tim13_ccr1(&self) -> &TIM13_CCR1 {
        &self.tim13_ccr1
    }
    ///0x68 - TIM13 timer input selection register
    #[inline(always)]
    pub const fn tim13_tisel(&self) -> &TIM13_TISEL {
        &self.tim13_tisel
    }
}
/**TIM13_CR1 (rw) register accessor: TIM13 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim13_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_CR1)

For information about available fields see [`mod@tim13_cr1`]
module*/
pub type TIM13_CR1 = crate::Reg<tim13_cr1::TIM13_CR1rs>;
///TIM13 control register 1
pub mod tim13_cr1;
/**TIM13_DIER (rw) register accessor: TIM13 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim13_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_DIER)

For information about available fields see [`mod@tim13_dier`]
module*/
pub type TIM13_DIER = crate::Reg<tim13_dier::TIM13_DIERrs>;
///TIM13 Interrupt enable register
pub mod tim13_dier;
/**TIM13_SR (rw) register accessor: TIM13 status register

You can [`read`](crate::Reg::read) this register and get [`tim13_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_SR)

For information about available fields see [`mod@tim13_sr`]
module*/
pub type TIM13_SR = crate::Reg<tim13_sr::TIM13_SRrs>;
///TIM13 status register
pub mod tim13_sr;
/**TIM13_EGR (w) register accessor: TIM13 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_EGR)

For information about available fields see [`mod@tim13_egr`]
module*/
pub type TIM13_EGR = crate::Reg<tim13_egr::TIM13_EGRrs>;
///TIM13 event generation register
pub mod tim13_egr;
/**TIM13_CCMR1 (rw) register accessor: The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode

You can [`read`](crate::Reg::read) this register and get [`tim13_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_CCMR1)

For information about available fields see [`mod@tim13_ccmr1`]
module*/
pub type TIM13_CCMR1 = crate::Reg<tim13_ccmr1::TIM13_CCMR1rs>;
///The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
pub mod tim13_ccmr1;
/**TIM13_CCER (rw) register accessor: TIM13 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim13_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_CCER)

For information about available fields see [`mod@tim13_ccer`]
module*/
pub type TIM13_CCER = crate::Reg<tim13_ccer::TIM13_CCERrs>;
///TIM13 capture/compare enable register
pub mod tim13_ccer;
/**TIM13_CNT (rw) register accessor: TIM13 counter

You can [`read`](crate::Reg::read) this register and get [`tim13_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_CNT)

For information about available fields see [`mod@tim13_cnt`]
module*/
pub type TIM13_CNT = crate::Reg<tim13_cnt::TIM13_CNTrs>;
///TIM13 counter
pub mod tim13_cnt;
/**TIM13_PSC (rw) register accessor: TIM13 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim13_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_PSC)

For information about available fields see [`mod@tim13_psc`]
module*/
pub type TIM13_PSC = crate::Reg<tim13_psc::TIM13_PSCrs>;
///TIM13 prescaler
pub mod tim13_psc;
/**TIM13_ARR (rw) register accessor: TIM13 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim13_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_ARR)

For information about available fields see [`mod@tim13_arr`]
module*/
pub type TIM13_ARR = crate::Reg<tim13_arr::TIM13_ARRrs>;
///TIM13 auto-reload register
pub mod tim13_arr;
/**TIM13_CCR1 (rw) register accessor: TIM13 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim13_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_CCR1)

For information about available fields see [`mod@tim13_ccr1`]
module*/
pub type TIM13_CCR1 = crate::Reg<tim13_ccr1::TIM13_CCR1rs>;
///TIM13 capture/compare register 1
pub mod tim13_ccr1;
/**TIM13_TISEL (rw) register accessor: TIM13 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim13_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_TISEL)

For information about available fields see [`mod@tim13_tisel`]
module*/
pub type TIM13_TISEL = crate::Reg<tim13_tisel::TIM13_TISELrs>;
///TIM13 timer input selection register
pub mod tim13_tisel;
