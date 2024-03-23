#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ptptscr: PTPTSCR,
    ptpssir: PTPSSIR,
    ptptshr: PTPTSHR,
    ptptslr: PTPTSLR,
    ptptshur: PTPTSHUR,
    ptptslur: PTPTSLUR,
    ptptsar: PTPTSAR,
    ptptthr: PTPTTHR,
    ptpttlr: PTPTTLR,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
    #[inline(always)]
    pub const fn ptptscr(&self) -> &PTPTSCR {
        &self.ptptscr
    }
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    #[inline(always)]
    pub const fn ptpssir(&self) -> &PTPSSIR {
        &self.ptpssir
    }
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    #[inline(always)]
    pub const fn ptptshr(&self) -> &PTPTSHR {
        &self.ptptshr
    }
    #[doc = "0x0c - Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
    #[inline(always)]
    pub const fn ptptslr(&self) -> &PTPTSLR {
        &self.ptptslr
    }
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    #[inline(always)]
    pub const fn ptptshur(&self) -> &PTPTSHUR {
        &self.ptptshur
    }
    #[doc = "0x14 - Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
    #[inline(always)]
    pub const fn ptptslur(&self) -> &PTPTSLUR {
        &self.ptptslur
    }
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    #[inline(always)]
    pub const fn ptptsar(&self) -> &PTPTSAR {
        &self.ptptsar
    }
    #[doc = "0x1c - Ethernet PTP target time high register"]
    #[inline(always)]
    pub const fn ptptthr(&self) -> &PTPTTHR {
        &self.ptptthr
    }
    #[doc = "0x20 - Ethernet PTP target time low register"]
    #[inline(always)]
    pub const fn ptpttlr(&self) -> &PTPTTLR {
        &self.ptpttlr
    }
}
#[doc = "PTPTSCR (rw) register accessor: Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptscr`]
module"]
pub type PTPTSCR = crate::Reg<ptptscr::PTPTSCRrs>;
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
pub mod ptptscr;
#[doc = "PTPSSIR (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpssir`]
module"]
pub type PTPSSIR = crate::Reg<ptpssir::PTPSSIRrs>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "PTPTSHR (r) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshr`]
module"]
pub type PTPTSHR = crate::Reg<ptptshr::PTPTSHRrs>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "PTPTSLR (r) register accessor: Ethernet PTP time stamp low register (ETH_PTPTSLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslr`]
module"]
pub type PTPTSLR = crate::Reg<ptptslr::PTPTSLRrs>;
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
pub mod ptptslr;
#[doc = "PTPTSHUR (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshur`]
module"]
pub type PTPTSHUR = crate::Reg<ptptshur::PTPTSHURrs>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "PTPTSLUR (rw) register accessor: Ethernet PTP time stamp low update register (ETH_PTPTSLUR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslur`]
module"]
pub type PTPTSLUR = crate::Reg<ptptslur::PTPTSLURrs>;
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
pub mod ptptslur;
#[doc = "PTPTSAR (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsar`]
module"]
pub type PTPTSAR = crate::Reg<ptptsar::PTPTSARrs>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "PTPTTHR (rw) register accessor: Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptthr`]
module"]
pub type PTPTTHR = crate::Reg<ptptthr::PTPTTHRrs>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "PTPTTLR (rw) register accessor: Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpttlr`]
module"]
pub type PTPTTLR = crate::Reg<ptpttlr::PTPTTLRrs>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
