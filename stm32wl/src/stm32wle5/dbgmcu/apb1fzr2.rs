#[doc = "Reader of register APB1FZR2"]
pub type R = crate::R<u32, super::APB1FZR2>;
#[doc = "Writer for register APB1FZR2"]
pub type W = crate::W<u32, super::APB1FZR2>;
#[doc = "Register APB1FZR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB1FZR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_LPTIM2_STOP`"]
pub type DBG_LPTIM2_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_LPTIM2_STOP`"]
pub struct DBG_LPTIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM2_STOP_W<'a> {
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
#[doc = "Reader of field `DBG_LPTIM3_STOP`"]
pub type DBG_LPTIM3_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_LPTIM3_STOP`"]
pub struct DBG_LPTIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM3_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W {
        DBG_LPTIM2_STOP_W { w: self }
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W {
        DBG_LPTIM3_STOP_W { w: self }
    }
}
