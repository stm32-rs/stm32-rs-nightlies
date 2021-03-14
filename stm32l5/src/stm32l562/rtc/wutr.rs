#[doc = "Reader of register WUTR"]
pub type R = crate::R<u32, super::WUTR>;
#[doc = "Writer for register WUTR"]
pub type W = crate::W<u32, super::WUTR>;
#[doc = "Register WUTR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::WUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `WUT`"]
pub type WUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WUT`"]
pub struct WUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WUTOCLR`"]
pub type WUTOCLR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WUTOCLR`"]
pub struct WUTOCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTOCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits"]
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WUTOCLR"]
    #[inline(always)]
    pub fn wutoclr(&self) -> WUTOCLR_R {
        WUTOCLR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits"]
    #[inline(always)]
    pub fn wut(&mut self) -> WUT_W {
        WUT_W { w: self }
    }
    #[doc = "Bits 16:31 - WUTOCLR"]
    #[inline(always)]
    pub fn wutoclr(&mut self) -> WUTOCLR_W {
        WUTOCLR_W { w: self }
    }
}
