#[doc = "Writer for register NSBOOTADD0R"]
pub type W = crate::W<u32, super::NSBOOTADD0R>;
#[doc = "Register NSBOOTADD0R `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::NSBOOTADD0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Write proxy for field `NSBOOTADD0`"]
pub struct NSBOOTADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> NSBOOTADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD0"]
    #[inline(always)]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W {
        NSBOOTADD0_W { w: self }
    }
}
