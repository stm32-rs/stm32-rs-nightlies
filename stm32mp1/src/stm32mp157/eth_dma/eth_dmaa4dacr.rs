#[doc = "Reader of register ETH_DMAA4DACR"]
pub type R = crate::R<u32, super::ETH_DMAA4DACR>;
#[doc = "Writer for register ETH_DMAA4DACR"]
pub type W = crate::W<u32, super::ETH_DMAA4DACR>;
#[doc = "Register ETH_DMAA4DACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAA4DACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDWC`"]
pub type TDWC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDWC`"]
pub struct TDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TDWD`"]
pub type TDWD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDWD`"]
pub struct TDWD_W<'a> {
    w: &'a mut W,
}
impl<'a> TDWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RDRC`"]
pub type RDRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDRC`"]
pub struct RDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RDP`"]
pub type RDP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDP`"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `WRP`"]
pub type WRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRP`"]
pub struct WRP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&self) -> TDWC_R {
        TDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&self) -> TDWD_R {
        TDWD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&self) -> RDRC_R {
        RDRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&mut self) -> TDWC_W {
        TDWC_W { w: self }
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&mut self) -> TDWD_W {
        TDWD_W { w: self }
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&mut self) -> RDRC_W {
        RDRC_W { w: self }
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WRP_W {
        WRP_W { w: self }
    }
}
