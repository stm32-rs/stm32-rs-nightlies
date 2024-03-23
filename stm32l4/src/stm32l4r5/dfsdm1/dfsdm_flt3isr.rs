#[doc = "Register `DFSDM_FLT3ISR` reader"]
pub type R = crate::R<DFSDM_FLT3ISRrs>;
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub type JEOCF_R = crate::BitReader;
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub type REOCF_R = crate::BitReader;
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub type JOVRF_R = crate::BitReader;
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub type ROVRF_R = crate::BitReader;
#[doc = "Field `AWDF` reader - Analog watchdog"]
pub type AWDF_R = crate::BitReader;
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub type JCIP_R = crate::BitReader;
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub type RCIP_R = crate::BitReader;
#[doc = "Field `CKABF` reader - Clock absence flag"]
pub type CKABF_R = crate::FieldReader;
#[doc = "Field `SCDF` reader - short-circuit detector flag"]
pub type SCDF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock absence flag"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - short-circuit detector flag"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT3ISRrs;
impl crate::RegisterSpec for DFSDM_FLT3ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3isr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT3ISRrs {}
#[doc = "`reset()` method sets DFSDM_FLT3ISR to value 0x00ff_0000"]
impl crate::Resettable for DFSDM_FLT3ISRrs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
