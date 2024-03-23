#[doc = "Register `AWD3TR` reader"]
pub type R = crate::R<AWD3TRrs>;
#[doc = "Register `AWD3TR` writer"]
pub type W = crate::W<AWD3TRrs>;
#[doc = "Field `LT3` reader - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
pub type LT3_R = crate::FieldReader<u16>;
#[doc = "Field `LT3` writer - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
pub type LT3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `HT3` reader - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
pub type HT3_R = crate::FieldReader<u16>;
#[doc = "Field `HT3` writer - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
pub type HT3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<AWD3TRrs> {
        LT3_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<AWD3TRrs> {
        HT3_W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD3TRrs;
impl crate::RegisterSpec for AWD3TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3tr::R`](R) reader structure"]
impl crate::Readable for AWD3TRrs {}
#[doc = "`write(|w| ..)` method takes [`awd3tr::W`](W) writer structure"]
impl crate::Writable for AWD3TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3TR to value 0x0fff_0000"]
impl crate::Resettable for AWD3TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
