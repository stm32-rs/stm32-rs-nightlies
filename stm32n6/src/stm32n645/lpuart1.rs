#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    _reserved4: [u8; 0x08],
    rqr: RQR,
    _reserved_5_isr: [u8; 0x04],
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - LPUART control register 1
    #[inline(always)]
    pub const fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPUART control register 1
    #[inline(always)]
    pub const fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPUART control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - LPUART control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - LPUART baud rate register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x18 - LPUART request register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - LPUART interrupt and status register
    #[inline(always)]
    pub const fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - LPUART interrupt and status register
    #[inline(always)]
    pub const fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - LPUART interrupt flag clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x24 - LPUART receive data register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x28 - LPUART transmit data register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x2c - LPUART prescaler register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
/**CR1_ENABLED (rw) register accessor: LPUART control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1_enabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_enabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:CR1_ENABLED)

For information about available fields see [`mod@cr1_enabled`] module*/
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLEDrs>;
///LPUART control register 1
pub mod cr1_enabled;
/**CR1_DISABLED (rw) register accessor: LPUART control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:CR1_DISABLED)

For information about available fields see [`mod@cr1_disabled`] module*/
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLEDrs>;
///LPUART control register 1
pub mod cr1_disabled;
/**CR2 (rw) register accessor: LPUART control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///LPUART control register 2
pub mod cr2;
/**CR3 (rw) register accessor: LPUART control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///LPUART control register 3
pub mod cr3;
/**BRR (rw) register accessor: LPUART baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///LPUART baud rate register
pub mod brr;
/**RQR (w) register accessor: LPUART request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:RQR)

For information about available fields see [`mod@rqr`] module*/
pub type RQR = crate::Reg<rqr::RQRrs>;
///LPUART request register
pub mod rqr;
/**ISR_ENABLED (r) register accessor: LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_enabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:ISR_ENABLED)

For information about available fields see [`mod@isr_enabled`] module*/
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLEDrs>;
///LPUART interrupt and status register
pub mod isr_enabled;
/**ISR_DISABLED (r) register accessor: LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_disabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:ISR_DISABLED)

For information about available fields see [`mod@isr_disabled`] module*/
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLEDrs>;
///LPUART interrupt and status register
pub mod isr_disabled;
/**ICR (w) register accessor: LPUART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LPUART interrupt flag clear register
pub mod icr;
/**RDR (r) register accessor: LPUART receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///LPUART receive data register
pub mod rdr;
/**TDR (rw) register accessor: LPUART transmit data register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///LPUART transmit data register
pub mod tdr;
/**PRESC (rw) register accessor: LPUART prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:PRESC)

For information about available fields see [`mod@presc`] module*/
pub type PRESC = crate::Reg<presc::PRESCrs>;
///LPUART prescaler register
pub mod presc;
