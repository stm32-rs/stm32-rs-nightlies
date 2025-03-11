#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    dcr: DCR,
    sr: SR,
    fcr: FCR,
    dlr: DLR,
    ccr: CCR,
    ar: AR,
    abr: ABR,
    dr: DR,
    psmkr: PSMKR,
    psmar: PSMAR,
    pir: PIR,
    lptr: LPTR,
    _reserved13: [u8; 0x03bc],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - QUADSPI control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - QUADSPI device configuration register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x08 - QUADSPI status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - QUADSPI flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - QUADSPI data length register
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    ///0x14 - QUADSPI communication configuration register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x18 - QUADSPI address register
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    ///0x1c - QUADSPI alternate bytes registers
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    ///0x20 - QUADSPI data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x24 - QUADSPI polling status mask register
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    ///0x28 - QUADSPI polling status match register
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    ///0x2c - QUADSPI polling interval register
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    ///0x30 - QUADSPI low-power timeout register
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
    ///0x3f0 - QUADSPI HW configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - QUADSPI version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - QUADSPI identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - QUADSPI size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR (rw) register accessor: QUADSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///QUADSPI control register
pub mod cr;
/**DCR (rw) register accessor: QUADSPI device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///QUADSPI device configuration register
pub mod dcr;
/**SR (r) register accessor: QUADSPI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///QUADSPI status register
pub mod sr;
/**FCR (w) register accessor: QUADSPI flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///QUADSPI flag clear register
pub mod fcr;
/**DLR (rw) register accessor: QUADSPI data length register

You can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:DLR)

For information about available fields see [`mod@dlr`] module*/
pub type DLR = crate::Reg<dlr::DLRrs>;
///QUADSPI data length register
pub mod dlr;
/**CCR (rw) register accessor: QUADSPI communication configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///QUADSPI communication configuration register
pub mod ccr;
/**AR (rw) register accessor: QUADSPI address register

You can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:AR)

For information about available fields see [`mod@ar`] module*/
pub type AR = crate::Reg<ar::ARrs>;
///QUADSPI address register
pub mod ar;
/**ABR (rw) register accessor: QUADSPI alternate bytes registers

You can [`read`](crate::Reg::read) this register and get [`abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:ABR)

For information about available fields see [`mod@abr`] module*/
pub type ABR = crate::Reg<abr::ABRrs>;
///QUADSPI alternate bytes registers
pub mod abr;
/**DR (rw) register accessor: QUADSPI data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///QUADSPI data register
pub mod dr;
/**PSMKR (rw) register accessor: QUADSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`psmkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:PSMKR)

For information about available fields see [`mod@psmkr`] module*/
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
///QUADSPI polling status mask register
pub mod psmkr;
/**PSMAR (rw) register accessor: QUADSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`psmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:PSMAR)

For information about available fields see [`mod@psmar`] module*/
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
///QUADSPI polling status match register
pub mod psmar;
/**PIR (rw) register accessor: QUADSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:PIR)

For information about available fields see [`mod@pir`] module*/
pub type PIR = crate::Reg<pir::PIRrs>;
///QUADSPI polling interval register
pub mod pir;
/**LPTR (rw) register accessor: QUADSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`lptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:LPTR)

For information about available fields see [`mod@lptr`] module*/
pub type LPTR = crate::Reg<lptr::LPTRrs>;
///QUADSPI low-power timeout register
pub mod lptr;
/**HWCFGR (r) register accessor: QUADSPI HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///QUADSPI HW configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: QUADSPI version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///QUADSPI version register
pub mod verr;
/**IPIDR (r) register accessor: QUADSPI identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///QUADSPI identification register
pub mod ipidr;
/**SIDR (r) register accessor: QUADSPI size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///QUADSPI size identification register
pub mod sidr;
