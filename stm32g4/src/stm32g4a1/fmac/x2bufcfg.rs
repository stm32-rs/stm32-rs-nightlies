#[doc = "Register `X2BUFCFG` reader"]
pub struct R(crate::R<X2BUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<X2BUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<X2BUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<X2BUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `X2BUFCFG` writer"]
pub struct W(crate::W<X2BUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<X2BUFCFG_SPEC>;
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
impl From<crate::W<X2BUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<X2BUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X2_BASE` reader - X1_BASE"]
pub struct X2_BASE_R(crate::FieldReader<u8, u8>);
impl X2_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        X2_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X2_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X2_BASE` writer - X1_BASE"]
pub struct X2_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `X2_BUF_SIZE` reader - X1_BUF_SIZE"]
pub struct X2_BUF_SIZE_R(crate::FieldReader<u8, u8>);
impl X2_BUF_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        X2_BUF_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X2_BUF_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X2_BUF_SIZE` writer - X1_BUF_SIZE"]
pub struct X2_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC X2 Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x2bufcfg](index.html) module"]
pub struct X2BUFCFG_SPEC;
impl crate::RegisterSpec for X2BUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [x2bufcfg::R](R) reader structure"]
impl crate::Readable for X2BUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [x2bufcfg::W](W) writer structure"]
impl crate::Writable for X2BUFCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets X2BUFCFG to value 0"]
impl crate::Resettable for X2BUFCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
