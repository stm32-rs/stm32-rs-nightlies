#[doc = "Reader of register PR1"]
pub type R = crate::R<u32, super::PR1>;
#[doc = "Writer for register PR1"]
pub type W = crate::W<u32, super::PR1>;
#[doc = "Register PR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIF`"]
pub type PIF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PIF`"]
pub struct PIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `PIF_31`"]
pub type PIF_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIF_31`"]
pub struct PIF_31_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF_31_W<'a> {
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
    #[doc = "Bits 0:21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif(&self) -> PIF_R {
        PIF_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif_31(&self) -> PIF_31_R {
        PIF_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif(&mut self) -> PIF_W {
        PIF_W { w: self }
    }
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif_31(&mut self) -> PIF_31_W {
        PIF_31_W { w: self }
    }
}
