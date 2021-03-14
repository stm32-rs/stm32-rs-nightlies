#[doc = "Reader of register FLTINR2"]
pub type R = crate::R<u32, super::FLTINR2>;
#[doc = "Writer for register FLTINR2"]
pub type W = crate::W<u32, super::FLTINR2>;
#[doc = "Register FLTINR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTINR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FLTSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTSD_A {
    #[doc = "0: f_FLTS=f_HRTIM"]
    DIV1 = 0,
    #[doc = "1: f_FLTS=f_HRTIM/2"]
    DIV2 = 1,
    #[doc = "2: f_FLTS=f_HRTIM/4"]
    DIV4 = 2,
    #[doc = "3: f_FLTS=f_HRTIM/8"]
    DIV8 = 3,
}
impl From<FLTSD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTSD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTSD`"]
pub type FLTSD_R = crate::R<u8, FLTSD_A>;
impl FLTSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTSD_A {
        match self.bits {
            0 => FLTSD_A::DIV1,
            1 => FLTSD_A::DIV2,
            2 => FLTSD_A::DIV4,
            3 => FLTSD_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FLTSD_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FLTSD_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FLTSD_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FLTSD_A::DIV8
    }
}
#[doc = "Write proxy for field `FLTSD`"]
pub struct FLTSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTSD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f_FLTS=f_HRTIM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FLTSD_A::DIV1)
    }
    #[doc = "f_FLTS=f_HRTIM/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(FLTSD_A::DIV2)
    }
    #[doc = "f_FLTS=f_HRTIM/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(FLTSD_A::DIV4)
    }
    #[doc = "f_FLTS=f_HRTIM/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FLTSD_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "FLT5LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5LCK_A {
    #[doc = "0: Fault bits are read/write"]
    UNLOCKED = 0,
    #[doc = "1: Fault bits are read-only"]
    LOCKED = 1,
}
impl From<FLT5LCK_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT5LCK`"]
pub type FLT5LCK_R = crate::R<bool, FLT5LCK_A>;
impl FLT5LCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5LCK_A {
        match self.bits {
            false => FLT5LCK_A::UNLOCKED,
            true => FLT5LCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT5LCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT5LCK_A::LOCKED
    }
}
#[doc = "FLT5LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5LCK_AW {
    #[doc = "1: Lock corresponding fault bits"]
    LOCK = 1,
}
impl From<FLT5LCK_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FLT5LCK`"]
pub struct FLT5LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5LCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5LCK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT5LCK_AW::LOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "FLT5F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT5F_A {
    #[doc = "0: No filter, FLTx acts asynchronously"]
    DISABLED = 0,
    #[doc = "1: f_SAMPLING=f_HRTIM, N=2"]
    DIV1_N2 = 1,
    #[doc = "2: f_SAMPLING=f_HRTIM, N=4"]
    DIV1_N4 = 2,
    #[doc = "3: f_SAMPLING=f_HRTIM, N=8"]
    DIV1_N8 = 3,
    #[doc = "4: f_SAMPLING=f_HRTIM/2, N=6"]
    DIV2_N6 = 4,
    #[doc = "5: f_SAMPLING=f_HRTIM/2, N=8"]
    DIV2_N8 = 5,
    #[doc = "6: f_SAMPLING=f_HRTIM/4, N=6"]
    DIV4_N6 = 6,
    #[doc = "7: f_SAMPLING=f_HRTIM/4, N=8"]
    DIV4_N8 = 7,
    #[doc = "8: f_SAMPLING=f_HRTIM/8, N=6"]
    DIV8_N6 = 8,
    #[doc = "9: f_SAMPLING=f_HRTIM/8, N=8"]
    DIV8_N8 = 9,
    #[doc = "10: f_SAMPLING=f_HRTIM/16, N=5"]
    DIV16_N5 = 10,
    #[doc = "11: f_SAMPLING=f_HRTIM/16, N=6"]
    DIV16_N6 = 11,
    #[doc = "12: f_SAMPLING=f_HRTIM/16, N=8"]
    DIV16_N8 = 12,
    #[doc = "13: f_SAMPLING=f_HRTIM/32, N=5"]
    DIV32_N5 = 13,
    #[doc = "14: f_SAMPLING=f_HRTIM/32, N=6"]
    DIV32_N6 = 14,
    #[doc = "15: f_SAMPLING=f_HRTIM/32, N=8"]
    DIV32_N8 = 15,
}
impl From<FLT5F_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT5F_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLT5F`"]
pub type FLT5F_R = crate::R<u8, FLT5F_A>;
impl FLT5F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5F_A {
        match self.bits {
            0 => FLT5F_A::DISABLED,
            1 => FLT5F_A::DIV1_N2,
            2 => FLT5F_A::DIV1_N4,
            3 => FLT5F_A::DIV1_N8,
            4 => FLT5F_A::DIV2_N6,
            5 => FLT5F_A::DIV2_N8,
            6 => FLT5F_A::DIV4_N6,
            7 => FLT5F_A::DIV4_N8,
            8 => FLT5F_A::DIV8_N6,
            9 => FLT5F_A::DIV8_N8,
            10 => FLT5F_A::DIV16_N5,
            11 => FLT5F_A::DIV16_N6,
            12 => FLT5F_A::DIV16_N8,
            13 => FLT5F_A::DIV32_N5,
            14 => FLT5F_A::DIV32_N6,
            15 => FLT5F_A::DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5F_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIV1_N2`"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT5F_A::DIV1_N2
    }
    #[doc = "Checks if the value of the field is `DIV1_N4`"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT5F_A::DIV1_N4
    }
    #[doc = "Checks if the value of the field is `DIV1_N8`"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT5F_A::DIV1_N8
    }
    #[doc = "Checks if the value of the field is `DIV2_N6`"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT5F_A::DIV2_N6
    }
    #[doc = "Checks if the value of the field is `DIV2_N8`"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT5F_A::DIV2_N8
    }
    #[doc = "Checks if the value of the field is `DIV4_N6`"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT5F_A::DIV4_N6
    }
    #[doc = "Checks if the value of the field is `DIV4_N8`"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT5F_A::DIV4_N8
    }
    #[doc = "Checks if the value of the field is `DIV8_N6`"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT5F_A::DIV8_N6
    }
    #[doc = "Checks if the value of the field is `DIV8_N8`"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT5F_A::DIV8_N8
    }
    #[doc = "Checks if the value of the field is `DIV16_N5`"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT5F_A::DIV16_N5
    }
    #[doc = "Checks if the value of the field is `DIV16_N6`"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT5F_A::DIV16_N6
    }
    #[doc = "Checks if the value of the field is `DIV16_N8`"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT5F_A::DIV16_N8
    }
    #[doc = "Checks if the value of the field is `DIV32_N5`"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT5F_A::DIV32_N5
    }
    #[doc = "Checks if the value of the field is `DIV32_N6`"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT5F_A::DIV32_N6
    }
    #[doc = "Checks if the value of the field is `DIV32_N8`"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT5F_A::DIV32_N8
    }
}
#[doc = "Write proxy for field `FLT5F`"]
pub struct FLT5F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5F_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT5F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT5F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "FLT5SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5SRC_A {
    #[doc = "0: Fault input is FLTx input pin"]
    INPUT = 0,
    #[doc = "1: Fault input is FLTn_Int signal"]
    INTERNAL = 1,
}
impl From<FLT5SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT5SRC`"]
pub type FLT5SRC_R = crate::R<bool, FLT5SRC_A>;
impl FLT5SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5SRC_A {
        match self.bits {
            false => FLT5SRC_A::INPUT,
            true => FLT5SRC_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT5SRC_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT5SRC_A::INTERNAL
    }
}
#[doc = "Write proxy for field `FLT5SRC`"]
pub struct FLT5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT5SRC_A::INPUT)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT5SRC_A::INTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "FLT5P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5P_A {
    #[doc = "0: Fault input is active low"]
    ACTIVELOW = 0,
    #[doc = "1: Fault input is active high"]
    ACTIVEHIGH = 1,
}
impl From<FLT5P_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT5P`"]
pub type FLT5P_R = crate::R<bool, FLT5P_A>;
impl FLT5P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5P_A {
        match self.bits {
            false => FLT5P_A::ACTIVELOW,
            true => FLT5P_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT5P_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT5P_A::ACTIVEHIGH
    }
}
#[doc = "Write proxy for field `FLT5P`"]
pub struct FLT5P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT5P_A::ACTIVELOW)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT5P_A::ACTIVEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "FLT5E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5E_A {
    #[doc = "0: Fault input disabled"]
    DISABLED = 0,
    #[doc = "1: Fault input enabled"]
    ENABLED = 1,
}
impl From<FLT5E_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT5E`"]
pub type FLT5E_R = crate::R<bool, FLT5E_A>;
impl FLT5E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5E_A {
        match self.bits {
            false => FLT5E_A::DISABLED,
            true => FLT5E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT5E_A::ENABLED
    }
}
#[doc = "Write proxy for field `FLT5E`"]
pub struct FLT5E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT5E_A::DISABLED)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT5E_A::ENABLED)
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
impl R {
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&mut self) -> FLTSD_W {
        FLTSD_W { w: self }
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&mut self) -> FLT5LCK_W {
        FLT5LCK_W { w: self }
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&mut self) -> FLT5F_W {
        FLT5F_W { w: self }
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&mut self) -> FLT5SRC_W {
        FLT5SRC_W { w: self }
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&mut self) -> FLT5P_W {
        FLT5P_W { w: self }
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&mut self) -> FLT5E_W {
        FLT5E_W { w: self }
    }
}
