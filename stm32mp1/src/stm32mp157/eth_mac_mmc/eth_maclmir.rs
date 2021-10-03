#[doc = "Register `ETH_MACLMIR` reader"]
pub struct R(crate::R<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLMIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLMIR` writer"]
pub struct W(crate::W<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLMIR_SPEC>;
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
impl From<crate::W<ETH_MACLMIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLMIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSI` reader - LSI"]
pub struct LSI_R(crate::FieldReader<u8, u8>);
impl LSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI` writer - LSI"]
pub struct LSI_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DRSYNCR` reader - DRSYNCR"]
pub struct DRSYNCR_R(crate::FieldReader<u8, u8>);
impl DRSYNCR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRSYNCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRSYNCR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRSYNCR` writer - DRSYNCR"]
pub struct DRSYNCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRSYNCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `LMPDRI` reader - LMPDRI"]
pub struct LMPDRI_R(crate::FieldReader<u8, u8>);
impl LMPDRI_R {
    pub(crate) fn new(bits: u8) -> Self {
        LMPDRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMPDRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMPDRI` writer - LMPDRI"]
pub struct LMPDRI_W<'a> {
    w: &'a mut W,
}
impl<'a> LMPDRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the periodic intervals for automatic PTP packet generation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclmir](index.html) module"]
pub struct ETH_MACLMIR_SPEC;
impl crate::RegisterSpec for ETH_MACLMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maclmir::R](R) reader structure"]
impl crate::Readable for ETH_MACLMIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maclmir::W](W) writer structure"]
impl crate::Writable for ETH_MACLMIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACLMIR to value 0"]
impl crate::Resettable for ETH_MACLMIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}