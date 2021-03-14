#[doc = "Reader of register M3FAR"]
pub type R = crate::R<u32, super::M3FAR>;
#[doc = "Writer for register M3FAR"]
pub type W = crate::W<u32, super::M3FAR>;
#[doc = "Register M3FAR `reset()`'s with value 0"]
impl crate::ResetValue for super::M3FAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FADD`"]
pub type FADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FADD`"]
pub struct FADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W {
        FADD_W { w: self }
    }
}
