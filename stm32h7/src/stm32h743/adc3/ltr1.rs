#[doc = "Register `LTR1` reader"]
pub type R = crate::R<LTR1rs>;
#[doc = "Register `LTR1` writer"]
pub type W = crate::W<LTR1rs>;
#[doc = "Field `LTR1` reader - ADC analog watchdog 1 threshold low"]
pub type LTR1_R = crate::FieldReader<u32>;
#[doc = "Field `LTR1` writer - ADC analog watchdog 1 threshold low"]
pub type LTR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn ltr1(&mut self) -> LTR1_W<LTR1rs> {
        LTR1_W::new(self, 0)
    }
}
#[doc = "ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTR1rs;
impl crate::RegisterSpec for LTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr1::R`](R) reader structure"]
impl crate::Readable for LTR1rs {}
#[doc = "`write(|w| ..)` method takes [`ltr1::W`](W) writer structure"]
impl crate::Writable for LTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTR1 to value 0x0fff_0000"]
impl crate::Resettable for LTR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
