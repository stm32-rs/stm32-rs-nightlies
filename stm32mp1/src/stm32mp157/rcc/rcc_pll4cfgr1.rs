#[doc = "Reader of register RCC_PLL4CFGR1"]
pub type R = crate::R<u32, super::RCC_PLL4CFGR1>;
#[doc = "Writer for register RCC_PLL4CFGR1"]
pub type W = crate::W<u32, super::RCC_PLL4CFGR1>;
#[doc = "Register RCC_PLL4CFGR1 `reset()`'s with value 0x0001_0031"]
impl crate::ResetValue for super::RCC_PLL4CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0031
    }
}
#[doc = "Reader of field `DIVN`"]
pub type DIVN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVN`"]
pub struct DIVN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `DIVM4`"]
pub type DIVM4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVM4`"]
pub struct DIVM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IFRGE`"]
pub type IFRGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFRGE`"]
pub struct IFRGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM4"]
    #[inline(always)]
    pub fn divm4(&self) -> DIVM4_R {
        DIVM4_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W {
        DIVN_W { w: self }
    }
    #[doc = "Bits 16:21 - DIVM4"]
    #[inline(always)]
    pub fn divm4(&mut self) -> DIVM4_W {
        DIVM4_W { w: self }
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&mut self) -> IFRGE_W {
        IFRGE_W { w: self }
    }
}
