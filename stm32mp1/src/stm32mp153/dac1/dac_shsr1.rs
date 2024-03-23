#[doc = "Register `DAC_SHSR1` reader"]
pub type R = crate::R<DAC_SHSR1rs>;
#[doc = "Register `DAC_SHSR1` writer"]
pub type W = crate::W<DAC_SHSR1rs>;
#[doc = "Field `TSAMPLE1` reader - TSAMPLE1"]
pub type TSAMPLE1_R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE1` writer - TSAMPLE1"]
pub type TSAMPLE1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TSAMPLE1"]
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TSAMPLE1"]
    #[inline(always)]
    #[must_use]
    pub fn tsample1(&mut self) -> TSAMPLE1_W<DAC_SHSR1rs> {
        TSAMPLE1_W::new(self, 0)
    }
}
#[doc = "DAC channel 1 sample and hold sample time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_SHSR1rs;
impl crate::RegisterSpec for DAC_SHSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shsr1::R`](R) reader structure"]
impl crate::Readable for DAC_SHSR1rs {}
#[doc = "`write(|w| ..)` method takes [`dac_shsr1::W`](W) writer structure"]
impl crate::Writable for DAC_SHSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHSR1 to value 0"]
impl crate::Resettable for DAC_SHSR1rs {
    const RESET_VALUE: u32 = 0;
}
