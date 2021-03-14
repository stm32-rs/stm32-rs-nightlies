#[doc = "Reader of register DDRPHYC_GPR0"]
pub type R = crate::R<u32, super::DDRPHYC_GPR0>;
#[doc = "Writer for register DDRPHYC_GPR0"]
pub type W = crate::W<u32, super::DDRPHYC_GPR0>;
#[doc = "Register DDRPHYC_GPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_GPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPR0`"]
pub type GPR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPR0`"]
pub struct GPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPR0"]
    #[inline(always)]
    pub fn gpr0(&self) -> GPR0_R {
        GPR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPR0"]
    #[inline(always)]
    pub fn gpr0(&mut self) -> GPR0_W {
        GPR0_W { w: self }
    }
}
