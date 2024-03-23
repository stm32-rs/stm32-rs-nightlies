#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hspi_cr: HSPI_CR,
    _reserved1: [u8; 0x04],
    hspi_dcr1: HSPI_DCR1,
    hspi_dcr2: HSPI_DCR2,
    hspi_dcr3: HSPI_DCR3,
    hspi_dcr4: HSPI_DCR4,
    _reserved5: [u8; 0x08],
    hspi_sr: HSPI_SR,
    hspi_fcr: HSPI_FCR,
    _reserved7: [u8; 0x18],
    hspi_dlr: HSPI_DLR,
    _reserved8: [u8; 0x04],
    hspi_ar: HSPI_AR,
    _reserved9: [u8; 0x04],
    hspi_dr: HSPI_DR,
    _reserved10: [u8; 0x2c],
    hspi_psmkr: HSPI_PSMKR,
    _reserved11: [u8; 0x04],
    hspi_psmar: HSPI_PSMAR,
    _reserved12: [u8; 0x04],
    hspi_pir: HSPI_PIR,
    _reserved13: [u8; 0x6c],
    hspi_ccr: HSPI_CCR,
    _reserved14: [u8; 0x04],
    hspi_tcr: HSPI_TCR,
    _reserved15: [u8; 0x04],
    hspi_ir: HSPI_IR,
    _reserved16: [u8; 0x0c],
    hspi_abr: HSPI_ABR,
    _reserved17: [u8; 0x0c],
    hspi_lptr: HSPI_LPTR,
    _reserved18: [u8; 0x0c],
    hspi_wpccr: HSPI_WPCCR,
    _reserved19: [u8; 0x04],
    hspi_wptcr: HSPI_WPTCR,
    _reserved20: [u8; 0x04],
    hspi_wpir: HSPI_WPIR,
    _reserved21: [u8; 0x0c],
    hspi_wpabr: HSPI_WPABR,
    _reserved22: [u8; 0x1c],
    hspi_wccr: HSPI_WCCR,
    _reserved23: [u8; 0x04],
    hspi_wtcr: HSPI_WTCR,
    _reserved24: [u8; 0x04],
    hspi_wir: HSPI_WIR,
    _reserved25: [u8; 0x0c],
    hspi_wabr: HSPI_WABR,
    _reserved26: [u8; 0x5c],
    hspi_hlcr: HSPI_HLCR,
    _reserved27: [u8; 0x0c],
    hspi_calfcr: HSPI_CALFCR,
    _reserved28: [u8; 0x04],
    hspi_calmr: HSPI_CALMR,
    _reserved29: [u8; 0x04],
    hspi_calsor: HSPI_CALSOR,
    _reserved30: [u8; 0x04],
    hspi_calsir: HSPI_CALSIR,
}
impl RegisterBlock {
    #[doc = "0x00 - HSPI control register"]
    #[inline(always)]
    pub const fn hspi_cr(&self) -> &HSPI_CR {
        &self.hspi_cr
    }
    #[doc = "0x08 - HSPI device configuration register 1"]
    #[inline(always)]
    pub const fn hspi_dcr1(&self) -> &HSPI_DCR1 {
        &self.hspi_dcr1
    }
    #[doc = "0x0c - HSPI device configuration register 2"]
    #[inline(always)]
    pub const fn hspi_dcr2(&self) -> &HSPI_DCR2 {
        &self.hspi_dcr2
    }
    #[doc = "0x10 - HSPI device configuration register 3"]
    #[inline(always)]
    pub const fn hspi_dcr3(&self) -> &HSPI_DCR3 {
        &self.hspi_dcr3
    }
    #[doc = "0x14 - HSPI device configuration register 4"]
    #[inline(always)]
    pub const fn hspi_dcr4(&self) -> &HSPI_DCR4 {
        &self.hspi_dcr4
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn hspi_sr(&self) -> &HSPI_SR {
        &self.hspi_sr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn hspi_fcr(&self) -> &HSPI_FCR {
        &self.hspi_fcr
    }
    #[doc = "0x40 - HSPI data length register"]
    #[inline(always)]
    pub const fn hspi_dlr(&self) -> &HSPI_DLR {
        &self.hspi_dlr
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn hspi_ar(&self) -> &HSPI_AR {
        &self.hspi_ar
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn hspi_dr(&self) -> &HSPI_DR {
        &self.hspi_dr
    }
    #[doc = "0x80 - HSPI polling status mask register"]
    #[inline(always)]
    pub const fn hspi_psmkr(&self) -> &HSPI_PSMKR {
        &self.hspi_psmkr
    }
    #[doc = "0x88 - HSPI polling status match register"]
    #[inline(always)]
    pub const fn hspi_psmar(&self) -> &HSPI_PSMAR {
        &self.hspi_psmar
    }
    #[doc = "0x90 - HSPI polling interval register"]
    #[inline(always)]
    pub const fn hspi_pir(&self) -> &HSPI_PIR {
        &self.hspi_pir
    }
    #[doc = "0x100 - HSPI communication configuration register"]
    #[inline(always)]
    pub const fn hspi_ccr(&self) -> &HSPI_CCR {
        &self.hspi_ccr
    }
    #[doc = "0x108 - HSPI timing configuration register"]
    #[inline(always)]
    pub const fn hspi_tcr(&self) -> &HSPI_TCR {
        &self.hspi_tcr
    }
    #[doc = "0x110 - HSPI instruction register"]
    #[inline(always)]
    pub const fn hspi_ir(&self) -> &HSPI_IR {
        &self.hspi_ir
    }
    #[doc = "0x120 - HSPI alternate bytes register"]
    #[inline(always)]
    pub const fn hspi_abr(&self) -> &HSPI_ABR {
        &self.hspi_abr
    }
    #[doc = "0x130 - HSPI low-power timeout register"]
    #[inline(always)]
    pub const fn hspi_lptr(&self) -> &HSPI_LPTR {
        &self.hspi_lptr
    }
    #[doc = "0x140 - HSPI wrap communication configuration register"]
    #[inline(always)]
    pub const fn hspi_wpccr(&self) -> &HSPI_WPCCR {
        &self.hspi_wpccr
    }
    #[doc = "0x148 - HSPI wrap timing configuration register"]
    #[inline(always)]
    pub const fn hspi_wptcr(&self) -> &HSPI_WPTCR {
        &self.hspi_wptcr
    }
    #[doc = "0x150 - HSPI wrap instruction register"]
    #[inline(always)]
    pub const fn hspi_wpir(&self) -> &HSPI_WPIR {
        &self.hspi_wpir
    }
    #[doc = "0x160 - HSPI wrap alternate bytes register"]
    #[inline(always)]
    pub const fn hspi_wpabr(&self) -> &HSPI_WPABR {
        &self.hspi_wpabr
    }
    #[doc = "0x180 - HSPI write communication configuration register"]
    #[inline(always)]
    pub const fn hspi_wccr(&self) -> &HSPI_WCCR {
        &self.hspi_wccr
    }
    #[doc = "0x188 - HSPI write timing configuration register"]
    #[inline(always)]
    pub const fn hspi_wtcr(&self) -> &HSPI_WTCR {
        &self.hspi_wtcr
    }
    #[doc = "0x190 - HSPI write instruction register"]
    #[inline(always)]
    pub const fn hspi_wir(&self) -> &HSPI_WIR {
        &self.hspi_wir
    }
    #[doc = "0x1a0 - HSPI write alternate bytes register"]
    #[inline(always)]
    pub const fn hspi_wabr(&self) -> &HSPI_WABR {
        &self.hspi_wabr
    }
    #[doc = "0x200 - HSPI HyperBus latency configuration register"]
    #[inline(always)]
    pub const fn hspi_hlcr(&self) -> &HSPI_HLCR {
        &self.hspi_hlcr
    }
    #[doc = "0x210 - HSPI full-cycle calibration configuration"]
    #[inline(always)]
    pub const fn hspi_calfcr(&self) -> &HSPI_CALFCR {
        &self.hspi_calfcr
    }
    #[doc = "0x218 - HSPI DLL master calibration configuration"]
    #[inline(always)]
    pub const fn hspi_calmr(&self) -> &HSPI_CALMR {
        &self.hspi_calmr
    }
    #[doc = "0x220 - HSPI DLL slave output calibration configuration"]
    #[inline(always)]
    pub const fn hspi_calsor(&self) -> &HSPI_CALSOR {
        &self.hspi_calsor
    }
    #[doc = "0x228 - HSPI DLL slave input calibration configuration"]
    #[inline(always)]
    pub const fn hspi_calsir(&self) -> &HSPI_CALSIR {
        &self.hspi_calsir
    }
}
#[doc = "HSPI_CR (rw) register accessor: HSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_cr`]
module"]
pub type HSPI_CR = crate::Reg<hspi_cr::HSPI_CRrs>;
#[doc = "HSPI control register"]
pub mod hspi_cr;
#[doc = "HSPI_DCR1 (rw) register accessor: HSPI device configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dcr1`]
module"]
pub type HSPI_DCR1 = crate::Reg<hspi_dcr1::HSPI_DCR1rs>;
#[doc = "HSPI device configuration register 1"]
pub mod hspi_dcr1;
#[doc = "HSPI_DCR2 (rw) register accessor: HSPI device configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dcr2`]
module"]
pub type HSPI_DCR2 = crate::Reg<hspi_dcr2::HSPI_DCR2rs>;
#[doc = "HSPI device configuration register 2"]
pub mod hspi_dcr2;
#[doc = "HSPI_DCR3 (rw) register accessor: HSPI device configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dcr3`]
module"]
pub type HSPI_DCR3 = crate::Reg<hspi_dcr3::HSPI_DCR3rs>;
#[doc = "HSPI device configuration register 3"]
pub mod hspi_dcr3;
#[doc = "HSPI_DCR4 (rw) register accessor: HSPI device configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dcr4`]
module"]
pub type HSPI_DCR4 = crate::Reg<hspi_dcr4::HSPI_DCR4rs>;
#[doc = "HSPI device configuration register 4"]
pub mod hspi_dcr4;
#[doc = "HSPI_SR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_sr`]
module"]
pub type HSPI_SR = crate::Reg<hspi_sr::HSPI_SRrs>;
#[doc = ""]
pub mod hspi_sr;
#[doc = "HSPI_FCR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_fcr`]
module"]
pub type HSPI_FCR = crate::Reg<hspi_fcr::HSPI_FCRrs>;
#[doc = ""]
pub mod hspi_fcr;
#[doc = "HSPI_DLR (rw) register accessor: HSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dlr`]
module"]
pub type HSPI_DLR = crate::Reg<hspi_dlr::HSPI_DLRrs>;
#[doc = "HSPI data length register"]
pub mod hspi_dlr;
#[doc = "HSPI_AR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_ar`]
module"]
pub type HSPI_AR = crate::Reg<hspi_ar::HSPI_ARrs>;
#[doc = ""]
pub mod hspi_ar;
#[doc = "HSPI_DR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_dr`]
module"]
pub type HSPI_DR = crate::Reg<hspi_dr::HSPI_DRrs>;
#[doc = ""]
pub mod hspi_dr;
#[doc = "HSPI_PSMKR (rw) register accessor: HSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_psmkr`]
module"]
pub type HSPI_PSMKR = crate::Reg<hspi_psmkr::HSPI_PSMKRrs>;
#[doc = "HSPI polling status mask register"]
pub mod hspi_psmkr;
#[doc = "HSPI_PSMAR (rw) register accessor: HSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_psmar`]
module"]
pub type HSPI_PSMAR = crate::Reg<hspi_psmar::HSPI_PSMARrs>;
#[doc = "HSPI polling status match register"]
pub mod hspi_psmar;
#[doc = "HSPI_PIR (rw) register accessor: HSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_pir`]
module"]
pub type HSPI_PIR = crate::Reg<hspi_pir::HSPI_PIRrs>;
#[doc = "HSPI polling interval register"]
pub mod hspi_pir;
#[doc = "HSPI_CCR (rw) register accessor: HSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_ccr`]
module"]
pub type HSPI_CCR = crate::Reg<hspi_ccr::HSPI_CCRrs>;
#[doc = "HSPI communication configuration register"]
pub mod hspi_ccr;
#[doc = "HSPI_TCR (rw) register accessor: HSPI timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_tcr`]
module"]
pub type HSPI_TCR = crate::Reg<hspi_tcr::HSPI_TCRrs>;
#[doc = "HSPI timing configuration register"]
pub mod hspi_tcr;
#[doc = "HSPI_IR (rw) register accessor: HSPI instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_ir`]
module"]
pub type HSPI_IR = crate::Reg<hspi_ir::HSPI_IRrs>;
#[doc = "HSPI instruction register"]
pub mod hspi_ir;
#[doc = "HSPI_ABR (rw) register accessor: HSPI alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_abr`]
module"]
pub type HSPI_ABR = crate::Reg<hspi_abr::HSPI_ABRrs>;
#[doc = "HSPI alternate bytes register"]
pub mod hspi_abr;
#[doc = "HSPI_LPTR (rw) register accessor: HSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_lptr`]
module"]
pub type HSPI_LPTR = crate::Reg<hspi_lptr::HSPI_LPTRrs>;
#[doc = "HSPI low-power timeout register"]
pub mod hspi_lptr;
#[doc = "HSPI_WPCCR (rw) register accessor: HSPI wrap communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wpccr`]
module"]
pub type HSPI_WPCCR = crate::Reg<hspi_wpccr::HSPI_WPCCRrs>;
#[doc = "HSPI wrap communication configuration register"]
pub mod hspi_wpccr;
#[doc = "HSPI_WPTCR (rw) register accessor: HSPI wrap timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wptcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wptcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wptcr`]
module"]
pub type HSPI_WPTCR = crate::Reg<hspi_wptcr::HSPI_WPTCRrs>;
#[doc = "HSPI wrap timing configuration register"]
pub mod hspi_wptcr;
#[doc = "HSPI_WPIR (rw) register accessor: HSPI wrap instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wpir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wpir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wpir`]
module"]
pub type HSPI_WPIR = crate::Reg<hspi_wpir::HSPI_WPIRrs>;
#[doc = "HSPI wrap instruction register"]
pub mod hspi_wpir;
#[doc = "HSPI_WPABR (rw) register accessor: HSPI wrap alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wpabr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wpabr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wpabr`]
module"]
pub type HSPI_WPABR = crate::Reg<hspi_wpabr::HSPI_WPABRrs>;
#[doc = "HSPI wrap alternate bytes register"]
pub mod hspi_wpabr;
#[doc = "HSPI_WCCR (rw) register accessor: HSPI write communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wccr`]
module"]
pub type HSPI_WCCR = crate::Reg<hspi_wccr::HSPI_WCCRrs>;
#[doc = "HSPI write communication configuration register"]
pub mod hspi_wccr;
#[doc = "HSPI_WTCR (rw) register accessor: HSPI write timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wtcr`]
module"]
pub type HSPI_WTCR = crate::Reg<hspi_wtcr::HSPI_WTCRrs>;
#[doc = "HSPI write timing configuration register"]
pub mod hspi_wtcr;
#[doc = "HSPI_WIR (rw) register accessor: HSPI write instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wir`]
module"]
pub type HSPI_WIR = crate::Reg<hspi_wir::HSPI_WIRrs>;
#[doc = "HSPI write instruction register"]
pub mod hspi_wir;
#[doc = "HSPI_WABR (rw) register accessor: HSPI write alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wabr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wabr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_wabr`]
module"]
pub type HSPI_WABR = crate::Reg<hspi_wabr::HSPI_WABRrs>;
#[doc = "HSPI write alternate bytes register"]
pub mod hspi_wabr;
#[doc = "HSPI_HLCR (rw) register accessor: HSPI HyperBus latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_hlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_hlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_hlcr`]
module"]
pub type HSPI_HLCR = crate::Reg<hspi_hlcr::HSPI_HLCRrs>;
#[doc = "HSPI HyperBus latency configuration register"]
pub mod hspi_hlcr;
#[doc = "HSPI_CALFCR (r) register accessor: HSPI full-cycle calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calfcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_calfcr`]
module"]
pub type HSPI_CALFCR = crate::Reg<hspi_calfcr::HSPI_CALFCRrs>;
#[doc = "HSPI full-cycle calibration configuration"]
pub mod hspi_calfcr;
#[doc = "HSPI_CALMR (rw) register accessor: HSPI DLL master calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_calmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_calmr`]
module"]
pub type HSPI_CALMR = crate::Reg<hspi_calmr::HSPI_CALMRrs>;
#[doc = "HSPI DLL master calibration configuration"]
pub mod hspi_calmr;
#[doc = "HSPI_CALSOR (rw) register accessor: HSPI DLL slave output calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calsor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_calsor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_calsor`]
module"]
pub type HSPI_CALSOR = crate::Reg<hspi_calsor::HSPI_CALSORrs>;
#[doc = "HSPI DLL slave output calibration configuration"]
pub mod hspi_calsor;
#[doc = "HSPI_CALSIR (rw) register accessor: HSPI DLL slave input calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calsir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_calsir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_calsir`]
module"]
pub type HSPI_CALSIR = crate::Reg<hspi_calsir::HSPI_CALSIRrs>;
#[doc = "HSPI DLL slave input calibration configuration"]
pub mod hspi_calsir;
