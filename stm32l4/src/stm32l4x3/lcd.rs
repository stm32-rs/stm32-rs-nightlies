#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    fcr: FCR,
    sr: SR,
    clr: CLR,
    _reserved4: [u8; 0x04],
    ram_com: [RAM_COM; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - frame control register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x0c - clear register"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x14..0x54 - display memory"]
    #[inline(always)]
    pub const fn ram_com(&self, n: usize) -> &RAM_COM {
        &self.ram_com[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x54 - display memory"]
    #[inline(always)]
    pub fn ram_com_iter(&self) -> impl Iterator<Item = &RAM_COM> {
        self.ram_com.iter()
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "FCR (rw) register accessor: frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "frame control register"]
pub mod fcr;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "CLR (w) register accessor: clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLRrs>;
#[doc = "clear register"]
pub mod clr;
#[doc = "RAM_COM (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com`]
module"]
pub type RAM_COM = crate::Reg<ram_com::RAM_COMrs>;
#[doc = "display memory"]
pub mod ram_com;
