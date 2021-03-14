#[doc = "Reader of register WRP2AR"]
pub type R = crate::R<u32, super::WRP2AR>;
#[doc = "Writer for register WRP2AR"]
pub type W = crate::W<u32, super::WRP2AR>;
#[doc = "Register WRP2AR `reset()`'s with value 0xff00_ff00"]
impl crate::ResetValue for super::WRP2AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_ff00
    }
}
#[doc = "Reader of field `WRP2A_STRT`"]
pub type WRP2A_STRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP2A_STRT`"]
pub struct WRP2A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WRP2A_END`"]
pub type WRP2A_END_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP2A_END`"]
pub struct WRP2A_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 WRP first area A start offset"]
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W {
        WRP2A_STRT_W { w: self }
    }
    #[doc = "Bits 16:23 - Bank 2 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W {
        WRP2A_END_W { w: self }
    }
}
