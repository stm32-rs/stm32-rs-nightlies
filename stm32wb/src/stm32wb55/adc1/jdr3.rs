#[doc = "Register `JDR3` reader"]
pub type R = crate::R<JDR3rs>;
#[doc = "Field `JDATA3` reader - ADC group injected sequencer rank 3 conversion data"]
pub type JDATA3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC group injected sequencer rank 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR3rs;
impl crate::RegisterSpec for JDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr3::R`](R) reader structure"]
impl crate::Readable for JDR3rs {}
#[doc = "`reset()` method sets JDR3 to value 0"]
impl crate::Resettable for JDR3rs {
    const RESET_VALUE: u32 = 0;
}
