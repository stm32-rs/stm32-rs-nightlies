#[doc = "Reader of register WRP1BR"]
pub type R = crate::R<u32, super::WRP1BR>;
#[doc = "Writer for register WRP1BR"]
pub type W = crate::W<u32, super::WRP1BR>;
#[doc = "Register WRP1BR `reset()`'s with value 0"]
impl crate::ResetValue for super::WRP1BR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP1B_STRT`"]
pub type WRP1B_STRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP1B_STRT`"]
pub struct WRP1B_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1B_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `WRP1B_END`"]
pub type WRP1B_END_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP1B_END`"]
pub struct WRP1B_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1B_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> WRP1B_STRT_R {
        WRP1B_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> WRP1B_END_R {
        WRP1B_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&mut self) -> WRP1B_STRT_W {
        WRP1B_STRT_W { w: self }
    }
    #[doc = "Bits 16:22 - Bank 1 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp1b_end(&mut self) -> WRP1B_END_W {
        WRP1B_END_W { w: self }
    }
}
