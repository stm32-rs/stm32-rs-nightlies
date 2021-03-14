#[doc = "Reader of register SWIER1"]
pub type R = crate::R<u32, super::SWIER1>;
#[doc = "Writer for register SWIER1"]
pub type W = crate::W<u32, super::SWIER1>;
#[doc = "Register SWIER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWI`"]
pub type SWI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SWI`"]
pub struct SWI_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `SWI_31`"]
pub type SWI_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI_31`"]
pub struct SWI_31_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI_31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi(&self) -> SWI_R {
        SWI_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 31 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi_31(&self) -> SWI_31_R {
        SWI_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi(&mut self) -> SWI_W {
        SWI_W { w: self }
    }
    #[doc = "Bit 31 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi_31(&mut self) -> SWI_31_W {
        SWI_31_W { w: self }
    }
}
