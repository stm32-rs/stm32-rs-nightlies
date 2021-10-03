#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA3CALOUT` reader - OPAMP3 calibration output"]
pub struct OPA3CALOUT_R(crate::FieldReader<bool, bool>);
impl OPA3CALOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA3CALOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA3CALOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA3CALOUT` writer - OPAMP3 calibration output"]
pub struct OPA3CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CALOUT_W<'a> {
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
#[doc = "Field `OPA2CALOUT` reader - OPAMP2 calibration output"]
pub struct OPA2CALOUT_R(crate::FieldReader<bool, bool>);
impl OPA2CALOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA2CALOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA2CALOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA2CALOUT` writer - OPAMP2 calibration output"]
pub struct OPA2CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CALOUT_W<'a> {
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
#[doc = "Field `OPA1CALOUT` reader - OPAMP1 calibration output"]
pub struct OPA1CALOUT_R(crate::FieldReader<bool, bool>);
impl OPA1CALOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA1CALOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA1CALOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA1CALOUT` writer - OPAMP1 calibration output"]
pub struct OPA1CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CALOUT_W<'a> {
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
#[doc = "Field `AOP_RANGE` reader - Power range selection"]
pub struct AOP_RANGE_R(crate::FieldReader<bool, bool>);
impl AOP_RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AOP_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AOP_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AOP_RANGE` writer - Power range selection"]
pub struct AOP_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOP_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `S7SEL2` reader - Switch 7 for OPAMP2 enable"]
pub struct S7SEL2_R(crate::FieldReader<bool, bool>);
impl S7SEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        S7SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S7SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S7SEL2` writer - Switch 7 for OPAMP2 enable"]
pub struct S7SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S7SEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `ANAWSEL3` reader - Switch SanA enable for OPAMP3"]
pub struct ANAWSEL3_R(crate::FieldReader<bool, bool>);
impl ANAWSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAWSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANAWSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAWSEL3` writer - Switch SanA enable for OPAMP3"]
pub struct ANAWSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ANAWSEL2` reader - Switch SanA enable for OPAMP2"]
pub struct ANAWSEL2_R(crate::FieldReader<bool, bool>);
impl ANAWSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAWSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANAWSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAWSEL2` writer - Switch SanA enable for OPAMP2"]
pub struct ANAWSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ANAWSEL1` reader - Switch SanA enable for OPAMP1"]
pub struct ANAWSEL1_R(crate::FieldReader<bool, bool>);
impl ANAWSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAWSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANAWSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAWSEL1` writer - Switch SanA enable for OPAMP1"]
pub struct ANAWSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `OPA3LPM` reader - OPAMP3 low power mode"]
pub struct OPA3LPM_R(crate::FieldReader<bool, bool>);
impl OPA3LPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA3LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA3LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA3LPM` writer - OPAMP3 low power mode"]
pub struct OPA3LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3LPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `OPA3CAL_H` reader - OPAMP3 offset calibration for N differential pair"]
pub struct OPA3CAL_H_R(crate::FieldReader<bool, bool>);
impl OPA3CAL_H_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA3CAL_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA3CAL_H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA3CAL_H` writer - OPAMP3 offset calibration for N differential pair"]
pub struct OPA3CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CAL_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `OPA3CAL_L` reader - OPAMP3 offset Calibration for P differential pair"]
pub struct OPA3CAL_L_R(crate::FieldReader<bool, bool>);
impl OPA3CAL_L_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA3CAL_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA3CAL_L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA3CAL_L` writer - OPAMP3 offset Calibration for P differential pair"]
pub struct OPA3CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CAL_L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `S6SEL3` reader - Switch 6 for OPAMP3 enable"]
pub struct S6SEL3_R(crate::FieldReader<bool, bool>);
impl S6SEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        S6SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S6SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S6SEL3` writer - Switch 6 for OPAMP3 enable"]
pub struct S6SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL3_W<'a> {
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
#[doc = "Field `S5SEL3` reader - Switch 5 for OPAMP3 enable"]
pub struct S5SEL3_R(crate::FieldReader<bool, bool>);
impl S5SEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        S5SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S5SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S5SEL3` writer - Switch 5 for OPAMP3 enable"]
pub struct S5SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `S4SEL3` reader - Switch 4 for OPAMP3 enable"]
pub struct S4SEL3_R(crate::FieldReader<bool, bool>);
impl S4SEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        S4SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S4SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S4SEL3` writer - Switch 4 for OPAMP3 enable"]
pub struct S4SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL3_W<'a> {
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
#[doc = "Field `S3SEL3` reader - Switch 3 for OPAMP3 Enable"]
pub struct S3SEL3_R(crate::FieldReader<bool, bool>);
impl S3SEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S3SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S3SEL3` writer - Switch 3 for OPAMP3 Enable"]
pub struct S3SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL3_W<'a> {
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
#[doc = "Field `OPA3PD` reader - OPAMP3 power down"]
pub struct OPA3PD_R(crate::FieldReader<bool, bool>);
impl OPA3PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA3PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA3PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA3PD` writer - OPAMP3 power down"]
pub struct OPA3PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3PD_W<'a> {
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
#[doc = "Field `OPA2LPM` reader - OPAMP2 low power mode"]
pub struct OPA2LPM_R(crate::FieldReader<bool, bool>);
impl OPA2LPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA2LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA2LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA2LPM` writer - OPAMP2 low power mode"]
pub struct OPA2LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2LPM_W<'a> {
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
#[doc = "Field `OPA2CAL_H` reader - OPAMP2 offset calibration for N differential pair"]
pub struct OPA2CAL_H_R(crate::FieldReader<bool, bool>);
impl OPA2CAL_H_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA2CAL_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA2CAL_H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA2CAL_H` writer - OPAMP2 offset calibration for N differential pair"]
pub struct OPA2CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CAL_H_W<'a> {
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
#[doc = "Field `OPA2CAL_L` reader - OPAMP2 offset Calibration for P differential pair"]
pub struct OPA2CAL_L_R(crate::FieldReader<bool, bool>);
impl OPA2CAL_L_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA2CAL_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA2CAL_L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA2CAL_L` writer - OPAMP2 offset Calibration for P differential pair"]
pub struct OPA2CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CAL_L_W<'a> {
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
#[doc = "Field `S6SEL2` reader - Switch 6 for OPAMP2 enable"]
pub struct S6SEL2_R(crate::FieldReader<bool, bool>);
impl S6SEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        S6SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S6SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S6SEL2` writer - Switch 6 for OPAMP2 enable"]
pub struct S6SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL2_W<'a> {
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
#[doc = "Field `S5SEL2` reader - Switch 5 for OPAMP2 enable"]
pub struct S5SEL2_R(crate::FieldReader<bool, bool>);
impl S5SEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        S5SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S5SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S5SEL2` writer - Switch 5 for OPAMP2 enable"]
pub struct S5SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL2_W<'a> {
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
#[doc = "Field `S4SEL2` reader - Switch 4 for OPAMP2 enable"]
pub struct S4SEL2_R(crate::FieldReader<bool, bool>);
impl S4SEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        S4SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S4SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S4SEL2` writer - Switch 4 for OPAMP2 enable"]
pub struct S4SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL2_W<'a> {
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
#[doc = "Field `S3SEL2` reader - Switch 3 for OPAMP2 enable"]
pub struct S3SEL2_R(crate::FieldReader<bool, bool>);
impl S3SEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S3SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S3SEL2` writer - Switch 3 for OPAMP2 enable"]
pub struct S3SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OPA2PD` reader - OPAMP2 power down"]
pub struct OPA2PD_R(crate::FieldReader<bool, bool>);
impl OPA2PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA2PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA2PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA2PD` writer - OPAMP2 power down"]
pub struct OPA2PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2PD_W<'a> {
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
#[doc = "Field `OPA1LPM` reader - OPAMP1 low power mode"]
pub struct OPA1LPM_R(crate::FieldReader<bool, bool>);
impl OPA1LPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA1LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA1LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA1LPM` writer - OPAMP1 low power mode"]
pub struct OPA1LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1LPM_W<'a> {
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
#[doc = "Field `OPA1CAL_H` reader - OPAMP1 offset calibration for N differential pair"]
pub struct OPA1CAL_H_R(crate::FieldReader<bool, bool>);
impl OPA1CAL_H_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA1CAL_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA1CAL_H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA1CAL_H` writer - OPAMP1 offset calibration for N differential pair"]
pub struct OPA1CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CAL_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OPA1CAL_L` reader - OPAMP1 offset calibration for P differential pair"]
pub struct OPA1CAL_L_R(crate::FieldReader<bool, bool>);
impl OPA1CAL_L_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA1CAL_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA1CAL_L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA1CAL_L` writer - OPAMP1 offset calibration for P differential pair"]
pub struct OPA1CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CAL_L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `S6SEL1` reader - Switch 6 for OPAMP1 enable"]
pub struct S6SEL1_R(crate::FieldReader<bool, bool>);
impl S6SEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        S6SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S6SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S6SEL1` writer - Switch 6 for OPAMP1 enable"]
pub struct S6SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL1_W<'a> {
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
#[doc = "Field `S5SEL1` reader - Switch 5 for OPAMP1 enable"]
pub struct S5SEL1_R(crate::FieldReader<bool, bool>);
impl S5SEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        S5SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S5SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S5SEL1` writer - Switch 5 for OPAMP1 enable"]
pub struct S5SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `S4SEL1` reader - Switch 4 for OPAMP1 enable"]
pub struct S4SEL1_R(crate::FieldReader<bool, bool>);
impl S4SEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        S4SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S4SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S4SEL1` writer - Switch 4 for OPAMP1 enable"]
pub struct S4SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL1_W<'a> {
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
#[doc = "Field `S3SEL1` reader - Switch 3 for OPAMP1 enable"]
pub struct S3SEL1_R(crate::FieldReader<bool, bool>);
impl S3SEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S3SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S3SEL1` writer - Switch 3 for OPAMP1 enable"]
pub struct S3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL1_W<'a> {
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
#[doc = "Field `OPA1PD` reader - OPAMP1 power down"]
pub struct OPA1PD_R(crate::FieldReader<bool, bool>);
impl OPA1PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPA1PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPA1PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPA1PD` writer - OPAMP1 power down"]
pub struct OPA1PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1PD_W<'a> {
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
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&self) -> OPA3CALOUT_R {
        OPA3CALOUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&self) -> OPA2CALOUT_R {
        OPA2CALOUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&self) -> OPA1CALOUT_R {
        OPA1CALOUT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&self) -> AOP_RANGE_R {
        AOP_RANGE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&self) -> S7SEL2_R {
        S7SEL2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&self) -> ANAWSEL3_R {
        ANAWSEL3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&self) -> ANAWSEL2_R {
        ANAWSEL2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&self) -> ANAWSEL1_R {
        ANAWSEL1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&self) -> OPA3LPM_R {
        OPA3LPM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&self) -> OPA3CAL_H_R {
        OPA3CAL_H_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&self) -> OPA3CAL_L_R {
        OPA3CAL_L_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&self) -> S6SEL3_R {
        S6SEL3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&self) -> S5SEL3_R {
        S5SEL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&self) -> S4SEL3_R {
        S4SEL3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&self) -> S3SEL3_R {
        S3SEL3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&self) -> OPA3PD_R {
        OPA3PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&self) -> OPA2LPM_R {
        OPA2LPM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&self) -> OPA2CAL_H_R {
        OPA2CAL_H_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&self) -> OPA2CAL_L_R {
        OPA2CAL_L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&self) -> S6SEL2_R {
        S6SEL2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&self) -> S5SEL2_R {
        S5SEL2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&self) -> S4SEL2_R {
        S4SEL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&self) -> S3SEL2_R {
        S3SEL2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&self) -> OPA2PD_R {
        OPA2PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&self) -> OPA1LPM_R {
        OPA1LPM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&self) -> OPA1CAL_H_R {
        OPA1CAL_H_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&self) -> OPA1CAL_L_R {
        OPA1CAL_L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&self) -> S6SEL1_R {
        S6SEL1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&self) -> S5SEL1_R {
        S5SEL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&self) -> S4SEL1_R {
        S4SEL1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&self) -> S3SEL1_R {
        S3SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&self) -> OPA1PD_R {
        OPA1PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&mut self) -> OPA3CALOUT_W {
        OPA3CALOUT_W { w: self }
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&mut self) -> OPA2CALOUT_W {
        OPA2CALOUT_W { w: self }
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&mut self) -> OPA1CALOUT_W {
        OPA1CALOUT_W { w: self }
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&mut self) -> AOP_RANGE_W {
        AOP_RANGE_W { w: self }
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&mut self) -> S7SEL2_W {
        S7SEL2_W { w: self }
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&mut self) -> ANAWSEL3_W {
        ANAWSEL3_W { w: self }
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&mut self) -> ANAWSEL2_W {
        ANAWSEL2_W { w: self }
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&mut self) -> ANAWSEL1_W {
        ANAWSEL1_W { w: self }
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&mut self) -> OPA3LPM_W {
        OPA3LPM_W { w: self }
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&mut self) -> OPA3CAL_H_W {
        OPA3CAL_H_W { w: self }
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&mut self) -> OPA3CAL_L_W {
        OPA3CAL_L_W { w: self }
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&mut self) -> S6SEL3_W {
        S6SEL3_W { w: self }
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&mut self) -> S5SEL3_W {
        S5SEL3_W { w: self }
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&mut self) -> S4SEL3_W {
        S4SEL3_W { w: self }
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&mut self) -> S3SEL3_W {
        S3SEL3_W { w: self }
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&mut self) -> OPA3PD_W {
        OPA3PD_W { w: self }
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&mut self) -> OPA2LPM_W {
        OPA2LPM_W { w: self }
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&mut self) -> OPA2CAL_H_W {
        OPA2CAL_H_W { w: self }
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&mut self) -> OPA2CAL_L_W {
        OPA2CAL_L_W { w: self }
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&mut self) -> S6SEL2_W {
        S6SEL2_W { w: self }
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&mut self) -> S5SEL2_W {
        S5SEL2_W { w: self }
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&mut self) -> S4SEL2_W {
        S4SEL2_W { w: self }
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&mut self) -> S3SEL2_W {
        S3SEL2_W { w: self }
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&mut self) -> OPA2PD_W {
        OPA2PD_W { w: self }
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&mut self) -> OPA1LPM_W {
        OPA1LPM_W { w: self }
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&mut self) -> OPA1CAL_H_W {
        OPA1CAL_H_W { w: self }
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&mut self) -> OPA1CAL_L_W {
        OPA1CAL_L_W { w: self }
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&mut self) -> S6SEL1_W {
        S6SEL1_W { w: self }
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&mut self) -> S5SEL1_W {
        S5SEL1_W { w: self }
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&mut self) -> S4SEL1_W {
        S4SEL1_W { w: self }
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&mut self) -> S3SEL1_W {
        S3SEL1_W { w: self }
    }
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&mut self) -> OPA1PD_W {
        OPA1PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x0001_0101"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0101
    }
}
