#[doc = "Register `PUCRC` reader"]
pub struct R(crate::R<PUCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRC` writer"]
pub struct W(crate::W<PUCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRC_SPEC>;
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
impl From<crate::W<PUCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub type PU15_A = PU13_A;
#[doc = "Field `PU15` reader - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub type PU15_R = PU13_R;
#[doc = "Field `PU15` writer - Port PC\\[y\\]
pull-up (y=13 to 15)"]
pub struct PU15_W<'a> {
    w: &'a mut W,
}
impl<'a> PU15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU15_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU15_A::ENABLED)
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
#[doc = "PU14"]
pub type PU14_A = PU13_A;
#[doc = "Field `PU14` reader - PU14"]
pub type PU14_R = PU13_R;
#[doc = "Field `PU14` writer - PU14"]
pub struct PU14_W<'a> {
    w: &'a mut W,
}
impl<'a> PU14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU14_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU14_A::ENABLED)
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
#[doc = "PU13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PU13_A {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    DISABLED = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    ENABLED = 1,
}
impl From<PU13_A> for bool {
    #[inline(always)]
    fn from(variant: PU13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU13` reader - PU13"]
pub struct PU13_R(crate::FieldReader<bool, PU13_A>);
impl PU13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU13_A {
        match self.bits {
            false => PU13_A::DISABLED,
            true => PU13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PU13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PU13_A::ENABLED
    }
}
impl core::ops::Deref for PU13_R {
    type Target = crate::FieldReader<bool, PU13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU13` writer - PU13"]
pub struct PU13_W<'a> {
    w: &'a mut W,
}
impl<'a> PU13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU13_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU13_A::ENABLED)
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
#[doc = "PU2"]
pub type PU2_A = PU0_A;
#[doc = "Field `PU2` reader - PU2"]
pub type PU2_R = PU0_R;
#[doc = "Field `PU2` writer - PU2"]
pub struct PU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PU2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU2_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU2_A::ENABLED)
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
#[doc = "PU1"]
pub type PU1_A = PU0_A;
#[doc = "Field `PU1` reader - PU1"]
pub type PU1_R = PU0_R;
#[doc = "Field `PU1` writer - PU1"]
pub struct PU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU1_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU1_A::ENABLED)
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
#[doc = "PU0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PU0_A {
    #[doc = "0: Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    DISABLED = 0,
    #[doc = "1: Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    ENABLED = 1,
}
impl From<PU0_A> for bool {
    #[inline(always)]
    fn from(variant: PU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU0` reader - PU0"]
pub struct PU0_R(crate::FieldReader<bool, PU0_A>);
impl PU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU0_A {
        match self.bits {
            false => PU0_A::DISABLED,
            true => PU0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PU0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PU0_A::ENABLED
    }
}
impl core::ops::Deref for PU0_R {
    type Target = crate::FieldReader<bool, PU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU0` writer - PU0"]
pub struct PU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU0_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU0_A::ENABLED)
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
#[doc = "PU3"]
pub type PU3_A = PU0_A;
#[doc = "Field `PU3` reader - PU3"]
pub type PU3_R = PU0_R;
#[doc = "Field `PU3` writer - PU3"]
pub struct PU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PU3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU3_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU3_A::ENABLED)
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
#[doc = "PU4"]
pub type PU4_A = PU0_A;
#[doc = "Field `PU4` reader - PU4"]
pub type PU4_R = PU0_R;
#[doc = "Field `PU4` writer - PU4"]
pub struct PU4_W<'a> {
    w: &'a mut W,
}
impl<'a> PU4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU4_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU4_A::ENABLED)
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
#[doc = "PU5"]
pub type PU5_A = PU0_A;
#[doc = "Field `PU5` reader - PU5"]
pub type PU5_R = PU0_R;
#[doc = "Field `PU5` writer - PU5"]
pub struct PU5_W<'a> {
    w: &'a mut W,
}
impl<'a> PU5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU5_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU5_A::ENABLED)
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
#[doc = "PU6"]
pub type PU6_A = PU0_A;
#[doc = "Field `PU6` reader - PU6"]
pub type PU6_R = PU0_R;
#[doc = "Field `PU6` writer - PU6"]
pub struct PU6_W<'a> {
    w: &'a mut W,
}
impl<'a> PU6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU6_A::DISABLED)
    }
    #[doc = "Enable pull-up on PC\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU6_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port PC\\[y\\]
pull-up (y=13 to 15)"]
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W {
        PU15_W { w: self }
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W {
        PU14_W { w: self }
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W {
        PU13_W { w: self }
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W {
        PU2_W { w: self }
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W {
        PU1_W { w: self }
    }
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W {
        PU0_W { w: self }
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W {
        PU3_W { w: self }
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W {
        PU4_W { w: self }
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W {
        PU5_W { w: self }
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W {
        PU6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port C pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrc](index.html) module"]
pub struct PUCRC_SPEC;
impl crate::RegisterSpec for PUCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrc::R](R) reader structure"]
impl crate::Readable for PUCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrc::W](W) writer structure"]
impl crate::Writable for PUCRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRC to value 0"]
impl crate::Resettable for PUCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
