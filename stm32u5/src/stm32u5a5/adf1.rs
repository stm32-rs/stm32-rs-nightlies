#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    adf_gcr: ADF_GCR,
    adf_ckgcr: ADF_CKGCR,
    _reserved2: [u8; 0x78],
    adf_sitf0cr: ADF_SITF0CR,
    adf_bsmx0cr: ADF_BSMX0CR,
    adf_dflt0cr: ADF_DFLT0CR,
    adf_dflt0cicr: ADF_DFLT0CICR,
    adf_dflt0rsfr: ADF_DFLT0RSFR,
    _reserved7: [u8; 0x10],
    adf_dly0cr: ADF_DLY0CR,
    _reserved8: [u8; 0x04],
    adf_dflt0ier: ADF_DFLT0IER,
    adf_dflt0isr: ADF_DFLT0ISR,
    _reserved10: [u8; 0x04],
    adf_sadcr: ADF_SADCR,
    adf_sadcfgr: ADF_SADCFGR,
    adf_sadsdlvr: ADF_SADSDLVR,
    adf_sadanlvr: ADF_SADANLVR,
    _reserved14: [u8; 0x28],
    adf_dflt0dr: ADF_DFLT0DR,
}
impl RegisterBlock {
    ///0x00 - ADF Global Control Register
    #[inline(always)]
    pub const fn adf_gcr(&self) -> &ADF_GCR {
        &self.adf_gcr
    }
    ///0x04 - ADF clock generator control register
    #[inline(always)]
    pub const fn adf_ckgcr(&self) -> &ADF_CKGCR {
        &self.adf_ckgcr
    }
    ///0x80 - ADF serial interface control register 0
    #[inline(always)]
    pub const fn adf_sitf0cr(&self) -> &ADF_SITF0CR {
        &self.adf_sitf0cr
    }
    ///0x84 - ADF bitstream matrix control register 0
    #[inline(always)]
    pub const fn adf_bsmx0cr(&self) -> &ADF_BSMX0CR {
        &self.adf_bsmx0cr
    }
    ///0x88 - ADF digital filter control register 0
    #[inline(always)]
    pub const fn adf_dflt0cr(&self) -> &ADF_DFLT0CR {
        &self.adf_dflt0cr
    }
    ///0x8c - ADF digital filer configuration register 0
    #[inline(always)]
    pub const fn adf_dflt0cicr(&self) -> &ADF_DFLT0CICR {
        &self.adf_dflt0cicr
    }
    ///0x90 - ADF reshape filter configuration register 0
    #[inline(always)]
    pub const fn adf_dflt0rsfr(&self) -> &ADF_DFLT0RSFR {
        &self.adf_dflt0rsfr
    }
    ///0xa4 - ADF delay control register 0
    #[inline(always)]
    pub const fn adf_dly0cr(&self) -> &ADF_DLY0CR {
        &self.adf_dly0cr
    }
    ///0xac - ADF DFLT0 interrupt enable register
    #[inline(always)]
    pub const fn adf_dflt0ier(&self) -> &ADF_DFLT0IER {
        &self.adf_dflt0ier
    }
    ///0xb0 - ADF DFLT0 interrupt status register 0
    #[inline(always)]
    pub const fn adf_dflt0isr(&self) -> &ADF_DFLT0ISR {
        &self.adf_dflt0isr
    }
    ///0xb8 - ADF SAD control register
    #[inline(always)]
    pub const fn adf_sadcr(&self) -> &ADF_SADCR {
        &self.adf_sadcr
    }
    ///0xbc - ADF SAD configuration register
    #[inline(always)]
    pub const fn adf_sadcfgr(&self) -> &ADF_SADCFGR {
        &self.adf_sadcfgr
    }
    ///0xc0 - ADF SAD sound level register
    #[inline(always)]
    pub const fn adf_sadsdlvr(&self) -> &ADF_SADSDLVR {
        &self.adf_sadsdlvr
    }
    ///0xc4 - ADF SAD ambient noise level register
    #[inline(always)]
    pub const fn adf_sadanlvr(&self) -> &ADF_SADANLVR {
        &self.adf_sadanlvr
    }
    ///0xf0 - ADF digital filter data register 0
    #[inline(always)]
    pub const fn adf_dflt0dr(&self) -> &ADF_DFLT0DR {
        &self.adf_dflt0dr
    }
}
/**ADF_GCR (rw) register accessor: ADF Global Control Register

You can [`read`](crate::Reg::read) this register and get [`adf_gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_GCR)

For information about available fields see [`mod@adf_gcr`]
module*/
pub type ADF_GCR = crate::Reg<adf_gcr::ADF_GCRrs>;
///ADF Global Control Register
pub mod adf_gcr;
/**ADF_CKGCR (rw) register accessor: ADF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`adf_ckgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_ckgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_CKGCR)

For information about available fields see [`mod@adf_ckgcr`]
module*/
pub type ADF_CKGCR = crate::Reg<adf_ckgcr::ADF_CKGCRrs>;
///ADF clock generator control register
pub mod adf_ckgcr;
/**ADF_SITF0CR (rw) register accessor: ADF serial interface control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_sitf0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_sitf0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_SITF0CR)

For information about available fields see [`mod@adf_sitf0cr`]
module*/
pub type ADF_SITF0CR = crate::Reg<adf_sitf0cr::ADF_SITF0CRrs>;
///ADF serial interface control register 0
pub mod adf_sitf0cr;
/**ADF_BSMX0CR (rw) register accessor: ADF bitstream matrix control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_bsmx0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_bsmx0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_BSMX0CR)

For information about available fields see [`mod@adf_bsmx0cr`]
module*/
pub type ADF_BSMX0CR = crate::Reg<adf_bsmx0cr::ADF_BSMX0CRrs>;
///ADF bitstream matrix control register 0
pub mod adf_bsmx0cr;
/**ADF_DFLT0CR (rw) register accessor: ADF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0CR)

For information about available fields see [`mod@adf_dflt0cr`]
module*/
pub type ADF_DFLT0CR = crate::Reg<adf_dflt0cr::ADF_DFLT0CRrs>;
///ADF digital filter control register 0
pub mod adf_dflt0cr;
/**ADF_DFLT0CICR (rw) register accessor: ADF digital filer configuration register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0CICR)

For information about available fields see [`mod@adf_dflt0cicr`]
module*/
pub type ADF_DFLT0CICR = crate::Reg<adf_dflt0cicr::ADF_DFLT0CICRrs>;
///ADF digital filer configuration register 0
pub mod adf_dflt0cicr;
/**ADF_DFLT0RSFR (rw) register accessor: ADF reshape filter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0RSFR)

For information about available fields see [`mod@adf_dflt0rsfr`]
module*/
pub type ADF_DFLT0RSFR = crate::Reg<adf_dflt0rsfr::ADF_DFLT0RSFRrs>;
///ADF reshape filter configuration register 0
pub mod adf_dflt0rsfr;
/**ADF_DLY0CR (rw) register accessor: ADF delay control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dly0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dly0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DLY0CR)

For information about available fields see [`mod@adf_dly0cr`]
module*/
pub type ADF_DLY0CR = crate::Reg<adf_dly0cr::ADF_DLY0CRrs>;
///ADF delay control register 0
pub mod adf_dly0cr;
/**ADF_DFLT0IER (rw) register accessor: ADF DFLT0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0IER)

For information about available fields see [`mod@adf_dflt0ier`]
module*/
pub type ADF_DFLT0IER = crate::Reg<adf_dflt0ier::ADF_DFLT0IERrs>;
///ADF DFLT0 interrupt enable register
pub mod adf_dflt0ier;
/**ADF_DFLT0ISR (rw) register accessor: ADF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0ISR)

For information about available fields see [`mod@adf_dflt0isr`]
module*/
pub type ADF_DFLT0ISR = crate::Reg<adf_dflt0isr::ADF_DFLT0ISRrs>;
///ADF DFLT0 interrupt status register 0
pub mod adf_dflt0isr;
/**ADF_SADCR (rw) register accessor: ADF SAD control register

You can [`read`](crate::Reg::read) this register and get [`adf_sadcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_sadcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_SADCR)

For information about available fields see [`mod@adf_sadcr`]
module*/
pub type ADF_SADCR = crate::Reg<adf_sadcr::ADF_SADCRrs>;
///ADF SAD control register
pub mod adf_sadcr;
/**ADF_SADCFGR (rw) register accessor: ADF SAD configuration register

You can [`read`](crate::Reg::read) this register and get [`adf_sadcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_sadcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_SADCFGR)

For information about available fields see [`mod@adf_sadcfgr`]
module*/
pub type ADF_SADCFGR = crate::Reg<adf_sadcfgr::ADF_SADCFGRrs>;
///ADF SAD configuration register
pub mod adf_sadcfgr;
/**ADF_SADSDLVR (r) register accessor: ADF SAD sound level register

You can [`read`](crate::Reg::read) this register and get [`adf_sadsdlvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_SADSDLVR)

For information about available fields see [`mod@adf_sadsdlvr`]
module*/
pub type ADF_SADSDLVR = crate::Reg<adf_sadsdlvr::ADF_SADSDLVRrs>;
///ADF SAD sound level register
pub mod adf_sadsdlvr;
/**ADF_SADANLVR (r) register accessor: ADF SAD ambient noise level register

You can [`read`](crate::Reg::read) this register and get [`adf_sadanlvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_SADANLVR)

For information about available fields see [`mod@adf_sadanlvr`]
module*/
pub type ADF_SADANLVR = crate::Reg<adf_sadanlvr::ADF_SADANLVRrs>;
///ADF SAD ambient noise level register
pub mod adf_sadanlvr;
/**ADF_DFLT0DR (r) register accessor: ADF digital filter data register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:ADF_DFLT0DR)

For information about available fields see [`mod@adf_dflt0dr`]
module*/
pub type ADF_DFLT0DR = crate::Reg<adf_dflt0dr::ADF_DFLT0DRrs>;
///ADF digital filter data register 0
pub mod adf_dflt0dr;
