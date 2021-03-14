#[doc = "Reader of register IWDG_WINR"]
pub type R = crate::R<u32, super::IWDG_WINR>;
#[doc = "Writer for register IWDG_WINR"]
pub type W = crate::W<u32, super::IWDG_WINR>;
#[doc = "Register IWDG_WINR `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::IWDG_WINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `WIN`"]
pub type WIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN`"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - WIN"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WIN"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
}
