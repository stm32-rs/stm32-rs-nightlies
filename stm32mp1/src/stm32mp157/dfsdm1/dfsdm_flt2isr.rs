#[doc = "Register `DFSDM_FLT2ISR` reader"]
pub type R = crate::R<DFSDM_FLT2ISRrs>;
#[doc = "Field `JEOCF` reader - JEOCF"]
pub type JEOCF_R = crate::BitReader;
#[doc = "Field `REOCF` reader - REOCF"]
pub type REOCF_R = crate::BitReader;
#[doc = "Field `JOVRF` reader - JOVRF"]
pub type JOVRF_R = crate::BitReader;
#[doc = "Field `ROVRF` reader - ROVRF"]
pub type ROVRF_R = crate::BitReader;
#[doc = "Field `AWDF` reader - AWDF"]
pub type AWDF_R = crate::BitReader;
#[doc = "Field `JCIP` reader - JCIP"]
pub type JCIP_R = crate::BitReader;
#[doc = "Field `RCIP` reader - RCIP"]
pub type RCIP_R = crate::BitReader;
#[doc = "Field `CKABF` reader - CKABF"]
pub type CKABF_R = crate::FieldReader;
#[doc = "Field `SCDF` reader - SCDF"]
pub type SCDF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - JEOCF"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REOCF"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JOVRF"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ROVRF"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWDF"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - JCIP"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RCIP"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CKABF"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCDF"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DFSDM filter 2 interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT2ISRrs;
impl crate::RegisterSpec for DFSDM_FLT2ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt2isr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT2ISRrs {}
#[doc = "`reset()` method sets DFSDM_FLT2ISR to value 0x00ff_0000"]
impl crate::Resettable for DFSDM_FLT2ISRrs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
