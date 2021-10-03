#[doc = "Register `PR1` reader"]
pub struct R(crate::R<PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR1` writer"]
pub struct W(crate::W<PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1_SPEC>;
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
impl From<crate::W<PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configurable event inputs Pending bit"]
pub type PIF21_A = PIF0_A;
#[doc = "Field `PIF21` reader - Configurable event inputs Pending bit"]
pub type PIF21_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF21_AW = PIF0_AW;
#[doc = "Field `PIF21` writer - Configurable event inputs Pending bit"]
pub struct PIF21_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF21_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF21_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF22_A = PIF0_A;
#[doc = "Field `PIF22` reader - Configurable event inputs Pending bit"]
pub type PIF22_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF22_AW = PIF0_AW;
#[doc = "Field `PIF22` writer - Configurable event inputs Pending bit"]
pub struct PIF22_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF22_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF22_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF0_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PIF0_A> for bool {
    #[inline(always)]
    fn from(variant: PIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF0` reader - Configurable event inputs Pending bit"]
pub struct PIF0_R(crate::FieldReader<bool, PIF0_A>);
impl PIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF0_A {
        match self.bits {
            false => PIF0_A::NOTPENDING,
            true => PIF0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == PIF0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == PIF0_A::PENDING
    }
}
impl core::ops::Deref for PIF0_R {
    type Target = crate::FieldReader<bool, PIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Configurable event inputs Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF0_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF0` writer - Configurable event inputs Pending bit"]
pub struct PIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF0_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF1_A = PIF0_A;
#[doc = "Field `PIF1` reader - Configurable event inputs Pending bit"]
pub type PIF1_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF1_AW = PIF0_AW;
#[doc = "Field `PIF1` writer - Configurable event inputs Pending bit"]
pub struct PIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF1_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF2_A = PIF0_A;
#[doc = "Field `PIF2` reader - Configurable event inputs Pending bit"]
pub type PIF2_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF2_AW = PIF0_AW;
#[doc = "Field `PIF2` writer - Configurable event inputs Pending bit"]
pub struct PIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF2_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF3_A = PIF0_A;
#[doc = "Field `PIF3` reader - Configurable event inputs Pending bit"]
pub type PIF3_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF3_AW = PIF0_AW;
#[doc = "Field `PIF3` writer - Configurable event inputs Pending bit"]
pub struct PIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF3_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF4_A = PIF0_A;
#[doc = "Field `PIF4` reader - Configurable event inputs Pending bit"]
pub type PIF4_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF4_AW = PIF0_AW;
#[doc = "Field `PIF4` writer - Configurable event inputs Pending bit"]
pub struct PIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF4_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF5_A = PIF0_A;
#[doc = "Field `PIF5` reader - Configurable event inputs Pending bit"]
pub type PIF5_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF5_AW = PIF0_AW;
#[doc = "Field `PIF5` writer - Configurable event inputs Pending bit"]
pub struct PIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF5_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF6_A = PIF0_A;
#[doc = "Field `PIF6` reader - Configurable event inputs Pending bit"]
pub type PIF6_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF6_AW = PIF0_AW;
#[doc = "Field `PIF6` writer - Configurable event inputs Pending bit"]
pub struct PIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF6_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF7_A = PIF0_A;
#[doc = "Field `PIF7` reader - Configurable event inputs Pending bit"]
pub type PIF7_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF7_AW = PIF0_AW;
#[doc = "Field `PIF7` writer - Configurable event inputs Pending bit"]
pub struct PIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Configurable event inputs Pending bit"]
pub type PIF8_A = PIF0_A;
#[doc = "Field `PIF8` reader - Configurable event inputs Pending bit"]
pub type PIF8_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF8_AW = PIF0_AW;
#[doc = "Field `PIF8` writer - Configurable event inputs Pending bit"]
pub struct PIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF8_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF9_A = PIF0_A;
#[doc = "Field `PIF9` reader - Configurable event inputs Pending bit"]
pub type PIF9_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF9_AW = PIF0_AW;
#[doc = "Field `PIF9` writer - Configurable event inputs Pending bit"]
pub struct PIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF9_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF10_A = PIF0_A;
#[doc = "Field `PIF10` reader - Configurable event inputs Pending bit"]
pub type PIF10_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF10_AW = PIF0_AW;
#[doc = "Field `PIF10` writer - Configurable event inputs Pending bit"]
pub struct PIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF10_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF11_A = PIF0_A;
#[doc = "Field `PIF11` reader - Configurable event inputs Pending bit"]
pub type PIF11_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF11_AW = PIF0_AW;
#[doc = "Field `PIF11` writer - Configurable event inputs Pending bit"]
pub struct PIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF11_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF12_A = PIF0_A;
#[doc = "Field `PIF12` reader - Configurable event inputs Pending bit"]
pub type PIF12_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF12_AW = PIF0_AW;
#[doc = "Field `PIF12` writer - Configurable event inputs Pending bit"]
pub struct PIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF12_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF13_A = PIF0_A;
#[doc = "Field `PIF13` reader - Configurable event inputs Pending bit"]
pub type PIF13_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF13_AW = PIF0_AW;
#[doc = "Field `PIF13` writer - Configurable event inputs Pending bit"]
pub struct PIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF13_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF14_A = PIF0_A;
#[doc = "Field `PIF14` reader - Configurable event inputs Pending bit"]
pub type PIF14_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF14_AW = PIF0_AW;
#[doc = "Field `PIF14` writer - Configurable event inputs Pending bit"]
pub struct PIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF14_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF15_A = PIF0_A;
#[doc = "Field `PIF15` reader - Configurable event inputs Pending bit"]
pub type PIF15_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF15_AW = PIF0_AW;
#[doc = "Field `PIF15` writer - Configurable event inputs Pending bit"]
pub struct PIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF15_AW::CLEAR)
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
#[doc = "Configurable event inputs Pending bit"]
pub type PIF16_A = PIF0_A;
#[doc = "Field `PIF16` reader - Configurable event inputs Pending bit"]
pub type PIF16_R = PIF0_R;
#[doc = "Configurable event inputs Pending bit"]
pub type PIF16_AW = PIF0_AW;
#[doc = "Field `PIF16` writer - Configurable event inputs Pending bit"]
pub struct PIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF16_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF16_AW::CLEAR)
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
impl R {
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W {
        PIF21_W { w: self }
    }
    #[doc = "Bit 22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif22(&mut self) -> PIF22_W {
        PIF22_W { w: self }
    }
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W {
        PIF0_W { w: self }
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&mut self) -> PIF1_W {
        PIF1_W { w: self }
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W {
        PIF2_W { w: self }
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&mut self) -> PIF3_W {
        PIF3_W { w: self }
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W {
        PIF4_W { w: self }
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&mut self) -> PIF5_W {
        PIF5_W { w: self }
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&mut self) -> PIF6_W {
        PIF6_W { w: self }
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&mut self) -> PIF7_W {
        PIF7_W { w: self }
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&mut self) -> PIF8_W {
        PIF8_W { w: self }
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&mut self) -> PIF9_W {
        PIF9_W { w: self }
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&mut self) -> PIF10_W {
        PIF10_W { w: self }
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&mut self) -> PIF11_W {
        PIF11_W { w: self }
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&mut self) -> PIF12_W {
        PIF12_W { w: self }
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&mut self) -> PIF13_W {
        PIF13_W { w: self }
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&mut self) -> PIF14_W {
        PIF14_W { w: self }
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&mut self) -> PIF15_W {
        PIF15_W { w: self }
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&mut self) -> PIF16_W {
        PIF16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](index.html) module"]
pub struct PR1_SPEC;
impl crate::RegisterSpec for PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr1::R](R) reader structure"]
impl crate::Readable for PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr1::W](W) writer structure"]
impl crate::Writable for PR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
