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
#[doc = "REF_CTRL lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_LOCK_AW {
    #[doc = "0: SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    READWRITE = 0,
    #[doc = "1: SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    READONLY = 1,
}
impl From<REF_LOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: REF_LOCK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REF_LOCK` writer - REF_CTRL lock bit"]
pub struct REF_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_LOCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(REF_LOCK_AW::READWRITE)
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(REF_LOCK_AW::READONLY)
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
#[doc = "Field `VREFINT_COMP_RDYF` reader - VREFINT for comparator ready flag"]
pub struct VREFINT_COMP_RDYF_R(crate::FieldReader<bool, bool>);
impl VREFINT_COMP_RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFINT_COMP_RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFINT_COMP_RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFINT_ADC_RDYF` reader - VREFINT for ADC ready flag"]
pub struct VREFINT_ADC_RDYF_R(crate::FieldReader<bool, bool>);
impl VREFINT_ADC_RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFINT_ADC_RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFINT_ADC_RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSOR_ADC_RDYF` reader - Sensor for ADC ready flag"]
pub struct SENSOR_ADC_RDYF_R(crate::FieldReader<bool, bool>);
impl SENSOR_ADC_RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SENSOR_ADC_RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSOR_ADC_RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_RC48MHz_RDYF` reader - VREFINT for 48 MHz RC oscillator ready flag"]
pub struct REF_RC48MHZ_RDYF_R(crate::FieldReader<bool, bool>);
impl REF_RC48MHZ_RDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REF_RC48MHZ_RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_RC48MHZ_RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENREF_RC48MHz` reader - VREFINT reference for 48 MHz RC oscillator enable bit"]
pub struct ENREF_RC48MHZ_R(crate::FieldReader<bool, bool>);
impl ENREF_RC48MHZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENREF_RC48MHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENREF_RC48MHZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENREF_RC48MHz` writer - VREFINT reference for 48 MHz RC oscillator enable bit"]
pub struct ENREF_RC48MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENREF_RC48MHZ_W<'a> {
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
#[doc = "Field `ENBUF_VREFINT_COMP` reader - VREFINT reference for comparator 2 enable bit"]
pub struct ENBUF_VREFINT_COMP_R(crate::FieldReader<bool, bool>);
impl ENBUF_VREFINT_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBUF_VREFINT_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENBUF_VREFINT_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP` writer - VREFINT reference for comparator 2 enable bit"]
pub struct ENBUF_VREFINT_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_VREFINT_COMP_W<'a> {
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
#[doc = "Field `ENBUF_BGAP_ADC` reader - VREFINT reference for ADC enable bit"]
pub struct ENBUF_BGAP_ADC_R(crate::FieldReader<bool, bool>);
impl ENBUF_BGAP_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBUF_BGAP_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENBUF_BGAP_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBUF_BGAP_ADC` writer - VREFINT reference for ADC enable bit"]
pub struct ENBUF_BGAP_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_BGAP_ADC_W<'a> {
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
#[doc = "Field `EN_BGAP` reader - Vref Enable bit"]
pub struct EN_BGAP_R(crate::FieldReader<bool, bool>);
impl EN_BGAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_BGAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_BGAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_BGAP` writer - Vref Enable bit"]
pub struct EN_BGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_BGAP_W<'a> {
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
    #[doc = "Bit 29 - VREFINT for comparator ready flag"]
    #[inline(always)]
    pub fn vrefint_comp_rdyf(&self) -> VREFINT_COMP_RDYF_R {
        VREFINT_COMP_RDYF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFINT for ADC ready flag"]
    #[inline(always)]
    pub fn vrefint_adc_rdyf(&self) -> VREFINT_ADC_RDYF_R {
        VREFINT_ADC_RDYF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Sensor for ADC ready flag"]
    #[inline(always)]
    pub fn sensor_adc_rdyf(&self) -> SENSOR_ADC_RDYF_R {
        SENSOR_ADC_RDYF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - VREFINT for 48 MHz RC oscillator ready flag"]
    #[inline(always)]
    pub fn ref_rc48mhz_rdyf(&self) -> REF_RC48MHZ_RDYF_R {
        REF_RC48MHZ_RDYF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit"]
    #[inline(always)]
    pub fn enref_rc48mhz(&self) -> ENREF_RC48MHZ_R {
        ENREF_RC48MHZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VREFINT reference for comparator 2 enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&self) -> ENBUF_VREFINT_COMP_R {
        ENBUF_VREFINT_COMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_bgap_adc(&self) -> ENBUF_BGAP_ADC_R {
        ENBUF_BGAP_ADC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Vref Enable bit"]
    #[inline(always)]
    pub fn en_bgap(&self) -> EN_BGAP_R {
        EN_BGAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - REF_CTRL lock bit"]
    #[inline(always)]
    pub fn ref_lock(&mut self) -> REF_LOCK_W {
        REF_LOCK_W { w: self }
    }
    #[doc = "Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit"]
    #[inline(always)]
    pub fn enref_rc48mhz(&mut self) -> ENREF_RC48MHZ_W {
        ENREF_RC48MHZ_W { w: self }
    }
    #[doc = "Bit 12 - VREFINT reference for comparator 2 enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&mut self) -> ENBUF_VREFINT_COMP_W {
        ENBUF_VREFINT_COMP_W { w: self }
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W {
        ENBUF_SENSOR_ADC_W { w: self }
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_bgap_adc(&mut self) -> ENBUF_BGAP_ADC_W {
        ENBUF_BGAP_ADC_W { w: self }
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W {
        SEL_VREF_OUT_W { w: self }
    }
    #[doc = "Bit 0 - Vref Enable bit"]
    #[inline(always)]
    pub fn en_bgap(&mut self) -> EN_BGAP_W {
        EN_BGAP_W { w: self }
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
