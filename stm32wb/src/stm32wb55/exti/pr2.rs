#[doc = "Reader of register PR2"]
pub type R = crate::R<u32, super::PR2>;
#[doc = "Writer for register PR2"]
pub type W = crate::W<u32, super::PR2>;
#[doc = "Register PR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIF33`"]
pub type PIF33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIF33`"]
pub struct PIF33_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF33_W<'a> {
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
#[doc = "Reader of field `PIF40_41`"]
pub type PIF40_41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIF40_41`"]
pub struct PIF40_41_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF40_41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif40_41(&self) -> PIF40_41_R {
        PIF40_41_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif33(&mut self) -> PIF33_W {
        PIF33_W { w: self }
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif40_41(&mut self) -> PIF40_41_W {
        PIF40_41_W { w: self }
    }
}
