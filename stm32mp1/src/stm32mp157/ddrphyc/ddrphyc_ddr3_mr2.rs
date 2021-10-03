#[doc = "Register `DDRPHYC_DDR3_MR2` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR2` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR2_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASR` reader - PASR"]
pub struct PASR_R(crate::FieldReader<u8, u8>);
impl PASR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PASR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASR` writer - PASR"]
pub struct PASR_W<'a> {
    w: &'a mut W,
}
impl<'a> PASR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `CWL` reader - CWL"]
pub struct CWL_R(crate::FieldReader<u8, u8>);
impl CWL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWL` writer - CWL"]
pub struct CWL_W<'a> {
    w: &'a mut W,
}
impl<'a> CWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u16 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `ASR` reader - ASR"]
pub struct ASR_R(crate::FieldReader<bool, bool>);
impl ASR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASR` writer - ASR"]
pub struct ASR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SRT` reader - SRT"]
pub struct SRT_R(crate::FieldReader<bool, bool>);
impl SRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRT` writer - SRT"]
pub struct SRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTTWR` reader - RTTWR"]
pub struct RTTWR_R(crate::FieldReader<u8, u8>);
impl RTTWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTTWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTWR` writer - RTTWR"]
pub struct RTTWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u16 & 0x03) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&self) -> CWL_R {
        CWL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&self) -> RTTWR_R {
        RTTWR_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W {
        PASR_W { w: self }
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&mut self) -> CWL_W {
        CWL_W { w: self }
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&mut self) -> ASR_W {
        ASR_W { w: self }
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W {
        SRT_W { w: self }
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&mut self) -> RTTWR_W {
        RTTWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR2 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr2](index.html) module"]
pub struct DDRPHYC_DDR3_MR2_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr2::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr2::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR2 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
