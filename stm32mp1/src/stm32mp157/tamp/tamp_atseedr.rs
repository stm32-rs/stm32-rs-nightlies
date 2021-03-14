#[doc = "Writer for register TAMP_ATSEEDR"]
pub type W = crate::W<u32, super::TAMP_ATSEEDR>;
#[doc = "Register TAMP_ATSEEDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMP_ATSEEDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SEED`"]
pub struct SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W {
        SEED_W { w: self }
    }
}
