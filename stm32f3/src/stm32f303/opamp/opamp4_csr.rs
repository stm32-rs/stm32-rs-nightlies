#[doc = "Register `OPAMP4_CSR` reader"]
pub struct R(crate::R<OPAMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP4_CSR` writer"]
pub struct W(crate::W<OPAMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP4_CSR_SPEC>;
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
impl From<crate::W<OPAMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAMP4EN` reader - OPAMP4 enable"]
pub struct OPAMP4EN_R(crate::FieldReader<bool, bool>);
impl OPAMP4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMP4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMP4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMP4EN` writer - OPAMP4 enable"]
pub struct OPAMP4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMP4EN_W<'a> {
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
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
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
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
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
#[doc = "Field `VP_SEL` reader - OPAMP Non inverting input selection"]
pub struct VP_SEL_R(crate::FieldReader<u8, u8>);
impl VP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VP_SEL` writer - OPAMP Non inverting input selection"]
pub struct VP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `VM_SEL` reader - OPAMP inverting input selection"]
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
#[doc = "Field `VM_SEL` writer - OPAMP inverting input selection"]
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
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub struct TCM_EN_R(crate::FieldReader<bool, bool>);
impl TCM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub struct TCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `VMS_SEL` reader - OPAMP inverting input secondary selection"]
pub struct VMS_SEL_R(crate::FieldReader<bool, bool>);
impl VMS_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMS_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMS_SEL` writer - OPAMP inverting input secondary selection"]
pub struct VMS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMS_SEL_W<'a> {
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
#[doc = "Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection"]
pub struct VPS_SEL_R(crate::FieldReader<u8, u8>);
impl VPS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VPS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection"]
pub struct VPS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VPS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `CALON` reader - Calibration mode enable"]
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
#[doc = "Field `CALON` writer - Calibration mode enable"]
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
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
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
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
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
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub struct USER_TRIM_R(crate::FieldReader<bool, bool>);
impl USER_TRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USER_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USER_TRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub struct USER_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_TRIM_W<'a> {
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
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub struct TRIMOFFSETP_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub struct TRIMOFFSETN_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `TSTREF` reader - TSTREF"]
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
#[doc = "Field `TSTREF` writer - TSTREF"]
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
#[doc = "Field `OUTCAL` reader - OPAMP ouput status flag"]
pub struct OUTCAL_R(crate::FieldReader<bool, bool>);
impl OUTCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` reader - OPAMP lock"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - OPAMP lock"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    #[doc = "Bit 0 - OPAMP4 enable"]
    #[inline(always)]
    pub fn opamp4en(&self) -> OPAMP4EN_R {
        OPAMP4EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPAMP ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP4 enable"]
    #[inline(always)]
    pub fn opamp4en(&mut self) -> OPAMP4EN_W {
        OPAMP4EN_W { w: self }
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W {
        FORCE_VP_W { w: self }
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W {
        VP_SEL_W { w: self }
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TCM_EN_W {
        TCM_EN_W { w: self }
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W {
        VMS_SEL_W { w: self }
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W {
        VPS_SEL_W { w: self }
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&mut self) -> USER_TRIM_W {
        USER_TRIM_W { w: self }
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W {
        TSTREF_W { w: self }
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp4_csr](index.html) module"]
pub struct OPAMP4_CSR_SPEC;
impl crate::RegisterSpec for OPAMP4_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp4_csr::R](R) reader structure"]
impl crate::Readable for OPAMP4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp4_csr::W](W) writer structure"]
impl crate::Writable for OPAMP4_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP4_CSR to value 0"]
impl crate::Resettable for OPAMP4_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
