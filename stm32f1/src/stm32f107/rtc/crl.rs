#[doc = "Register `CRL` reader"]
pub struct R(crate::R<CRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRL` writer"]
pub struct W(crate::W<CRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRL_SPEC>;
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
impl From<crate::W<CRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Second Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECF_A {
    #[doc = "0: Second flag condition not met"]
    NOPRESCALEROVERFLOW = 0,
    #[doc = "1: Second flag condition met"]
    PRESCALEROVERFLOW = 1,
}
impl From<SECF_A> for bool {
    #[inline(always)]
    fn from(variant: SECF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECF` reader - Second Flag"]
pub struct SECF_R(crate::FieldReader<bool, SECF_A>);
impl SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECF_A {
        match self.bits {
            false => SECF_A::NOPRESCALEROVERFLOW,
            true => SECF_A::PRESCALEROVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOPRESCALEROVERFLOW`"]
    #[inline(always)]
    pub fn is_no_prescaler_overflow(&self) -> bool {
        **self == SECF_A::NOPRESCALEROVERFLOW
    }
    #[doc = "Checks if the value of the field is `PRESCALEROVERFLOW`"]
    #[inline(always)]
    pub fn is_prescaler_overflow(&self) -> bool {
        **self == SECF_A::PRESCALEROVERFLOW
    }
}
impl core::ops::Deref for SECF_R {
    type Target = crate::FieldReader<bool, SECF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Second Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECF_AW {
    #[doc = "0: Clear flag"]
    CLEAR = 0,
}
impl From<SECF_AW> for bool {
    #[inline(always)]
    fn from(variant: SECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECF` writer - Second Flag"]
pub struct SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> SECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SECF_AW::CLEAR)
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
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRF_A {
    #[doc = "0: Alarm not detected"]
    NOALARM = 0,
    #[doc = "1: Alarm detected"]
    ALARM = 1,
}
impl From<ALRF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRF` reader - Alarm Flag"]
pub struct ALRF_R(crate::FieldReader<bool, ALRF_A>);
impl ALRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRF_A {
        match self.bits {
            false => ALRF_A::NOALARM,
            true => ALRF_A::ALARM,
        }
    }
    #[doc = "Checks if the value of the field is `NOALARM`"]
    #[inline(always)]
    pub fn is_no_alarm(&self) -> bool {
        **self == ALRF_A::NOALARM
    }
    #[doc = "Checks if the value of the field is `ALARM`"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        **self == ALRF_A::ALARM
    }
}
impl core::ops::Deref for ALRF_R {
    type Target = crate::FieldReader<bool, ALRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRF_AW {
    #[doc = "0: Clear flag"]
    CLEAR = 0,
}
impl From<ALRF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRF` writer - Alarm Flag"]
pub struct ALRF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWF_A {
    #[doc = "0: Overflow not detected"]
    NOOVERFLOW = 0,
    #[doc = "1: 32-bit programmable counter overflow occurred"]
    OVERFLOW = 1,
}
impl From<OWF_A> for bool {
    #[inline(always)]
    fn from(variant: OWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWF` reader - Overflow Flag"]
pub struct OWF_R(crate::FieldReader<bool, OWF_A>);
impl OWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWF_A {
        match self.bits {
            false => OWF_A::NOOVERFLOW,
            true => OWF_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        **self == OWF_A::NOOVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == OWF_A::OVERFLOW
    }
}
impl core::ops::Deref for OWF_R {
    type Target = crate::FieldReader<bool, OWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWF_AW {
    #[doc = "0: Clear flag"]
    CLEAR = 0,
}
impl From<OWF_AW> for bool {
    #[inline(always)]
    fn from(variant: OWF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWF` writer - Overflow Flag"]
pub struct OWF_W<'a> {
    w: &'a mut W,
}
impl<'a> OWF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OWF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Registers Synchronized Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Registers not yet synchronized"]
    NOTSYNCHRONIZED = 0,
    #[doc = "1: Registers synchronized"]
    SYNCHRONIZED = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers Synchronized Flag"]
pub struct RSF_R(crate::FieldReader<bool, RSF_A>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NOTSYNCHRONIZED,
            true => RSF_A::SYNCHRONIZED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_not_synchronized(&self) -> bool {
        **self == RSF_A::NOTSYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        **self == RSF_A::SYNCHRONIZED
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, RSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Registers Synchronized Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    #[doc = "0: Clear flag"]
    CLEAR = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers Synchronized Flag"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Configuration Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF_A {
    #[doc = "0: Exit configuration mode (start update of RTC registers)"]
    EXIT = 0,
    #[doc = "1: Enter configuration mode"]
    ENTER = 1,
}
impl From<CNF_A> for bool {
    #[inline(always)]
    fn from(variant: CNF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNF` reader - Configuration Flag"]
pub struct CNF_R(crate::FieldReader<bool, CNF_A>);
impl CNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNF_A {
        match self.bits {
            false => CNF_A::EXIT,
            true => CNF_A::ENTER,
        }
    }
    #[doc = "Checks if the value of the field is `EXIT`"]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        **self == CNF_A::EXIT
    }
    #[doc = "Checks if the value of the field is `ENTER`"]
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        **self == CNF_A::ENTER
    }
}
impl core::ops::Deref for CNF_R {
    type Target = crate::FieldReader<bool, CNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNF` writer - Configuration Flag"]
pub struct CNF_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Exit configuration mode (start update of RTC registers)"]
    #[inline(always)]
    pub fn exit(self) -> &'a mut W {
        self.variant(CNF_A::EXIT)
    }
    #[doc = "Enter configuration mode"]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(CNF_A::ENTER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "RTC operation OFF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOFF_A {
    #[doc = "0: Last write operation on RTC registers is still ongoing"]
    ENABLED = 0,
    #[doc = "1: Last write operation on RTC registers terminated"]
    DISABLED = 1,
}
impl From<RTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RTOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub struct RTOFF_R(crate::FieldReader<bool, RTOFF_A>);
impl RTOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTOFF_A {
        match self.bits {
            false => RTOFF_A::ENABLED,
            true => RTOFF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RTOFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RTOFF_A::DISABLED
    }
}
impl core::ops::Deref for RTOFF_R {
    type Target = crate::FieldReader<bool, RTOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&mut self) -> SECF_W {
        SECF_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&mut self) -> ALRF_W {
        ALRF_W { w: self }
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&mut self) -> OWF_W {
        OWF_W { w: self }
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&mut self) -> CNF_W {
        CNF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crl](index.html) module"]
pub struct CRL_SPEC;
impl crate::RegisterSpec for CRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crl::R](R) reader structure"]
impl crate::Readable for CRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crl::W](W) writer structure"]
impl crate::Writable for CRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRL to value 0x20"]
impl crate::Resettable for CRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
