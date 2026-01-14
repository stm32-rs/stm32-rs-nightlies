#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ch: [CH; 8],
    flt: [FLT; 4],
}
impl RegisterBlock {
    ///0x00..0x100 - DFSDM channel configuration cluster
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x00..0x100 - DFSDM channel configuration cluster
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x100..0x300 - DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
    #[inline(always)]
    pub const fn flt(&self, n: usize) -> &FLT {
        &self.flt[n]
    }
    ///Iterator for array of:
    ///0x100..0x300 - DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
    #[inline(always)]
    pub fn flt_iter(&self) -> impl Iterator<Item = &FLT> {
        self.flt.iter()
    }
}
///DFSDM channel configuration cluster
pub use self::ch::CH;
///Cluster
///DFSDM channel configuration cluster
pub mod ch;
///DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
pub use self::flt::FLT;
///Cluster
///DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
pub mod flt;
