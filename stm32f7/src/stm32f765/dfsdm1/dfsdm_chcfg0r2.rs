#[doc = "Register `DFSDM_CHCFG0R2` reader"]
pub struct R(crate::R<DFSDM_CHCFG0R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHCFG0R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHCFG0R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHCFG0R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_CHCFG0R2` writer"]
pub struct W(crate::W<DFSDM_CHCFG0R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_CHCFG0R2_SPEC>;
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
impl From<crate::W<DFSDM_CHCFG0R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_CHCFG0R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTRBS` reader - Data right bit-shift for channel 0"]
pub struct DTRBS_R(crate::FieldReader<u8, u8>);
impl DTRBS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTRBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRBS` writer - Data right bit-shift for channel 0"]
pub struct DTRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `OFFSET` reader - 24-bit calibration offset for channel 0"]
pub struct OFFSET_R(crate::FieldReader<u32, u32>);
impl OFFSET_R {
    pub(crate) fn new(bits: u32) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - 24-bit calibration offset for channel 0"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Data right bit-shift for channel 0"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 24-bit calibration offset for channel 0"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:7 - Data right bit-shift for channel 0"]
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W {
        DTRBS_W { w: self }
    }
    #[doc = "Bits 8:31 - 24-bit calibration offset for channel 0"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel configuration 0 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg0r2](index.html) module"]
pub struct DFSDM_CHCFG0R2_SPEC;
impl crate::RegisterSpec for DFSDM_CHCFG0R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_chcfg0r2::R](R) reader structure"]
impl crate::Readable for DFSDM_CHCFG0R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg0r2::W](W) writer structure"]
impl crate::Writable for DFSDM_CHCFG0R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_CHCFG0R2 to value 0"]
impl crate::Resettable for DFSDM_CHCFG0R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
