#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    ris: RIS,
    ier: IER,
    mis: MIS,
    icr: ICR,
    escr: ESCR,
    esur: ESUR,
    cwstrt: CWSTRT,
    cwsize: CWSIZE,
    dr: DR,
}
impl RegisterBlock {
    ///0x00 - DCMI control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DCMI status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
    #[inline(always)]
    pub const fn ris(&self) -> &RIS {
        &self.ris
    }
    ///0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
    #[inline(always)]
    pub const fn mis(&self) -> &MIS {
        &self.mis
    }
    ///0x14 - The DCMI_ICR register is write-only.
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x18 - DCMI embedded synchronization code register
    #[inline(always)]
    pub const fn escr(&self) -> &ESCR {
        &self.escr
    }
    ///0x1c - DCMI embedded synchronization unmask register
    #[inline(always)]
    pub const fn esur(&self) -> &ESUR {
        &self.esur
    }
    ///0x20 - DCMI crop window start
    #[inline(always)]
    pub const fn cwstrt(&self) -> &CWSTRT {
        &self.cwstrt
    }
    ///0x24 - DCMI crop window size
    #[inline(always)]
    pub const fn cwsize(&self) -> &CWSIZE {
        &self.cwsize
    }
    ///0x28 - DCMI data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
/**CR (rw) register accessor: DCMI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DCMI control register
pub mod cr;
/**SR (r) register accessor: DCMI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DCMI status register
pub mod sr;
/**RIS (r) register accessor: DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.

You can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:RIS)

For information about available fields see [`mod@ris`] module*/
pub type RIS = crate::Reg<ris::RISrs>;
///DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.
pub mod ris;
/**IER (rw) register accessor: The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.
pub mod ier;
/**MIS (r) register accessor: This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.

You can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:MIS)

For information about available fields see [`mod@mis`] module*/
pub type MIS = crate::Reg<mis::MISrs>;
///This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
pub mod mis;
/**ICR (w) register accessor: The DCMI_ICR register is write-only.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///The DCMI_ICR register is write-only.
pub mod icr;
/**ESCR (rw) register accessor: DCMI embedded synchronization code register

You can [`read`](crate::Reg::read) this register and get [`escr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`escr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:ESCR)

For information about available fields see [`mod@escr`] module*/
pub type ESCR = crate::Reg<escr::ESCRrs>;
///DCMI embedded synchronization code register
pub mod escr;
/**ESUR (rw) register accessor: DCMI embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`esur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:ESUR)

For information about available fields see [`mod@esur`] module*/
pub type ESUR = crate::Reg<esur::ESURrs>;
///DCMI embedded synchronization unmask register
pub mod esur;
/**CWSTRT (rw) register accessor: DCMI crop window start

You can [`read`](crate::Reg::read) this register and get [`cwstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:CWSTRT)

For information about available fields see [`mod@cwstrt`] module*/
pub type CWSTRT = crate::Reg<cwstrt::CWSTRTrs>;
///DCMI crop window start
pub mod cwstrt;
/**CWSIZE (rw) register accessor: DCMI crop window size

You can [`read`](crate::Reg::read) this register and get [`cwsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:CWSIZE)

For information about available fields see [`mod@cwsize`] module*/
pub type CWSIZE = crate::Reg<cwsize::CWSIZErs>;
///DCMI crop window size
pub mod cwsize;
/**DR (r) register accessor: DCMI data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///DCMI data register
pub mod dr;
