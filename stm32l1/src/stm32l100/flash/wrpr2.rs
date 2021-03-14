#[doc = "Reader of register WRPR2"]
pub type R = crate::R<u32, super::WRPR2>;
#[doc = "Writer for register WRPR2"]
pub type W = crate::W<u32, super::WRPR2>;
#[doc = "Register WRPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::WRPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP2`"]
pub type WRP2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRP2`"]
pub struct WRP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    pub fn wrp2(&self) -> WRP2_R {
        WRP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    pub fn wrp2(&mut self) -> WRP2_W {
        WRP2_W { w: self }
    }
}
