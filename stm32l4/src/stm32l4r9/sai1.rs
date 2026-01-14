#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    ch: [CH; 2],
}
impl RegisterBlock {
    ///0x00 - Global configuration register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CHA` cluster.</div>
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x04..0x24 - Cluster CHA, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub const fn cha(&self) -> &CH {
        self.ch(0)
    }
    ///0x24..0x44 - Cluster CHB, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub const fn chb(&self) -> &CH {
        self.ch(1)
    }
}
/**GCR (rw) register accessor: Global configuration register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#SAI1:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///Global configuration register
pub mod gcr;
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub mod ch;
