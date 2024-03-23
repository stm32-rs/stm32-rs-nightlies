#[doc = "Register `JDR1` reader"]
pub type R = crate::R<JDR1rs>;
#[doc = "Field `JDATA1` reader - ADC group injected sequencer rank 1 conversion data"]
pub type JDATA1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 1 conversion data"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC group injected sequencer rank 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR1rs;
impl crate::RegisterSpec for JDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr1::R`](R) reader structure"]
impl crate::Readable for JDR1rs {}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for JDR1rs {
    const RESET_VALUE: u32 = 0;
}
