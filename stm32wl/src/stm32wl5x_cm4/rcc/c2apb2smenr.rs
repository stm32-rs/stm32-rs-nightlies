#[doc = "Reader of register C2APB2SMENR"]
pub type R = crate::R<u32, super::C2APB2SMENR>;
#[doc = "Writer for register C2APB2SMENR"]
pub type W = crate::W<u32, super::C2APB2SMENR>;
#[doc = "Register C2APB2SMENR `reset()`'s with value 0x0006_5a00"]
impl crate::ResetValue for super::C2APB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0006_5a00
    }
}
#[doc = "Reader of field `TIM17SMEN`"]
pub type TIM17SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17SMEN`"]
pub struct TIM17SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17SMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIM16SMEN`"]
pub type TIM16SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16SMEN`"]
pub struct TIM16SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `USART1SMEN`"]
pub type USART1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1SMEN`"]
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI1SMEN`"]
pub type SPI1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1SMEN`"]
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TIM1SMEN`"]
pub type TIM1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1SMEN`"]
pub struct TIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADCSMEN`"]
pub type ADCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSMEN`"]
pub struct ADCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W {
        TIM17SMEN_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W {
        TIM16SMEN_W { w: self }
    }
    #[doc = "Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode"]
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W {
        TIM1SMEN_W { w: self }
    }
    #[doc = "Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes"]
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W {
        ADCSMEN_W { w: self }
    }
}
