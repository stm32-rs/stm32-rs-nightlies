#[doc = "Reader of register RCC_MSSCKSELR"]
pub type R = crate::R<u32, super::RCC_MSSCKSELR>;
#[doc = "Writer for register RCC_MSSCKSELR"]
pub type W = crate::W<u32, super::RCC_MSSCKSELR>;
#[doc = "Register RCC_MSSCKSELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_MSSCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `MCUSSRC`"]
pub type MCUSSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCUSSRC`"]
pub struct MCUSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUSSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `MCUSSRCRDY`"]
pub type MCUSSRCRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&self) -> MCUSSRC_R {
        MCUSSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - MCUSSRCRDY"]
    #[inline(always)]
    pub fn mcussrcrdy(&self) -> MCUSSRCRDY_R {
        MCUSSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&mut self) -> MCUSSRC_W {
        MCUSSRC_W { w: self }
    }
}
