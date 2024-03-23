#[doc = "Register `TR3` reader"]
pub type R = crate::R<TR3rs>;
#[doc = "Register `TR3` writer"]
pub type W = crate::W<TR3rs>;
#[doc = "Field `LT3` reader - ADC analog watchdog 3 threshold low"]
pub type LT3_R = crate::FieldReader;
#[doc = "Field `LT3` writer - ADC analog watchdog 3 threshold low"]
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HT3` reader - ADC analog watchdog 3 threshold high"]
pub type HT3_R = crate::FieldReader;
#[doc = "Field `HT3` writer - ADC analog watchdog 3 threshold high"]
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADC analog watchdog 3 threshold low"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC analog watchdog 3 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<TR3rs> {
        LT3_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<TR3rs> {
        HT3_W::new(self, 16)
    }
}
#[doc = "ADC analog watchdog 3 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR3rs;
impl crate::RegisterSpec for TR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr3::R`](R) reader structure"]
impl crate::Readable for TR3rs {}
#[doc = "`write(|w| ..)` method takes [`tr3::W`](W) writer structure"]
impl crate::Writable for TR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR3 to value 0x0fff_0000"]
impl crate::Resettable for TR3rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
