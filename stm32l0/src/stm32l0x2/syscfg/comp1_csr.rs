#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "COMP1_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1LOCK_A {
    #[doc = "0: COMP1_CSR\\[31:0\\]
for comparator 1 are read/write"]
    READWRITE = 0,
    #[doc = "1: COMP1_CSR\\[31:0\\]
for comparator 1 are read-only"]
    READONLY = 1,
}
impl From<COMP1LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1LOCK` reader - COMP1_CSR register lock bit"]
pub struct COMP1LOCK_R(crate::FieldReader<bool, COMP1LOCK_A>);
impl COMP1LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LOCK_A {
        match self.bits {
            false => COMP1LOCK_A::READWRITE,
            true => COMP1LOCK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        **self == COMP1LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        **self == COMP1LOCK_A::READONLY
    }
}
impl core::ops::Deref for COMP1LOCK_R {
    type Target = crate::FieldReader<bool, COMP1LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 1 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1VALUE_A {
    #[doc = "0: Comparator values are not equal"]
    NOTEQUAL = 0,
    #[doc = "1: Comparator values are equal"]
    EQUAL = 1,
}
impl From<COMP1VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1VALUE` reader - Comparator 1 output status bit"]
pub struct COMP1VALUE_R(crate::FieldReader<bool, COMP1VALUE_A>);
impl COMP1VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1VALUE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1VALUE_A {
        match self.bits {
            false => COMP1VALUE_A::NOTEQUAL,
            true => COMP1VALUE_A::EQUAL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEQUAL`"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        **self == COMP1VALUE_A::NOTEQUAL
    }
    #[doc = "Checks if the value of the field is `EQUAL`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        **self == COMP1VALUE_A::EQUAL
    }
}
impl core::ops::Deref for COMP1VALUE_R {
    type Target = crate::FieldReader<bool, COMP1VALUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 1 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1POLARITY_A {
    #[doc = "0: Comparator 1 output value not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Comparator 1 output value inverted"]
    INVERTED = 1,
}
impl From<COMP1POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1POLARITY` reader - Comparator 1 polarity selection bit"]
pub struct COMP1POLARITY_R(crate::FieldReader<bool, COMP1POLARITY_A>);
impl COMP1POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1POLARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1POLARITY_A {
        match self.bits {
            false => COMP1POLARITY_A::NOTINVERTED,
            true => COMP1POLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == COMP1POLARITY_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == COMP1POLARITY_A::INVERTED
    }
}
impl core::ops::Deref for COMP1POLARITY_R {
    type Target = crate::FieldReader<bool, COMP1POLARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1POLARITY` writer - Comparator 1 polarity selection bit"]
pub struct COMP1POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1POLARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP1POLARITY_A::NOTINVERTED)
    }
    #[doc = "Comparator 1 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP1POLARITY_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Comparator 1 LPTIM input propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1LPTIMIN1_A {
    #[doc = "0: Comparator 1 output gated"]
    GATED = 0,
    #[doc = "1: Comparator 1 output sent to LPTIM input 1"]
    NOTGATED = 1,
}
impl From<COMP1LPTIMIN1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LPTIMIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1LPTIMIN1` reader - Comparator 1 LPTIM input propagation bit"]
pub struct COMP1LPTIMIN1_R(crate::FieldReader<bool, COMP1LPTIMIN1_A>);
impl COMP1LPTIMIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1LPTIMIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LPTIMIN1_A {
        match self.bits {
            false => COMP1LPTIMIN1_A::GATED,
            true => COMP1LPTIMIN1_A::NOTGATED,
        }
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        **self == COMP1LPTIMIN1_A::GATED
    }
    #[doc = "Checks if the value of the field is `NOTGATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        **self == COMP1LPTIMIN1_A::NOTGATED
    }
}
impl core::ops::Deref for COMP1LPTIMIN1_R {
    type Target = crate::FieldReader<bool, COMP1LPTIMIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1LPTIMIN1` writer - Comparator 1 LPTIM input propagation bit"]
pub struct COMP1LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LPTIMIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1LPTIMIN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP1LPTIMIN1_A::GATED)
    }
    #[doc = "Comparator 1 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP1LPTIMIN1_A::NOTGATED)
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
#[doc = "Comparator 1 window mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1WM_A {
    #[doc = "0: Plus input of comparator 1 connected to PA1"]
    PA1 = 0,
    #[doc = "1: Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    COMP2PLUS = 1,
}
impl From<COMP1WM_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1WM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1WM` reader - Comparator 1 window mode selection bit"]
pub struct COMP1WM_R(crate::FieldReader<bool, COMP1WM_A>);
impl COMP1WM_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1WM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1WM_A {
        match self.bits {
            false => COMP1WM_A::PA1,
            true => COMP1WM_A::COMP2PLUS,
        }
    }
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        **self == COMP1WM_A::PA1
    }
    #[doc = "Checks if the value of the field is `COMP2PLUS`"]
    #[inline(always)]
    pub fn is_comp2plus(&self) -> bool {
        **self == COMP1WM_A::COMP2PLUS
    }
}
impl core::ops::Deref for COMP1WM_R {
    type Target = crate::FieldReader<bool, COMP1WM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1WM` writer - Comparator 1 window mode selection bit"]
pub struct COMP1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1WM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1WM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Plus input of comparator 1 connected to PA1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(COMP1WM_A::PA1)
    }
    #[doc = "Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    #[inline(always)]
    pub fn comp2plus(self) -> &'a mut W {
        self.variant(COMP1WM_A::COMP2PLUS)
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
#[doc = "Comparator 1 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP1INNSEL_A {
    #[doc = "0: VREFINT"]
    VREFINT = 0,
    #[doc = "1: PA0"]
    PA0 = 1,
    #[doc = "2: PA4"]
    PA4 = 2,
    #[doc = "3: PA5"]
    PA5 = 3,
}
impl From<COMP1INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP1INNSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1INNSEL_R(crate::FieldReader<u8, COMP1INNSEL_A>);
impl COMP1INNSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1INNSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1INNSEL_A {
        match self.bits {
            0 => COMP1INNSEL_A::VREFINT,
            1 => COMP1INNSEL_A::PA0,
            2 => COMP1INNSEL_A::PA4,
            3 => COMP1INNSEL_A::PA5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFINT`"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        **self == COMP1INNSEL_A::VREFINT
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        **self == COMP1INNSEL_A::PA0
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        **self == COMP1INNSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        **self == COMP1INNSEL_A::PA5
    }
}
impl core::ops::Deref for COMP1INNSEL_R {
    type Target = crate::FieldReader<u8, COMP1INNSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1INNSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1INNSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::VREFINT)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::PA0)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::PA5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Comparator 1 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1EN_A {
    #[doc = "0: Comparator 1 switched OFF"]
    DISABLED = 0,
    #[doc = "1: Comparator 1 switched ON"]
    ENABLED = 1,
}
impl From<COMP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable bit"]
pub struct COMP1EN_R(crate::FieldReader<bool, COMP1EN_A>);
impl COMP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1EN_A {
        match self.bits {
            false => COMP1EN_A::DISABLED,
            true => COMP1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMP1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMP1EN_A::ENABLED
    }
}
impl core::ops::Deref for COMP1EN_R {
    type Target = crate::FieldReader<bool, COMP1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable bit"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::DISABLED)
    }
    #[doc = "Comparator 1 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::ENABLED)
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
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W {
        COMP1POLARITY_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W {
        COMP1LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&mut self) -> COMP1WM_W {
        COMP1WM_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W {
        COMP1INNSEL_W { w: self }
    }
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
