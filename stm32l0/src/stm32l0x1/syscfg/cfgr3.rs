#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VREFINT ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFINT_RDYF_A {
    #[doc = "0: VREFINT OFF"]
    NOTREADY = 0,
    #[doc = "1: VREFINT ready"]
    READY = 1,
}
impl From<VREFINT_RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINT_RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFINT_RDYF` reader - VREFINT ready flag"]
pub struct VREFINT_RDYF_R(crate::FieldReader<bool, VREFINT_RDYF_A>);
impl VREFINT_RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFINT_RDYF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFINT_RDYF_A {
        match self.bits {
            false => VREFINT_RDYF_A::NOTREADY,
            true => VREFINT_RDYF_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == VREFINT_RDYF_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == VREFINT_RDYF_A::READY
    }
}
impl core::ops::Deref for VREFINT_RDYF_R {
    type Target = crate::FieldReader<bool, VREFINT_RDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sensor reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBUF_SENSOR_ADC_A {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    DISABLED = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    ENABLED = 1,
}
impl From<ENBUF_SENSOR_ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_SENSOR_ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` reader - Sensor reference for ADC enable bit"]
pub struct ENBUF_SENSOR_ADC_R(crate::FieldReader<bool, ENBUF_SENSOR_ADC_A>);
impl ENBUF_SENSOR_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBUF_SENSOR_ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_SENSOR_ADC_A {
        match self.bits {
            false => ENBUF_SENSOR_ADC_A::DISABLED,
            true => ENBUF_SENSOR_ADC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENBUF_SENSOR_ADC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENBUF_SENSOR_ADC_A::ENABLED
    }
}
impl core::ops::Deref for ENBUF_SENSOR_ADC_R {
    type Target = crate::FieldReader<bool, ENBUF_SENSOR_ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` writer - Sensor reference for ADC enable bit"]
pub struct ENBUF_SENSOR_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_SENSOR_ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBUF_SENSOR_ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_SENSOR_ADC_A::DISABLED)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_SENSOR_ADC_A::ENABLED)
    }
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
#[doc = "BGAP_ADC connection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_VREF_OUT_A {
    #[doc = "0: no pad connected"]
    NOCONNECTION = 0,
    #[doc = "1: PB0 connected"]
    PB0 = 1,
    #[doc = "2: PB1 connected"]
    PB1 = 2,
    #[doc = "3: PB0 and PB1 connected"]
    BOTH = 3,
}
impl From<SEL_VREF_OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_VREF_OUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL_VREF_OUT` reader - BGAP_ADC connection bit"]
pub struct SEL_VREF_OUT_R(crate::FieldReader<u8, SEL_VREF_OUT_A>);
impl SEL_VREF_OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_VREF_OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_VREF_OUT_A {
        match self.bits {
            0 => SEL_VREF_OUT_A::NOCONNECTION,
            1 => SEL_VREF_OUT_A::PB0,
            2 => SEL_VREF_OUT_A::PB1,
            3 => SEL_VREF_OUT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCONNECTION`"]
    #[inline(always)]
    pub fn is_no_connection(&self) -> bool {
        **self == SEL_VREF_OUT_A::NOCONNECTION
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        **self == SEL_VREF_OUT_A::PB0
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        **self == SEL_VREF_OUT_A::PB1
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SEL_VREF_OUT_A::BOTH
    }
}
impl core::ops::Deref for SEL_VREF_OUT_R {
    type Target = crate::FieldReader<u8, SEL_VREF_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_VREF_OUT` writer - BGAP_ADC connection bit"]
pub struct SEL_VREF_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_VREF_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_VREF_OUT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "no pad connected"]
    #[inline(always)]
    pub fn no_connection(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::NOCONNECTION)
    }
    #[doc = "PB0 connected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::PB0)
    }
    #[doc = "PB1 connected"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::PB1)
    }
    #[doc = "PB0 and PB1 connected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "SYSCFG_CFGR3 lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_LOCK_A {
    #[doc = "0: SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    READWRITE = 0,
    #[doc = "1: SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    READONLY = 1,
}
impl From<REF_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: REF_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REF_LOCK` reader - SYSCFG_CFGR3 lock bit"]
pub struct REF_LOCK_R(crate::FieldReader<bool, REF_LOCK_A>);
impl REF_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REF_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_LOCK_A {
        match self.bits {
            false => REF_LOCK_A::READWRITE,
            true => REF_LOCK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        **self == REF_LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        **self == REF_LOCK_A::READONLY
    }
}
impl core::ops::Deref for REF_LOCK_R {
    type Target = crate::FieldReader<bool, REF_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_LOCK` writer - SYSCFG_CFGR3 lock bit"]
pub struct REF_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(REF_LOCK_A::READWRITE)
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(REF_LOCK_A::READONLY)
    }
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
#[doc = "VREFINT reference for COMP2 scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBUF_VREFINT_COMP2_A {
    #[doc = "0: Disables the buffer used to generate VREFINT references for COMP2"]
    DISABLED = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT references for COMP2"]
    ENABLED = 1,
}
impl From<ENBUF_VREFINT_COMP2_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_COMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` reader - VREFINT reference for COMP2 scaler enable bit"]
pub struct ENBUF_VREFINT_COMP2_R(crate::FieldReader<bool, ENBUF_VREFINT_COMP2_A>);
impl ENBUF_VREFINT_COMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBUF_VREFINT_COMP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_VREFINT_COMP2_A {
        match self.bits {
            false => ENBUF_VREFINT_COMP2_A::DISABLED,
            true => ENBUF_VREFINT_COMP2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENBUF_VREFINT_COMP2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENBUF_VREFINT_COMP2_A::ENABLED
    }
}
impl core::ops::Deref for ENBUF_VREFINT_COMP2_R {
    type Target = crate::FieldReader<bool, ENBUF_VREFINT_COMP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` writer - VREFINT reference for COMP2 scaler enable bit"]
pub struct ENBUF_VREFINT_COMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_VREFINT_COMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBUF_VREFINT_COMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_COMP2_A::DISABLED)
    }
    #[doc = "Enables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_COMP2_A::ENABLED)
    }
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
#[doc = "VREFINT reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBUF_VREFINT_ADC_A {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the ADC"]
    DISABLED = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the ADC"]
    ENABLED = 1,
}
impl From<ENBUF_VREFINT_ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` reader - VREFINT reference for ADC enable bit"]
pub struct ENBUF_VREFINT_ADC_R(crate::FieldReader<bool, ENBUF_VREFINT_ADC_A>);
impl ENBUF_VREFINT_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBUF_VREFINT_ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_VREFINT_ADC_A {
        match self.bits {
            false => ENBUF_VREFINT_ADC_A::DISABLED,
            true => ENBUF_VREFINT_ADC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENBUF_VREFINT_ADC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENBUF_VREFINT_ADC_A::ENABLED
    }
}
impl core::ops::Deref for ENBUF_VREFINT_ADC_R {
    type Target = crate::FieldReader<bool, ENBUF_VREFINT_ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` writer - VREFINT reference for ADC enable bit"]
pub struct ENBUF_VREFINT_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_VREFINT_ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBUF_VREFINT_ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_ADC_A::DISABLED)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_ADC_A::ENABLED)
    }
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
#[doc = "VREFINT enable and scaler control for COMP2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_VREFINT_A {
    #[doc = "0: VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    DISABLED = 0,
    #[doc = "1: VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    ENABLED = 1,
}
impl From<EN_VREFINT_A> for bool {
    #[inline(always)]
    fn from(variant: EN_VREFINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_VREFINT` reader - VREFINT enable and scaler control for COMP2 enable bit"]
pub struct EN_VREFINT_R(crate::FieldReader<bool, EN_VREFINT_A>);
impl EN_VREFINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_VREFINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_VREFINT_A {
        match self.bits {
            false => EN_VREFINT_A::DISABLED,
            true => EN_VREFINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_VREFINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_VREFINT_A::ENABLED
    }
}
impl core::ops::Deref for EN_VREFINT_R {
    type Target = crate::FieldReader<bool, EN_VREFINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_VREFINT` writer - VREFINT enable and scaler control for COMP2 enable bit"]
pub struct EN_VREFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_VREFINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_VREFINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_VREFINT_A::DISABLED)
    }
    #[doc = "VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_VREFINT_A::ENABLED)
    }
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
    #[doc = "Bit 30 - VREFINT ready flag"]
    #[inline(always)]
    pub fn vrefint_rdyf(&self) -> VREFINT_RDYF_R {
        VREFINT_RDYF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    pub fn ref_lock(&self) -> REF_LOCK_R {
        REF_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp2(&self) -> ENBUF_VREFINT_COMP2_R {
        ENBUF_VREFINT_COMP2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_adc(&self) -> ENBUF_VREFINT_ADC_R {
        ENBUF_VREFINT_ADC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    pub fn en_vrefint(&self) -> EN_VREFINT_R {
        EN_VREFINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W {
        ENBUF_SENSOR_ADC_W { w: self }
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W {
        SEL_VREF_OUT_W { w: self }
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    pub fn ref_lock(&mut self) -> REF_LOCK_W {
        REF_LOCK_W { w: self }
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp2(&mut self) -> ENBUF_VREFINT_COMP2_W {
        ENBUF_VREFINT_COMP2_W { w: self }
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_adc(&mut self) -> ENBUF_VREFINT_ADC_W {
        ENBUF_VREFINT_ADC_W { w: self }
    }
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    pub fn en_vrefint(&mut self) -> EN_VREFINT_W {
        EN_VREFINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
