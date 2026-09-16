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
    _reserved_8_dr: [u8; 0x04],
    psmkr: PSMKR,
    psmar: PSMAR,
    pir: PIR,
    lptr: LPTR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - device configuration register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x08 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - data length register
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    ///0x14 - communication configuration register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x18 - address register
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    ///0x1c - ABR
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    ///0x20 - Data register: one byte (8 bit) access
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Data register: half word (16 bit) access
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Data register: full word (32 bit) access
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - polling status mask register
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    ///0x28 - polling status match register
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    ///0x2c - polling interval register
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    ///0x30 - low-power timeout register
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**DCR (rw) register accessor: device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///device configuration register
pub mod dcr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**FCR (rw) register accessor: flag clear register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///flag clear register
pub mod fcr;
/**DLR (rw) register accessor: data length register

You can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:DLR)

For information about available fields see [`mod@dlr`] module*/
pub type DLR = crate::Reg<dlr::DLRrs>;
///data length register
pub mod dlr;
/**CCR (rw) register accessor: communication configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///communication configuration register
pub mod ccr;
/**AR (rw) register accessor: address register

You can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:AR)

For information about available fields see [`mod@ar`] module*/
pub type AR = crate::Reg<ar::ARrs>;
///address register
pub mod ar;
/**ABR (rw) register accessor: ABR

You can [`read`](crate::Reg::read) this register and get [`abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:ABR)

For information about available fields see [`mod@abr`] module*/
pub type ABR = crate::Reg<abr::ABRrs>;
///ABR
pub mod abr;
/**DR (rw) register accessor: Data register: full word (32 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///Data register: full word (32 bit) access
pub mod dr;
/**DR16 (rw) register accessor: Data register: half word (16 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:DR16)

For information about available fields see [`mod@dr16`] module*/
pub type DR16 = crate::Reg<dr16::DR16rs>;
///Data register: half word (16 bit) access
pub mod dr16;
/**DR8 (rw) register accessor: Data register: one byte (8 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:DR8)

For information about available fields see [`mod@dr8`] module*/
pub type DR8 = crate::Reg<dr8::DR8rs>;
///Data register: one byte (8 bit) access
pub mod dr8;
/**PSMKR (rw) register accessor: polling status mask register

You can [`read`](crate::Reg::read) this register and get [`psmkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:PSMKR)

For information about available fields see [`mod@psmkr`] module*/
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
///polling status mask register
pub mod psmkr;
/**PSMAR (rw) register accessor: polling status match register

You can [`read`](crate::Reg::read) this register and get [`psmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:PSMAR)

For information about available fields see [`mod@psmar`] module*/
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
///polling status match register
pub mod psmar;
/**PIR (rw) register accessor: polling interval register

You can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:PIR)

For information about available fields see [`mod@pir`] module*/
pub type PIR = crate::Reg<pir::PIRrs>;
///polling interval register
pub mod pir;
/**LPTR (rw) register accessor: low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`lptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#QUADSPI:LPTR)

For information about available fields see [`mod@lptr`] module*/
pub type LPTR = crate::Reg<lptr::LPTRrs>;
///low-power timeout register
pub mod lptr;
