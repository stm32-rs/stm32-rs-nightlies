///Register `FLT0ISR` reader
pub type R = crate::R<FLT0ISRrs>;
///Field `JEOCF` reader - JEOCF
pub type JEOCF_R = crate::BitReader;
///Field `REOCF` reader - REOCF
pub type REOCF_R = crate::BitReader;
///Field `JOVRF` reader - JOVRF
pub type JOVRF_R = crate::BitReader;
///Field `ROVRF` reader - ROVRF
pub type ROVRF_R = crate::BitReader;
///Field `AWDF` reader - AWDF
pub type AWDF_R = crate::BitReader;
///Field `JCIP` reader - JCIP
pub type JCIP_R = crate::BitReader;
///Field `RCIP` reader - RCIP
pub type RCIP_R = crate::BitReader;
///Field `CKABF` reader - CKABF
pub type CKABF_R = crate::FieldReader;
///Field `SCDF` reader - SCDF
pub type SCDF_R = crate::FieldReader;
impl R {
    ///Bit 0 - JEOCF
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - REOCF
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - JOVRF
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ROVRF
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWDF
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - JCIP
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RCIP
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - CKABF
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - SCDF
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0ISR")
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
/**DFSDM filter 0 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt0isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0ISR)*/
pub struct FLT0ISRrs;
impl crate::RegisterSpec for FLT0ISRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0isr::R`](R) reader structure
impl crate::Readable for FLT0ISRrs {}
///`reset()` method sets FLT0ISR to value 0x00ff_0000
impl crate::Resettable for FLT0ISRrs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
