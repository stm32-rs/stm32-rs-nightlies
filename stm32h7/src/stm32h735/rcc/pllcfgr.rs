#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLL1 fractional latch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL1FRACEN_A {
    #[doc = "0: Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    RESET = 0,
    #[doc = "1: Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    SET = 1,
}
impl From<PLL1FRACEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable"]
pub struct PLL1FRACEN_R(crate::FieldReader<bool, PLL1FRACEN_A>);
impl PLL1FRACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1FRACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1FRACEN_A {
        match self.bits {
            false => PLL1FRACEN_A::RESET,
            true => PLL1FRACEN_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == PLL1FRACEN_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == PLL1FRACEN_A::SET
    }
}
impl core::ops::Deref for PLL1FRACEN_R {
    type Target = crate::FieldReader<bool, PLL1FRACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable"]
pub struct PLL1FRACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1FRACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1FRACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PLL1FRACEN_A::RESET)
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PLL1FRACEN_A::SET)
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
#[doc = "PLL1 VCO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL1VCOSEL_A {
    #[doc = "0: VCO frequency range 192 to 836 MHz"]
    WIDEVCO = 0,
    #[doc = "1: VCO frequency range 150 to 420 MHz"]
    MEDIUMVCO = 1,
}
impl From<PLL1VCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1VCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection"]
pub struct PLL1VCOSEL_R(crate::FieldReader<bool, PLL1VCOSEL_A>);
impl PLL1VCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1VCOSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1VCOSEL_A {
        match self.bits {
            false => PLL1VCOSEL_A::WIDEVCO,
            true => PLL1VCOSEL_A::MEDIUMVCO,
        }
    }
    #[doc = "Checks if the value of the field is `WIDEVCO`"]
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        **self == PLL1VCOSEL_A::WIDEVCO
    }
    #[doc = "Checks if the value of the field is `MEDIUMVCO`"]
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        **self == PLL1VCOSEL_A::MEDIUMVCO
    }
}
impl core::ops::Deref for PLL1VCOSEL_R {
    type Target = crate::FieldReader<bool, PLL1VCOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection"]
pub struct PLL1VCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1VCOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1VCOSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut W {
        self.variant(PLL1VCOSEL_A::WIDEVCO)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut W {
        self.variant(PLL1VCOSEL_A::MEDIUMVCO)
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
#[doc = "PLL1 input frequency range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL1RGE_A {
    #[doc = "0: Frequency is between 1 and 2 MHz"]
    RANGE1 = 0,
    #[doc = "1: Frequency is between 2 and 4 MHz"]
    RANGE2 = 1,
    #[doc = "2: Frequency is between 4 and 8 MHz"]
    RANGE4 = 2,
    #[doc = "3: Frequency is between 8 and 16 MHz"]
    RANGE8 = 3,
}
impl From<PLL1RGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1RGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range"]
pub struct PLL1RGE_R(crate::FieldReader<u8, PLL1RGE_A>);
impl PLL1RGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL1RGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1RGE_A {
        match self.bits {
            0 => PLL1RGE_A::RANGE1,
            1 => PLL1RGE_A::RANGE2,
            2 => PLL1RGE_A::RANGE4,
            3 => PLL1RGE_A::RANGE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        **self == PLL1RGE_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        **self == PLL1RGE_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        **self == PLL1RGE_A::RANGE4
    }
    #[doc = "Checks if the value of the field is `RANGE8`"]
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        **self == PLL1RGE_A::RANGE8
    }
}
impl core::ops::Deref for PLL1RGE_R {
    type Target = crate::FieldReader<u8, PLL1RGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range"]
pub struct PLL1RGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1RGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1RGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(PLL1RGE_A::RANGE1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(PLL1RGE_A::RANGE2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(PLL1RGE_A::RANGE4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut W {
        self.variant(PLL1RGE_A::RANGE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "PLL2 fractional latch enable"]
pub type PLL2FRACEN_A = PLL1FRACEN_A;
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable"]
pub type PLL2FRACEN_R = PLL1FRACEN_R;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable"]
pub struct PLL2FRACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2FRACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2FRACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PLL2FRACEN_A::RESET)
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PLL2FRACEN_A::SET)
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
#[doc = "PLL2 VCO selection"]
pub type PLL2VCOSEL_A = PLL1VCOSEL_A;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection"]
pub type PLL2VCOSEL_R = PLL1VCOSEL_R;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection"]
pub struct PLL2VCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2VCOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2VCOSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut W {
        self.variant(PLL2VCOSEL_A::WIDEVCO)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut W {
        self.variant(PLL2VCOSEL_A::MEDIUMVCO)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL2 input frequency range"]
pub type PLL2RGE_A = PLL1RGE_A;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range"]
pub type PLL2RGE_R = PLL1RGE_R;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range"]
pub struct PLL2RGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2RGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2RGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(PLL2RGE_A::RANGE1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(PLL2RGE_A::RANGE2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(PLL2RGE_A::RANGE4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut W {
        self.variant(PLL2RGE_A::RANGE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "PLL3 fractional latch enable"]
pub type PLL3FRACEN_A = PLL1FRACEN_A;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable"]
pub type PLL3FRACEN_R = PLL1FRACEN_R;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable"]
pub struct PLL3FRACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3FRACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3FRACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PLL3FRACEN_A::RESET)
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PLL3FRACEN_A::SET)
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
#[doc = "PLL3 VCO selection"]
pub type PLL3VCOSEL_A = PLL1VCOSEL_A;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection"]
pub type PLL3VCOSEL_R = PLL1VCOSEL_R;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection"]
pub struct PLL3VCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3VCOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3VCOSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut W {
        self.variant(PLL3VCOSEL_A::WIDEVCO)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut W {
        self.variant(PLL3VCOSEL_A::MEDIUMVCO)
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
#[doc = "PLL3 input frequency range"]
pub type PLL3RGE_A = PLL1RGE_A;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range"]
pub type PLL3RGE_R = PLL1RGE_R;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range"]
pub struct PLL3RGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3RGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3RGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(PLL3RGE_A::RANGE1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(PLL3RGE_A::RANGE2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(PLL3RGE_A::RANGE4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut W {
        self.variant(PLL3RGE_A::RANGE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "PLL1 DIVP divider output enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVP1EN_A {
    #[doc = "0: Clock ouput is disabled"]
    DISABLED = 0,
    #[doc = "1: Clock output is enabled"]
    ENABLED = 1,
}
impl From<DIVP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVP1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVP1EN` reader - PLL1 DIVP divider output enable"]
pub struct DIVP1EN_R(crate::FieldReader<bool, DIVP1EN_A>);
impl DIVP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVP1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVP1EN_A {
        match self.bits {
            false => DIVP1EN_A::DISABLED,
            true => DIVP1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DIVP1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DIVP1EN_A::ENABLED
    }
}
impl core::ops::Deref for DIVP1EN_R {
    type Target = crate::FieldReader<bool, DIVP1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVP1EN` writer - PLL1 DIVP divider output enable"]
pub struct DIVP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVP1EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVP1EN_A::ENABLED)
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
#[doc = "PLL1 DIVQ divider output enable"]
pub type DIVQ1EN_A = DIVP1EN_A;
#[doc = "Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable"]
pub type DIVQ1EN_R = DIVP1EN_R;
#[doc = "Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable"]
pub struct DIVQ1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQ1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVQ1EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVQ1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "PLL1 DIVR divider output enable"]
pub type DIVR1EN_A = DIVP1EN_A;
#[doc = "Field `DIVR1EN` reader - PLL1 DIVR divider output enable"]
pub type DIVR1EN_R = DIVP1EN_R;
#[doc = "Field `DIVR1EN` writer - PLL1 DIVR divider output enable"]
pub struct DIVR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVR1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVR1EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVR1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "PLL2 DIVP divider output enable"]
pub type DIVP2EN_A = DIVP1EN_A;
#[doc = "Field `DIVP2EN` reader - PLL2 DIVP divider output enable"]
pub type DIVP2EN_R = DIVP1EN_R;
#[doc = "Field `DIVP2EN` writer - PLL2 DIVP divider output enable"]
pub struct DIVP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVP2EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVP2EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "PLL2 DIVQ divider output enable"]
pub type DIVQ2EN_A = DIVP1EN_A;
#[doc = "Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable"]
pub type DIVQ2EN_R = DIVP1EN_R;
#[doc = "Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable"]
pub struct DIVQ2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQ2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVQ2EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVQ2EN_A::ENABLED)
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
#[doc = "PLL2 DIVR divider output enable"]
pub type DIVR2EN_A = DIVP1EN_A;
#[doc = "Field `DIVR2EN` reader - PLL2 DIVR divider output enable"]
pub type DIVR2EN_R = DIVP1EN_R;
#[doc = "Field `DIVR2EN` writer - PLL2 DIVR divider output enable"]
pub struct DIVR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVR2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVR2EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVR2EN_A::ENABLED)
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
#[doc = "PLL3 DIVP divider output enable"]
pub type DIVP3EN_A = DIVP1EN_A;
#[doc = "Field `DIVP3EN` reader - PLL3 DIVP divider output enable"]
pub type DIVP3EN_R = DIVP1EN_R;
#[doc = "Field `DIVP3EN` writer - PLL3 DIVP divider output enable"]
pub struct DIVP3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVP3EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVP3EN_A::ENABLED)
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
#[doc = "PLL3 DIVQ divider output enable"]
pub type DIVQ3EN_A = DIVP1EN_A;
#[doc = "Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable"]
pub type DIVQ3EN_R = DIVP1EN_R;
#[doc = "Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable"]
pub struct DIVQ3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQ3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVQ3EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVQ3EN_A::ENABLED)
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
#[doc = "PLL3 DIVR divider output enable"]
pub type DIVR3EN_A = DIVP1EN_A;
#[doc = "Field `DIVR3EN` reader - PLL3 DIVR divider output enable"]
pub type DIVR3EN_R = DIVP1EN_R;
#[doc = "Field `DIVR3EN` writer - PLL3 DIVR divider output enable"]
pub struct DIVR3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVR3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVR3EN_A::DISABLED)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVR3EN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W {
        PLL1FRACEN_W { w: self }
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W {
        PLL1VCOSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W {
        PLL1RGE_W { w: self }
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W {
        PLL2FRACEN_W { w: self }
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W {
        PLL2VCOSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&mut self) -> PLL2RGE_W {
        PLL2RGE_W { w: self }
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W {
        PLL3FRACEN_W { w: self }
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W {
        PLL3VCOSEL_W { w: self }
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&mut self) -> PLL3RGE_W {
        PLL3RGE_W { w: self }
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&mut self) -> DIVP1EN_W {
        DIVP1EN_W { w: self }
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&mut self) -> DIVQ1EN_W {
        DIVQ1EN_W { w: self }
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&mut self) -> DIVR1EN_W {
        DIVR1EN_W { w: self }
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&mut self) -> DIVP2EN_W {
        DIVP2EN_W { w: self }
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&mut self) -> DIVQ2EN_W {
        DIVQ2EN_W { w: self }
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&mut self) -> DIVR2EN_W {
        DIVR2EN_W { w: self }
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&mut self) -> DIVP3EN_W {
        DIVP3EN_W { w: self }
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&mut self) -> DIVQ3EN_W {
        DIVQ3EN_W { w: self }
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&mut self) -> DIVR3EN_W {
        DIVR3EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLLs Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x01ff_0000"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff_0000
    }
}
