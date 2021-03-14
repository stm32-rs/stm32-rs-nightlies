#[doc = "Reader of register EECR3"]
pub type R = crate::R<u32, super::EECR3>;
#[doc = "Writer for register EECR3"]
pub type W = crate::W<u32, super::EECR3>;
#[doc = "Register EECR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EECR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EE6F`"]
pub type EE6F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE6F`"]
pub struct EE6F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EE7F`"]
pub type EE7F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE7F`"]
pub struct EE7F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `EE8F`"]
pub type EE8F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE8F`"]
pub struct EE8F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EE9F`"]
pub type EE9F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE9F`"]
pub struct EE9F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `EE10F`"]
pub type EE10F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EE10F`"]
pub struct EE10F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `EEVSD`"]
pub type EEVSD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EEVSD`"]
pub struct EEVSD_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&mut self) -> EE6F_W {
        EE6F_W { w: self }
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&mut self) -> EE7F_W {
        EE7F_W { w: self }
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&mut self) -> EE8F_W {
        EE8F_W { w: self }
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&mut self) -> EE9F_W {
        EE9F_W { w: self }
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&mut self) -> EE10F_W {
        EE10F_W { w: self }
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&mut self) -> EEVSD_W {
        EEVSD_W { w: self }
    }
}
