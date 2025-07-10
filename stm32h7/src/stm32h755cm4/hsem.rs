#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    r: [R; 32],
    rlr: [RLR; 32],
    c1ier: C1IER,
    c1icr: C1ICR,
    c1isr: C1ISR,
    c1misr: C1MISR,
    c2ier: C2IER,
    c2icr: C2ICR,
    c2isr: C2ISR,
    c2misr: C2MISR,
    _reserved10: [u8; 0x20],
    cr: CR,
    keyr: KEYR,
}
impl RegisterBlock {
    ///0x00..0x80 - HSEM register HSEM_R%s
    #[inline(always)]
    pub const fn r(&self, n: usize) -> &R {
        &self.r[n]
    }
    ///Iterator for array of:
    ///0x00..0x80 - HSEM register HSEM_R%s
    #[inline(always)]
    pub fn r_iter(&self) -> impl Iterator<Item = &R> {
        self.r.iter()
    }
    ///0x80..0x100 - Semaphore %s read lock register
    #[inline(always)]
    pub const fn rlr(&self, n: usize) -> &RLR {
        &self.rlr[n]
    }
    ///Iterator for array of:
    ///0x80..0x100 - Semaphore %s read lock register
    #[inline(always)]
    pub fn rlr_iter(&self) -> impl Iterator<Item = &RLR> {
        self.rlr.iter()
    }
    ///0x100 - HSEM Interrupt enable register
    #[inline(always)]
    pub const fn c1ier(&self) -> &C1IER {
        &self.c1ier
    }
    ///0x104 - HSEM Interrupt clear register
    #[inline(always)]
    pub const fn c1icr(&self) -> &C1ICR {
        &self.c1icr
    }
    ///0x108 - HSEM Interrupt status register
    #[inline(always)]
    pub const fn c1isr(&self) -> &C1ISR {
        &self.c1isr
    }
    ///0x10c - HSEM Masked interrupt status register
    #[inline(always)]
    pub const fn c1misr(&self) -> &C1MISR {
        &self.c1misr
    }
    ///0x110 - HSEM Interrupt enable register
    #[inline(always)]
    pub const fn c2ier(&self) -> &C2IER {
        &self.c2ier
    }
    ///0x114 - HSEM Interrupt clear register
    #[inline(always)]
    pub const fn c2icr(&self) -> &C2ICR {
        &self.c2icr
    }
    ///0x118 - HSEM Interrupt status register
    #[inline(always)]
    pub const fn c2isr(&self) -> &C2ISR {
        &self.c2isr
    }
    ///0x11c - HSEM Masked interrupt status register
    #[inline(always)]
    pub const fn c2misr(&self) -> &C2MISR {
        &self.c2misr
    }
    ///0x140 - HSEM Clear register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x144 - HSEM Interrupt clear register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
}
/**R (rw) register accessor: HSEM register HSEM_R%s

You can [`read`](crate::Reg::read) this register and get [`r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:R[0])

For information about available fields see [`mod@r`] module*/
pub type R = crate::Reg<r::Rrs>;
///HSEM register HSEM_R%s
pub mod r;
/**RLR (r) register accessor: Semaphore %s read lock register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:RLR[0])

For information about available fields see [`mod@rlr`] module*/
pub type RLR = crate::Reg<rlr::RLRrs>;
///Semaphore %s read lock register
pub mod rlr;
/**C1IER (rw) register accessor: HSEM Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C1IER)

For information about available fields see [`mod@c1ier`] module*/
pub type C1IER = crate::Reg<c1ier::C1IERrs>;
///HSEM Interrupt enable register
pub mod c1ier;
/**C1ICR (rw) register accessor: HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C1ICR)

For information about available fields see [`mod@c1icr`] module*/
pub type C1ICR = crate::Reg<c1icr::C1ICRrs>;
///HSEM Interrupt clear register
pub mod c1icr;
/**C1ISR (r) register accessor: HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C1ISR)

For information about available fields see [`mod@c1isr`] module*/
pub type C1ISR = crate::Reg<c1isr::C1ISRrs>;
///HSEM Interrupt status register
pub mod c1isr;
/**C1MISR (r) register accessor: HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C1MISR)

For information about available fields see [`mod@c1misr`] module*/
pub type C1MISR = crate::Reg<c1misr::C1MISRrs>;
///HSEM Masked interrupt status register
pub mod c1misr;
/**C2IER (rw) register accessor: HSEM Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C2IER)

For information about available fields see [`mod@c2ier`] module*/
pub type C2IER = crate::Reg<c2ier::C2IERrs>;
///HSEM Interrupt enable register
pub mod c2ier;
/**C2ICR (rw) register accessor: HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C2ICR)

For information about available fields see [`mod@c2icr`] module*/
pub type C2ICR = crate::Reg<c2icr::C2ICRrs>;
///HSEM Interrupt clear register
pub mod c2icr;
/**C2ISR (r) register accessor: HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C2ISR)

For information about available fields see [`mod@c2isr`] module*/
pub type C2ISR = crate::Reg<c2isr::C2ISRrs>;
///HSEM Interrupt status register
pub mod c2isr;
/**C2MISR (r) register accessor: HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C2MISR)

For information about available fields see [`mod@c2misr`] module*/
pub type C2MISR = crate::Reg<c2misr::C2MISRrs>;
///HSEM Masked interrupt status register
pub mod c2misr;
/**CR (w) register accessor: HSEM Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///HSEM Clear register
pub mod cr;
/**KEYR (rw) register accessor: HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///HSEM Interrupt clear register
pub mod keyr;
