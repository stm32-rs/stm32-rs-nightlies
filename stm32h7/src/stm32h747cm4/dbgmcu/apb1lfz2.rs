#[doc = "Reader of register APB1LFZ2"]
pub type R = crate::R<u32, super::APB1LFZ2>;
#[doc = "Writer for register APB1LFZ2"]
pub type W = crate::W<u32, super::APB1LFZ2>;
#[doc = "Register APB1LFZ2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB1LFZ2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C3`"]
pub type I2C3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3`"]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `I2C2`"]
pub type I2C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2`"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1`"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
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
#[doc = "Reader of field `WWDG2`"]
pub type WWDG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDG2`"]
pub struct WWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG2_W<'a> {
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
#[doc = "Reader of field `LPTIM1`"]
pub type LPTIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1`"]
pub struct LPTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1_W<'a> {
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
#[doc = "Reader of field `TIM14`"]
pub type TIM14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM14`"]
pub struct TIM14_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIM13`"]
pub type TIM13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM13`"]
pub struct TIM13_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIM12`"]
pub type TIM12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM12`"]
pub struct TIM12_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TIM7`"]
pub type TIM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM7`"]
pub struct TIM7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIM6`"]
pub type TIM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM6`"]
pub struct TIM6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TIM5`"]
pub type TIM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM5`"]
pub struct TIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIM4`"]
pub type TIM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM4`"]
pub struct TIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIM3`"]
pub type TIM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM3`"]
pub struct TIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_W<'a> {
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
#[doc = "Reader of field `TIM2`"]
pub type TIM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2`"]
pub struct TIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_W<'a> {
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
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WWDG2 stop in when Cortex-M4 when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn wwdg2(&self) -> WWDG2_R {
        WWDG2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim13(&self) -> TIM13_R {
        TIM13_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim12(&self) -> TIM12_R {
        TIM12_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TIM2 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 11 - WWDG2 stop in when Cortex-M4 when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn wwdg2(&mut self) -> WWDG2_W {
        WWDG2_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn lptim1(&mut self) -> LPTIM1_W {
        LPTIM1_W { w: self }
    }
    #[doc = "Bit 8 - TIM14 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim14(&mut self) -> TIM14_W {
        TIM14_W { w: self }
    }
    #[doc = "Bit 7 - TIM13 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim13(&mut self) -> TIM13_W {
        TIM13_W { w: self }
    }
    #[doc = "Bit 6 - TIM12 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim12(&mut self) -> TIM12_W {
        TIM12_W { w: self }
    }
    #[doc = "Bit 5 - TIM7 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim7(&mut self) -> TIM7_W {
        TIM7_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim6(&mut self) -> TIM6_W {
        TIM6_W { w: self }
    }
    #[doc = "Bit 3 - TIM5 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim5(&mut self) -> TIM5_W {
        TIM5_W { w: self }
    }
    #[doc = "Bit 2 - TIM4 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim4(&mut self) -> TIM4_W {
        TIM4_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim3(&mut self) -> TIM3_W {
        TIM3_W { w: self }
    }
    #[doc = "Bit 0 - TIM2 stop when Cortex-M4 in debug mode"]
    #[inline(always)]
    pub fn tim2(&mut self) -> TIM2_W {
        TIM2_W { w: self }
    }
}
