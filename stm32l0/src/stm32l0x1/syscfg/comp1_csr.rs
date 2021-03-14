#[doc = "Reader of register COMP1_CSR"]
pub type R = crate::R<u32, super::COMP1_CSR>;
#[doc = "Writer for register COMP1_CSR"]
pub type W = crate::W<u32, super::COMP1_CSR>;
#[doc = "Register COMP1_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP1_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `COMP1EN`"]
pub type COMP1EN_R = crate::R<bool, COMP1EN_A>;
impl COMP1EN_R {
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
        *self == COMP1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMP1EN`"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `COMP1INNSEL`"]
pub type COMP1INNSEL_R = crate::R<u8, COMP1INNSEL_A>;
impl COMP1INNSEL_R {
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
        *self == COMP1INNSEL_A::VREFINT
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == COMP1INNSEL_A::PA0
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == COMP1INNSEL_A::PA4
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == COMP1INNSEL_A::PA5
    }
}
#[doc = "Write proxy for field `COMP1INNSEL`"]
pub struct COMP1INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1INNSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Reader of field `COMP1WM`"]
pub type COMP1WM_R = crate::R<bool, COMP1WM_A>;
impl COMP1WM_R {
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
        *self == COMP1WM_A::PA1
    }
    #[doc = "Checks if the value of the field is `COMP2PLUS`"]
    #[inline(always)]
    pub fn is_comp2plus(&self) -> bool {
        *self == COMP1WM_A::COMP2PLUS
    }
}
#[doc = "Write proxy for field `COMP1WM`"]
pub struct COMP1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1WM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1WM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
#[doc = "Reader of field `COMP1LPTIMIN1`"]
pub type COMP1LPTIMIN1_R = crate::R<bool, COMP1LPTIMIN1_A>;
impl COMP1LPTIMIN1_R {
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
        *self == COMP1LPTIMIN1_A::GATED
    }
    #[doc = "Checks if the value of the field is `NOTGATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP1LPTIMIN1_A::NOTGATED
    }
}
#[doc = "Write proxy for field `COMP1LPTIMIN1`"]
pub struct COMP1LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LPTIMIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1LPTIMIN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
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
#[doc = "Reader of field `COMP1POLARITY`"]
pub type COMP1POLARITY_R = crate::R<bool, COMP1POLARITY_A>;
impl COMP1POLARITY_R {
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
        *self == COMP1POLARITY_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POLARITY_A::INVERTED
    }
}
#[doc = "Write proxy for field `COMP1POLARITY`"]
pub struct COMP1POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1POLARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
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
#[doc = "Reader of field `COMP1VALUE`"]
pub type COMP1VALUE_R = crate::R<bool, COMP1VALUE_A>;
impl COMP1VALUE_R {
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
        *self == COMP1VALUE_A::NOTEQUAL
    }
    #[doc = "Checks if the value of the field is `EQUAL`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == COMP1VALUE_A::EQUAL
    }
}
#[doc = "Write proxy for field `COMP1VALUE`"]
pub struct COMP1VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1VALUE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator values are not equal"]
    #[inline(always)]
    pub fn not_equal(self) -> &'a mut W {
        self.variant(COMP1VALUE_A::NOTEQUAL)
    }
    #[doc = "Comparator values are equal"]
    #[inline(always)]
    pub fn equal(self) -> &'a mut W {
        self.variant(COMP1VALUE_A::EQUAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
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
#[doc = "Reader of field `COMP1LOCK`"]
pub type COMP1LOCK_R = crate::R<bool, COMP1LOCK_A>;
impl COMP1LOCK_R {
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
        *self == COMP1LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == COMP1LOCK_A::READONLY
    }
}
#[doc = "Write proxy for field `COMP1LOCK`"]
pub struct COMP1LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP1_CSR\\[31:0\\]
for comparator 1 are read/write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::READWRITE)
    }
    #[doc = "COMP1_CSR\\[31:0\\]
for comparator 1 are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::READONLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W {
        COMP1INNSEL_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&mut self) -> COMP1WM_W {
        COMP1WM_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W {
        COMP1LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W {
        COMP1POLARITY_W { w: self }
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&mut self) -> COMP1VALUE_W {
        COMP1VALUE_W { w: self }
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W {
        COMP1LOCK_W { w: self }
    }
}
