#[doc = "Writer for register CFR"]
pub type W = crate::W<u32, super::CFR>;
#[doc = "Register CFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CSOF`"]
pub struct CSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Clear synchronization overrun event flag"]
    #[inline(always)]
    pub fn csof(&mut self) -> CSOF_W {
        CSOF_W { w: self }
    }
}
