#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dcmi_cr: DCMI_CR,
    dcmi_sr: DCMI_SR,
    dcmi_ris: DCMI_RIS,
    dcmi_ier: DCMI_IER,
    dcmi_mis: DCMI_MIS,
    dcmi_icr: DCMI_ICR,
    dcmi_escr: DCMI_ESCR,
    dcmi_esur: DCMI_ESUR,
    dcmi_cwstrt: DCMI_CWSTRT,
    dcmi_cwsize: DCMI_CWSIZE,
    dcmi_dr: DCMI_DR,
}
impl RegisterBlock {
    ///0x00 - DCMI control register
    #[inline(always)]
    pub const fn dcmi_cr(&self) -> &DCMI_CR {
        &self.dcmi_cr
    }
    ///0x04 - DCMI status register
    #[inline(always)]
    pub const fn dcmi_sr(&self) -> &DCMI_SR {
        &self.dcmi_sr
    }
    ///0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
    #[inline(always)]
    pub const fn dcmi_ris(&self) -> &DCMI_RIS {
        &self.dcmi_ris
    }
    ///0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
    #[inline(always)]
    pub const fn dcmi_ier(&self) -> &DCMI_IER {
        &self.dcmi_ier
    }
    ///0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
    #[inline(always)]
    pub const fn dcmi_mis(&self) -> &DCMI_MIS {
        &self.dcmi_mis
    }
    ///0x14 - The DCMI_ICR register is write-only.
    #[inline(always)]
    pub const fn dcmi_icr(&self) -> &DCMI_ICR {
        &self.dcmi_icr
    }
    ///0x18 - DCMI embedded synchronization code register
    #[inline(always)]
    pub const fn dcmi_escr(&self) -> &DCMI_ESCR {
        &self.dcmi_escr
    }
    ///0x1c - DCMI embedded synchronization unmask register
    #[inline(always)]
    pub const fn dcmi_esur(&self) -> &DCMI_ESUR {
        &self.dcmi_esur
    }
    ///0x20 - DCMI crop window start
    #[inline(always)]
    pub const fn dcmi_cwstrt(&self) -> &DCMI_CWSTRT {
        &self.dcmi_cwstrt
    }
    ///0x24 - DCMI crop window size
    #[inline(always)]
    pub const fn dcmi_cwsize(&self) -> &DCMI_CWSIZE {
        &self.dcmi_cwsize
    }
    ///0x28 - DCMI data register
    #[inline(always)]
    pub const fn dcmi_dr(&self) -> &DCMI_DR {
        &self.dcmi_dr
    }
}
/**DCMI_CR (rw) register accessor: DCMI control register

You can [`read`](crate::Reg::read) this register and get [`dcmi_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_CR)

For information about available fields see [`mod@dcmi_cr`]
module*/
pub type DCMI_CR = crate::Reg<dcmi_cr::DCMI_CRrs>;
///DCMI control register
pub mod dcmi_cr;
/**DCMI_SR (r) register accessor: DCMI status register

You can [`read`](crate::Reg::read) this register and get [`dcmi_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_SR)

For information about available fields see [`mod@dcmi_sr`]
module*/
pub type DCMI_SR = crate::Reg<dcmi_sr::DCMI_SRrs>;
///DCMI status register
pub mod dcmi_sr;
/**DCMI_RIS (r) register accessor: DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.

You can [`read`](crate::Reg::read) this register and get [`dcmi_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_RIS)

For information about available fields see [`mod@dcmi_ris`]
module*/
pub type DCMI_RIS = crate::Reg<dcmi_ris::DCMI_RISrs>;
///DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
pub mod dcmi_ris;
/**DCMI_IER (rw) register accessor: The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.

You can [`read`](crate::Reg::read) this register and get [`dcmi_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_IER)

For information about available fields see [`mod@dcmi_ier`]
module*/
pub type DCMI_IER = crate::Reg<dcmi_ier::DCMI_IERrs>;
///The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
pub mod dcmi_ier;
/**DCMI_MIS (r) register accessor: This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.

You can [`read`](crate::Reg::read) this register and get [`dcmi_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_MIS)

For information about available fields see [`mod@dcmi_mis`]
module*/
pub type DCMI_MIS = crate::Reg<dcmi_mis::DCMI_MISrs>;
///This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
pub mod dcmi_mis;
/**DCMI_ICR (w) register accessor: The DCMI_ICR register is write-only.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_ICR)

For information about available fields see [`mod@dcmi_icr`]
module*/
pub type DCMI_ICR = crate::Reg<dcmi_icr::DCMI_ICRrs>;
///The DCMI_ICR register is write-only.
pub mod dcmi_icr;
/**DCMI_ESCR (rw) register accessor: DCMI embedded synchronization code register

You can [`read`](crate::Reg::read) this register and get [`dcmi_escr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_escr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_ESCR)

For information about available fields see [`mod@dcmi_escr`]
module*/
pub type DCMI_ESCR = crate::Reg<dcmi_escr::DCMI_ESCRrs>;
///DCMI embedded synchronization code register
pub mod dcmi_escr;
/**DCMI_ESUR (rw) register accessor: DCMI embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`dcmi_esur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_esur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_ESUR)

For information about available fields see [`mod@dcmi_esur`]
module*/
pub type DCMI_ESUR = crate::Reg<dcmi_esur::DCMI_ESURrs>;
///DCMI embedded synchronization unmask register
pub mod dcmi_esur;
/**DCMI_CWSTRT (rw) register accessor: DCMI crop window start

You can [`read`](crate::Reg::read) this register and get [`dcmi_cwstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_cwstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_CWSTRT)

For information about available fields see [`mod@dcmi_cwstrt`]
module*/
pub type DCMI_CWSTRT = crate::Reg<dcmi_cwstrt::DCMI_CWSTRTrs>;
///DCMI crop window start
pub mod dcmi_cwstrt;
/**DCMI_CWSIZE (rw) register accessor: DCMI crop window size

You can [`read`](crate::Reg::read) this register and get [`dcmi_cwsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_cwsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_CWSIZE)

For information about available fields see [`mod@dcmi_cwsize`]
module*/
pub type DCMI_CWSIZE = crate::Reg<dcmi_cwsize::DCMI_CWSIZErs>;
///DCMI crop window size
pub mod dcmi_cwsize;
/**DCMI_DR (r) register accessor: DCMI data register

You can [`read`](crate::Reg::read) this register and get [`dcmi_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DCMI_DR)

For information about available fields see [`mod@dcmi_dr`]
module*/
pub type DCMI_DR = crate::Reg<dcmi_dr::DCMI_DRrs>;
///DCMI data register
pub mod dcmi_dr;
