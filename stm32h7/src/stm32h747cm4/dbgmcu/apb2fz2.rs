#[doc = "Reader of register APB2FZ2"]
pub type R = crate::R<u32, super::APB2FZ2>;
#[doc = "Writer for register APB2FZ2"]
pub type W = crate::W<u32, super::APB2FZ2>;
#[doc = "Register APB2FZ2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2FZ2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HRTIM`"]
pub type HRTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRTIM`"]
pub struct HRTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HRTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIM17`"]
pub type TIM17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17`"]
pub struct TIM17_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_W<'a> {
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
#[doc = "Reader of field `TIM16`"]
pub type TIM16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16`"]
pub struct TIM16_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_W<'a> {
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
#[doc = "Reader of field `TIM15`"]
pub type TIM15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM15`"]
pub struct TIM15_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIM8`"]
pub type TIM8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8`"]
pub struct TIM8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TIM1`"]
pub type TIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1`"]
pub struct TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - HRTIM stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn hrtim(&self) -> HRTIM_R {
        HRTIM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim8(&self) -> TIM8_R {
        TIM8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TIM1 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - HRTIM stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn hrtim(&mut self) -> HRTIM_W {
        HRTIM_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim17(&mut self) -> TIM17_W {
        TIM17_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim16(&mut self) -> TIM16_W {
        TIM16_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim15(&mut self) -> TIM15_W {
        TIM15_W { w: self }
    }
    #[doc = "Bit 1 - TIM8 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim8(&mut self) -> TIM8_W {
        TIM8_W { w: self }
    }
    #[doc = "Bit 0 - TIM1 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim1(&mut self) -> TIM1_W {
        TIM1_W { w: self }
    }
}
