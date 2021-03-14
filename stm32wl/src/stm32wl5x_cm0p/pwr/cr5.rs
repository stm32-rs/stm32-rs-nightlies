#[doc = "Reader of register CR5"]
pub type R = crate::R<u32, super::CR5>;
#[doc = "Writer for register CR5"]
pub type W = crate::W<u32, super::CR5>;
#[doc = "Register CR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPSEN`"]
pub type SMPSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPSEN`"]
pub struct SMPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RFEOLEN`"]
pub type RFEOLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEOLEN`"]
pub struct RFEOLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEOLEN_W<'a> {
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
impl R {
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&self) -> RFEOLEN_R {
        RFEOLEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&mut self) -> SMPSEN_W {
        SMPSEN_W { w: self }
    }
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&mut self) -> RFEOLEN_W {
        RFEOLEN_W { w: self }
    }
}
