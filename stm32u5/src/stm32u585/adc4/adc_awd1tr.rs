#[doc = "Register `ADC_AWD1TR` reader"]
pub type R = crate::R<ADC_AWD1TRrs>;
#[doc = "Register `ADC_AWD1TR` writer"]
pub type W = crate::W<ADC_AWD1TRrs>;
#[doc = "Field `LT1` reader - LT1"]
pub type LT1_R = crate::FieldReader<u16>;
#[doc = "Field `LT1` writer - LT1"]
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT1` reader - HT1"]
pub type HT1_R = crate::FieldReader<u16>;
#[doc = "Field `HT1` writer - HT1"]
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - LT1"]
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HT1"]
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LT1"]
    #[inline(always)]
    #[must_use]
    pub fn lt1(&mut self) -> LT1_W<ADC_AWD1TRrs> {
        LT1_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - HT1"]
    #[inline(always)]
    #[must_use]
    pub fn ht1(&mut self) -> HT1_W<ADC_AWD1TRrs> {
        HT1_W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd1tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd1tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_AWD1TRrs;
impl crate::RegisterSpec for ADC_AWD1TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_awd1tr::R`](R) reader structure"]
impl crate::Readable for ADC_AWD1TRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_awd1tr::W`](W) writer structure"]
impl crate::Writable for ADC_AWD1TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_AWD1TR to value 0x0fff_0000"]
impl crate::Resettable for ADC_AWD1TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
