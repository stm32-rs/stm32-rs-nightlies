#[doc = "Register `JDR%s` reader"]
pub type R = crate::R<JDRrs>;
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDRrs;
impl crate::RegisterSpec for JDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr::R`](R) reader structure"]
impl crate::Readable for JDRrs {}
#[doc = "`reset()` method sets JDR%s to value 0"]
impl crate::Resettable for JDRrs {
    const RESET_VALUE: u32 = 0;
}
