#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1rs>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1rs>;
#[doc = "Field `ETR_ADC2_RMP` reader - External trigger remap on ADC2 analog watchdog"]
pub type ETR_ADC2_RMP_R = crate::FieldReader;
#[doc = "Field `ETR_ADC2_RMP` writer - External trigger remap on ADC2 analog watchdog"]
pub type ETR_ADC2_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TI1_RMP` reader - Input Capture 1 remap"]
pub type TI1_RMP_R = crate::BitReader;
#[doc = "Field `TI1_RMP` writer - Input Capture 1 remap"]
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc2_rmp(&self) -> ETR_ADC2_RMP_R {
        ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn etr_adc2_rmp(&mut self) -> ETR_ADC2_RMP_W<OR1rs> {
        ETR_ADC2_RMP_W::new(self, 0)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<OR1rs> {
        TI1_RMP_W::new(self, 4)
    }
}
#[doc = "TIM1 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1rs {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1rs {
    const RESET_VALUE: u32 = 0;
}
