#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _cr1: _CR1,
    _reserved1: [u8; 0x0a],
    _dier: _DIER,
    _reserved2: [u8; 0x02],
    _sr: _SR,
    _reserved3: [u8; 0x02],
    _egr: _EGR,
    _reserved4: [u8; 0x02],
    _ccmr1: _CCMR1,
    _reserved5: [u8; 0x04],
    _ccer: _CCER,
    _reserved6: [u8; 0x02],
    _cnt: _CNT,
    _psc: _PSC,
    _reserved8: [u8; 0x02],
    _arr: _ARR,
    _reserved9: [u8; 0x06],
    _ccr1: _CCR1,
    _reserved10: [u8; 0x32],
    _tisel: _TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM14 control register 1
    #[inline(always)]
    pub const fn _cr1(&self) -> &_CR1 {
        &self._cr1
    }
    ///0x0c - TIM14 Interrupt enable register
    #[inline(always)]
    pub const fn _dier(&self) -> &_DIER {
        &self._dier
    }
    ///0x10 - TIM14 status register
    #[inline(always)]
    pub const fn _sr(&self) -> &_SR {
        &self._sr
    }
    ///0x14 - TIM14 event generation register
    #[inline(always)]
    pub const fn _egr(&self) -> &_EGR {
        &self._egr
    }
    ///0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
    #[inline(always)]
    pub const fn _ccmr1(&self) -> &_CCMR1 {
        &self._ccmr1
    }
    ///0x20 - TIM14 capture/compare enable register
    #[inline(always)]
    pub const fn _ccer(&self) -> &_CCER {
        &self._ccer
    }
    ///0x24 - TIM14 counter
    #[inline(always)]
    pub const fn _cnt(&self) -> &_CNT {
        &self._cnt
    }
    ///0x28 - TIM14 prescaler
    #[inline(always)]
    pub const fn _psc(&self) -> &_PSC {
        &self._psc
    }
    ///0x2c - TIM14 auto-reload register
    #[inline(always)]
    pub const fn _arr(&self) -> &_ARR {
        &self._arr
    }
    ///0x34 - TIM14 capture/compare register 1
    #[inline(always)]
    pub const fn _ccr1(&self) -> &_CCR1 {
        &self._ccr1
    }
    ///0x68 - TIM14 timer input selection register
    #[inline(always)]
    pub const fn _tisel(&self) -> &_TISEL {
        &self._tisel
    }
}
/**_CR1 (rw) register accessor: TIM14 control register 1

You can [`read`](crate::Reg::read) this register and get [`_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CR1)

For information about available fields see [`mod@_cr1`] module*/
pub type _CR1 = crate::Reg<_cr1::_CR1rs>;
///TIM14 control register 1
pub mod _cr1;
/**_DIER (rw) register accessor: TIM14 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_DIER)

For information about available fields see [`mod@_dier`] module*/
pub type _DIER = crate::Reg<_dier::_DIERrs>;
///TIM14 Interrupt enable register
pub mod _dier;
/**_SR (rw) register accessor: TIM14 status register

You can [`read`](crate::Reg::read) this register and get [`_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_SR)

For information about available fields see [`mod@_sr`] module*/
pub type _SR = crate::Reg<_sr::_SRrs>;
///TIM14 status register
pub mod _sr;
/**_EGR (w) register accessor: TIM14 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_EGR)

For information about available fields see [`mod@_egr`] module*/
pub type _EGR = crate::Reg<_egr::_EGRrs>;
///TIM14 event generation register
pub mod _egr;
/**_CCMR1 (rw) register accessor: The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode

You can [`read`](crate::Reg::read) this register and get [`_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CCMR1)

For information about available fields see [`mod@_ccmr1`] module*/
pub type _CCMR1 = crate::Reg<_ccmr1::_CCMR1rs>;
///The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
pub mod _ccmr1;
/**_CCER (rw) register accessor: TIM14 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CCER)

For information about available fields see [`mod@_ccer`] module*/
pub type _CCER = crate::Reg<_ccer::_CCERrs>;
///TIM14 capture/compare enable register
pub mod _ccer;
/**_CNT (rw) register accessor: TIM14 counter

You can [`read`](crate::Reg::read) this register and get [`_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CNT)

For information about available fields see [`mod@_cnt`] module*/
pub type _CNT = crate::Reg<_cnt::_CNTrs>;
///TIM14 counter
pub mod _cnt;
/**_PSC (rw) register accessor: TIM14 prescaler

You can [`read`](crate::Reg::read) this register and get [`_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_PSC)

For information about available fields see [`mod@_psc`] module*/
pub type _PSC = crate::Reg<_psc::_PSCrs>;
///TIM14 prescaler
pub mod _psc;
/**_ARR (rw) register accessor: TIM14 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_ARR)

For information about available fields see [`mod@_arr`] module*/
pub type _ARR = crate::Reg<_arr::_ARRrs>;
///TIM14 auto-reload register
pub mod _arr;
/**_CCR1 (rw) register accessor: TIM14 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_CCR1)

For information about available fields see [`mod@_ccr1`] module*/
pub type _CCR1 = crate::Reg<_ccr1::_CCR1rs>;
///TIM14 capture/compare register 1
pub mod _ccr1;
/**_TISEL (rw) register accessor: TIM14 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_TISEL)

For information about available fields see [`mod@_tisel`] module*/
pub type _TISEL = crate::Reg<_tisel::_TISELrs>;
///TIM14 timer input selection register
pub mod _tisel;
