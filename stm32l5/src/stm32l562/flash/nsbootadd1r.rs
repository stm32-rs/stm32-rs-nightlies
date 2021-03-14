#[doc = "Writer for register NSBOOTADD1R"]
pub type W = crate::W<u32, super::NSBOOTADD1R>;
#[doc = "Register NSBOOTADD1R `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::NSBOOTADD1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Write proxy for field `NSBOOTADD1`"]
pub struct NSBOOTADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSBOOTADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD1"]
    #[inline(always)]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W {
        NSBOOTADD1_W { w: self }
    }
}
