#[doc = "Register `HWCFGR` reader"]
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR` writer"]
pub struct W(crate::W<HWCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR_SPEC>;
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
impl From<crate::W<HWCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINDOW` reader - Support of Window function"]
pub struct WINDOW_R(crate::FieldReader<u8, u8>);
impl WINDOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        WINDOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINDOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINDOW` writer - Support of Window function"]
pub struct WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PR_DEFAULT` reader - Prescaler default value"]
pub struct PR_DEFAULT_R(crate::FieldReader<u8, u8>);
impl PR_DEFAULT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PR_DEFAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_DEFAULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR_DEFAULT` writer - Prescaler default value"]
pub struct PR_DEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_DEFAULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W {
        WINDOW_W { w: self }
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    pub fn pr_default(&mut self) -> PR_DEFAULT_W {
        PR_DEFAULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr::W](W) writer structure"]
impl crate::Writable for HWCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR to value 0x71"]
impl crate::Resettable for HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x71
    }
}
