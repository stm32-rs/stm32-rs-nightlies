#[doc = "Reader of register SWIER2"]
pub type R = crate::R<u32, super::SWIER2>;
#[doc = "Writer for register SWIER2"]
pub type W = crate::W<u32, super::SWIER2>;
#[doc = "Register SWIER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWI33`"]
pub type SWI33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI33`"]
pub struct SWI33_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI33_W<'a> {
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
#[doc = "Reader of field `SWI40_41`"]
pub type SWI40_41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWI40_41`"]
pub struct SWI40_41_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI40_41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi40_41(&self) -> SWI40_41_R {
        SWI40_41_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi33(&mut self) -> SWI33_W {
        SWI33_W { w: self }
    }
    #[doc = "Bits 8:9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi40_41(&mut self) -> SWI40_41_W {
        SWI40_41_W { w: self }
    }
}
