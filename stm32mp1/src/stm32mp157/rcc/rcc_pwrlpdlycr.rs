#[doc = "Reader of register RCC_PWRLPDLYCR"]
pub type R = crate::R<u32, super::RCC_PWRLPDLYCR>;
#[doc = "Writer for register RCC_PWRLPDLYCR"]
pub type W = crate::W<u32, super::RCC_PWRLPDLYCR>;
#[doc = "Register RCC_PWRLPDLYCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PWRLPDLYCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRLP_DLY`"]
pub type PWRLP_DLY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PWRLP_DLY`"]
pub struct PWRLP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLP_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `MCTMPSKP`"]
pub type MCTMPSKP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCTMPSKP`"]
pub struct MCTMPSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTMPSKP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&self) -> PWRLP_DLY_R {
        PWRLP_DLY_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&self) -> MCTMPSKP_R {
        MCTMPSKP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&mut self) -> PWRLP_DLY_W {
        PWRLP_DLY_W { w: self }
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&mut self) -> MCTMPSKP_W {
        MCTMPSKP_W { w: self }
    }
}
