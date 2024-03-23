#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    fcr: FCR,
    sr: SR,
    clr: CLR,
    _reserved4: [u8; 0x04],
    ram_com0: RAM_COM0,
    _reserved5: [u8; 0x04],
    ram_com1: RAM_COM1,
    _reserved6: [u8; 0x04],
    ram_com2: RAM_COM2,
    _reserved7: [u8; 0x04],
    ram_com3: RAM_COM3,
    _reserved8: [u8; 0x04],
    ram_com4: RAM_COM4,
    _reserved9: [u8; 0x04],
    ram_com5: RAM_COM5,
    _reserved10: [u8; 0x04],
    ram_com6: RAM_COM6,
    _reserved11: [u8; 0x04],
    ram_com7: RAM_COM7,
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
    #[doc = "0x14 - display memory"]
    #[inline(always)]
    pub const fn ram_com0(&self) -> &RAM_COM0 {
        &self.ram_com0
    }
    #[doc = "0x1c - display memory"]
    #[inline(always)]
    pub const fn ram_com1(&self) -> &RAM_COM1 {
        &self.ram_com1
    }
    #[doc = "0x24 - display memory"]
    #[inline(always)]
    pub const fn ram_com2(&self) -> &RAM_COM2 {
        &self.ram_com2
    }
    #[doc = "0x2c - display memory"]
    #[inline(always)]
    pub const fn ram_com3(&self) -> &RAM_COM3 {
        &self.ram_com3
    }
    #[doc = "0x34 - display memory"]
    #[inline(always)]
    pub const fn ram_com4(&self) -> &RAM_COM4 {
        &self.ram_com4
    }
    #[doc = "0x3c - display memory"]
    #[inline(always)]
    pub const fn ram_com5(&self) -> &RAM_COM5 {
        &self.ram_com5
    }
    #[doc = "0x44 - display memory"]
    #[inline(always)]
    pub const fn ram_com6(&self) -> &RAM_COM6 {
        &self.ram_com6
    }
    #[doc = "0x4c - display memory"]
    #[inline(always)]
    pub const fn ram_com7(&self) -> &RAM_COM7 {
        &self.ram_com7
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
#[doc = "RAM_COM0 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com0`]
module"]
pub type RAM_COM0 = crate::Reg<ram_com0::RAM_COM0rs>;
#[doc = "display memory"]
pub mod ram_com0;
#[doc = "RAM_COM1 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com1`]
module"]
pub type RAM_COM1 = crate::Reg<ram_com1::RAM_COM1rs>;
#[doc = "display memory"]
pub mod ram_com1;
#[doc = "RAM_COM2 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com2`]
module"]
pub type RAM_COM2 = crate::Reg<ram_com2::RAM_COM2rs>;
#[doc = "display memory"]
pub mod ram_com2;
#[doc = "RAM_COM3 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com3`]
module"]
pub type RAM_COM3 = crate::Reg<ram_com3::RAM_COM3rs>;
#[doc = "display memory"]
pub mod ram_com3;
#[doc = "RAM_COM4 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com4`]
module"]
pub type RAM_COM4 = crate::Reg<ram_com4::RAM_COM4rs>;
#[doc = "display memory"]
pub mod ram_com4;
#[doc = "RAM_COM5 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com5`]
module"]
pub type RAM_COM5 = crate::Reg<ram_com5::RAM_COM5rs>;
#[doc = "display memory"]
pub mod ram_com5;
#[doc = "RAM_COM6 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com6`]
module"]
pub type RAM_COM6 = crate::Reg<ram_com6::RAM_COM6rs>;
#[doc = "display memory"]
pub mod ram_com6;
#[doc = "RAM_COM7 (rw) register accessor: display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_com7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_com7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_com7`]
module"]
pub type RAM_COM7 = crate::Reg<ram_com7::RAM_COM7rs>;
#[doc = "display memory"]
pub mod ram_com7;
