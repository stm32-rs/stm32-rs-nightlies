#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Writer for register OR"]
pub type W = crate::W<u32, super::OR>;
#[doc = "Register OR `reset()`'s with value 0"]
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM1_ETR_ADC1_RMP`"]
pub type TIM1_ETR_ADC1_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIM1_ETR_ADC1_RMP`"]
pub struct TIM1_ETR_ADC1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_ETR_ADC1_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TIM1_ETR_ADC4_RMP`"]
pub type TIM1_ETR_ADC4_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIM1_ETR_ADC4_RMP`"]
pub struct TIM1_ETR_ADC4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_ETR_ADC4_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&self) -> TIM1_ETR_ADC4_RMP_R {
        TIM1_ETR_ADC4_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W {
        TIM1_ETR_ADC1_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&mut self) -> TIM1_ETR_ADC4_RMP_W {
        TIM1_ETR_ADC4_RMP_W { w: self }
    }
}
