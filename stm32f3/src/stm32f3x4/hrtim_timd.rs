#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    timdcr: TIMDCR,
    timdisr: TIMDISR,
    timdicr: TIMDICR,
    timddier: TIMDDIER,
    cntdr: CNTDR,
    perdr: PERDR,
    repdr: REPDR,
    cmp1dr: CMP1DR,
    cmp1cdr: CMP1CDR,
    cmp2dr: CMP2DR,
    cmp3dr: CMP3DR,
    cmp4dr: CMP4DR,
    cpt1dr: CPT1DR,
    cpt2dr: CPT2DR,
    dtdr: DTDR,
    setd1r: SETD1R,
    rstd1r: RSTD1R,
    setd2r: SETD2R,
    rstd2r: RSTD2R,
    eefdr1: EEFDR1,
    eefdr2: EEFDR2,
    rstdr: RSTDR,
    chpdr: CHPDR,
    cpt1dcr: CPT1DCR,
    cpt2dcr: CPT2DCR,
    outdr: OUTDR,
    fltdr: FLTDR,
}
impl RegisterBlock {
    ///0x00 - Timerx Control Register
    #[inline(always)]
    pub const fn timdcr(&self) -> &TIMDCR {
        &self.timdcr
    }
    ///0x04 - Timerx Interrupt Status Register
    #[inline(always)]
    pub const fn timdisr(&self) -> &TIMDISR {
        &self.timdisr
    }
    ///0x08 - Timerx Interrupt Clear Register
    #[inline(always)]
    pub const fn timdicr(&self) -> &TIMDICR {
        &self.timdicr
    }
    ///0x0c - TIMxDIER5
    #[inline(always)]
    pub const fn timddier(&self) -> &TIMDDIER {
        &self.timddier
    }
    ///0x10 - Timerx Counter Register
    #[inline(always)]
    pub const fn cntdr(&self) -> &CNTDR {
        &self.cntdr
    }
    ///0x14 - Timerx Period Register
    #[inline(always)]
    pub const fn perdr(&self) -> &PERDR {
        &self.perdr
    }
    ///0x18 - Timerx Repetition Register
    #[inline(always)]
    pub const fn repdr(&self) -> &REPDR {
        &self.repdr
    }
    ///0x1c - Timerx Compare 1 Register
    #[inline(always)]
    pub const fn cmp1dr(&self) -> &CMP1DR {
        &self.cmp1dr
    }
    ///0x20 - Timerx Compare 1 Compound Register
    #[inline(always)]
    pub const fn cmp1cdr(&self) -> &CMP1CDR {
        &self.cmp1cdr
    }
    ///0x24 - Timerx Compare 2 Register
    #[inline(always)]
    pub const fn cmp2dr(&self) -> &CMP2DR {
        &self.cmp2dr
    }
    ///0x28 - Timerx Compare 3 Register
    #[inline(always)]
    pub const fn cmp3dr(&self) -> &CMP3DR {
        &self.cmp3dr
    }
    ///0x2c - Timerx Compare 4 Register
    #[inline(always)]
    pub const fn cmp4dr(&self) -> &CMP4DR {
        &self.cmp4dr
    }
    ///0x30 - Timerx Capture 1 Register
    #[inline(always)]
    pub const fn cpt1dr(&self) -> &CPT1DR {
        &self.cpt1dr
    }
    ///0x34 - Timerx Capture 2 Register
    #[inline(always)]
    pub const fn cpt2dr(&self) -> &CPT2DR {
        &self.cpt2dr
    }
    ///0x38 - Timerx Deadtime Register
    #[inline(always)]
    pub const fn dtdr(&self) -> &DTDR {
        &self.dtdr
    }
    ///0x3c - Timerx Output1 Set Register
    #[inline(always)]
    pub const fn setd1r(&self) -> &SETD1R {
        &self.setd1r
    }
    ///0x40 - Timerx Output1 Reset Register
    #[inline(always)]
    pub const fn rstd1r(&self) -> &RSTD1R {
        &self.rstd1r
    }
    ///0x44 - Timerx Output2 Set Register
    #[inline(always)]
    pub const fn setd2r(&self) -> &SETD2R {
        &self.setd2r
    }
    ///0x48 - Timerx Output2 Reset Register
    #[inline(always)]
    pub const fn rstd2r(&self) -> &RSTD2R {
        &self.rstd2r
    }
    ///0x4c - Timerx External Event Filtering Register 1
    #[inline(always)]
    pub const fn eefdr1(&self) -> &EEFDR1 {
        &self.eefdr1
    }
    ///0x50 - Timerx External Event Filtering Register 2
    #[inline(always)]
    pub const fn eefdr2(&self) -> &EEFDR2 {
        &self.eefdr2
    }
    ///0x54 - TimerA Reset Register
    #[inline(always)]
    pub const fn rstdr(&self) -> &RSTDR {
        &self.rstdr
    }
    ///0x58 - Timerx Chopper Register
    #[inline(always)]
    pub const fn chpdr(&self) -> &CHPDR {
        &self.chpdr
    }
    ///0x5c - Timerx Capture 2 Control Register
    #[inline(always)]
    pub const fn cpt1dcr(&self) -> &CPT1DCR {
        &self.cpt1dcr
    }
    ///0x60 - CPT2xCR
    #[inline(always)]
    pub const fn cpt2dcr(&self) -> &CPT2DCR {
        &self.cpt2dcr
    }
    ///0x64 - Timerx Output Register
    #[inline(always)]
    pub const fn outdr(&self) -> &OUTDR {
        &self.outdr
    }
    ///0x68 - Timerx Fault Register
    #[inline(always)]
    pub const fn fltdr(&self) -> &FLTDR {
        &self.fltdr
    }
}
/**TIMDCR (rw) register accessor: Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:TIMDCR)

For information about available fields see [`mod@timdcr`]
module*/
pub type TIMDCR = crate::Reg<timdcr::TIMDCRrs>;
///Timerx Control Register
pub mod timdcr;
/**TIMDISR (r) register accessor: Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timdisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:TIMDISR)

For information about available fields see [`mod@timdisr`]
module*/
pub type TIMDISR = crate::Reg<timdisr::TIMDISRrs>;
///Timerx Interrupt Status Register
pub mod timdisr;
/**TIMDICR (w) register accessor: Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timdicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:TIMDICR)

For information about available fields see [`mod@timdicr`]
module*/
pub type TIMDICR = crate::Reg<timdicr::TIMDICRrs>;
///Timerx Interrupt Clear Register
pub mod timdicr;
/**TIMDDIER (rw) register accessor: TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`timddier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timddier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:TIMDDIER)

For information about available fields see [`mod@timddier`]
module*/
pub type TIMDDIER = crate::Reg<timddier::TIMDDIERrs>;
///TIMxDIER5
pub mod timddier;
/**CNTDR (rw) register accessor: Timerx Counter Register

You can [`read`](crate::Reg::read) this register and get [`cntdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CNTDR)

For information about available fields see [`mod@cntdr`]
module*/
pub type CNTDR = crate::Reg<cntdr::CNTDRrs>;
///Timerx Counter Register
pub mod cntdr;
/**PERDR (rw) register accessor: Timerx Period Register

You can [`read`](crate::Reg::read) this register and get [`perdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:PERDR)

For information about available fields see [`mod@perdr`]
module*/
pub type PERDR = crate::Reg<perdr::PERDRrs>;
///Timerx Period Register
pub mod perdr;
/**REPDR (rw) register accessor: Timerx Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:REPDR)

For information about available fields see [`mod@repdr`]
module*/
pub type REPDR = crate::Reg<repdr::REPDRrs>;
///Timerx Repetition Register
pub mod repdr;
/**CMP1DR (rw) register accessor: Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CMP1DR)

For information about available fields see [`mod@cmp1dr`]
module*/
pub type CMP1DR = crate::Reg<cmp1dr::CMP1DRrs>;
///Timerx Compare 1 Register
pub mod cmp1dr;
/**CMP1CDR (rw) register accessor: Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CMP1CDR)

For information about available fields see [`mod@cmp1cdr`]
module*/
pub type CMP1CDR = crate::Reg<cmp1cdr::CMP1CDRrs>;
///Timerx Compare 1 Compound Register
pub mod cmp1cdr;
/**CMP2DR (rw) register accessor: Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CMP2DR)

For information about available fields see [`mod@cmp2dr`]
module*/
pub type CMP2DR = crate::Reg<cmp2dr::CMP2DRrs>;
///Timerx Compare 2 Register
pub mod cmp2dr;
/**CMP3DR (rw) register accessor: Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CMP3DR)

For information about available fields see [`mod@cmp3dr`]
module*/
pub type CMP3DR = crate::Reg<cmp3dr::CMP3DRrs>;
///Timerx Compare 3 Register
pub mod cmp3dr;
/**CMP4DR (rw) register accessor: Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CMP4DR)

For information about available fields see [`mod@cmp4dr`]
module*/
pub type CMP4DR = crate::Reg<cmp4dr::CMP4DRrs>;
///Timerx Compare 4 Register
pub mod cmp4dr;
/**CPT1DR (r) register accessor: Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CPT1DR)

For information about available fields see [`mod@cpt1dr`]
module*/
pub type CPT1DR = crate::Reg<cpt1dr::CPT1DRrs>;
///Timerx Capture 1 Register
pub mod cpt1dr;
/**CPT2DR (r) register accessor: Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CPT2DR)

For information about available fields see [`mod@cpt2dr`]
module*/
pub type CPT2DR = crate::Reg<cpt2dr::CPT2DRrs>;
///Timerx Capture 2 Register
pub mod cpt2dr;
/**DTDR (rw) register accessor: Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:DTDR)

For information about available fields see [`mod@dtdr`]
module*/
pub type DTDR = crate::Reg<dtdr::DTDRrs>;
///Timerx Deadtime Register
pub mod dtdr;
/**SETD1R (rw) register accessor: Timerx Output1 Set Register

You can [`read`](crate::Reg::read) this register and get [`setd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:SETD1R)

For information about available fields see [`mod@setd1r`]
module*/
pub type SETD1R = crate::Reg<setd1r::SETD1Rrs>;
///Timerx Output1 Set Register
pub mod setd1r;
/**RSTD1R (rw) register accessor: Timerx Output1 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:RSTD1R)

For information about available fields see [`mod@rstd1r`]
module*/
pub type RSTD1R = crate::Reg<rstd1r::RSTD1Rrs>;
///Timerx Output1 Reset Register
pub mod rstd1r;
/**SETD2R (rw) register accessor: Timerx Output2 Set Register

You can [`read`](crate::Reg::read) this register and get [`setd2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setd2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:SETD2R)

For information about available fields see [`mod@setd2r`]
module*/
pub type SETD2R = crate::Reg<setd2r::SETD2Rrs>;
///Timerx Output2 Set Register
pub mod setd2r;
/**RSTD2R (rw) register accessor: Timerx Output2 Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstd2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstd2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:RSTD2R)

For information about available fields see [`mod@rstd2r`]
module*/
pub type RSTD2R = crate::Reg<rstd2r::RSTD2Rrs>;
///Timerx Output2 Reset Register
pub mod rstd2r;
/**EEFDR1 (rw) register accessor: Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:EEFDR1)

For information about available fields see [`mod@eefdr1`]
module*/
pub type EEFDR1 = crate::Reg<eefdr1::EEFDR1rs>;
///Timerx External Event Filtering Register 1
pub mod eefdr1;
/**EEFDR2 (rw) register accessor: Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:EEFDR2)

For information about available fields see [`mod@eefdr2`]
module*/
pub type EEFDR2 = crate::Reg<eefdr2::EEFDR2rs>;
///Timerx External Event Filtering Register 2
pub mod eefdr2;
/**RSTDR (rw) register accessor: TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:RSTDR)

For information about available fields see [`mod@rstdr`]
module*/
pub type RSTDR = crate::Reg<rstdr::RSTDRrs>;
///TimerA Reset Register
pub mod rstdr;
/**CHPDR (rw) register accessor: Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CHPDR)

For information about available fields see [`mod@chpdr`]
module*/
pub type CHPDR = crate::Reg<chpdr::CHPDRrs>;
///Timerx Chopper Register
pub mod chpdr;
/**CPT1DCR (rw) register accessor: Timerx Capture 2 Control Register

You can [`read`](crate::Reg::read) this register and get [`cpt1dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt1dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CPT1DCR)

For information about available fields see [`mod@cpt1dcr`]
module*/
pub type CPT1DCR = crate::Reg<cpt1dcr::CPT1DCRrs>;
///Timerx Capture 2 Control Register
pub mod cpt1dcr;
/**CPT2DCR (rw) register accessor: CPT2xCR

You can [`read`](crate::Reg::read) this register and get [`cpt2dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpt2dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CPT2DCR)

For information about available fields see [`mod@cpt2dcr`]
module*/
pub type CPT2DCR = crate::Reg<cpt2dcr::CPT2DCRrs>;
///CPT2xCR
pub mod cpt2dcr;
/**OUTDR (rw) register accessor: Timerx Output Register

You can [`read`](crate::Reg::read) this register and get [`outdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:OUTDR)

For information about available fields see [`mod@outdr`]
module*/
pub type OUTDR = crate::Reg<outdr::OUTDRrs>;
///Timerx Output Register
pub mod outdr;
/**FLTDR (rw) register accessor: Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:FLTDR)

For information about available fields see [`mod@fltdr`]
module*/
pub type FLTDR = crate::Reg<fltdr::FLTDRrs>;
///Timerx Fault Register
pub mod fltdr;
