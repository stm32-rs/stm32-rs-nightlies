#[doc = "Writer for register JPEG_CFR"]
pub type W = crate::W<u32, super::JPEG_CFR>;
#[doc = "Register JPEG_CFR `reset()`'s with value 0"]
impl crate::ResetValue for super::JPEG_CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CEOCF`"]
pub struct CEOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEOCF_W<'a> {
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
#[doc = "Write proxy for field `CHPDF`"]
pub struct CHPDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPDF_W<'a> {
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
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag"]
    #[inline(always)]
    pub fn ceocf(&mut self) -> CEOCF_W {
        CEOCF_W { w: self }
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag"]
    #[inline(always)]
    pub fn chpdf(&mut self) -> CHPDF_W {
        CHPDF_W { w: self }
    }
}
