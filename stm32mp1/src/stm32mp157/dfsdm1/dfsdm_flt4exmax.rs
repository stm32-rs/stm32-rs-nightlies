#[doc = "Register `DFSDM_FLT4EXMAX` reader"]
pub type R = crate::R<DFSDM_FLT4EXMAXrs>;
#[doc = "Field `EXMAXCH` reader - EXMAXCH"]
pub type EXMAXCH_R = crate::FieldReader;
#[doc = "Field `EXMAX` reader - EXMAX"]
pub type EXMAX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - EXMAXCH"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMAX"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM filter 4 extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt4exmax::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT4EXMAXrs;
impl crate::RegisterSpec for DFSDM_FLT4EXMAXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt4exmax::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT4EXMAXrs {}
#[doc = "`reset()` method sets DFSDM_FLT4EXMAX to value 0x8000_0000"]
impl crate::Resettable for DFSDM_FLT4EXMAXrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
