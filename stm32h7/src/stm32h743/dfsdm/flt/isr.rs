///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `JEOCF` reader - End of injected conversion flag
pub type JEOCF_R = crate::BitReader;
///Field `REOCF` reader - End of regular conversion flag
pub type REOCF_R = crate::BitReader;
///Field `JOVRF` reader - Injected conversion overrun flag
pub type JOVRF_R = crate::BitReader;
///Field `ROVRF` reader - Regular conversion overrun flag
pub type ROVRF_R = crate::BitReader;
///Field `AWDF` reader - Analog watchdog
pub type AWDF_R = crate::BitReader;
///Field `JCIP` reader - Injected conversion in progress status
pub type JCIP_R = crate::BitReader;
///Field `RCIP` reader - Regular conversion in progress status
pub type RCIP_R = crate::BitReader;
///Field `CKABF` reader - Clock absence flag
pub type CKABF_R = crate::FieldReader;
///Field `SCDF` reader - short-circuit detector flag
pub type SCDF_R = crate::FieldReader;
impl R {
    ///Bit 0 - End of injected conversion flag
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of regular conversion flag
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected conversion overrun flag
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular conversion overrun flag
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - Injected conversion in progress status
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Regular conversion in progress status
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - Clock absence flag
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - short-circuit detector flag
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("jeocf", &self.jeocf())
            .field("reocf", &self.reocf())
            .field("jovrf", &self.jovrf())
            .field("rovrf", &self.rovrf())
            .field("awdf", &self.awdf())
            .field("jcip", &self.jcip())
            .field("rcip", &self.rcip())
            .field("ckabf", &self.ckabf())
            .field("scdf", &self.scdf())
            .finish()
    }
}
/**DFSDM interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
