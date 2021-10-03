#[doc = "Register `ADC_CR` reader"]
pub struct R(crate::R<ADC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CR` writer"]
pub struct W(crate::W<ADC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CR_SPEC>;
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
impl From<crate::W<ADC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEN` reader - ADEN"]
pub struct ADEN_R(crate::FieldReader<bool, bool>);
impl ADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEN` writer - ADEN"]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
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
#[doc = "Field `ADDIS` reader - ADDIS"]
pub struct ADDIS_R(crate::FieldReader<bool, bool>);
impl ADDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDIS` writer - ADDIS"]
pub struct ADDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIS_W<'a> {
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
#[doc = "Field `ADSTART` reader - ADSTART"]
pub struct ADSTART_R(crate::FieldReader<bool, bool>);
impl ADSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTART` writer - ADSTART"]
pub struct ADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTART_W<'a> {
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
#[doc = "Field `JADSTART` reader - JADSTART"]
pub struct JADSTART_R(crate::FieldReader<bool, bool>);
impl JADSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        JADSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JADSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JADSTART` writer - JADSTART"]
pub struct JADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTART_W<'a> {
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
#[doc = "Field `ADSTP` reader - ADSTP"]
pub struct ADSTP_R(crate::FieldReader<bool, bool>);
impl ADSTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTP` writer - ADSTP"]
pub struct ADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTP_W<'a> {
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
#[doc = "Field `JADSTP` reader - JADSTP"]
pub struct JADSTP_R(crate::FieldReader<bool, bool>);
impl JADSTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        JADSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JADSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JADSTP` writer - JADSTP"]
pub struct JADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTP_W<'a> {
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
#[doc = "Field `BOOST` reader - BOOST"]
pub struct BOOST_R(crate::FieldReader<bool, bool>);
impl BOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOST` writer - BOOST"]
pub struct BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_W<'a> {
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
#[doc = "Field `ADCALLIN` reader - ADCALLIN"]
pub struct ADCALLIN_R(crate::FieldReader<bool, bool>);
impl ADCALLIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCALLIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCALLIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCALLIN` writer - ADCALLIN"]
pub struct ADCALLIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCALLIN_W<'a> {
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
#[doc = "Field `LINCALRDYW1` reader - LINCALRDYW1"]
pub struct LINCALRDYW1_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW1` writer - LINCALRDYW1"]
pub struct LINCALRDYW1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW1_W<'a> {
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
#[doc = "Field `LINCALRDYW2` reader - LINCALRDYW2"]
pub struct LINCALRDYW2_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW2` writer - LINCALRDYW2"]
pub struct LINCALRDYW2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW2_W<'a> {
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
#[doc = "Field `LINCALRDYW3` reader - LINCALRDYW3"]
pub struct LINCALRDYW3_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW3` writer - LINCALRDYW3"]
pub struct LINCALRDYW3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW3_W<'a> {
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
#[doc = "Field `LINCALRDYW4` reader - LINCALRDYW4"]
pub struct LINCALRDYW4_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW4` writer - LINCALRDYW4"]
pub struct LINCALRDYW4_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW4_W<'a> {
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
#[doc = "Field `LINCALRDYW5` reader - LINCALRDYW5"]
pub struct LINCALRDYW5_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW5` writer - LINCALRDYW5"]
pub struct LINCALRDYW5_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW5_W<'a> {
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
#[doc = "Field `LINCALRDYW6` reader - LINCALRDYW6"]
pub struct LINCALRDYW6_R(crate::FieldReader<bool, bool>);
impl LINCALRDYW6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALRDYW6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW6` writer - LINCALRDYW6"]
pub struct LINCALRDYW6_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW6_W<'a> {
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
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub struct ADVREGEN_R(crate::FieldReader<bool, bool>);
impl ADVREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADVREGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADVREGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub struct ADVREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVREGEN_W<'a> {
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
#[doc = "Field `DEEPPWD` reader - DEEPPWD"]
pub struct DEEPPWD_R(crate::FieldReader<bool, bool>);
impl DEEPPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPPWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEPPWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPPWD` writer - DEEPPWD"]
pub struct DEEPPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPPWD_W<'a> {
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
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub struct ADCALDIF_R(crate::FieldReader<bool, bool>);
impl ADCALDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCALDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCALDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub struct ADCALDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCALDIF_W<'a> {
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
#[doc = "Field `ADCAL` reader - ADCAL"]
pub struct ADCAL_R(crate::FieldReader<bool, bool>);
impl ADCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCAL` writer - ADCAL"]
pub struct ADCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCAL_W<'a> {
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
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W {
        ADDIS_W { w: self }
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W {
        ADSTART_W { w: self }
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W {
        JADSTART_W { w: self }
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W {
        ADSTP_W { w: self }
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W {
        JADSTP_W { w: self }
    }
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W {
        BOOST_W { w: self }
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    pub fn adcallin(&mut self) -> ADCALLIN_W {
        ADCALLIN_W { w: self }
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W {
        LINCALRDYW1_W { w: self }
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W {
        LINCALRDYW2_W { w: self }
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W {
        LINCALRDYW3_W { w: self }
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W {
        LINCALRDYW4_W { w: self }
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W {
        LINCALRDYW5_W { w: self }
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W {
        LINCALRDYW6_W { w: self }
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W {
        ADVREGEN_W { w: self }
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W {
        DEEPPWD_W { w: self }
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W {
        ADCALDIF_W { w: self }
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W {
        ADCAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cr](index.html) module"]
pub struct ADC_CR_SPEC;
impl crate::RegisterSpec for ADC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cr::R](R) reader structure"]
impl crate::Readable for ADC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cr::W](W) writer structure"]
impl crate::Writable for ADC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CR to value 0x2000_0000"]
impl crate::Resettable for ADC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
