#[doc = "Reader of register ETH_MACLMIR"]
pub type R = crate::R<u32, super::ETH_MACLMIR>;
#[doc = "Writer for register ETH_MACLMIR"]
pub type W = crate::W<u32, super::ETH_MACLMIR>;
#[doc = "Register ETH_MACLMIR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACLMIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSI`"]
pub type LSI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSI`"]
pub struct LSI_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DRSYNCR`"]
pub type DRSYNCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRSYNCR`"]
pub struct DRSYNCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRSYNCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `LMPDRI`"]
pub type LMPDRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LMPDRI`"]
pub struct LMPDRI_W<'a> {
    w: &'a mut W,
}
impl<'a> LMPDRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&mut self) -> LSI_W {
        LSI_W { w: self }
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&mut self) -> DRSYNCR_W {
        DRSYNCR_W { w: self }
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&mut self) -> LMPDRI_W {
        LMPDRI_W { w: self }
    }
}
