#[doc = "Reader of register FPUIMR"]
pub type R = crate::R<u32, super::FPUIMR>;
#[doc = "Writer for register FPUIMR"]
pub type W = crate::W<u32, super::FPUIMR>;
#[doc = "Register FPUIMR `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::FPUIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `FPU_IE`"]
pub type FPU_IE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPU_IE`"]
pub struct FPU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FPU_IE_W {
        FPU_IE_W { w: self }
    }
}
