#[doc = "Reader of register TAMP_CFGR"]
pub type R = crate::R<u32, super::TAMP_CFGR>;
#[doc = "Writer for register TAMP_CFGR"]
pub type W = crate::W<u32, super::TAMP_CFGR>;
#[doc = "Register TAMP_CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMP_CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT3_RMP`"]
pub type OUT3_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT3_RMP`"]
pub struct OUT3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_RMP_W<'a> {
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
    #[doc = "Bit 0 - OUT3_RMP"]
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT3_RMP"]
    #[inline(always)]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W {
        OUT3_RMP_W { w: self }
    }
}
