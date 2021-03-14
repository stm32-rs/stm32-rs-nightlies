#[doc = "Reader of register X2BUFCFG"]
pub type R = crate::R<u32, super::X2BUFCFG>;
#[doc = "Writer for register X2BUFCFG"]
pub type W = crate::W<u32, super::X2BUFCFG>;
#[doc = "Register X2BUFCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::X2BUFCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X2_BASE`"]
pub type X2_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X2_BASE`"]
pub struct X2_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `X2_BUF_SIZE`"]
pub type X2_BUF_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X2_BUF_SIZE`"]
pub struct X2_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x2_base(&mut self) -> X2_BASE_W {
        X2_BASE_W { w: self }
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W {
        X2_BUF_SIZE_W { w: self }
    }
}
