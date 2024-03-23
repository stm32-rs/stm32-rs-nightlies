#[doc = "Register `DFSDM_FLT1JDATAR` reader"]
pub type R = crate::R<DFSDM_FLT1JDATARrs>;
#[doc = "Field `JDATACH` reader - JDATACH"]
pub type JDATACH_R = crate::FieldReader;
#[doc = "Field `JDATA` reader - JDATA"]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - JDATACH"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM filter 1 data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT1JDATARrs;
impl crate::RegisterSpec for DFSDM_FLT1JDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1jdatar::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT1JDATARrs {}
#[doc = "`reset()` method sets DFSDM_FLT1JDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT1JDATARrs {
    const RESET_VALUE: u32 = 0;
}
