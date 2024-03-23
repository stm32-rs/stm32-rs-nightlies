#[doc = "Register `JDR4` reader"]
pub type R = crate::R<JDR4rs>;
#[doc = "Field `JDATA4` reader - JDATA4"]
pub type JDATA4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA4"]
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR4rs;
impl crate::RegisterSpec for JDR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr4::R`](R) reader structure"]
impl crate::Readable for JDR4rs {}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4rs {
    const RESET_VALUE: u32 = 0;
}
