#[doc = "Reader of register FDCAN_TXBAR"]
pub type R = crate::R<u32, super::FDCAN_TXBAR>;
#[doc = "Writer for register FDCAN_TXBAR"]
pub type W = crate::W<u32, super::FDCAN_TXBAR>;
#[doc = "Register FDCAN_TXBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AR`"]
pub type AR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AR`"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AR"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AR"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
}
