#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 8],
    flt: [FLT; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x100 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x100 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x100..0x500 - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    #[inline(always)]
    pub const fn flt(&self, n: usize) -> &FLT {
        &self.flt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x500 - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    #[inline(always)]
    pub fn flt_iter(&self) -> impl Iterator<Item = &FLT> {
        self.flt.iter()
    }
}
#[doc = "DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
pub mod ch;
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub use self::flt::FLT;
#[doc = r"Cluster"]
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub mod flt;
