#[doc = "Reader of register APB2_FZ"]
pub type R = crate::R<u32, super::APB2_FZ>;
#[doc = "Writer for register APB2_FZ"]
pub type W = crate::W<u32, super::APB2_FZ>;
#[doc = "Register APB2_FZ `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2_FZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_TIM1_STOP`"]
pub type DBG_TIM1_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM1_STOP`"]
pub struct DBG_TIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM1_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM8_STOP`"]
pub type DBG_TIM8_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM8_STOP`"]
pub struct DBG_TIM8_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM8_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DBG_TIM15_STOP`"]
pub type DBG_TIM15_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM15_STOP`"]
pub struct DBG_TIM15_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM15_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM16_STOP`"]
pub type DBG_TIM16_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM16_STOP`"]
pub struct DBG_TIM16_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM16_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM17_STOP`"]
pub type DBG_TIM17_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM17_STOP`"]
pub struct DBG_TIM17_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM20_STOP`"]
pub type DBG_TIM20_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM20_STOP`"]
pub struct DBG_TIM20_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM20_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DBG_HRTIM0_STOP`"]
pub type DBG_HRTIM0_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_HRTIM0_STOP`"]
pub struct DBG_HRTIM0_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_HRTIM0_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DBG_HRTIM1_STOP`"]
pub type DBG_HRTIM1_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_HRTIM1_STOP`"]
pub struct DBG_HRTIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_HRTIM1_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DBG_HRTIM2_STOP`"]
pub type DBG_HRTIM2_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_HRTIM2_STOP`"]
pub struct DBG_HRTIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_HRTIM2_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DBG_HRTIM3_STOP`"]
pub type DBG_HRTIM3_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_HRTIM3_STOP`"]
pub struct DBG_HRTIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_HRTIM3_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim20_stop(&self) -> DBG_TIM20_STOP_R {
        DBG_TIM20_STOP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&self) -> DBG_HRTIM0_STOP_R {
        DBG_HRTIM0_STOP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&self) -> DBG_HRTIM1_STOP_R {
        DBG_HRTIM1_STOP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&self) -> DBG_HRTIM2_STOP_R {
        DBG_HRTIM2_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&self) -> DBG_HRTIM3_STOP_R {
        DBG_HRTIM3_STOP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W {
        DBG_TIM1_STOP_W { w: self }
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W {
        DBG_TIM8_STOP_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W {
        DBG_TIM15_STOP_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W {
        DBG_TIM16_STOP_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W {
        DBG_TIM17_STOP_W { w: self }
    }
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim20_stop(&mut self) -> DBG_TIM20_STOP_W {
        DBG_TIM20_STOP_W { w: self }
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&mut self) -> DBG_HRTIM0_STOP_W {
        DBG_HRTIM0_STOP_W { w: self }
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&mut self) -> DBG_HRTIM1_STOP_W {
        DBG_HRTIM1_STOP_W { w: self }
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&mut self) -> DBG_HRTIM2_STOP_W {
        DBG_HRTIM2_STOP_W { w: self }
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&mut self) -> DBG_HRTIM3_STOP_W {
        DBG_HRTIM3_STOP_W { w: self }
    }
}
