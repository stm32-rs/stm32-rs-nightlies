#[doc = "Reader of register WRPR3"]
pub type R = crate::R<u32, super::WRPR3>;
#[doc = "Writer for register WRPR3"]
pub type W = crate::W<u32, super::WRPR3>;
#[doc = "Register WRPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::WRPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP3`"]
pub type WRP3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRP3`"]
pub struct WRP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - WRP3"]
    #[inline(always)]
    pub fn wrp3(&self) -> WRP3_R {
        WRP3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRP3"]
    #[inline(always)]
    pub fn wrp3(&mut self) -> WRP3_W {
        WRP3_W { w: self }
    }
}
