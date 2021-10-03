#[doc = "Register `AF2` reader"]
pub struct R(crate::R<AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF2` writer"]
pub struct W(crate::W<AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF2_SPEC>;
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
impl From<crate::W<AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INE_A {
    #[doc = "0: BKIN2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input enabled"]
    B_0X1 = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2INE_R(crate::FieldReader<bool, BK2INE_A>);
impl BK2INE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::B_0X0,
            true => BK2INE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2INE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2INE_A::B_0X1
    }
}
impl core::ops::Deref for BK2INE_R {
    type Target = crate::FieldReader<bool, BK2INE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2INE_A::B_0X0)
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2INE_A::B_0X1)
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
#[doc = "BRK2 COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK2 input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input enabled"]
    B_0X1 = 1,
}
impl From<BK2CMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK2 input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP1E_R(crate::FieldReader<bool, BK2CMP1E_A>);
impl BK2CMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1E_A {
        match self.bits {
            false => BK2CMP1E_A::B_0X0,
            true => BK2CMP1E_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2CMP1E_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2CMP1E_A::B_0X1
    }
}
impl core::ops::Deref for BK2CMP1E_R {
    type Target = crate::FieldReader<bool, BK2CMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK2 input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::B_0X0)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::B_0X1)
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
#[doc = "BRK2 COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK2 input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input enabled"]
    B_0X1 = 1,
}
impl From<BK2CMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK2 input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP2E_R(crate::FieldReader<bool, BK2CMP2E_A>);
impl BK2CMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2E_A {
        match self.bits {
            false => BK2CMP2E_A::B_0X0,
            true => BK2CMP2E_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2CMP2E_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2CMP2E_A::B_0X1
    }
}
impl core::ops::Deref for BK2CMP2E_R {
    type Target = crate::FieldReader<bool, BK2CMP2E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK2 input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::B_0X0)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::B_0X1)
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
#[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INP_A {
    #[doc = "0: BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2INP_R(crate::FieldReader<bool, BK2INP_A>);
impl BK2INP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::B_0X0,
            true => BK2INP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2INP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2INP_A::B_0X1
    }
}
impl core::ops::Deref for BK2INP_R {
    type Target = crate::FieldReader<bool, BK2INP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2INP_A::B_0X0)
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2INP_A::B_0X1)
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
#[doc = "BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP1P_A {
    #[doc = "0: COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2CMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP1P_R(crate::FieldReader<bool, BK2CMP1P_A>);
impl BK2CMP1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1P_A {
        match self.bits {
            false => BK2CMP1P_A::B_0X0,
            true => BK2CMP1P_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2CMP1P_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2CMP1P_A::B_0X1
    }
}
impl core::ops::Deref for BK2CMP1P_R {
    type Target = crate::FieldReader<bool, BK2CMP1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::B_0X0)
    }
    #[doc = "COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP2P_A {
    #[doc = "0: COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2CMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP2P_R(crate::FieldReader<bool, BK2CMP2P_A>);
impl BK2CMP2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2P_A {
        match self.bits {
            false => BK2CMP2P_A::B_0X0,
            true => BK2CMP2P_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BK2CMP2P_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BK2CMP2P_A::B_0X1
    }
}
impl core::ops::Deref for BK2CMP2P_R {
    type Target = crate::FieldReader<bool, BK2CMP2P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub struct BK2CMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::B_0X0)
    }
    #[doc = "COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK2 input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK2 input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK2 input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W {
        BK2CMP1E_W { w: self }
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK2 input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W {
        BK2CMP2E_W { w: self }
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W {
        BK2CMP1P_W { w: self }
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W {
        BK2CMP2P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af2::R](R) reader structure"]
impl crate::Readable for AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af2::W](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
