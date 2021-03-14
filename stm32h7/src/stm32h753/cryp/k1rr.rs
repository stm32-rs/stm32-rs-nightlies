#[doc = "Writer for register K1RR"]
pub type W = crate::W<u32, super::K1RR>;
#[doc = "Register K1RR `reset()`'s with value 0"]
impl crate::ResetValue for super::K1RR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `K1`"]
pub struct K1_W<'a> {
    w: &'a mut W,
}
impl<'a> K1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - K128"]
    #[inline(always)]
    pub fn k1(&mut self) -> K1_W {
        K1_W { w: self }
    }
}
