#[doc = "Register `TIMECR2` reader"]
pub struct R(crate::R<TIMECR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMECR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMECR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMECR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMECR2` writer"]
pub struct W(crate::W<TIMECR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMECR2_SPEC>;
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
impl From<crate::W<TIMECR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMECR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGHLF` reader - Triggered-half mode"]
pub struct TRGHLF_R(crate::FieldReader<bool, bool>);
impl TRGHLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGHLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGHLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGHLF` writer - Triggered-half mode"]
pub struct TRGHLF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGHLF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `GTCMP3` reader - Greater than Compare 3 PWM mode"]
pub struct GTCMP3_R(crate::FieldReader<bool, bool>);
impl GTCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTCMP3` writer - Greater than Compare 3 PWM mode"]
pub struct GTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> GTCMP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `GTCMP1` reader - Greater than Compare 1 PWM mode"]
pub struct GTCMP1_R(crate::FieldReader<bool, bool>);
impl GTCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTCMP1` writer - Greater than Compare 1 PWM mode"]
pub struct GTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GTCMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FEROM` reader - Fault and Event Roll-Over Mode"]
pub struct FEROM_R(crate::FieldReader<u8, u8>);
impl FEROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FEROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEROM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEROM` writer - Fault and Event Roll-Over Mode"]
pub struct FEROM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEROM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `BMROM` reader - Burst Mode Roll-Over Mode"]
pub struct BMROM_R(crate::FieldReader<u8, u8>);
impl BMROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMROM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMROM` writer - Burst Mode Roll-Over Mode"]
pub struct BMROM_W<'a> {
    w: &'a mut W,
}
impl<'a> BMROM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ADROM` reader - ADC Roll-Over Mode"]
pub struct ADROM_R(crate::FieldReader<u8, u8>);
impl ADROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADROM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADROM` writer - ADC Roll-Over Mode"]
pub struct ADROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADROM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `OUTROM` reader - Output Roll-Over Mode"]
pub struct OUTROM_R(crate::FieldReader<u8, u8>);
impl OUTROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTROM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTROM` writer - Output Roll-Over Mode"]
pub struct OUTROM_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTROM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ROM` reader - Roll-Over Mode"]
pub struct ROM_R(crate::FieldReader<u8, u8>);
impl ROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - Roll-Over Mode"]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `UDM` reader - Up-Down Mode"]
pub struct UDM_R(crate::FieldReader<bool, bool>);
impl UDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDM` writer - Up-Down Mode"]
pub struct UDM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DCDR` reader - Dual Channel DAC Reset trigger"]
pub struct DCDR_R(crate::FieldReader<bool, bool>);
impl DCDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDR` writer - Dual Channel DAC Reset trigger"]
pub struct DCDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DCDS` reader - Dual Channel DAC Step trigger"]
pub struct DCDS_R(crate::FieldReader<bool, bool>);
impl DCDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDS` writer - Dual Channel DAC Step trigger"]
pub struct DCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDS_W<'a> {
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
#[doc = "Field `DCDE` reader - Dual Channel DAC trigger enable"]
pub struct DCDE_R(crate::FieldReader<bool, bool>);
impl DCDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDE` writer - Dual Channel DAC trigger enable"]
pub struct DCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDE_W<'a> {
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
impl R {
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    pub fn trghlf(&self) -> TRGHLF_R {
        TRGHLF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    pub fn gtcmp3(&self) -> GTCMP3_R {
        GTCMP3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    pub fn gtcmp1(&self) -> GTCMP1_R {
        GTCMP1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    pub fn ferom(&self) -> FEROM_R {
        FEROM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    pub fn bmrom(&self) -> BMROM_R {
        BMROM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    pub fn adrom(&self) -> ADROM_R {
        ADROM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    pub fn outrom(&self) -> OUTROM_R {
        OUTROM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    pub fn udm(&self) -> UDM_R {
        UDM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    pub fn dcde(&self) -> DCDE_R {
        DCDE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    pub fn trghlf(&mut self) -> TRGHLF_W {
        TRGHLF_W { w: self }
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    pub fn gtcmp3(&mut self) -> GTCMP3_W {
        GTCMP3_W { w: self }
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    pub fn gtcmp1(&mut self) -> GTCMP1_W {
        GTCMP1_W { w: self }
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    pub fn ferom(&mut self) -> FEROM_W {
        FEROM_W { w: self }
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    pub fn bmrom(&mut self) -> BMROM_W {
        BMROM_W { w: self }
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    pub fn adrom(&mut self) -> ADROM_W {
        ADROM_W { w: self }
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    pub fn outrom(&mut self) -> OUTROM_W {
        OUTROM_W { w: self }
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    pub fn udm(&mut self) -> UDM_W {
        UDM_W { w: self }
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    pub fn dcdr(&mut self) -> DCDR_W {
        DCDR_W { w: self }
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    pub fn dcds(&mut self) -> DCDS_W {
        DCDS_W { w: self }
    }
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    pub fn dcde(&mut self) -> DCDE_W {
        DCDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecr2](index.html) module"]
pub struct TIMECR2_SPEC;
impl crate::RegisterSpec for TIMECR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timecr2::R](R) reader structure"]
impl crate::Readable for TIMECR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timecr2::W](W) writer structure"]
impl crate::Writable for TIMECR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMECR2 to value 0"]
impl crate::Resettable for TIMECR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
