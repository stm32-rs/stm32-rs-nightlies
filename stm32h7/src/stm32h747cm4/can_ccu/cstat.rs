#[doc = "Register `CSTAT` reader"]
pub struct R(crate::R<CSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSTAT` writer"]
pub struct W(crate::W<CSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSTAT_SPEC>;
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
impl From<crate::W<CSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCPC` reader - Oscillator Clock Period Counter"]
pub struct OCPC_R(crate::FieldReader<u32, u32>);
impl OCPC_R {
    pub(crate) fn new(bits: u32) -> Self {
        OCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCPC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCPC` writer - Oscillator Clock Period Counter"]
pub struct OCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
#[doc = "Field `TQC` reader - Time Quanta Counter"]
pub struct TQC_R(crate::FieldReader<u16, u16>);
impl TQC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TQC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQC` writer - Time Quanta Counter"]
pub struct TQC_W<'a> {
    w: &'a mut W,
}
impl<'a> TQC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | ((value as u32 & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Field `CALS` reader - Calibration State"]
pub struct CALS_R(crate::FieldReader<u8, u8>);
impl CALS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALS` writer - Calibration State"]
pub struct CALS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&mut self) -> OCPC_W {
        OCPC_W { w: self }
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&mut self) -> TQC_W {
        TQC_W { w: self }
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&mut self) -> CALS_W {
        CALS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstat](index.html) module"]
pub struct CSTAT_SPEC;
impl crate::RegisterSpec for CSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cstat::R](R) reader structure"]
impl crate::Readable for CSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cstat::W](W) writer structure"]
impl crate::Writable for CSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSTAT to value 0"]
impl crate::Resettable for CSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
