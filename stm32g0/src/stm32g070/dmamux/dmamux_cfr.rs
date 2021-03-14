#[doc = "Writer for register DMAMUX_CFR"]
pub type W = crate::W<u32, super::DMAMUX_CFR>;
#[doc = "Register DMAMUX_CFR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAMUX_CFR {
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:6 - Clear synchronization overrun event flag"]
    #[inline(always)]
    pub fn csof(&mut self) -> CSOF_W {
        CSOF_W { w: self }
    }
}
