#[doc = "Reader of register APB2FZ"]
pub type R = crate::R<u32, super::APB2FZ>;
#[doc = "Writer for register APB2FZ"]
pub type W = crate::W<u32, super::APB2FZ>;
#[doc = "Register APB2FZ `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2FZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DBG_TIM17_STO`"]
pub type DBG_TIM17_STO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM17_STO`"]
pub struct DBG_TIM17_STO_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_STO_W<'a> {
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
#[doc = "Reader of field `DBG_TIM19_STOP`"]
pub type DBG_TIM19_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM19_STOP`"]
pub struct DBG_TIM19_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM19_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&self) -> DBG_TIM17_STO_R {
        DBG_TIM17_STO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DBG_TIM19_STOP_R {
        DBG_TIM19_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W {
        DBG_TIM15_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W {
        DBG_TIM16_STOP_W { w: self }
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&mut self) -> DBG_TIM17_STO_W {
        DBG_TIM17_STO_W { w: self }
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&mut self) -> DBG_TIM19_STOP_W {
        DBG_TIM19_STOP_W { w: self }
    }
}
