#[doc = "Register `JDATAR` reader"]
pub type R = crate::R<JDATARrs>;
#[doc = "Field `JDATA` reader - Injected group conversion data"]
pub type JDATA_R = crate::FieldReader<u16>;
#[doc = "Field `JDATACH` reader - Injected channel most recently converted"]
pub type JDATACH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 25:28 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[doc = "data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDATARrs;
impl crate::RegisterSpec for JDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdatar::R`](R) reader structure"]
impl crate::Readable for JDATARrs {}
#[doc = "`reset()` method sets JDATAR to value 0"]
impl crate::Resettable for JDATARrs {
    const RESET_VALUE: u32 = 0;
}
