#[doc = "Reader of register YBUFCFG"]
pub type R = crate::R<u32, super::YBUFCFG>;
#[doc = "Writer for register YBUFCFG"]
pub type W = crate::W<u32, super::YBUFCFG>;
#[doc = "Register YBUFCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::YBUFCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Y_BASE`"]
pub type Y_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_BASE`"]
pub struct Y_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `Y_BUF_SIZE`"]
pub type Y_BUF_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_BUF_SIZE`"]
pub struct Y_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EMPTY_WM`"]
pub type EMPTY_WM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EMPTY_WM`"]
pub struct EMPTY_WM_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn y_base(&self) -> Y_BASE_R {
        Y_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn y_buf_size(&self) -> Y_BUF_SIZE_R {
        Y_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - EMPTY_WM"]
    #[inline(always)]
    pub fn empty_wm(&self) -> EMPTY_WM_R {
        EMPTY_WM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn y_base(&mut self) -> Y_BASE_W {
        Y_BASE_W { w: self }
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn y_buf_size(&mut self) -> Y_BUF_SIZE_W {
        Y_BUF_SIZE_W { w: self }
    }
    #[doc = "Bits 24:25 - EMPTY_WM"]
    #[inline(always)]
    pub fn empty_wm(&mut self) -> EMPTY_WM_W {
        EMPTY_WM_W { w: self }
    }
}
