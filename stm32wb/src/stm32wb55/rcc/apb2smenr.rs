#[doc = "Reader of register APB2SMENR"]
pub type R = crate::R<u32, super::APB2SMENR>;
#[doc = "Writer for register APB2SMENR"]
pub type W = crate::W<u32, super::APB2SMENR>;
#[doc = "Register APB2SMENR `reset()`'s with value 0x0026_5800"]
impl crate::ResetValue for super::APB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0026_5800
    }
}
#[doc = "Reader of field `SAI1SMEN`"]
pub type SAI1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1SMEN`"]
pub struct SAI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
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
impl R {
    #[doc = "Bit 21 - SAI1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - SAI1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W {
        SAI1SMEN_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W {
        TIM17SMEN_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W {
        TIM16SMEN_W { w: self }
    }
    #[doc = "Bit 14 - USART1clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W {
        TIM1SMEN_W { w: self }
    }
}
