#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier1: IER1,
    _reserved1: [u8; 0x0c],
    misr1: MISR1,
    _reserved2: [u8; 0x0c],
    icr1: ICR1,
}
impl RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0x10 - TZIC status register 1
    #[inline(always)]
    pub const fn misr1(&self) -> &MISR1 {
        &self.misr1
    }
    ///0x20 - TZIC interrupt status clear register 1
    #[inline(always)]
    pub const fn icr1(&self) -> &ICR1 {
        &self.icr1
    }
}
/**IER1 (rw) register accessor: TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TZIC:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///TZIC interrupt enable register 1
pub mod ier1;
/**MISR1 (r) register accessor: TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`misr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TZIC:MISR1)

For information about available fields see [`mod@misr1`] module*/
pub type MISR1 = crate::Reg<misr1::MISR1rs>;
///TZIC status register 1
pub mod misr1;
/**ICR1 (rw) register accessor: TZIC interrupt status clear register 1

You can [`read`](crate::Reg::read) this register and get [`icr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TZIC:ICR1)

For information about available fields see [`mod@icr1`] module*/
pub type ICR1 = crate::Reg<icr1::ICR1rs>;
///TZIC interrupt status clear register 1
pub mod icr1;
