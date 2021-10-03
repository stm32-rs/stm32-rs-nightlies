#[doc = "Register `CWD` reader"]
pub struct R(crate::R<CWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWD` writer"]
pub struct W(crate::W<CWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWD_SPEC>;
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
impl From<crate::W<CWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDC` reader - WDC"]
pub struct WDC_R(crate::FieldReader<u16, u16>);
impl WDC_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDC` writer - WDC"]
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `WDV` reader - WDV"]
pub struct WDV_R(crate::FieldReader<u16, u16>);
impl WDV_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDV` writer - WDV"]
pub struct WDV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&mut self) -> WDV_W {
        WDV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwd](index.html) module"]
pub struct CWD_SPEC;
impl crate::RegisterSpec for CWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwd::R](R) reader structure"]
impl crate::Readable for CWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwd::W](W) writer structure"]
impl crate::Writable for CWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWD to value 0"]
impl crate::Resettable for CWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
