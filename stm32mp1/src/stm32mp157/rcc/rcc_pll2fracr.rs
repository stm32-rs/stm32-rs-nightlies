#[doc = "Reader of register RCC_PLL2FRACR"]
pub type R = crate::R<u32, super::RCC_PLL2FRACR>;
#[doc = "Writer for register RCC_PLL2FRACR"]
pub type W = crate::W<u32, super::RCC_PLL2FRACR>;
#[doc = "Register RCC_PLL2FRACR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PLL2FRACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACV`"]
pub type FRACV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRACV`"]
pub struct FRACV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
#[doc = "Reader of field `FRACLE`"]
pub type FRACLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRACLE`"]
pub struct FRACLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&self) -> FRACV_R {
        FRACV_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&self) -> FRACLE_R {
        FRACLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&mut self) -> FRACV_W {
        FRACV_W { w: self }
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&mut self) -> FRACLE_W {
        FRACLE_W { w: self }
    }
}
