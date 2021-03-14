#[doc = "Writer for register AR"]
pub type W = crate::W<u32, super::AR>;
#[doc = "Register AR `reset()`'s with value 0"]
impl crate::ResetValue for super::AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FAR`"]
pub struct FAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    pub fn far(&mut self) -> FAR_W {
        FAR_W { w: self }
    }
}
