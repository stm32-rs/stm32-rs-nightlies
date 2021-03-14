#[doc = "Reader of register RCC_MP_APRSTCR"]
pub type R = crate::R<u32, super::RCC_MP_APRSTCR>;
#[doc = "Writer for register RCC_MP_APRSTCR"]
pub type W = crate::W<u32, super::RCC_MP_APRSTCR>;
#[doc = "Register RCC_MP_APRSTCR `reset()`'s with value 0x7f00"]
impl crate::ResetValue for super::RCC_MP_APRSTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f00
    }
}
#[doc = "Reader of field `RDCTLEN`"]
pub type RDCTLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDCTLEN`"]
pub struct RDCTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCTLEN_W<'a> {
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
#[doc = "Reader of field `RSTTO`"]
pub type RSTTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSTTO`"]
pub struct RSTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&mut self) -> RDCTLEN_W {
        RDCTLEN_W { w: self }
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&mut self) -> RSTTO_W {
        RSTTO_W { w: self }
    }
}
