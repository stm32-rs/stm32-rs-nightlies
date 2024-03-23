#[doc = "Register `JDATAR` reader"]
pub type R = crate::R<JDATARrs>;
#[doc = "Field `JDATACH` reader - Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
pub type JDATACH_R = crate::FieldReader;
#[doc = "Field `JDATA` reader - Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
