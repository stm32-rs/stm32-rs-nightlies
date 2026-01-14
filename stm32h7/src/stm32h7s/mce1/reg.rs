#[repr(C)]
#[derive(Debug)]
///Region cluster
pub struct REG {
    regcr: REGCR,
    saddr: SADDR,
    eaddr: EADDR,
    attr: ATTR,
}
impl REG {
    ///0x00 - Region configuration register
    #[inline(always)]
    pub const fn regcr(&self) -> &REGCR {
        &self.regcr
    }
    ///0x04 - Region start address register
    #[inline(always)]
    pub const fn saddr(&self) -> &SADDR {
        &self.saddr
    }
    ///0x08 - Region end address register
    #[inline(always)]
    pub const fn eaddr(&self) -> &EADDR {
        &self.eaddr
    }
    ///0x0c - Region attribute register
    #[inline(always)]
    pub const fn attr(&self) -> &ATTR {
        &self.attr
    }
}
/**REGCR (rw) register accessor: Region configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regcr`] module*/
pub type REGCR = crate::Reg<regcr::REGCRrs>;
///Region configuration register
pub mod regcr;
/**SADDR (rw) register accessor: Region start address register

You can [`read`](crate::Reg::read) this register and get [`saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@saddr`] module*/
pub type SADDR = crate::Reg<saddr::SADDRrs>;
///Region start address register
pub mod saddr;
/**EADDR (rw) register accessor: Region end address register

You can [`read`](crate::Reg::read) this register and get [`eaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eaddr`] module*/
pub type EADDR = crate::Reg<eaddr::EADDRrs>;
///Region end address register
pub mod eaddr;
/**ATTR (rw) register accessor: Region attribute register

You can [`read`](crate::Reg::read) this register and get [`attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@attr`] module*/
pub type ATTR = crate::Reg<attr::ATTRrs>;
///Region attribute register
pub mod attr;
