#[doc = "Register `YBUFCFG` reader"]
pub struct R(crate::R<YBUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YBUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YBUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YBUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `YBUFCFG` writer"]
pub struct W(crate::W<YBUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YBUFCFG_SPEC>;
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
impl From<crate::W<YBUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YBUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_BASE` reader - X1_BASE"]
pub struct Y_BASE_R(crate::FieldReader<u8, u8>);
impl Y_BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        Y_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y_BASE` writer - X1_BASE"]
pub struct Y_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Y_BUF_SIZE` reader - X1_BUF_SIZE"]
pub struct Y_BUF_SIZE_R(crate::FieldReader<u8, u8>);
impl Y_BUF_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        Y_BUF_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_BUF_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y_BUF_SIZE` writer - X1_BUF_SIZE"]
pub struct Y_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EMPTY_WM` reader - EMPTY_WM"]
pub struct EMPTY_WM_R(crate::FieldReader<u8, u8>);
impl EMPTY_WM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMPTY_WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY_WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY_WM` writer - EMPTY_WM"]
pub struct EMPTY_WM_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_WM_W<'a> {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC Y Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ybufcfg](index.html) module"]
pub struct YBUFCFG_SPEC;
impl crate::RegisterSpec for YBUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ybufcfg::R](R) reader structure"]
impl crate::Readable for YBUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ybufcfg::W](W) writer structure"]
impl crate::Writable for YBUFCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets YBUFCFG to value 0"]
impl crate::Resettable for YBUFCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
