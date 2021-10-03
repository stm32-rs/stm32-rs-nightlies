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
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub struct COMP1EN_R(crate::FieldReader<bool, bool>);
impl COMP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
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
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub struct COMP1MODE_R(crate::FieldReader<u8, u8>);
impl COMP1MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub struct COMP1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub struct COMP1INSEL_R(crate::FieldReader<u8, u8>);
impl COMP1INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub struct COMP1INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP1OUTSEL` reader - Comparator 1 output selection"]
pub struct COMP1OUTSEL_R(crate::FieldReader<u8, u8>);
impl COMP1OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1OUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1OUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1OUTSEL` writer - Comparator 1 output selection"]
pub struct COMP1OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub struct COMP1POL_R(crate::FieldReader<bool, bool>);
impl COMP1POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub struct COMP1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POL_W<'a> {
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
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub struct COMP1HYST_R(crate::FieldReader<u8, u8>);
impl COMP1HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub struct COMP1HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub struct COMP1OUT_R(crate::FieldReader<bool, bool>);
impl COMP1OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub struct COMP1LOCK_R(crate::FieldReader<bool, bool>);
impl COMP1LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub struct COMP1LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LOCK_W<'a> {
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
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub struct COMP2EN_R(crate::FieldReader<bool, bool>);
impl COMP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
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
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub struct COMP2MODE_R(crate::FieldReader<u8, u8>);
impl COMP2MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub struct COMP2MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub struct COMP2INSEL_R(crate::FieldReader<u8, u8>);
impl COMP2INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub struct COMP2INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `WNDWEN` reader - Window mode enable"]
pub struct WNDWEN_R(crate::FieldReader<bool, bool>);
impl WNDWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WNDWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WNDWEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WNDWEN` writer - Window mode enable"]
pub struct WNDWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WNDWEN_W<'a> {
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
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub struct COMP2OUTSEL_R(crate::FieldReader<u8, u8>);
impl COMP2OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2OUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2OUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub struct COMP2OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub struct COMP2POL_R(crate::FieldReader<bool, bool>);
impl COMP2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub struct COMP2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POL_W<'a> {
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
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub struct COMP2HYST_R(crate::FieldReader<u8, u8>);
impl COMP2HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub struct COMP2HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub struct COMP2OUT_R(crate::FieldReader<bool, bool>);
impl COMP2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub struct COMP2LOCK_R(crate::FieldReader<bool, bool>);
impl COMP2LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub struct COMP2LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LOCK_W<'a> {
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
#[doc = "Field `COMP1_INP_DAC` reader - Comparator 1 non inverting input connection to DAC output"]
pub struct COMP1_INP_DAC_R(crate::FieldReader<bool, bool>);
impl COMP1_INP_DAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_INP_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_INP_DAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_INP_DAC` writer - Comparator 1 non inverting input connection to DAC output"]
pub struct COMP1_INP_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_INP_DAC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    pub fn comp1_inp_dac(&self) -> COMP1_INP_DAC_R {
        COMP1_INP_DAC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&mut self) -> COMP1MODE_W {
        COMP1MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W {
        COMP1INSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W {
        COMP1OUTSEL_W { w: self }
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&mut self) -> COMP1POL_W {
        COMP1POL_W { w: self }
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W {
        COMP1HYST_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W {
        COMP1LOCK_W { w: self }
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&mut self) -> COMP2MODE_W {
        COMP2MODE_W { w: self }
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W {
        COMP2INSEL_W { w: self }
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&mut self) -> WNDWEN_W {
        WNDWEN_W { w: self }
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W {
        COMP2OUTSEL_W { w: self }
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W {
        COMP2POL_W { w: self }
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W {
        COMP2HYST_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W {
        COMP2LOCK_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    pub fn comp1_inp_dac(&mut self) -> COMP1_INP_DAC_W {
        COMP1_INP_DAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
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
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
