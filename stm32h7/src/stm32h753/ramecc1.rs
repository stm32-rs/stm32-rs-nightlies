#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier: IER,
    _reserved1: [u8; 0x1c],
    m: [M; 5],
}
impl RegisterBlock {
    ///0x00 - RAMECC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x20..0xc0 - Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `M1` cluster.</div>
    #[inline(always)]
    pub const fn m(&self, n: usize) -> &M {
        &self.m[n]
    }
    ///Iterator for array of:
    ///0x20..0xc0 - Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub fn m_iter(&self) -> impl Iterator<Item = &M> {
        self.m.iter()
    }
    ///0x20..0x40 - Cluster M1, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m1(&self) -> &M {
        self.m(0)
    }
    ///0x40..0x60 - Cluster M2, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m2(&self) -> &M {
        self.m(1)
    }
    ///0x60..0x80 - Cluster M3, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m3(&self) -> &M {
        self.m(2)
    }
    ///0x80..0xa0 - Cluster M4, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m4(&self) -> &M {
        self.m(3)
    }
    ///0xa0..0xc0 - Cluster M5, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
    #[inline(always)]
    pub const fn m5(&self) -> &M {
        self.m(4)
    }
}
/**IER (rw) register accessor: RAMECC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#RAMECC1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///RAMECC interrupt enable register
pub mod ier;
///Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
pub use self::m::M;
///Cluster
///Cluster M%s, containing M?CR, M?SR, M?FAR, M?FDRL, M?FDRH, M?FECR
pub mod m;
