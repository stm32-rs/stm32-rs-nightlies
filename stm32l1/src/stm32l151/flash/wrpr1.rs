#[doc = "Reader of register WRPR1"]
pub type R = crate::R<u32, super::WRPR1>;
#[doc = "Writer for register WRPR1"]
pub type W = crate::W<u32, super::WRPR1>;
#[doc = "Register WRPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::WRPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP1`"]
pub type WRP1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRP1`"]
pub struct WRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&mut self) -> WRP1_W {
        WRP1_W { w: self }
    }
}
