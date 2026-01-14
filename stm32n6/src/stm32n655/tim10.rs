#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x0a],
    dier: DIER,
    _reserved2: [u8; 0x02],
    sr: SR,
    _reserved3: [u8; 0x02],
    egr: EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    ccer: CCER,
    _reserved6: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved8: [u8; 0x02],
    arr: ARR,
    _reserved9: [u8; 0x04],
    ccr1: CCR1,
    _reserved10: [u8; 0x24],
    tisel: TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM10 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x0c - TIM10 Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM10 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM10 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - TIM10 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM10 capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM10 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - TIM10 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - TIM10 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM10 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x34 - TIM10 capture/compare register 1
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x5c - TIM10 timer input selection register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
/**CR1 (rw) register accessor: TIM10 control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///TIM10 control register 1
pub mod cr1;
/**DIER (rw) register accessor: TIM10 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///TIM10 Interrupt enable register
pub mod dier;
/**SR (rw) register accessor: TIM10 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TIM10 status register
pub mod sr;
/**EGR (w) register accessor: TIM10 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///TIM10 event generation register
pub mod egr;
/**CCMR1_Input (rw) register accessor: TIM10 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CCMR1_Input)

For information about available fields see [`mod@ccmr1_input`] module*/
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
///TIM10 capture/compare mode register 1
pub mod ccmr1_input;
/**CCMR1_Output (rw) register accessor: TIM10 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CCMR1_Output)

For information about available fields see [`mod@ccmr1_output`] module*/
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
///TIM10 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_output;
/**CCER (rw) register accessor: TIM10 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CCER)

For information about available fields see [`mod@ccer`] module*/
pub type CCER = crate::Reg<ccer::CCERrs>;
///TIM10 capture/compare enable register
pub mod ccer;
/**CNT (rw) register accessor: TIM10 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///TIM10 counter
pub mod cnt;
/**PSC (rw) register accessor: TIM10 prescaler

You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:PSC)

For information about available fields see [`mod@psc`] module*/
pub type PSC = crate::Reg<psc::PSCrs>;
///TIM10 prescaler
pub mod psc;
/**ARR (rw) register accessor: TIM10 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///TIM10 auto-reload register
pub mod arr;
/**CCR1 (rw) register accessor: TIM10 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///TIM10 capture/compare register 1
pub mod ccr1;
/**TISEL (rw) register accessor: TIM10 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM10:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TIM10 timer input selection register
pub mod tisel;
