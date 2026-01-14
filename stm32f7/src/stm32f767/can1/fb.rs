#[repr(C)]
#[derive(Debug)]
///CAN Filter Bank cluster
pub struct FB {
    fr1: FR1,
    fr2: FR2,
}
impl FB {
    ///0x00 - Filter bank x register 1
    #[inline(always)]
    pub const fn fr1(&self) -> &FR1 {
        &self.fr1
    }
    ///0x04 - Filter bank x register 2
    #[inline(always)]
    pub const fn fr2(&self) -> &FR2 {
        &self.fr2
    }
}
/**FR1 (rw) register accessor: Filter bank x register 1

You can [`read`](crate::Reg::read) this register and get [`fr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fr1`] module*/
pub type FR1 = crate::Reg<fr1::FR1rs>;
///Filter bank x register 1
pub mod fr1;
/**FR2 (rw) register accessor: Filter bank x register 2

You can [`read`](crate::Reg::read) this register and get [`fr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fr2`] module*/
pub type FR2 = crate::Reg<fr2::FR2rs>;
///Filter bank x register 2
pub mod fr2;
