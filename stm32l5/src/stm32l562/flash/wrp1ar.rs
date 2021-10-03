#[doc = "Register `WRP1AR` reader"]
pub struct R(crate::R<WRP1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP1AR` writer"]
pub struct W(crate::W<WRP1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP1AR_SPEC>;
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
impl From<crate::W<WRP1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP1A_PSTRT` reader - WRP1A_PSTRT"]
pub struct WRP1A_PSTRT_R(crate::FieldReader<u8, u8>);
impl WRP1A_PSTRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP1A_PSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP1A_PSTRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP1A_PSTRT` writer - WRP1A_PSTRT"]
pub struct WRP1A_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1A_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `WRP1A_PEND` reader - WRP1A_PEND"]
pub struct WRP1A_PEND_R(crate::FieldReader<u8, u8>);
impl WRP1A_PEND_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP1A_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP1A_PEND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP1A_PEND` writer - WRP1A_PEND"]
pub struct WRP1A_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1A_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - WRP1A_PSTRT"]
    #[inline(always)]
    pub fn wrp1a_pstrt(&self) -> WRP1A_PSTRT_R {
        WRP1A_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP1A_PEND"]
    #[inline(always)]
    pub fn wrp1a_pend(&self) -> WRP1A_PEND_R {
        WRP1A_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP1A_PSTRT"]
    #[inline(always)]
    pub fn wrp1a_pstrt(&mut self) -> WRP1A_PSTRT_W {
        WRP1A_PSTRT_W { w: self }
    }
    #[doc = "Bits 16:22 - WRP1A_PEND"]
    #[inline(always)]
    pub fn wrp1a_pend(&mut self) -> WRP1A_PEND_W {
        WRP1A_PEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 WRP area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1ar](index.html) module"]
pub struct WRP1AR_SPEC;
impl crate::RegisterSpec for WRP1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp1ar::R](R) reader structure"]
impl crate::Readable for WRP1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp1ar::W](W) writer structure"]
impl crate::Writable for WRP1AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRP1AR to value 0xff00_ff00"]
impl crate::Resettable for WRP1AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00_ff00
    }
}
