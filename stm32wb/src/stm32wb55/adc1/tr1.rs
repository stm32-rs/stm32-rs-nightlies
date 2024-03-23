#[doc = "Register `TR1` reader"]
pub type R = crate::R<TR1rs>;
#[doc = "Register `TR1` writer"]
pub type W = crate::W<TR1rs>;
#[doc = "Field `LT1` reader - ADC analog watchdog 1 threshold low"]
pub type LT1_R = crate::FieldReader<u16>;
#[doc = "Field `LT1` writer - ADC analog watchdog 1 threshold low"]
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT1` reader - ADC analog watchdog 1 threshold high"]
pub type HT1_R = crate::FieldReader<u16>;
#[doc = "Field `HT1` writer - ADC analog watchdog 1 threshold high"]
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn lt1(&mut self) -> LT1_W<TR1rs> {
        LT1_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    #[must_use]
    pub fn ht1(&mut self) -> HT1_W<TR1rs> {
        HT1_W::new(self, 16)
    }
}
#[doc = "ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr1::R`](R) reader structure"]
impl crate::Readable for TR1rs {}
#[doc = "`write(|w| ..)` method takes [`tr1::W`](W) writer structure"]
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR1 to value 0x0fff_0000"]
impl crate::Resettable for TR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
