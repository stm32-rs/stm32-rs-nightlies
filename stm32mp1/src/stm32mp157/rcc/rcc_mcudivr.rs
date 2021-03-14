#[doc = "Reader of register RCC_MCUDIVR"]
pub type R = crate::R<u32, super::RCC_MCUDIVR>;
#[doc = "Writer for register RCC_MCUDIVR"]
pub type W = crate::W<u32, super::RCC_MCUDIVR>;
#[doc = "Register RCC_MCUDIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_MCUDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `MCUDIV`"]
pub type MCUDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCUDIV`"]
pub struct MCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MCUDIVRDY`"]
pub type MCUDIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MCUDIVRDY"]
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&mut self) -> MCUDIV_W {
        MCUDIV_W { w: self }
    }
}
