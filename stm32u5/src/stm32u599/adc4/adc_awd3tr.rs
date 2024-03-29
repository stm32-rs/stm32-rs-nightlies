#[doc = "Register `ADC_AWD3TR` reader"]
pub type R = crate::R<ADC_AWD3TRrs>;
#[doc = "Register `ADC_AWD3TR` writer"]
pub type W = crate::W<ADC_AWD3TRrs>;
#[doc = "Field `LT3` reader - LT3"]
pub type LT3_R = crate::FieldReader<u16>;
#[doc = "Field `LT3` writer - LT3"]
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT3` reader - HT3"]
pub type HT3_R = crate::FieldReader<u16>;
#[doc = "Field `HT3` writer - HT3"]
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - LT3"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HT3"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LT3"]
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<ADC_AWD3TRrs> {
        LT3_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - HT3"]
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<ADC_AWD3TRrs> {
        HT3_W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd3tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd3tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_AWD3TRrs;
impl crate::RegisterSpec for ADC_AWD3TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_awd3tr::R`](R) reader structure"]
impl crate::Readable for ADC_AWD3TRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_awd3tr::W`](W) writer structure"]
impl crate::Writable for ADC_AWD3TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_AWD3TR to value 0x0fff_0000"]
impl crate::Resettable for ADC_AWD3TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
