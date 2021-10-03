#[doc = "Register `TIM8_CR2` reader"]
pub struct R(crate::R<TIM8_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM8_CR2` writer"]
pub struct W(crate::W<TIM8_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_CR2_SPEC>;
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
impl From<crate::W<TIM8_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCPC` reader - CCPC"]
pub struct CCPC_R(crate::FieldReader<bool, bool>);
impl CCPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCPC` writer - CCPC"]
pub struct CCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPC_W<'a> {
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
#[doc = "Field `CCUS` reader - CCUS"]
pub struct CCUS_R(crate::FieldReader<bool, bool>);
impl CCUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUS` writer - CCUS"]
pub struct CCUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUS_W<'a> {
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
#[doc = "Field `CCDS` reader - CCDS"]
pub struct CCDS_R(crate::FieldReader<bool, bool>);
impl CCDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCDS` writer - CCDS"]
pub struct CCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDS_W<'a> {
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
#[doc = "Field `MMS` reader - MMS"]
pub struct MMS_R(crate::FieldReader<u8, u8>);
impl MMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMS` writer - MMS"]
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `TI1S` reader - TI1S"]
pub struct TI1S_R(crate::FieldReader<bool, bool>);
impl TI1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI1S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1S` writer - TI1S"]
pub struct TI1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1S_W<'a> {
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
#[doc = "Field `OIS1` reader - OIS1"]
pub struct OIS1_R(crate::FieldReader<bool, bool>);
impl OIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1` writer - OIS1"]
pub struct OIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1_W<'a> {
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
#[doc = "Field `OIS1N` reader - OIS1N"]
pub struct OIS1N_R(crate::FieldReader<bool, bool>);
impl OIS1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS1N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS1N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS1N` writer - OIS1N"]
pub struct OIS1N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1N_W<'a> {
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
#[doc = "Field `OIS2` reader - OIS2"]
pub struct OIS2_R(crate::FieldReader<bool, bool>);
impl OIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2` writer - OIS2"]
pub struct OIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2_W<'a> {
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
#[doc = "Field `OIS2N` reader - OIS2N"]
pub struct OIS2N_R(crate::FieldReader<bool, bool>);
impl OIS2N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS2N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS2N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS2N` writer - OIS2N"]
pub struct OIS2N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2N_W<'a> {
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
#[doc = "Field `OIS3` reader - OIS3"]
pub struct OIS3_R(crate::FieldReader<bool, bool>);
impl OIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3` writer - OIS3"]
pub struct OIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3_W<'a> {
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
#[doc = "Field `OIS3N` reader - OIS3N"]
pub struct OIS3N_R(crate::FieldReader<bool, bool>);
impl OIS3N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS3N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS3N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS3N` writer - OIS3N"]
pub struct OIS3N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3N_W<'a> {
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
#[doc = "Field `OIS4` reader - OIS4"]
pub struct OIS4_R(crate::FieldReader<bool, bool>);
impl OIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS4` writer - OIS4"]
pub struct OIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS4_W<'a> {
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
#[doc = "Field `OIS5` reader - OIS5"]
pub struct OIS5_R(crate::FieldReader<bool, bool>);
impl OIS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS5` writer - OIS5"]
pub struct OIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS5_W<'a> {
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
#[doc = "Field `OIS6` reader - OIS6"]
pub struct OIS6_R(crate::FieldReader<bool, bool>);
impl OIS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIS6` writer - OIS6"]
pub struct OIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS6_W<'a> {
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
#[doc = "Field `MMS2` reader - MMS2"]
pub struct MMS2_R(crate::FieldReader<u8, u8>);
impl MMS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MMS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMS2` writer - MMS2"]
pub struct MMS2_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OIS5"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OIS6"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - MMS2"]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W { w: self }
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W { w: self }
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W { w: self }
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W {
        TI1S_W { w: self }
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W {
        OIS1_W { w: self }
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W {
        OIS1N_W { w: self }
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W {
        OIS2_W { w: self }
    }
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W {
        OIS2N_W { w: self }
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W {
        OIS3_W { w: self }
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W {
        OIS3N_W { w: self }
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W {
        OIS4_W { w: self }
    }
    #[doc = "Bit 16 - OIS5"]
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W {
        OIS5_W { w: self }
    }
    #[doc = "Bit 18 - OIS6"]
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W {
        OIS6_W { w: self }
    }
    #[doc = "Bits 20:23 - MMS2"]
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W {
        MMS2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM8 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_cr2](index.html) module"]
pub struct TIM8_CR2_SPEC;
impl crate::RegisterSpec for TIM8_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim8_cr2::R](R) reader structure"]
impl crate::Readable for TIM8_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim8_cr2::W](W) writer structure"]
impl crate::Writable for TIM8_CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM8_CR2 to value 0"]
impl crate::Resettable for TIM8_CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}