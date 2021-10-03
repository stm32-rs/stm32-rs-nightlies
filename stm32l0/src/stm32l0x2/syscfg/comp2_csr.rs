#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "COMP2_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LOCK_A {
    #[doc = "0: COMP2_CSR\\[31:0\\]
for comparator 2 are read/write"]
    READWRITE = 0,
    #[doc = "1: COMP2_CSR\\[31:0\\]
for comparator 2 are read-only"]
    READONLY = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LOCK` reader - COMP2_CSR register lock bit"]
pub struct COMP2LOCK_R(crate::FieldReader<bool, COMP2LOCK_A>);
impl COMP2LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::READWRITE,
            true => COMP2LOCK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        **self == COMP2LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        **self == COMP2LOCK_A::READONLY
    }
}
impl core::ops::Deref for COMP2LOCK_R {
    type Target = crate::FieldReader<bool, COMP2LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 2 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2VALUE_A {
    #[doc = "0: Comparator values are not equal"]
    NOTEQUAL = 0,
    #[doc = "1: Comparator values are equal"]
    EQUAL = 1,
}
impl From<COMP2VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2VALUE` reader - Comparator 2 output status bit"]
pub struct COMP2VALUE_R(crate::FieldReader<bool, COMP2VALUE_A>);
impl COMP2VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2VALUE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2VALUE_A {
        match self.bits {
            false => COMP2VALUE_A::NOTEQUAL,
            true => COMP2VALUE_A::EQUAL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEQUAL`"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        **self == COMP2VALUE_A::NOTEQUAL
    }
    #[doc = "Checks if the value of the field is `EQUAL`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        **self == COMP2VALUE_A::EQUAL
    }
}
impl core::ops::Deref for COMP2VALUE_R {
    type Target = crate::FieldReader<bool, COMP2VALUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2POLARITY_A {
    #[doc = "0: Comparator 2 output value not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Comparator 2 output value inverted"]
    INVERTED = 1,
}
impl From<COMP2POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2POLARITY` reader - Comparator 2 polarity selection bit"]
pub struct COMP2POLARITY_R(crate::FieldReader<bool, COMP2POLARITY_A>);
impl COMP2POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2POLARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POLARITY_A {
        match self.bits {
            false => COMP2POLARITY_A::NOTINVERTED,
            true => COMP2POLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == COMP2POLARITY_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == COMP2POLARITY_A::INVERTED
    }
}
impl core::ops::Deref for COMP2POLARITY_R {
    type Target = crate::FieldReader<bool, COMP2POLARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2POLARITY` writer - Comparator 2 polarity selection bit"]
pub struct COMP2POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2POLARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 2 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POLARITY_A::NOTINVERTED)
    }
    #[doc = "Comparator 2 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POLARITY_A::INVERTED)
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
#[doc = "Comparator 2 LPTIM input 1 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LPTIMIN1_A {
    #[doc = "0: Comparator 2 output gated"]
    GATED = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 1"]
    NOTGATED = 1,
}
impl From<COMP2LPTIMIN1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LPTIMIN1` reader - Comparator 2 LPTIM input 1 propagation bit"]
pub struct COMP2LPTIMIN1_R(crate::FieldReader<bool, COMP2LPTIMIN1_A>);
impl COMP2LPTIMIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LPTIMIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LPTIMIN1_A {
        match self.bits {
            false => COMP2LPTIMIN1_A::GATED,
            true => COMP2LPTIMIN1_A::NOTGATED,
        }
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        **self == COMP2LPTIMIN1_A::GATED
    }
    #[doc = "Checks if the value of the field is `NOTGATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        **self == COMP2LPTIMIN1_A::NOTGATED
    }
}
impl core::ops::Deref for COMP2LPTIMIN1_R {
    type Target = crate::FieldReader<bool, COMP2LPTIMIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LPTIMIN1` writer - Comparator 2 LPTIM input 1 propagation bit"]
pub struct COMP2LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LPTIMIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2LPTIMIN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN1_A::GATED)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN1_A::NOTGATED)
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
#[doc = "Comparator 2 LPTIM input 2 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LPTIMIN2_A {
    #[doc = "0: Comparator 2 output gated"]
    GATED = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 2"]
    NOTGATED = 1,
}
impl From<COMP2LPTIMIN2_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LPTIMIN2` reader - Comparator 2 LPTIM input 2 propagation bit"]
pub struct COMP2LPTIMIN2_R(crate::FieldReader<bool, COMP2LPTIMIN2_A>);
impl COMP2LPTIMIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LPTIMIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LPTIMIN2_A {
        match self.bits {
            false => COMP2LPTIMIN2_A::GATED,
            true => COMP2LPTIMIN2_A::NOTGATED,
        }
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        **self == COMP2LPTIMIN2_A::GATED
    }
    #[doc = "Checks if the value of the field is `NOTGATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        **self == COMP2LPTIMIN2_A::NOTGATED
    }
}
impl core::ops::Deref for COMP2LPTIMIN2_R {
    type Target = crate::FieldReader<bool, COMP2LPTIMIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LPTIMIN2` writer - Comparator 2 LPTIM input 2 propagation bit"]
pub struct COMP2LPTIMIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LPTIMIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2LPTIMIN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN2_A::GATED)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 2"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN2_A::NOTGATED)
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
#[doc = "Comparator 2 Input Plus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2INPSEL_A {
    #[doc = "0: PA3"]
    PA3 = 0,
    #[doc = "1: PB4"]
    PB4 = 1,
    #[doc = "2: PB5"]
    PB5 = 2,
    #[doc = "3: PB6"]
    PB6 = 3,
    #[doc = "4: PB7"]
    PB7 = 4,
    #[doc = "5: PA7"]
    PA7 = 5,
}
impl From<COMP2INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2INPSEL_R(crate::FieldReader<u8, COMP2INPSEL_A>);
impl COMP2INPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INPSEL_A> {
        match self.bits {
            0 => Some(COMP2INPSEL_A::PA3),
            1 => Some(COMP2INPSEL_A::PB4),
            2 => Some(COMP2INPSEL_A::PB5),
            3 => Some(COMP2INPSEL_A::PB6),
            4 => Some(COMP2INPSEL_A::PB7),
            5 => Some(COMP2INPSEL_A::PA7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        **self == COMP2INPSEL_A::PA3
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        **self == COMP2INPSEL_A::PB4
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        **self == COMP2INPSEL_A::PB5
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        **self == COMP2INPSEL_A::PB6
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        **self == COMP2INPSEL_A::PB7
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        **self == COMP2INPSEL_A::PA7
    }
}
impl core::ops::Deref for COMP2INPSEL_R {
    type Target = crate::FieldReader<u8, COMP2INPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2INPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PA3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PA3)
    }
    #[doc = "PB4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PB4)
    }
    #[doc = "PB5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PB5)
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PB6)
    }
    #[doc = "PB7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PB7)
    }
    #[doc = "PA7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::PA7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Comparator 2 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2INNSEL_A {
    #[doc = "0: VREFINT"]
    VREFINT = 0,
    #[doc = "1: PA2"]
    PA2 = 1,
    #[doc = "2: PA4"]
    PA4 = 2,
    #[doc = "3: PA5"]
    PA5 = 3,
    #[doc = "4: 1/4 VREFINT"]
    VREFINT_DIV4 = 4,
    #[doc = "5: 1/2 VREFINT"]
    VREFINT_DIV2 = 5,
    #[doc = "6: 3/4 VREFINT"]
    VREFINT_DIV3_4 = 6,
    #[doc = "7: PB3"]
    PB3 = 7,
}
impl From<COMP2INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2INNSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2INNSEL_R(crate::FieldReader<u8, COMP2INNSEL_A>);
impl COMP2INNSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INNSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2INNSEL_A {
        match self.bits {
            0 => COMP2INNSEL_A::VREFINT,
            1 => COMP2INNSEL_A::PA2,
            2 => COMP2INNSEL_A::PA4,
            3 => COMP2INNSEL_A::PA5,
            4 => COMP2INNSEL_A::VREFINT_DIV4,
            5 => COMP2INNSEL_A::VREFINT_DIV2,
            6 => COMP2INNSEL_A::VREFINT_DIV3_4,
            7 => COMP2INNSEL_A::PB3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFINT`"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        **self == COMP2INNSEL_A::VREFINT
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        **self == COMP2INNSEL_A::PA2
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        **self == COMP2INNSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        **self == COMP2INNSEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `VREFINT_DIV4`"]
    #[inline(always)]
    pub fn is_vrefint_div4(&self) -> bool {
        **self == COMP2INNSEL_A::VREFINT_DIV4
    }
    #[doc = "Checks if the value of the field is `VREFINT_DIV2`"]
    #[inline(always)]
    pub fn is_vrefint_div2(&self) -> bool {
        **self == COMP2INNSEL_A::VREFINT_DIV2
    }
    #[doc = "Checks if the value of the field is `VREFINT_DIV3_4`"]
    #[inline(always)]
    pub fn is_vrefint_div3_4(&self) -> bool {
        **self == COMP2INNSEL_A::VREFINT_DIV3_4
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        **self == COMP2INNSEL_A::PB3
    }
}
impl core::ops::Deref for COMP2INNSEL_R {
    type Target = crate::FieldReader<u8, COMP2INNSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INNSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2INNSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VREFINT)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::PA2)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::PA4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::PA5)
    }
    #[doc = "1/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VREFINT_DIV4)
    }
    #[doc = "1/2 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div2(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VREFINT_DIV2)
    }
    #[doc = "3/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div3_4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VREFINT_DIV3_4)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::PB3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 2 power mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2SPEED_A {
    #[doc = "0: Slow speed"]
    SLOW = 0,
    #[doc = "1: Fast speed"]
    FAST = 1,
}
impl From<COMP2SPEED_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2SPEED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2SPEED` reader - Comparator 2 power mode selection bit"]
pub struct COMP2SPEED_R(crate::FieldReader<bool, COMP2SPEED_A>);
impl COMP2SPEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2SPEED_A {
        match self.bits {
            false => COMP2SPEED_A::SLOW,
            true => COMP2SPEED_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        **self == COMP2SPEED_A::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == COMP2SPEED_A::FAST
    }
}
impl core::ops::Deref for COMP2SPEED_R {
    type Target = crate::FieldReader<bool, COMP2SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2SPEED` writer - Comparator 2 power mode selection bit"]
pub struct COMP2SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2SPEED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(COMP2SPEED_A::SLOW)
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(COMP2SPEED_A::FAST)
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
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator 2 switched OFF"]
    DISABLED = 0,
    #[doc = "1: Comparator 2 switched ON"]
    ENABLED = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable bit"]
pub struct COMP2EN_R(crate::FieldReader<bool, COMP2EN_A>);
impl COMP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::DISABLED,
            true => COMP2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMP2EN_A::ENABLED
    }
}
impl core::ops::Deref for COMP2EN_R {
    type Target = crate::FieldReader<bool, COMP2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable bit"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 2 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::DISABLED)
    }
    #[doc = "Comparator 2 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::ENABLED)
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
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2value(&self) -> COMP2VALUE_R {
        COMP2VALUE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&self) -> COMP2POLARITY_R {
        COMP2POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&self) -> COMP2LPTIMIN1_R {
        COMP2LPTIMIN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&self) -> COMP2LPTIMIN2_R {
        COMP2LPTIMIN2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&self) -> COMP2INNSEL_R {
        COMP2INNSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&self) -> COMP2SPEED_R {
        COMP2SPEED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&mut self) -> COMP2POLARITY_W {
        COMP2POLARITY_W { w: self }
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&mut self) -> COMP2LPTIMIN1_W {
        COMP2LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&mut self) -> COMP2LPTIMIN2_W {
        COMP2LPTIMIN2_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W {
        COMP2INPSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&mut self) -> COMP2INNSEL_W {
        COMP2INNSEL_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&mut self) -> COMP2SPEED_W {
        COMP2SPEED_W { w: self }
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
