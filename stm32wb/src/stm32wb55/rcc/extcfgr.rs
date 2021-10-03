#[doc = "Register `EXTCFGR` reader"]
pub struct R(crate::R<EXTCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCFGR` writer"]
pub struct W(crate::W<EXTCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCFGR_SPEC>;
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
impl From<crate::W<EXTCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCSS` reader - RF clock source selected"]
pub struct RFCSS_R(crate::FieldReader<bool, bool>);
impl RFCSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2HPREF` reader - CPU2 AHB prescaler flag"]
pub struct C2HPREF_R(crate::FieldReader<bool, bool>);
impl C2HPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2HPREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HPREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHDHPREF` reader - Shared AHB prescaler flag"]
pub struct SHDHPREF_R(crate::FieldReader<bool, bool>);
impl SHDHPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHDHPREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHDHPREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2HPRE` reader - CPU2 AHB prescaler"]
pub struct C2HPRE_R(crate::FieldReader<u8, u8>);
impl C2HPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        C2HPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2HPRE` writer - CPU2 AHB prescaler"]
pub struct C2HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> C2HPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SHDHPRE` reader - Shared AHB prescaler"]
pub struct SHDHPRE_R(crate::FieldReader<u8, u8>);
impl SHDHPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHDHPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHDHPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHDHPRE` writer - Shared AHB prescaler"]
pub struct SHDHPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHDHPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - RF clock source selected"]
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU2 AHB prescaler flag"]
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Shared AHB prescaler flag"]
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&mut self) -> C2HPRE_W {
        C2HPRE_W { w: self }
    }
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&mut self) -> SHDHPRE_W {
        SHDHPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended clock recovery register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extcfgr](index.html) module"]
pub struct EXTCFGR_SPEC;
impl crate::RegisterSpec for EXTCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extcfgr::R](R) reader structure"]
impl crate::Readable for EXTCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extcfgr::W](W) writer structure"]
impl crate::Writable for EXTCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTCFGR to value 0x0003_0000"]
impl crate::Resettable for EXTCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0000
    }
}
