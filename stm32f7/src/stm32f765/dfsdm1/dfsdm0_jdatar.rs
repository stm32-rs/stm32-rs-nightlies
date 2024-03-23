#[doc = "Register `DFSDM0_JDATAR` reader"]
pub type R = crate::R<DFSDM0_JDATARrs>;
#[doc = "Field `JDATACH` reader - Injected channel most recently converted"]
pub type JDATACH_R = crate::FieldReader;
#[doc = "Field `JDATA` reader - Injected group conversion data"]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM0_JDATARrs;
impl crate::RegisterSpec for DFSDM0_JDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm0_jdatar::R`](R) reader structure"]
impl crate::Readable for DFSDM0_JDATARrs {}
#[doc = "`reset()` method sets DFSDM0_JDATAR to value 0"]
impl crate::Resettable for DFSDM0_JDATARrs {
    const RESET_VALUE: u32 = 0;
}
