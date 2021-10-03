#[doc = "Register `SMPSCR` reader"]
pub struct R(crate::R<SMPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPSCR` writer"]
pub struct W(crate::W<SMPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPSCR_SPEC>;
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
impl From<crate::W<SMPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMPSSWS` reader - Step Down converter clock switch status"]
pub struct SMPSSWS_R(crate::FieldReader<u8, u8>);
impl SMPSSWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPSSWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSSWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSDIV` reader - Step Down converter clock prescaler"]
pub struct SMPSDIV_R(crate::FieldReader<u8, u8>);
impl SMPSDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPSDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSDIV` writer - Step Down converter clock prescaler"]
pub struct SMPSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `SMPSSEL` reader - Step Down converter clock selection"]
pub struct SMPSSEL_R(crate::FieldReader<u8, u8>);
impl SMPSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSSEL` writer - Step Down converter clock selection"]
pub struct SMPSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Step Down converter clock switch status"]
    #[inline(always)]
    pub fn smpssws(&self) -> SMPSSWS_R {
        SMPSSWS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    pub fn smpssel(&self) -> SMPSSEL_R {
        SMPSSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W {
        SMPSDIV_W { w: self }
    }
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    pub fn smpssel(&mut self) -> SMPSSEL_W {
        SMPSSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Step Down converter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpscr](index.html) module"]
pub struct SMPSCR_SPEC;
impl crate::RegisterSpec for SMPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpscr::R](R) reader structure"]
impl crate::Readable for SMPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpscr::W](W) writer structure"]
impl crate::Writable for SMPSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPSCR to value 0x0301"]
impl crate::Resettable for SMPSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0301
    }
}
