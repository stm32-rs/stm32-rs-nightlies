#[doc = "Reader of register WRP1AR"]
pub type R = crate::R<u32, super::WRP1AR>;
#[doc = "Writer for register WRP1AR"]
pub type W = crate::W<u32, super::WRP1AR>;
#[doc = "Register WRP1AR `reset()`'s with value 0"]
impl crate::ResetValue for super::WRP1AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP1A_STRT`"]
pub type WRP1A_STRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP1A_STRT`"]
pub struct WRP1A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1A_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `WRP1A_END`"]
pub type WRP1A_END_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP1A_END`"]
pub struct WRP1A_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1A_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&mut self) -> WRP1A_STRT_W {
        WRP1A_STRT_W { w: self }
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&mut self) -> WRP1A_END_W {
        WRP1A_END_W { w: self }
    }
}
