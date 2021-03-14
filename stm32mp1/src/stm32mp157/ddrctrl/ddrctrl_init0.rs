#[doc = "Reader of register DDRCTRL_INIT0"]
pub type R = crate::R<u32, super::DDRCTRL_INIT0>;
#[doc = "Writer for register DDRCTRL_INIT0"]
pub type W = crate::W<u32, super::DDRCTRL_INIT0>;
#[doc = "Register DDRCTRL_INIT0 `reset()`'s with value 0x0002_004e"]
impl crate::ResetValue for super::DDRCTRL_INIT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_004e
    }
}
#[doc = "Reader of field `PRE_CKE_X1024`"]
pub type PRE_CKE_X1024_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRE_CKE_X1024`"]
pub struct PRE_CKE_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_CKE_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `POST_CKE_X1024`"]
pub type POST_CKE_X1024_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POST_CKE_X1024`"]
pub struct POST_CKE_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_CKE_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SKIP_DRAM_INIT`"]
pub type SKIP_DRAM_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SKIP_DRAM_INIT`"]
pub struct SKIP_DRAM_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_DRAM_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&self) -> PRE_CKE_X1024_R {
        PRE_CKE_X1024_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&self) -> POST_CKE_X1024_R {
        POST_CKE_X1024_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&self) -> SKIP_DRAM_INIT_R {
        SKIP_DRAM_INIT_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&mut self) -> PRE_CKE_X1024_W {
        PRE_CKE_X1024_W { w: self }
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&mut self) -> POST_CKE_X1024_W {
        POST_CKE_X1024_W { w: self }
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&mut self) -> SKIP_DRAM_INIT_W {
        SKIP_DRAM_INIT_W { w: self }
    }
}
