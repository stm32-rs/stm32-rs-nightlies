#[doc = "Writer for register K0LR"]
pub type W = crate::W<u32, super::K0LR>;
#[doc = "Register K0LR `reset()`'s with value 0"]
impl crate::ResetValue for super::K0LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `K2`"]
pub struct K2_W<'a> {
    w: &'a mut W,
}
impl<'a> K2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - K224"]
    #[inline(always)]
    pub fn k2(&mut self) -> K2_W {
        K2_W { w: self }
    }
}
