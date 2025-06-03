#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
    cfgr: CFGR,
    cr: CR,
    ccr1: CCR1,
    arr: ARR,
    cnt: CNT,
    _reserved8: [u8; 0x04],
    cfgr2: CFGR2,
    rcr: RCR,
    ccmr1: CCMR1,
    ccmr2: CCMR2,
    ccr2: CCR2,
    ccr3: CCR3,
    ccr4: CCR4,
}
impl RegisterBlock {
    ///0x00 - LPTIM2 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_input(&self) -> &ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM2 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPTIM2 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn icr_input(&self) -> &ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM2 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM2 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn dier_input(&self) -> &DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM2 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x14 - LPTIM compare register 1
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x28 - LPTIM repetition register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x2c - LPTIM capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1(&self) -> &CCMR1 {
        &self.ccmr1
    }
    ///0x30 - LPTIM capture/compare mode register 2
    #[inline(always)]
    pub const fn ccmr2(&self) -> &CCMR2 {
        &self.ccmr2
    }
    ///0x34 - LPTIM compare register 2
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    ///0x38 - LPTIM compare register 3
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    ///0x3c - LPTIM compare register 4
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
}
/**ISR_OUTPUT (r) register accessor: LPTIM2 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:ISR_OUTPUT)

For information about available fields see [`mod@isr_output`] module*/
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUTrs>;
///LPTIM2 interrupt and status register \[alternate\]
pub mod isr_output;
/**ISR_INPUT (r) register accessor: LPTIM2 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:ISR_INPUT)

For information about available fields see [`mod@isr_input`] module*/
pub type ISR_INPUT = crate::Reg<isr_input::ISR_INPUTrs>;
///LPTIM2 interrupt and status register \[alternate\]
pub mod isr_input;
/**ICR_OUTPUT (w) register accessor: LPTIM2 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:ICR_OUTPUT)

For information about available fields see [`mod@icr_output`] module*/
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUTrs>;
///LPTIM2 interrupt clear register \[alternate\]
pub mod icr_output;
/**ICR_INPUT (w) register accessor: LPTIM2 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:ICR_INPUT)

For information about available fields see [`mod@icr_input`] module*/
pub type ICR_INPUT = crate::Reg<icr_input::ICR_INPUTrs>;
///LPTIM2 interrupt clear register \[alternate\]
pub mod icr_input;
/**DIER_OUTPUT (rw) register accessor: LPTIM2 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:DIER_OUTPUT)

For information about available fields see [`mod@dier_output`] module*/
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUTrs>;
///LPTIM2 interrupt enable register \[alternate\]
pub mod dier_output;
/**DIER_INPUT (rw) register accessor: LPTIM2 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:DIER_INPUT)

For information about available fields see [`mod@dier_input`] module*/
pub type DIER_INPUT = crate::Reg<dier_input::DIER_INPUTrs>;
///LPTIM2 interrupt enable register \[alternate\]
pub mod dier_input;
/**CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///LPTIM configuration register
pub mod cfgr;
/**CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///LPTIM control register
pub mod cr;
/**CCR1 (rw) register accessor: LPTIM compare register 1

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///LPTIM compare register 1
pub mod ccr1;
/**ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///LPTIM autoreload register
pub mod arr;
/**CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///LPTIM counter register
pub mod cnt;
/**CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///LPTIM configuration register 2
pub mod cfgr2;
/**RCR (rw) register accessor: LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///LPTIM repetition register
pub mod rcr;
/**CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCMR1)

For information about available fields see [`mod@ccmr1`] module*/
pub type CCMR1 = crate::Reg<ccmr1::CCMR1rs>;
///LPTIM capture/compare mode register 1
pub mod ccmr1;
/**CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCMR2)

For information about available fields see [`mod@ccmr2`] module*/
pub type CCMR2 = crate::Reg<ccmr2::CCMR2rs>;
///LPTIM capture/compare mode register 2
pub mod ccmr2;
/**CCR2 (rw) register accessor: LPTIM compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCR2)

For information about available fields see [`mod@ccr2`] module*/
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
///LPTIM compare register 2
pub mod ccr2;
/**CCR3 (rw) register accessor: LPTIM compare register 3

You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCR3)

For information about available fields see [`mod@ccr3`] module*/
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
///LPTIM compare register 3
pub mod ccr3;
/**CCR4 (rw) register accessor: LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CCR4)

For information about available fields see [`mod@ccr4`] module*/
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
///LPTIM compare register 4
pub mod ccr4;
