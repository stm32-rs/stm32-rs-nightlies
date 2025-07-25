#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_cr1_fifo: [u8; 0x04],
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    gtpr: GTPR,
    rtor: RTOR,
    rqr: RQR,
    _reserved_7_isr_fifo: [u8; 0x04],
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - USART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_fifo_disabled(&self) -> &CR1_FIFO_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - USART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_fifo_enabled(&self) -> &CR1_FIFO_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - USART control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - USART control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - USART baud rate register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x10 - USART guard time and prescaler register
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    ///0x14 - USART receiver timeout register
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    ///0x18 - USART request register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - USART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_fifo_disabled(&self) -> &ISR_FIFO_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - USART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_fifo_enabled(&self) -> &ISR_FIFO_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - USART interrupt flag clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x24 - USART receive data register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x28 - USART transmit data register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x2c - USART prescaler register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
/**CR1_FIFO_ENABLED (rw) register accessor: USART control register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`cr1_fifo_enabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_fifo_enabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:CR1_FIFO_ENABLED)

For information about available fields see [`mod@cr1_fifo_enabled`] module*/
pub type CR1_FIFO_ENABLED = crate::Reg<cr1_fifo_enabled::CR1_FIFO_ENABLEDrs>;
///USART control register 1 \[alternate\]
pub mod cr1_fifo_enabled;
/**CR1_FIFO_DISABLED (rw) register accessor: USART control register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`cr1_fifo_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_fifo_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:CR1_FIFO_DISABLED)

For information about available fields see [`mod@cr1_fifo_disabled`] module*/
pub type CR1_FIFO_DISABLED = crate::Reg<cr1_fifo_disabled::CR1_FIFO_DISABLEDrs>;
///USART control register 1 \[alternate\]
pub mod cr1_fifo_disabled;
/**CR2 (rw) register accessor: USART control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///USART control register 2
pub mod cr2;
/**CR3 (rw) register accessor: USART control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///USART control register 3
pub mod cr3;
/**BRR (rw) register accessor: USART baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///USART baud rate register
pub mod brr;
/**GTPR (rw) register accessor: USART guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:GTPR)

For information about available fields see [`mod@gtpr`] module*/
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
///USART guard time and prescaler register
pub mod gtpr;
/**RTOR (rw) register accessor: USART receiver timeout register

You can [`read`](crate::Reg::read) this register and get [`rtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:RTOR)

For information about available fields see [`mod@rtor`] module*/
pub type RTOR = crate::Reg<rtor::RTORrs>;
///USART receiver timeout register
pub mod rtor;
/**RQR (w) register accessor: USART request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:RQR)

For information about available fields see [`mod@rqr`] module*/
pub type RQR = crate::Reg<rqr::RQRrs>;
///USART request register
pub mod rqr;
/**ISR_FIFO_ENABLED (r) register accessor: USART interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_fifo_enabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:ISR_FIFO_ENABLED)

For information about available fields see [`mod@isr_fifo_enabled`] module*/
pub type ISR_FIFO_ENABLED = crate::Reg<isr_fifo_enabled::ISR_FIFO_ENABLEDrs>;
///USART interrupt and status register \[alternate\]
pub mod isr_fifo_enabled;
/**ISR_FIFO_DISABLED (r) register accessor: USART interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_fifo_disabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:ISR_FIFO_DISABLED)

For information about available fields see [`mod@isr_fifo_disabled`] module*/
pub type ISR_FIFO_DISABLED = crate::Reg<isr_fifo_disabled::ISR_FIFO_DISABLEDrs>;
///USART interrupt and status register \[alternate\]
pub mod isr_fifo_disabled;
/**ICR (w) register accessor: USART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///USART interrupt flag clear register
pub mod icr;
/**RDR (r) register accessor: USART receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///USART receive data register
pub mod rdr;
/**TDR (rw) register accessor: USART transmit data register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///USART transmit data register
pub mod tdr;
/**PRESC (rw) register accessor: USART prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#USART1:PRESC)

For information about available fields see [`mod@presc`] module*/
pub type PRESC = crate::Reg<presc::PRESCrs>;
///USART prescaler register
pub mod presc;
