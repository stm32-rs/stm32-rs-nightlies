#[doc = "Register `HTR1` reader"]
pub type R = crate::R<HTR1rs>;
#[doc = "Register `HTR1` writer"]
pub type W = crate::W<HTR1rs>;
#[doc = "Field `HTR1` reader - ADC analog watchdog 2 threshold low"]
pub type HTR1_R = crate::FieldReader<u32>;
#[doc = "Field `HTR1` writer - ADC analog watchdog 2 threshold low"]
pub type HTR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn htr1(&mut self) -> HTR1_W<HTR1rs> {
        HTR1_W::new(self, 0)
    }
}
#[doc = "ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTR1rs;
impl crate::RegisterSpec for HTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr1::R`](R) reader structure"]
impl crate::Readable for HTR1rs {}
#[doc = "`write(|w| ..)` method takes [`htr1::W`](W) writer structure"]
impl crate::Writable for HTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTR1 to value 0x0fff_0000"]
impl crate::Resettable for HTR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
