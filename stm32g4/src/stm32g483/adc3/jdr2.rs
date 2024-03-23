#[doc = "Register `JDR2` reader"]
pub type R = crate::R<JDR2rs>;
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR2rs;
impl crate::RegisterSpec for JDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for JDR2rs {}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for JDR2rs {
    const RESET_VALUE: u32 = 0;
}
