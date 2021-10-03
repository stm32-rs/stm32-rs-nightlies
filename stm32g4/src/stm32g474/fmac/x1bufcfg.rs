#[doc = "Register `X1BUFCFG` reader"]
pub struct R(crate::R<X1BUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<X1BUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<X1BUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<X1BUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `X1BUFCFG` writer"]
pub struct W(crate::W<X1BUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<X1BUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<X1BUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<X1BUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X1_BASE` reader - X1_BASE"]
pub struct X1_BASE_R(crate::FieldReader<u8, u8>);
impl X1_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        X1_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X1_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X1_BASE` writer - X1_BASE"]
pub struct X1_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> X1_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `X1_BUF_SIZE` reader - X1_BUF_SIZE"]
pub struct X1_BUF_SIZE_R(crate::FieldReader<u8, u8>);
impl X1_BUF_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        X1_BUF_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X1_BUF_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X1_BUF_SIZE` writer - X1_BUF_SIZE"]
pub struct X1_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> X1_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FULL_WM` reader - FULL_WM"]
pub struct FULL_WM_R(crate::FieldReader<u8, u8>);
impl FULL_WM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FULL_WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL_WM` writer - FULL_WM"]
pub struct FULL_WM_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x1_base(&self) -> X1_BASE_R {
        X1_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1_BUF_SIZE_R {
        X1_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - FULL_WM"]
    #[inline(always)]
    pub fn full_wm(&self) -> FULL_WM_R {
        FULL_WM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x1_base(&mut self) -> X1_BASE_W {
        X1_BASE_W { w: self }
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x1_buf_size(&mut self) -> X1_BUF_SIZE_W {
        X1_BUF_SIZE_W { w: self }
    }
    #[doc = "Bits 24:25 - FULL_WM"]
    #[inline(always)]
    pub fn full_wm(&mut self) -> FULL_WM_W {
        FULL_WM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC X1 Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x1bufcfg](index.html) module"]
pub struct X1BUFCFG_SPEC;
impl crate::RegisterSpec for X1BUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [x1bufcfg::R](R) reader structure"]
impl crate::Readable for X1BUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [x1bufcfg::W](W) writer structure"]
impl crate::Writable for X1BUFCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets X1BUFCFG to value 0"]
impl crate::Resettable for X1BUFCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
