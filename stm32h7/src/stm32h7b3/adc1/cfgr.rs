#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC group injected contexts queue disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQDIS_A {
    #[doc = "0: Injected Queue enabled"]
    ENABLED = 0,
    #[doc = "1: Injected Queue disabled"]
    DISABLED = 1,
}
impl From<JQDIS_A> for bool {
    #[inline(always)]
    fn from(variant: JQDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQDIS` reader - ADC group injected contexts queue disable"]
pub struct JQDIS_R(crate::FieldReader<bool, JQDIS_A>);
impl JQDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        JQDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQDIS_A {
        match self.bits {
            false => JQDIS_A::ENABLED,
            true => JQDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JQDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JQDIS_A::DISABLED
    }
}
impl core::ops::Deref for JQDIS_R {
    type Target = crate::FieldReader<bool, JQDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JQDIS` writer - ADC group injected contexts queue disable"]
pub struct JQDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> JQDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JQDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Injected Queue enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JQDIS_A::ENABLED)
    }
    #[doc = "Injected Queue disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JQDIS_A::DISABLED)
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
#[doc = "Field `AWD1CH` reader - ADC analog watchdog 1 monitored channel selection"]
pub struct AWD1CH_R(crate::FieldReader<u8, u8>);
impl AWD1CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWD1CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1CH` writer - ADC analog watchdog 1 monitored channel selection"]
pub struct AWD1CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "ADC group injected automatic trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    ENABLED = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAUTO` reader - ADC group injected automatic trigger mode"]
pub struct JAUTO_R(crate::FieldReader<bool, JAUTO_A>);
impl JAUTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAUTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::DISABLED,
            true => JAUTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JAUTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JAUTO_A::ENABLED
    }
}
impl core::ops::Deref for JAUTO_R {
    type Target = crate::FieldReader<bool, JAUTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAUTO` writer - ADC group injected automatic trigger mode"]
pub struct JAUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> JAUTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JAUTO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::DISABLED)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "ADC analog watchdog 1 enable on scope ADC group injected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on injected channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog 1 enabled on injected channels"]
    ENABLED = 1,
}
impl From<JAWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group injected"]
pub struct JAWD1EN_R(crate::FieldReader<bool, JAWD1EN_A>);
impl JAWD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAWD1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAWD1EN_A {
        match self.bits {
            false => JAWD1EN_A::DISABLED,
            true => JAWD1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JAWD1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JAWD1EN_A::ENABLED
    }
}
impl core::ops::Deref for JAWD1EN_R {
    type Target = crate::FieldReader<bool, JAWD1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group injected"]
pub struct JAWD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> JAWD1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JAWD1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::DISABLED)
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "ADC analog watchdog 1 enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on regular channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog 1 enabled on regular channels"]
    ENABLED = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular"]
pub struct AWD1EN_R(crate::FieldReader<bool, AWD1EN_A>);
impl AWD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::DISABLED,
            true => AWD1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWD1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWD1EN_A::ENABLED
    }
}
impl core::ops::Deref for AWD1EN_R {
    type Target = crate::FieldReader<bool, AWD1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular"]
pub struct AWD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::DISABLED)
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "ADC analog watchdog 1 monitoring a single channel or all channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    ALL = 0,
    #[doc = "1: Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    SINGLE = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub struct AWD1SGL_R(crate::FieldReader<bool, AWD1SGL_A>);
impl AWD1SGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1SGL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::ALL,
            true => AWD1SGL_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == AWD1SGL_A::ALL
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == AWD1SGL_A::SINGLE
    }
}
impl core::ops::Deref for AWD1SGL_R {
    type Target = crate::FieldReader<bool, AWD1SGL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub struct AWD1SGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1SGL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD1SGL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWD1SGL_A::ALL)
    }
    #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWD1SGL_A::SINGLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "ADC group injected contexts queue mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQM_A {
    #[doc = "0: JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    MODE0 = 0,
    #[doc = "1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    MODE1 = 1,
}
impl From<JQM_A> for bool {
    #[inline(always)]
    fn from(variant: JQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQM` reader - ADC group injected contexts queue mode"]
pub struct JQM_R(crate::FieldReader<bool, JQM_A>);
impl JQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        JQM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQM_A {
        match self.bits {
            false => JQM_A::MODE0,
            true => JQM_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        **self == JQM_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == JQM_A::MODE1
    }
}
impl core::ops::Deref for JQM_R {
    type Target = crate::FieldReader<bool, JQM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JQM` writer - ADC group injected contexts queue mode"]
pub struct JQM_W<'a> {
    w: &'a mut W,
}
impl<'a> JQM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JQM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(JQM_A::MODE0)
    }
    #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(JQM_A::MODE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "ADC group injected sequencer discontinuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    ENABLED = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDISCEN` reader - ADC group injected sequencer discontinuous mode"]
pub struct JDISCEN_R(crate::FieldReader<bool, JDISCEN_A>);
impl JDISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDISCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::DISABLED,
            true => JDISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JDISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JDISCEN_A::ENABLED
    }
}
impl core::ops::Deref for JDISCEN_R {
    type Target = crate::FieldReader<bool, JDISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDISCEN` writer - ADC group injected sequencer discontinuous mode"]
pub struct JDISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDISCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JDISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DISCNUM` reader - ADC group regular sequencer discontinuous number of ranks"]
pub struct DISCNUM_R(crate::FieldReader<u8, u8>);
impl DISCNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DISCNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCNUM` writer - ADC group regular sequencer discontinuous number of ranks"]
pub struct DISCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "ADC group regular sequencer discontinuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - ADC group regular sequencer discontinuous mode"]
pub struct DISCEN_R(crate::FieldReader<bool, DISCEN_A>);
impl DISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISCEN_A::ENABLED
    }
}
impl core::ops::Deref for DISCEN_R {
    type Target = crate::FieldReader<bool, DISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCEN` writer - ADC group regular sequencer discontinuous mode"]
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "ADC low power auto wait\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTDLY_A {
    #[doc = "0: Auto delayed conversion mode off"]
    OFF = 0,
    #[doc = "1: Auto delayed conversion mode on"]
    ON = 1,
}
impl From<AUTDLY_A> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTDLY` reader - ADC low power auto wait"]
pub struct AUTDLY_R(crate::FieldReader<bool, AUTDLY_A>);
impl AUTDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTDLY_A {
        match self.bits {
            false => AUTDLY_A::OFF,
            true => AUTDLY_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == AUTDLY_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == AUTDLY_A::ON
    }
}
impl core::ops::Deref for AUTDLY_R {
    type Target = crate::FieldReader<bool, AUTDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTDLY` writer - ADC low power auto wait"]
pub struct AUTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTDLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto delayed conversion mode off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AUTDLY_A::OFF)
    }
    #[doc = "Auto delayed conversion mode on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(AUTDLY_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "ADC group regular continuous conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "1: Continuous conversion mode"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - ADC group regular continuous conversion mode"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == CONT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - ADC group regular continuous conversion mode"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "ADC group regular overrun configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRMOD_A {
    #[doc = "0: Preserve DR register when an overrun is detected"]
    PRESERVE = 0,
    #[doc = "1: Overwrite DR register when an overrun is detected"]
    OVERWRITE = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - ADC group regular overrun configuration"]
pub struct OVRMOD_R(crate::FieldReader<bool, OVRMOD_A>);
impl OVRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::PRESERVE,
            true => OVRMOD_A::OVERWRITE,
        }
    }
    #[doc = "Checks if the value of the field is `PRESERVE`"]
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        **self == OVRMOD_A::PRESERVE
    }
    #[doc = "Checks if the value of the field is `OVERWRITE`"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        **self == OVRMOD_A::OVERWRITE
    }
}
impl core::ops::Deref for OVRMOD_R {
    type Target = crate::FieldReader<bool, OVRMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRMOD` writer - ADC group regular overrun configuration"]
pub struct OVRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Preserve DR register when an overrun is detected"]
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::PRESERVE)
    }
    #[doc = "Overwrite DR register when an overrun is detected"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::OVERWRITE)
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
#[doc = "ADC group regular external trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTEN` reader - ADC group regular external trigger polarity"]
pub struct EXTEN_R(crate::FieldReader<u8, EXTEN_A>);
impl EXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISINGEDGE,
            2 => EXTEN_A::FALLINGEDGE,
            3 => EXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == EXTEN_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == EXTEN_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == EXTEN_A::BOTHEDGES
    }
}
impl core::ops::Deref for EXTEN_R {
    type Target = crate::FieldReader<u8, EXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEN` writer - ADC group regular external trigger polarity"]
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "ADC group regular external trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "5: Timer 4 CC4 event"]
    TIM4_CC4 = 5,
    #[doc = "7: Timer 8 TRGO event"]
    TIM8_TRGO = 7,
    #[doc = "8: Timer 8 TRGO2 event"]
    TIM8_TRGO2 = 8,
    #[doc = "12: Timer 4 TRGO event"]
    TIM4_TRGO = 12,
    #[doc = "16: HRTIM1_ADCTRG1 event"]
    HRTIM1_ADCTRG1 = 16,
    #[doc = "17: HRTIM1_ADCTRG3 event"]
    HRTIM1_ADCTRG3 = 17,
    #[doc = "18: LPTIM1_OUT event"]
    LPTIM1_OUT = 18,
    #[doc = "19: LPTIM2_OUT event"]
    LPTIM2_OUT = 19,
    #[doc = "20: LPTIM3_OUT event"]
    LPTIM3_OUT = 20,
    #[doc = "0: Timer 1 CC1 event"]
    TIM1_CC1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    TIM1_CC2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    TIM1_CC3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    TIM2_CC2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    TIM3_TRGO = 4,
    #[doc = "6: EXTI line 11"]
    EXTI11 = 6,
    #[doc = "9: Timer 1 TRGO event"]
    TIM1_TRGO = 9,
    #[doc = "10: Timer 1 TRGO2 event"]
    TIM1_TRGO2 = 10,
    #[doc = "11: Timer 2 TRGO event"]
    TIM2_TRGO = 11,
    #[doc = "13: Timer 6 TRGO event"]
    TIM6_TRGO = 13,
    #[doc = "14: Timer 15 TRGO event"]
    TIM15_TRGO = 14,
    #[doc = "15: Timer 3 CC4 event"]
    TIM3_CC4 = 15,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTSEL` reader - ADC group regular external trigger source"]
pub struct EXTSEL_R(crate::FieldReader<u8, EXTSEL_A>);
impl EXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            5 => Some(EXTSEL_A::TIM4_CC4),
            7 => Some(EXTSEL_A::TIM8_TRGO),
            8 => Some(EXTSEL_A::TIM8_TRGO2),
            12 => Some(EXTSEL_A::TIM4_TRGO),
            16 => Some(EXTSEL_A::HRTIM1_ADCTRG1),
            17 => Some(EXTSEL_A::HRTIM1_ADCTRG3),
            18 => Some(EXTSEL_A::LPTIM1_OUT),
            19 => Some(EXTSEL_A::LPTIM2_OUT),
            20 => Some(EXTSEL_A::LPTIM3_OUT),
            0 => Some(EXTSEL_A::TIM1_CC1),
            1 => Some(EXTSEL_A::TIM1_CC2),
            2 => Some(EXTSEL_A::TIM1_CC3),
            3 => Some(EXTSEL_A::TIM2_CC2),
            4 => Some(EXTSEL_A::TIM3_TRGO),
            6 => Some(EXTSEL_A::EXTI11),
            9 => Some(EXTSEL_A::TIM1_TRGO),
            10 => Some(EXTSEL_A::TIM1_TRGO2),
            11 => Some(EXTSEL_A::TIM2_TRGO),
            13 => Some(EXTSEL_A::TIM6_TRGO),
            14 => Some(EXTSEL_A::TIM15_TRGO),
            15 => Some(EXTSEL_A::TIM3_CC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIM4_CC4`"]
    #[inline(always)]
    pub fn is_tim4_cc4(&self) -> bool {
        **self == EXTSEL_A::TIM4_CC4
    }
    #[doc = "Checks if the value of the field is `TIM8_TRGO`"]
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM8_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8_TRGO2`"]
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        **self == EXTSEL_A::TIM8_TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM4_TRGO`"]
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM4_TRGO
    }
    #[doc = "Checks if the value of the field is `HRTIM1_ADCTRG1`"]
    #[inline(always)]
    pub fn is_hrtim1_adctrg1(&self) -> bool {
        **self == EXTSEL_A::HRTIM1_ADCTRG1
    }
    #[doc = "Checks if the value of the field is `HRTIM1_ADCTRG3`"]
    #[inline(always)]
    pub fn is_hrtim1_adctrg3(&self) -> bool {
        **self == EXTSEL_A::HRTIM1_ADCTRG3
    }
    #[doc = "Checks if the value of the field is `LPTIM1_OUT`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        **self == EXTSEL_A::LPTIM1_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == EXTSEL_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == EXTSEL_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `TIM1_CC1`"]
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        **self == EXTSEL_A::TIM1_CC1
    }
    #[doc = "Checks if the value of the field is `TIM1_CC2`"]
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        **self == EXTSEL_A::TIM1_CC2
    }
    #[doc = "Checks if the value of the field is `TIM1_CC3`"]
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        **self == EXTSEL_A::TIM1_CC3
    }
    #[doc = "Checks if the value of the field is `TIM2_CC2`"]
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        **self == EXTSEL_A::TIM2_CC2
    }
    #[doc = "Checks if the value of the field is `TIM3_TRGO`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM3_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        **self == EXTSEL_A::EXTI11
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM1_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO2`"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        **self == EXTSEL_A::TIM1_TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM15_TRGO`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM15_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3_CC4`"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        **self == EXTSEL_A::TIM3_CC4
    }
}
impl core::ops::Deref for EXTSEL_R {
    type Target = crate::FieldReader<u8, EXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL` writer - ADC group regular external trigger source"]
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn tim4_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM4_CC4)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM8_TRGO)
    }
    #[doc = "Timer 8 TRGO2 event"]
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM8_TRGO2)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM4_TRGO)
    }
    #[doc = "HRTIM1_ADCTRG1 event"]
    #[inline(always)]
    pub fn hrtim1_adctrg1(self) -> &'a mut W {
        self.variant(EXTSEL_A::HRTIM1_ADCTRG1)
    }
    #[doc = "HRTIM1_ADCTRG3 event"]
    #[inline(always)]
    pub fn hrtim1_adctrg3(self) -> &'a mut W {
        self.variant(EXTSEL_A::HRTIM1_ADCTRG3)
    }
    #[doc = "LPTIM1_OUT event"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::LPTIM1_OUT)
    }
    #[doc = "LPTIM2_OUT event"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::LPTIM2_OUT)
    }
    #[doc = "LPTIM3_OUT event"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::LPTIM3_OUT)
    }
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CC2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3_TRGO)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::EXTI11)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_TRGO)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_TRGO2)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_TRGO)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM6_TRGO)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM15_TRGO)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3_CC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "ADC data resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 16-bit resolution"]
    SIXTEENBIT = 0,
    #[doc = "1: 14-bit resolution in legacy mode (not optimized power consumption)"]
    FOURTEENBIT = 1,
    #[doc = "2: 12-bit resolution in legacy mode (not optimized power consumption)"]
    TWELVEBIT = 2,
    #[doc = "3: 10-bit resolution"]
    TENBIT = 3,
    #[doc = "5: 14-bit resolution"]
    FOURTEENBITV = 5,
    #[doc = "6: 12-bit resolution"]
    TWELVEBITV = 6,
    #[doc = "7: 8-bit resolution"]
    EIGHTBIT = 7,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES` reader - ADC data resolution"]
pub struct RES_R(crate::FieldReader<u8, RES_A>);
impl RES_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RES_A> {
        match self.bits {
            0 => Some(RES_A::SIXTEENBIT),
            1 => Some(RES_A::FOURTEENBIT),
            2 => Some(RES_A::TWELVEBIT),
            3 => Some(RES_A::TENBIT),
            5 => Some(RES_A::FOURTEENBITV),
            6 => Some(RES_A::TWELVEBITV),
            7 => Some(RES_A::EIGHTBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == RES_A::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `FOURTEENBIT`"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        **self == RES_A::FOURTEENBIT
    }
    #[doc = "Checks if the value of the field is `TWELVEBIT`"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        **self == RES_A::TWELVEBIT
    }
    #[doc = "Checks if the value of the field is `TENBIT`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        **self == RES_A::TENBIT
    }
    #[doc = "Checks if the value of the field is `FOURTEENBITV`"]
    #[inline(always)]
    pub fn is_fourteen_bit_v(&self) -> bool {
        **self == RES_A::FOURTEENBITV
    }
    #[doc = "Checks if the value of the field is `TWELVEBITV`"]
    #[inline(always)]
    pub fn is_twelve_bit_v(&self) -> bool {
        **self == RES_A::TWELVEBITV
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        **self == RES_A::EIGHTBIT
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES` writer - ADC data resolution"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16-bit resolution"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(RES_A::SIXTEENBIT)
    }
    #[doc = "14-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(RES_A::FOURTEENBIT)
    }
    #[doc = "12-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RES_A::TWELVEBIT)
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RES_A::TENBIT)
    }
    #[doc = "14-bit resolution"]
    #[inline(always)]
    pub fn fourteen_bit_v(self) -> &'a mut W {
        self.variant(RES_A::FOURTEENBITV)
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn twelve_bit_v(self) -> &'a mut W {
        self.variant(RES_A::TWELVEBITV)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RES_A::EIGHTBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "ADC DMA transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMNGT_A {
    #[doc = "0: Store output data in DR only"]
    DR = 0,
    #[doc = "1: DMA One Shot Mode selected"]
    DMA_ONESHOT = 1,
    #[doc = "2: DFSDM mode selected"]
    DFSDM = 2,
    #[doc = "3: DMA Circular Mode selected"]
    DMA_CIRCULAR = 3,
}
impl From<DMNGT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMNGT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMNGT` reader - ADC DMA transfer enable"]
pub struct DMNGT_R(crate::FieldReader<u8, DMNGT_A>);
impl DMNGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMNGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMNGT_A {
        match self.bits {
            0 => DMNGT_A::DR,
            1 => DMNGT_A::DMA_ONESHOT,
            2 => DMNGT_A::DFSDM,
            3 => DMNGT_A::DMA_CIRCULAR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DR`"]
    #[inline(always)]
    pub fn is_dr(&self) -> bool {
        **self == DMNGT_A::DR
    }
    #[doc = "Checks if the value of the field is `DMA_ONESHOT`"]
    #[inline(always)]
    pub fn is_dma_one_shot(&self) -> bool {
        **self == DMNGT_A::DMA_ONESHOT
    }
    #[doc = "Checks if the value of the field is `DFSDM`"]
    #[inline(always)]
    pub fn is_dfsdm(&self) -> bool {
        **self == DMNGT_A::DFSDM
    }
    #[doc = "Checks if the value of the field is `DMA_CIRCULAR`"]
    #[inline(always)]
    pub fn is_dma_circular(&self) -> bool {
        **self == DMNGT_A::DMA_CIRCULAR
    }
}
impl core::ops::Deref for DMNGT_R {
    type Target = crate::FieldReader<u8, DMNGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMNGT` writer - ADC DMA transfer enable"]
pub struct DMNGT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMNGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMNGT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Store output data in DR only"]
    #[inline(always)]
    pub fn dr(self) -> &'a mut W {
        self.variant(DMNGT_A::DR)
    }
    #[doc = "DMA One Shot Mode selected"]
    #[inline(always)]
    pub fn dma_one_shot(self) -> &'a mut W {
        self.variant(DMNGT_A::DMA_ONESHOT)
    }
    #[doc = "DFSDM mode selected"]
    #[inline(always)]
    pub fn dfsdm(self) -> &'a mut W {
        self.variant(DMNGT_A::DFSDM)
    }
    #[doc = "DMA Circular Mode selected"]
    #[inline(always)]
    pub fn dma_circular(self) -> &'a mut W {
        self.variant(DMNGT_A::DMA_CIRCULAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W {
        JQDIS_W { w: self }
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W {
        AWD1CH_W { w: self }
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W {
        JAUTO_W { w: self }
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W {
        JAWD1EN_W { w: self }
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W {
        AWD1EN_W { w: self }
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W {
        AWD1SGL_W { w: self }
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W {
        JQM_W { w: self }
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W {
        JDISCEN_W { w: self }
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W {
        DISCNUM_W { w: self }
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W {
        AUTDLY_W { w: self }
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W {
        OVRMOD_W { w: self }
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmngt(&mut self) -> DMNGT_W {
        DMNGT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0x8000_0000"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
