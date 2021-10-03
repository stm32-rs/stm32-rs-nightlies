#[doc = "Register `OPAMP1_CSR` reader"]
pub struct R(crate::R<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP1_CSR` writer"]
pub struct W(crate::W<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_CSR_SPEC>;
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
impl From<crate::W<OPAMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub struct OPAEN_R(crate::FieldReader<bool, bool>);
impl OPAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub struct OPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `OPALPM` reader - Operational amplifier Low Power Mode"]
pub struct OPALPM_R(crate::FieldReader<bool, bool>);
impl OPALPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPALPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPALPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPALPM` writer - Operational amplifier Low Power Mode"]
pub struct OPALPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPALPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `OPAMODE` reader - Operational amplifier PGA mode"]
pub struct OPAMODE_R(crate::FieldReader<u8, u8>);
impl OPAMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPAMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMODE` writer - Operational amplifier PGA mode"]
pub struct OPAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub struct PGA_GAIN_R(crate::FieldReader<u8, u8>);
impl PGA_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGA_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGA_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub struct PGA_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub struct VM_SEL_R(crate::FieldReader<u8, u8>);
impl VM_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VM_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub struct VM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VM_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub struct VP_SEL_R(crate::FieldReader<bool, bool>);
impl VP_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub struct VP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VP_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub struct CALON_R(crate::FieldReader<bool, bool>);
impl CALON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub struct CALON_W<'a> {
    w: &'a mut W,
}
impl<'a> CALON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub struct CALSEL_R(crate::FieldReader<bool, bool>);
impl CALSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub struct CALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset"]
pub struct USERTRIM_R(crate::FieldReader<bool, bool>);
impl USERTRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USERTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USERTRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset"]
pub struct USERTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USERTRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub struct CALOUT_R(crate::FieldReader<bool, bool>);
impl CALOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub struct CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `OPA_RANGE` reader - Operational amplifier power supply range for stability"]
pub struct OPA_RANGE_R(crate::FieldReader<bool, bool>);
impl OPA_RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA_RANGE` writer - Operational amplifier power supply range for stability"]
pub struct OPA_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability"]
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W {
        OPAEN_W { w: self }
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W {
        OPALPM_W { w: self }
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W {
        OPAMODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W {
        VP_SEL_W { w: self }
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W {
        USERTRIM_W { w: self }
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W {
        CALOUT_W { w: self }
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability"]
    #[inline(always)]
    pub fn opa_range(&mut self) -> OPA_RANGE_W {
        OPA_RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP1 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_csr](index.html) module"]
pub struct OPAMP1_CSR_SPEC;
impl crate::RegisterSpec for OPAMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp1_csr::R](R) reader structure"]
impl crate::Readable for OPAMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp1_csr::W](W) writer structure"]
impl crate::Writable for OPAMP1_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP1_CSR to value 0"]
impl crate::Resettable for OPAMP1_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
