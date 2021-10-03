#[doc = "Register `OPAMP2_CSR` reader"]
pub struct R(crate::R<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP2_CSR` writer"]
pub struct W(crate::W<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CSR_SPEC>;
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
impl From<crate::W<OPAMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CSR_SPEC>) -> Self {
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
#[doc = "Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)"]
pub struct FORCE_VP_R(crate::FieldReader<bool, bool>);
impl FORCE_VP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_VP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_VP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)"]
pub struct FORCE_VP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `OPAHSM` reader - Operational amplifier high-speed mode"]
pub struct OPAHSM_R(crate::FieldReader<bool, bool>);
impl OPAHSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAHSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAHSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAHSM` writer - Operational amplifier high-speed mode"]
pub struct OPAHSM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAHSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub struct CALSEL_R(crate::FieldReader<u8, u8>);
impl CALSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALSEL_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `USERTRIM` reader - User trimming enable"]
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
#[doc = "Field `USERTRIM` writer - User trimming enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)"]
pub struct TSTREF_R(crate::FieldReader<bool, bool>);
impl TSTREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)"]
pub struct TSTREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W {
        OPAEN_W { w: self }
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W {
        FORCE_VP_W { w: self }
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W {
        OPAHSM_W { w: self }
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W {
        USERTRIM_W { w: self }
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W {
        TSTREF_W { w: self }
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W {
        CALOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](index.html) module"]
pub struct OPAMP2_CSR_SPEC;
impl crate::RegisterSpec for OPAMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp2_csr::R](R) reader structure"]
impl crate::Readable for OPAMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](W) writer structure"]
impl crate::Writable for OPAMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
