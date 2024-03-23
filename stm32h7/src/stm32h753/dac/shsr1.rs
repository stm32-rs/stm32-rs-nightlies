#[doc = "Register `SHSR1` reader"]
pub type R = crate::R<SHSR1rs>;
#[doc = "Register `SHSR1` writer"]
pub type W = crate::W<SHSR1rs>;
#[doc = "Field `TSAMPLE1` reader - DAC Channel 1 sample Time (only valid in sample &amp;amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
pub type TSAMPLE1_R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE1` writer - DAC Channel 1 sample Time (only valid in sample &amp;amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
pub type TSAMPLE1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp;amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 sample Time (only valid in sample &amp;amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tsample1(&mut self) -> TSAMPLE1_W<SHSR1rs> {
        TSAMPLE1_W::new(self, 0)
    }
}
#[doc = "DAC Sample and Hold sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHSR1rs;
impl crate::RegisterSpec for SHSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shsr1::R`](R) reader structure"]
impl crate::Readable for SHSR1rs {}
#[doc = "`write(|w| ..)` method takes [`shsr1::W`](W) writer structure"]
impl crate::Writable for SHSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHSR1 to value 0"]
impl crate::Resettable for SHSR1rs {
    const RESET_VALUE: u32 = 0;
}
