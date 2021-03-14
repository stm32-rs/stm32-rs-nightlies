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
#[doc = "Reader of field `DBG_TIM9_STOP`"]
pub type DBG_TIM9_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM9_STOP`"]
pub struct DBG_TIM9_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM9_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM10_STOP`"]
pub type DBG_TIM10_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM10_STOP`"]
pub struct DBG_TIM10_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM10_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_TIM11_STOP`"]
pub type DBG_TIM11_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIM11_STOP`"]
pub struct DBG_TIM11_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM11_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 2 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W {
        DBG_TIM9_STOP_W { w: self }
    }
    #[doc = "Bit 3 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W {
        DBG_TIM10_STOP_W { w: self }
    }
    #[doc = "Bit 4 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W {
        DBG_TIM11_STOP_W { w: self }
    }
}
