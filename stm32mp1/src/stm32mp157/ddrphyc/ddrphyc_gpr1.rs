#[doc = "Reader of register DDRPHYC_GPR1"]
pub type R = crate::R<u32, super::DDRPHYC_GPR1>;
#[doc = "Writer for register DDRPHYC_GPR1"]
pub type W = crate::W<u32, super::DDRPHYC_GPR1>;
#[doc = "Register DDRPHYC_GPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_GPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPR1`"]
pub type GPR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPR1`"]
pub struct GPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    pub fn gpr1(&self) -> GPR1_R {
        GPR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    pub fn gpr1(&mut self) -> GPR1_W {
        GPR1_W { w: self }
    }
}
