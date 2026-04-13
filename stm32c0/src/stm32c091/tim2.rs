#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim2_cr1: TIM2_CR1,
    _reserved1: [u8; 0x02],
    tim2_cr2: TIM2_CR2,
    _reserved2: [u8; 0x02],
    tim2_smcr: TIM2_SMCR,
    tim2_dier: TIM2_DIER,
    _reserved4: [u8; 0x02],
    tim2_sr: TIM2_SR,
    _reserved5: [u8; 0x02],
    tim2_egr: TIM2_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim2_ccmr: [u8; 0x04],
    _reserved_7_tim2_ccmr: [u8; 0x04],
    tim2_ccer: TIM2_CCER,
    _reserved9: [u8; 0x02],
    _reserved_9_tim2_: [u8; 0x04],
    tim2_psc: TIM2_PSC,
    _reserved11: [u8; 0x02],
    tim2_arr: TIM2_ARR,
    _reserved12: [u8; 0x04],
    tim2_ccr1: TIM2_CCR1,
    tim2_ccr2: TIM2_CCR2,
    tim2_ccr3: TIM2_CCR3,
    tim2_ccr4: TIM2_CCR4,
    _reserved16: [u8; 0x04],
    tim2_dcr: TIM2_DCR,
    _reserved17: [u8; 0x02],
    tim2_dmar: TIM2_DMAR,
    _reserved18: [u8; 0x12],
    tim2_af1: TIM2_AF1,
    _reserved19: [u8; 0x04],
    tim2_tisel: TIM2_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM2 control register 1
    #[inline(always)]
    pub const fn tim2_cr1(&self) -> &TIM2_CR1 {
        &self.tim2_cr1
    }
    ///0x04 - TIM2 control register 2
    #[inline(always)]
    pub const fn tim2_cr2(&self) -> &TIM2_CR2 {
        &self.tim2_cr2
    }
    ///0x08 - TIM2 slave mode control register
    #[inline(always)]
    pub const fn tim2_smcr(&self) -> &TIM2_SMCR {
        &self.tim2_smcr
    }
    ///0x0c - TIM2 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn tim2_dier(&self) -> &TIM2_DIER {
        &self.tim2_dier
    }
    ///0x10 - TIM2 status register
    #[inline(always)]
    pub const fn tim2_sr(&self) -> &TIM2_SR {
        &self.tim2_sr
    }
    ///0x14 - TIM2 event generation register
    #[inline(always)]
    pub const fn tim2_egr(&self) -> &TIM2_EGR {
        &self.tim2_egr
    }
    ///0x18 - TIM2 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim2_ccmr1_alternate1(&self) -> &TIM2_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM2 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim2_ccmr1(&self) -> &TIM2_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM2 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim2_ccmr2_alternate1(&self) -> &TIM2_CCMR2_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM2 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim2_ccmr2(&self) -> &TIM2_CCMR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM2 capture/compare enable register
    #[inline(always)]
    pub const fn tim2_ccer(&self) -> &TIM2_CCER {
        &self.tim2_ccer
    }
    ///0x24 - TIM2 counter
    #[inline(always)]
    pub const fn tim2_cnt_alternate1(&self) -> &TIM2_CNT_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - TIM2 counter
    #[inline(always)]
    pub const fn tim2_cnt(&self) -> &TIM2_CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - TIM2 prescaler
    #[inline(always)]
    pub const fn tim2_psc(&self) -> &TIM2_PSC {
        &self.tim2_psc
    }
    ///0x2c - TIM2 auto-reload register
    #[inline(always)]
    pub const fn tim2_arr(&self) -> &TIM2_ARR {
        &self.tim2_arr
    }
    ///0x34 - TIM2 capture/compare register 1
    #[inline(always)]
    pub const fn tim2_ccr1(&self) -> &TIM2_CCR1 {
        &self.tim2_ccr1
    }
    ///0x38 - TIM2 capture/compare register 2
    #[inline(always)]
    pub const fn tim2_ccr2(&self) -> &TIM2_CCR2 {
        &self.tim2_ccr2
    }
    ///0x3c - TIM2 capture/compare register 3
    #[inline(always)]
    pub const fn tim2_ccr3(&self) -> &TIM2_CCR3 {
        &self.tim2_ccr3
    }
    ///0x40 - TIM2 capture/compare register 4
    #[inline(always)]
    pub const fn tim2_ccr4(&self) -> &TIM2_CCR4 {
        &self.tim2_ccr4
    }
    ///0x48 - TIM2 DMA control register
    #[inline(always)]
    pub const fn tim2_dcr(&self) -> &TIM2_DCR {
        &self.tim2_dcr
    }
    ///0x4c - TIM2 DMA address for full transfer
    #[inline(always)]
    pub const fn tim2_dmar(&self) -> &TIM2_DMAR {
        &self.tim2_dmar
    }
    ///0x60 - TIM2 alternate function option register 1
    #[inline(always)]
    pub const fn tim2_af1(&self) -> &TIM2_AF1 {
        &self.tim2_af1
    }
    ///0x68 - TIM2 timer input selection register
    #[inline(always)]
    pub const fn tim2_tisel(&self) -> &TIM2_TISEL {
        &self.tim2_tisel
    }
}
/**TIM2_CR1 (rw) register accessor: TIM2 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CR1)

For information about available fields see [`mod@tim2_cr1`] module*/
pub type TIM2_CR1 = crate::Reg<tim2_cr1::TIM2_CR1rs>;
///TIM2 control register 1
pub mod tim2_cr1;
/**TIM2_CR2 (rw) register accessor: TIM2 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CR2)

For information about available fields see [`mod@tim2_cr2`] module*/
pub type TIM2_CR2 = crate::Reg<tim2_cr2::TIM2_CR2rs>;
///TIM2 control register 2
pub mod tim2_cr2;
/**TIM2_SMCR (rw) register accessor: TIM2 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim2_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_SMCR)

For information about available fields see [`mod@tim2_smcr`] module*/
pub type TIM2_SMCR = crate::Reg<tim2_smcr::TIM2_SMCRrs>;
///TIM2 slave mode control register
pub mod tim2_smcr;
/**TIM2_DIER (rw) register accessor: TIM2 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim2_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_DIER)

For information about available fields see [`mod@tim2_dier`] module*/
pub type TIM2_DIER = crate::Reg<tim2_dier::TIM2_DIERrs>;
///TIM2 DMA/Interrupt enable register
pub mod tim2_dier;
/**TIM2_SR (rw) register accessor: TIM2 status register

You can [`read`](crate::Reg::read) this register and get [`tim2_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_SR)

For information about available fields see [`mod@tim2_sr`] module*/
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SRrs>;
///TIM2 status register
pub mod tim2_sr;
/**TIM2_EGR (w) register accessor: TIM2 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_EGR)

For information about available fields see [`mod@tim2_egr`] module*/
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGRrs>;
///TIM2 event generation register
pub mod tim2_egr;
/**TIM2_CCMR1 (rw) register accessor: TIM2 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCMR1)

For information about available fields see [`mod@tim2_ccmr1`] module*/
pub type TIM2_CCMR1 = crate::Reg<tim2_ccmr1::TIM2_CCMR1rs>;
///TIM2 capture/compare mode register 1
pub mod tim2_ccmr1;
/**TIM2_CCMR1_ALTERNATE1 (rw) register accessor: TIM2 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim2_ccmr1_alternate1`] module*/
pub type TIM2_CCMR1_ALTERNATE1 = crate::Reg<tim2_ccmr1_alternate1::TIM2_CCMR1_ALTERNATE1rs>;
///TIM2 capture/compare mode register 1
pub mod tim2_ccmr1_alternate1;
/**TIM2_CCMR2 (rw) register accessor: TIM2 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCMR2)

For information about available fields see [`mod@tim2_ccmr2`] module*/
pub type TIM2_CCMR2 = crate::Reg<tim2_ccmr2::TIM2_CCMR2rs>;
///TIM2 capture/compare mode register 2
pub mod tim2_ccmr2;
/**TIM2_CCMR2_ALTERNATE1 (rw) register accessor: TIM2 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr2_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr2_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCMR2_ALTERNATE1)

For information about available fields see [`mod@tim2_ccmr2_alternate1`] module*/
pub type TIM2_CCMR2_ALTERNATE1 = crate::Reg<tim2_ccmr2_alternate1::TIM2_CCMR2_ALTERNATE1rs>;
///TIM2 capture/compare mode register 2
pub mod tim2_ccmr2_alternate1;
/**TIM2_CCER (rw) register accessor: TIM2 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim2_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCER)

For information about available fields see [`mod@tim2_ccer`] module*/
pub type TIM2_CCER = crate::Reg<tim2_ccer::TIM2_CCERrs>;
///TIM2 capture/compare enable register
pub mod tim2_ccer;
/**TIM2_CNT (rw) register accessor: TIM2 counter

You can [`read`](crate::Reg::read) this register and get [`tim2_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CNT)

For information about available fields see [`mod@tim2_cnt`] module*/
pub type TIM2_CNT = crate::Reg<tim2_cnt::TIM2_CNTrs>;
///TIM2 counter
pub mod tim2_cnt;
/**TIM2_CNT_ALTERNATE1 (rw) register accessor: TIM2 counter

You can [`read`](crate::Reg::read) this register and get [`tim2_cnt_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cnt_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CNT_ALTERNATE1)

For information about available fields see [`mod@tim2_cnt_alternate1`] module*/
pub type TIM2_CNT_ALTERNATE1 = crate::Reg<tim2_cnt_alternate1::TIM2_CNT_ALTERNATE1rs>;
///TIM2 counter
pub mod tim2_cnt_alternate1;
/**TIM2_PSC (rw) register accessor: TIM2 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim2_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_PSC)

For information about available fields see [`mod@tim2_psc`] module*/
pub type TIM2_PSC = crate::Reg<tim2_psc::TIM2_PSCrs>;
///TIM2 prescaler
pub mod tim2_psc;
/**TIM2_ARR (rw) register accessor: TIM2 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim2_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_ARR)

For information about available fields see [`mod@tim2_arr`] module*/
pub type TIM2_ARR = crate::Reg<tim2_arr::TIM2_ARRrs>;
///TIM2 auto-reload register
pub mod tim2_arr;
/**TIM2_CCR1 (rw) register accessor: TIM2 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCR1)

For information about available fields see [`mod@tim2_ccr1`] module*/
pub type TIM2_CCR1 = crate::Reg<tim2_ccr1::TIM2_CCR1rs>;
///TIM2 capture/compare register 1
pub mod tim2_ccr1;
/**TIM2_CCR2 (rw) register accessor: TIM2 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCR2)

For information about available fields see [`mod@tim2_ccr2`] module*/
pub type TIM2_CCR2 = crate::Reg<tim2_ccr2::TIM2_CCR2rs>;
///TIM2 capture/compare register 2
pub mod tim2_ccr2;
/**TIM2_CCR3 (rw) register accessor: TIM2 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCR3)

For information about available fields see [`mod@tim2_ccr3`] module*/
pub type TIM2_CCR3 = crate::Reg<tim2_ccr3::TIM2_CCR3rs>;
///TIM2 capture/compare register 3
pub mod tim2_ccr3;
/**TIM2_CCR4 (rw) register accessor: TIM2 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCR4)

For information about available fields see [`mod@tim2_ccr4`] module*/
pub type TIM2_CCR4 = crate::Reg<tim2_ccr4::TIM2_CCR4rs>;
///TIM2 capture/compare register 4
pub mod tim2_ccr4;
/**TIM2_DCR (rw) register accessor: TIM2 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim2_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_DCR)

For information about available fields see [`mod@tim2_dcr`] module*/
pub type TIM2_DCR = crate::Reg<tim2_dcr::TIM2_DCRrs>;
///TIM2 DMA control register
pub mod tim2_dcr;
/**TIM2_DMAR (rw) register accessor: TIM2 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim2_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_DMAR)

For information about available fields see [`mod@tim2_dmar`] module*/
pub type TIM2_DMAR = crate::Reg<tim2_dmar::TIM2_DMARrs>;
///TIM2 DMA address for full transfer
pub mod tim2_dmar;
/**TIM2_AF1 (rw) register accessor: TIM2 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_AF1)

For information about available fields see [`mod@tim2_af1`] module*/
pub type TIM2_AF1 = crate::Reg<tim2_af1::TIM2_AF1rs>;
///TIM2 alternate function option register 1
pub mod tim2_af1;
/**TIM2_TISEL (rw) register accessor: TIM2 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim2_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_TISEL)

For information about available fields see [`mod@tim2_tisel`] module*/
pub type TIM2_TISEL = crate::Reg<tim2_tisel::TIM2_TISELrs>;
///TIM2 timer input selection register
pub mod tim2_tisel;
