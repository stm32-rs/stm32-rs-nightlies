#[doc = "Register `FLT2JDATAR` reader"]
pub type R = crate::R<FLT2JDATARrs>;
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
#[doc = "data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt2jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT2JDATARrs;
impl crate::RegisterSpec for FLT2JDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt2jdatar::R`](R) reader structure"]
impl crate::Readable for FLT2JDATARrs {}
#[doc = "`reset()` method sets FLT2JDATAR to value 0"]
impl crate::Resettable for FLT2JDATARrs {
    const RESET_VALUE: u32 = 0;
}
