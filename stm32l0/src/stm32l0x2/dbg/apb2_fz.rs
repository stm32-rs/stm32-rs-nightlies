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
#[doc = "Debug Timer 21 stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIMER21_STOP_A {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    CONTINUE = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    STOP = 1,
}
impl From<DBG_TIMER21_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER21_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBG_TIMER21_STOP`"]
pub type DBG_TIMER21_STOP_R = crate::R<bool, DBG_TIMER21_STOP_A>;
impl DBG_TIMER21_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIMER21_STOP_A {
        match self.bits {
            false => DBG_TIMER21_STOP_A::CONTINUE,
            true => DBG_TIMER21_STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue_(&self) -> bool {
        *self == DBG_TIMER21_STOP_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIMER21_STOP_A::STOP
    }
}
#[doc = "Write proxy for field `DBG_TIMER21_STOP`"]
pub struct DBG_TIMER21_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER21_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIMER21_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::CONTINUE)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::STOP)
    }
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
#[doc = "Reader of field `DBG_TIMER22_STO`"]
pub type DBG_TIMER22_STO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_TIMER22_STO`"]
pub struct DBG_TIMER22_STO_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER22_STO_W<'a> {
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
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer21_stop(&self) -> DBG_TIMER21_STOP_R {
        DBG_TIMER21_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer22_sto(&self) -> DBG_TIMER22_STO_R {
        DBG_TIMER22_STO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer21_stop(&mut self) -> DBG_TIMER21_STOP_W {
        DBG_TIMER21_STOP_W { w: self }
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer22_sto(&mut self) -> DBG_TIMER22_STO_W {
        DBG_TIMER22_STO_W { w: self }
    }
}
