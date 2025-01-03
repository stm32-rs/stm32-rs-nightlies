#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_ifcr: [u8; 0x04],
    _reserved_2_ch: [u8; 0xa0],
}
impl RegisterBlock {
    ///0x00 - Interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - BDMA interrupt status register
    #[inline(always)]
    pub const fn bdma_isr(&self) -> &BDMA_ISR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - BDMA interrupt flag clear register
    #[inline(always)]
    pub const fn bdma_ifcr(&self) -> &BDMA_IFCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        })
    }
    ///0x08 - BDMA channel 0 configuration register
    #[inline(always)]
    pub const fn bdma_ccr0(&self) -> &BDMA_CCR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c -
    #[inline(always)]
    pub const fn bdma_cndtr0(&self) -> &BDMA_CNDTR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 -
    #[inline(always)]
    pub const fn bdma_cpar0(&self) -> &BDMA_CPAR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x14 -
    #[inline(always)]
    pub const fn bdma_cm0ar0(&self) -> &BDMA_CM0AR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x18 -
    #[inline(always)]
    pub const fn bdma_cm1ar0(&self) -> &BDMA_CM1AR0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - BDMA channel 1 configuration register
    #[inline(always)]
    pub const fn bdma_ccr1(&self) -> &BDMA_CCR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 -
    #[inline(always)]
    pub const fn bdma_cndtr1(&self) -> &BDMA_CNDTR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 -
    #[inline(always)]
    pub const fn bdma_cpar1(&self) -> &BDMA_CPAR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 -
    #[inline(always)]
    pub const fn bdma_cm0ar1(&self) -> &BDMA_CM0AR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c -
    #[inline(always)]
    pub const fn bdma_cm1ar1(&self) -> &BDMA_CM1AR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    ///0x30 - BDMA channel 2 configuration register
    #[inline(always)]
    pub const fn bdma_ccr2(&self) -> &BDMA_CCR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x34 -
    #[inline(always)]
    pub const fn bdma_cndtr2(&self) -> &BDMA_CNDTR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    ///0x38 -
    #[inline(always)]
    pub const fn bdma_cpar2(&self) -> &BDMA_CPAR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3c -
    #[inline(always)]
    pub const fn bdma_cm0ar2(&self) -> &BDMA_CM0AR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(60).cast() }
    }
    ///0x40 -
    #[inline(always)]
    pub const fn bdma_cm1ar2(&self) -> &BDMA_CM1AR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x44 - BDMA channel 3 configuration register
    #[inline(always)]
    pub const fn bdma_ccr3(&self) -> &BDMA_CCR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x48 -
    #[inline(always)]
    pub const fn bdma_cndtr3(&self) -> &BDMA_CNDTR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(72).cast() }
    }
    ///0x4c -
    #[inline(always)]
    pub const fn bdma_cpar3(&self) -> &BDMA_CPAR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    ///0x50 -
    #[inline(always)]
    pub const fn bdma_cm0ar3(&self) -> &BDMA_CM0AR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    ///0x54 -
    #[inline(always)]
    pub const fn bdma_cm1ar3(&self) -> &BDMA_CM1AR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    ///0x58 - BDMA channel 4 configuration register
    #[inline(always)]
    pub const fn bdma_ccr4(&self) -> &BDMA_CCR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    ///0x5c -
    #[inline(always)]
    pub const fn bdma_cndtr4(&self) -> &BDMA_CNDTR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    ///0x60 -
    #[inline(always)]
    pub const fn bdma_cpar4(&self) -> &BDMA_CPAR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    ///0x64 -
    #[inline(always)]
    pub const fn bdma_cm0ar4(&self) -> &BDMA_CM0AR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    ///0x68 -
    #[inline(always)]
    pub const fn bdma_cm1ar4(&self) -> &BDMA_CM1AR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    ///0x6c - BDMA channel 5 configuration register
    #[inline(always)]
    pub const fn bdma_ccr5(&self) -> &BDMA_CCR5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x70 -
    #[inline(always)]
    pub const fn bdma_cndtr5(&self) -> &BDMA_CNDTR5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).cast() }
    }
    ///0x74 -
    #[inline(always)]
    pub const fn bdma_cpar5(&self) -> &BDMA_CPAR5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(116).cast() }
    }
    ///0x78 -
    #[inline(always)]
    pub const fn bdma_cm0ar5(&self) -> &BDMA_CM0AR5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(120).cast() }
    }
    ///0x7c -
    #[inline(always)]
    pub const fn bdma_cm1ar5(&self) -> &BDMA_CM1AR5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(124).cast() }
    }
    ///0x80 - BDMA channel 6 configuration register
    #[inline(always)]
    pub const fn bdma_ccr6(&self) -> &BDMA_CCR6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    ///0x84 -
    #[inline(always)]
    pub const fn bdma_cndtr6(&self) -> &BDMA_CNDTR6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    ///0x88 -
    #[inline(always)]
    pub const fn bdma_cpar6(&self) -> &BDMA_CPAR6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8c -
    #[inline(always)]
    pub const fn bdma_cm0ar6(&self) -> &BDMA_CM0AR6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    ///0x90 -
    #[inline(always)]
    pub const fn bdma_cm1ar6(&self) -> &BDMA_CM1AR6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    ///0x94 - BDMA channel 7 configuration register
    #[inline(always)]
    pub const fn bdma_ccr7(&self) -> &BDMA_CCR7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    ///0x98 -
    #[inline(always)]
    pub const fn bdma_cndtr7(&self) -> &BDMA_CNDTR7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    ///0x9c -
    #[inline(always)]
    pub const fn bdma_cpar7(&self) -> &BDMA_CPAR7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn bdma_cm0ar7(&self) -> &BDMA_CM0AR7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn bdma_cm1ar7(&self) -> &BDMA_CM1AR7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
}
/**BDMA_ISR (r) register accessor: BDMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`bdma_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_ISR)

For information about available fields see [`mod@bdma_isr`]
module*/
pub type BDMA_ISR = crate::Reg<bdma_isr::BDMA_ISRrs>;
///BDMA interrupt status register
pub mod bdma_isr;
/**BDMA_IFCR (w) register accessor: BDMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_IFCR)

For information about available fields see [`mod@bdma_ifcr`]
module*/
pub type BDMA_IFCR = crate::Reg<bdma_ifcr::BDMA_IFCRrs>;
///BDMA interrupt flag clear register
pub mod bdma_ifcr;
/**BDMA_CCR0 (rw) register accessor: BDMA channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR0)

For information about available fields see [`mod@bdma_ccr0`]
module*/
pub type BDMA_CCR0 = crate::Reg<bdma_ccr0::BDMA_CCR0rs>;
///BDMA channel 0 configuration register
pub mod bdma_ccr0;
/**BDMA_CNDTR0 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR0)

For information about available fields see [`mod@bdma_cndtr0`]
module*/
pub type BDMA_CNDTR0 = crate::Reg<bdma_cndtr0::BDMA_CNDTR0rs>;
///
pub mod bdma_cndtr0;
/**BDMA_CPAR0 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR0)

For information about available fields see [`mod@bdma_cpar0`]
module*/
pub type BDMA_CPAR0 = crate::Reg<bdma_cpar0::BDMA_CPAR0rs>;
///
pub mod bdma_cpar0;
/**BDMA_CM0AR0 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR0)

For information about available fields see [`mod@bdma_cm0ar0`]
module*/
pub type BDMA_CM0AR0 = crate::Reg<bdma_cm0ar0::BDMA_CM0AR0rs>;
///
pub mod bdma_cm0ar0;
/**BDMA_CM1AR0 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR0)

For information about available fields see [`mod@bdma_cm1ar0`]
module*/
pub type BDMA_CM1AR0 = crate::Reg<bdma_cm1ar0::BDMA_CM1AR0rs>;
///
pub mod bdma_cm1ar0;
/**BDMA_CCR1 (rw) register accessor: BDMA channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR1)

For information about available fields see [`mod@bdma_ccr1`]
module*/
pub type BDMA_CCR1 = crate::Reg<bdma_ccr1::BDMA_CCR1rs>;
///BDMA channel 1 configuration register
pub mod bdma_ccr1;
/**BDMA_CNDTR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR1)

For information about available fields see [`mod@bdma_cndtr1`]
module*/
pub type BDMA_CNDTR1 = crate::Reg<bdma_cndtr1::BDMA_CNDTR1rs>;
///
pub mod bdma_cndtr1;
/**BDMA_CPAR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR1)

For information about available fields see [`mod@bdma_cpar1`]
module*/
pub type BDMA_CPAR1 = crate::Reg<bdma_cpar1::BDMA_CPAR1rs>;
///
pub mod bdma_cpar1;
/**BDMA_CM0AR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR1)

For information about available fields see [`mod@bdma_cm0ar1`]
module*/
pub type BDMA_CM0AR1 = crate::Reg<bdma_cm0ar1::BDMA_CM0AR1rs>;
///
pub mod bdma_cm0ar1;
/**BDMA_CM1AR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR1)

For information about available fields see [`mod@bdma_cm1ar1`]
module*/
pub type BDMA_CM1AR1 = crate::Reg<bdma_cm1ar1::BDMA_CM1AR1rs>;
///
pub mod bdma_cm1ar1;
/**BDMA_CCR2 (rw) register accessor: BDMA channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR2)

For information about available fields see [`mod@bdma_ccr2`]
module*/
pub type BDMA_CCR2 = crate::Reg<bdma_ccr2::BDMA_CCR2rs>;
///BDMA channel 2 configuration register
pub mod bdma_ccr2;
/**BDMA_CNDTR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR2)

For information about available fields see [`mod@bdma_cndtr2`]
module*/
pub type BDMA_CNDTR2 = crate::Reg<bdma_cndtr2::BDMA_CNDTR2rs>;
///
pub mod bdma_cndtr2;
/**BDMA_CPAR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR2)

For information about available fields see [`mod@bdma_cpar2`]
module*/
pub type BDMA_CPAR2 = crate::Reg<bdma_cpar2::BDMA_CPAR2rs>;
///
pub mod bdma_cpar2;
/**BDMA_CM0AR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR2)

For information about available fields see [`mod@bdma_cm0ar2`]
module*/
pub type BDMA_CM0AR2 = crate::Reg<bdma_cm0ar2::BDMA_CM0AR2rs>;
///
pub mod bdma_cm0ar2;
/**BDMA_CM1AR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR2)

For information about available fields see [`mod@bdma_cm1ar2`]
module*/
pub type BDMA_CM1AR2 = crate::Reg<bdma_cm1ar2::BDMA_CM1AR2rs>;
///
pub mod bdma_cm1ar2;
/**BDMA_CCR3 (rw) register accessor: BDMA channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR3)

For information about available fields see [`mod@bdma_ccr3`]
module*/
pub type BDMA_CCR3 = crate::Reg<bdma_ccr3::BDMA_CCR3rs>;
///BDMA channel 3 configuration register
pub mod bdma_ccr3;
/**BDMA_CNDTR3 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR3)

For information about available fields see [`mod@bdma_cndtr3`]
module*/
pub type BDMA_CNDTR3 = crate::Reg<bdma_cndtr3::BDMA_CNDTR3rs>;
///
pub mod bdma_cndtr3;
/**BDMA_CPAR3 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR3)

For information about available fields see [`mod@bdma_cpar3`]
module*/
pub type BDMA_CPAR3 = crate::Reg<bdma_cpar3::BDMA_CPAR3rs>;
///
pub mod bdma_cpar3;
/**BDMA_CM0AR3 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR3)

For information about available fields see [`mod@bdma_cm0ar3`]
module*/
pub type BDMA_CM0AR3 = crate::Reg<bdma_cm0ar3::BDMA_CM0AR3rs>;
///
pub mod bdma_cm0ar3;
/**BDMA_CM1AR3 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR3)

For information about available fields see [`mod@bdma_cm1ar3`]
module*/
pub type BDMA_CM1AR3 = crate::Reg<bdma_cm1ar3::BDMA_CM1AR3rs>;
///
pub mod bdma_cm1ar3;
/**BDMA_CCR4 (rw) register accessor: BDMA channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR4)

For information about available fields see [`mod@bdma_ccr4`]
module*/
pub type BDMA_CCR4 = crate::Reg<bdma_ccr4::BDMA_CCR4rs>;
///BDMA channel 4 configuration register
pub mod bdma_ccr4;
/**BDMA_CNDTR4 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR4)

For information about available fields see [`mod@bdma_cndtr4`]
module*/
pub type BDMA_CNDTR4 = crate::Reg<bdma_cndtr4::BDMA_CNDTR4rs>;
///
pub mod bdma_cndtr4;
/**BDMA_CPAR4 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR4)

For information about available fields see [`mod@bdma_cpar4`]
module*/
pub type BDMA_CPAR4 = crate::Reg<bdma_cpar4::BDMA_CPAR4rs>;
///
pub mod bdma_cpar4;
/**BDMA_CM0AR4 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR4)

For information about available fields see [`mod@bdma_cm0ar4`]
module*/
pub type BDMA_CM0AR4 = crate::Reg<bdma_cm0ar4::BDMA_CM0AR4rs>;
///
pub mod bdma_cm0ar4;
/**BDMA_CM1AR4 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR4)

For information about available fields see [`mod@bdma_cm1ar4`]
module*/
pub type BDMA_CM1AR4 = crate::Reg<bdma_cm1ar4::BDMA_CM1AR4rs>;
///
pub mod bdma_cm1ar4;
/**BDMA_CCR5 (rw) register accessor: BDMA channel 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR5)

For information about available fields see [`mod@bdma_ccr5`]
module*/
pub type BDMA_CCR5 = crate::Reg<bdma_ccr5::BDMA_CCR5rs>;
///BDMA channel 5 configuration register
pub mod bdma_ccr5;
/**BDMA_CNDTR5 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR5)

For information about available fields see [`mod@bdma_cndtr5`]
module*/
pub type BDMA_CNDTR5 = crate::Reg<bdma_cndtr5::BDMA_CNDTR5rs>;
///
pub mod bdma_cndtr5;
/**BDMA_CPAR5 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR5)

For information about available fields see [`mod@bdma_cpar5`]
module*/
pub type BDMA_CPAR5 = crate::Reg<bdma_cpar5::BDMA_CPAR5rs>;
///
pub mod bdma_cpar5;
/**BDMA_CM0AR5 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR5)

For information about available fields see [`mod@bdma_cm0ar5`]
module*/
pub type BDMA_CM0AR5 = crate::Reg<bdma_cm0ar5::BDMA_CM0AR5rs>;
///
pub mod bdma_cm0ar5;
/**BDMA_CM1AR5 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR5)

For information about available fields see [`mod@bdma_cm1ar5`]
module*/
pub type BDMA_CM1AR5 = crate::Reg<bdma_cm1ar5::BDMA_CM1AR5rs>;
///
pub mod bdma_cm1ar5;
/**BDMA_CCR6 (rw) register accessor: BDMA channel 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR6)

For information about available fields see [`mod@bdma_ccr6`]
module*/
pub type BDMA_CCR6 = crate::Reg<bdma_ccr6::BDMA_CCR6rs>;
///BDMA channel 6 configuration register
pub mod bdma_ccr6;
/**BDMA_CNDTR6 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR6)

For information about available fields see [`mod@bdma_cndtr6`]
module*/
pub type BDMA_CNDTR6 = crate::Reg<bdma_cndtr6::BDMA_CNDTR6rs>;
///
pub mod bdma_cndtr6;
/**BDMA_CPAR6 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR6)

For information about available fields see [`mod@bdma_cpar6`]
module*/
pub type BDMA_CPAR6 = crate::Reg<bdma_cpar6::BDMA_CPAR6rs>;
///
pub mod bdma_cpar6;
/**BDMA_CM0AR6 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR6)

For information about available fields see [`mod@bdma_cm0ar6`]
module*/
pub type BDMA_CM0AR6 = crate::Reg<bdma_cm0ar6::BDMA_CM0AR6rs>;
///
pub mod bdma_cm0ar6;
/**BDMA_CM1AR6 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR6)

For information about available fields see [`mod@bdma_cm1ar6`]
module*/
pub type BDMA_CM1AR6 = crate::Reg<bdma_cm1ar6::BDMA_CM1AR6rs>;
///
pub mod bdma_cm1ar6;
/**BDMA_CCR7 (rw) register accessor: BDMA channel 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`bdma_ccr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ccr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CCR7)

For information about available fields see [`mod@bdma_ccr7`]
module*/
pub type BDMA_CCR7 = crate::Reg<bdma_ccr7::BDMA_CCR7rs>;
///BDMA channel 7 configuration register
pub mod bdma_ccr7;
/**BDMA_CNDTR7 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR7)

For information about available fields see [`mod@bdma_cndtr7`]
module*/
pub type BDMA_CNDTR7 = crate::Reg<bdma_cndtr7::BDMA_CNDTR7rs>;
///
pub mod bdma_cndtr7;
/**BDMA_CPAR7 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CPAR7)

For information about available fields see [`mod@bdma_cpar7`]
module*/
pub type BDMA_CPAR7 = crate::Reg<bdma_cpar7::BDMA_CPAR7rs>;
///
pub mod bdma_cpar7;
/**BDMA_CM0AR7 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm0ar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm0ar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM0AR7)

For information about available fields see [`mod@bdma_cm0ar7`]
module*/
pub type BDMA_CM0AR7 = crate::Reg<bdma_cm0ar7::BDMA_CM0AR7rs>;
///
pub mod bdma_cm0ar7;
/**BDMA_CM1AR7 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`bdma_cm1ar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cm1ar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CM1AR7)

For information about available fields see [`mod@bdma_cm1ar7`]
module*/
pub type BDMA_CM1AR7 = crate::Reg<bdma_cm1ar7::BDMA_CM1AR7rs>;
///
pub mod bdma_cm1ar7;
/**ISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:ISR)

For information about available fields see [`mod@isr`]
module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Interrupt status register
pub mod isr;
/**IFCR (w) register accessor: Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:IFCR)

For information about available fields see [`mod@ifcr`]
module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///Interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
pub mod ch;
