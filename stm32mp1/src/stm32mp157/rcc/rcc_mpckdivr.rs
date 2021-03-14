#[doc = "Reader of register RCC_MPCKDIVR"]
pub type R = crate::R<u32, super::RCC_MPCKDIVR>;
#[doc = "Writer for register RCC_MPCKDIVR"]
pub type W = crate::W<u32, super::RCC_MPCKDIVR>;
#[doc = "Register RCC_MPCKDIVR `reset()`'s with value 0x8000_0001"]
impl crate::ResetValue for super::RCC_MPCKDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0001
    }
}
#[doc = "Reader of field `MPUDIV`"]
pub type MPUDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MPUDIV`"]
pub struct MPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `MPUDIVRDY`"]
pub type MPUDIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - MPUDIV"]
    #[inline(always)]
    pub fn mpudiv(&self) -> MPUDIV_R {
        MPUDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - MPUDIVRDY"]
    #[inline(always)]
    pub fn mpudivrdy(&self) -> MPUDIVRDY_R {
        MPUDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MPUDIV"]
    #[inline(always)]
    pub fn mpudiv(&mut self) -> MPUDIV_W {
        MPUDIV_W { w: self }
    }
}
