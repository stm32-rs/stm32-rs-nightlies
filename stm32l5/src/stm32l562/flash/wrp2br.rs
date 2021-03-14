#[doc = "Reader of register WRP2BR"]
pub type R = crate::R<u32, super::WRP2BR>;
#[doc = "Writer for register WRP2BR"]
pub type W = crate::W<u32, super::WRP2BR>;
#[doc = "Register WRP2BR `reset()`'s with value 0xff00_ff00"]
impl crate::ResetValue for super::WRP2BR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_ff00
    }
}
#[doc = "Reader of field `WRP2B_PSTRT`"]
pub type WRP2B_PSTRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP2B_PSTRT`"]
pub struct WRP2B_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2B_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `WRP2B_PEND`"]
pub type WRP2B_PEND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP2B_PEND`"]
pub struct WRP2B_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2B_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - WRP2B_PSTRT"]
    #[inline(always)]
    pub fn wrp2b_pstrt(&self) -> WRP2B_PSTRT_R {
        WRP2B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2B_PEND"]
    #[inline(always)]
    pub fn wrp2b_pend(&self) -> WRP2B_PEND_R {
        WRP2B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2B_PSTRT"]
    #[inline(always)]
    pub fn wrp2b_pstrt(&mut self) -> WRP2B_PSTRT_W {
        WRP2B_PSTRT_W { w: self }
    }
    #[doc = "Bits 16:22 - WRP2B_PEND"]
    #[inline(always)]
    pub fn wrp2b_pend(&mut self) -> WRP2B_PEND_W {
        WRP2B_PEND_W { w: self }
    }
}
