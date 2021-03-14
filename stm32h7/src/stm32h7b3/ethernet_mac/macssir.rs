#[doc = "Reader of register MACSSIR"]
pub type R = crate::R<u32, super::MACSSIR>;
#[doc = "Writer for register MACSSIR"]
pub type W = crate::W<u32, super::MACSSIR>;
#[doc = "Register MACSSIR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACSSIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SNSINC`"]
pub type SNSINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SNSINC`"]
pub struct SNSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SNSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SSINC`"]
pub type SSINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSINC`"]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SNSINC"]
    #[inline(always)]
    pub fn snsinc(&mut self) -> SNSINC_W {
        SNSINC_W { w: self }
    }
    #[doc = "Bits 16:23 - SSINC"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
}
