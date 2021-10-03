#[doc = "Register `C2EMR2` reader"]
pub struct R(crate::R<C2EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2EMR2` writer"]
pub struct W(crate::W<C2EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2EMR2_SPEC>;
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
impl From<crate::W<C2EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR32_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR32_A> for bool {
    #[inline(always)]
    fn from(variant: MR32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR32` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR32_R(crate::FieldReader<bool, MR32_A>);
impl MR32_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR32_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR32_A {
        match self.bits {
            false => MR32_A::MASKED,
            true => MR32_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == MR32_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == MR32_A::UNMASKED
    }
}
impl core::ops::Deref for MR32_R {
    type Target = crate::FieldReader<bool, MR32_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR32` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR32_W<'a> {
    w: &'a mut W,
}
impl<'a> MR32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR32_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR32_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR32_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR33_A = MR32_A;
#[doc = "Field `MR33` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR33_R = MR32_R;
#[doc = "Field `MR33` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR33_W<'a> {
    w: &'a mut W,
}
impl<'a> MR33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR33_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR33_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR33_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR34_A = MR32_A;
#[doc = "Field `MR34` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR34_R = MR32_R;
#[doc = "Field `MR34` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR34_W<'a> {
    w: &'a mut W,
}
impl<'a> MR34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR34_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR34_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR35_A = MR32_A;
#[doc = "Field `MR35` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR35_R = MR32_R;
#[doc = "Field `MR35` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR35_W<'a> {
    w: &'a mut W,
}
impl<'a> MR35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR35_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR35_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR35_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR36_A = MR32_A;
#[doc = "Field `MR36` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR36_R = MR32_R;
#[doc = "Field `MR36` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR36_W<'a> {
    w: &'a mut W,
}
impl<'a> MR36_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR36_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR36_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR36_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR37_A = MR32_A;
#[doc = "Field `MR37` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR37_R = MR32_R;
#[doc = "Field `MR37` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR37_W<'a> {
    w: &'a mut W,
}
impl<'a> MR37_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR37_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR37_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR37_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR38_A = MR32_A;
#[doc = "Field `MR38` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR38_R = MR32_R;
#[doc = "Field `MR38` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR38_W<'a> {
    w: &'a mut W,
}
impl<'a> MR38_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR38_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR38_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR38_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR39_A = MR32_A;
#[doc = "Field `MR39` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR39_R = MR32_R;
#[doc = "Field `MR39` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR39_W<'a> {
    w: &'a mut W,
}
impl<'a> MR39_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR39_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR39_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR39_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR40_A = MR32_A;
#[doc = "Field `MR40` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR40_R = MR32_R;
#[doc = "Field `MR40` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR40_W<'a> {
    w: &'a mut W,
}
impl<'a> MR40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR40_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR40_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR41_A = MR32_A;
#[doc = "Field `MR41` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR41_R = MR32_R;
#[doc = "Field `MR41` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR41_W<'a> {
    w: &'a mut W,
}
impl<'a> MR41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR41_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR41_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR42_A = MR32_A;
#[doc = "Field `MR42` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR42_R = MR32_R;
#[doc = "Field `MR42` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR42_W<'a> {
    w: &'a mut W,
}
impl<'a> MR42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR42_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR42_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR42_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR43_A = MR32_A;
#[doc = "Field `MR43` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR43_R = MR32_R;
#[doc = "Field `MR43` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR43_W<'a> {
    w: &'a mut W,
}
impl<'a> MR43_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR43_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR43_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR43_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR44_A = MR32_A;
#[doc = "Field `MR44` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR44_R = MR32_R;
#[doc = "Field `MR44` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR44_W<'a> {
    w: &'a mut W,
}
impl<'a> MR44_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR44_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR44_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR44_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR46_A = MR32_A;
#[doc = "Field `MR46` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR46_R = MR32_R;
#[doc = "Field `MR46` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR46_W<'a> {
    w: &'a mut W,
}
impl<'a> MR46_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR46_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR46_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR46_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR47_A = MR32_A;
#[doc = "Field `MR47` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR47_R = MR32_R;
#[doc = "Field `MR47` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR47_W<'a> {
    w: &'a mut W,
}
impl<'a> MR47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR47_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR47_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR47_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR48_A = MR32_A;
#[doc = "Field `MR48` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR48_R = MR32_R;
#[doc = "Field `MR48` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR48_W<'a> {
    w: &'a mut W,
}
impl<'a> MR48_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR48_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR48_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR48_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR49_A = MR32_A;
#[doc = "Field `MR49` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR49_R = MR32_R;
#[doc = "Field `MR49` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR49_W<'a> {
    w: &'a mut W,
}
impl<'a> MR49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR49_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR49_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR49_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR50_A = MR32_A;
#[doc = "Field `MR50` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR50_R = MR32_R;
#[doc = "Field `MR50` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR50_W<'a> {
    w: &'a mut W,
}
impl<'a> MR50_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR50_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR50_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR50_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR51_A = MR32_A;
#[doc = "Field `MR51` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR51_R = MR32_R;
#[doc = "Field `MR51` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR51_W<'a> {
    w: &'a mut W,
}
impl<'a> MR51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR51_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR51_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR51_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR52_A = MR32_A;
#[doc = "Field `MR52` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR52_R = MR32_R;
#[doc = "Field `MR52` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR52_W<'a> {
    w: &'a mut W,
}
impl<'a> MR52_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR52_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR52_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR52_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR53_A = MR32_A;
#[doc = "Field `MR53` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR53_R = MR32_R;
#[doc = "Field `MR53` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR53_W<'a> {
    w: &'a mut W,
}
impl<'a> MR53_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR53_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR53_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR53_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR54_A = MR32_A;
#[doc = "Field `MR54` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR54_R = MR32_R;
#[doc = "Field `MR54` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR54_W<'a> {
    w: &'a mut W,
}
impl<'a> MR54_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR54_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR54_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR54_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR55_A = MR32_A;
#[doc = "Field `MR55` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR55_R = MR32_R;
#[doc = "Field `MR55` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR55_W<'a> {
    w: &'a mut W,
}
impl<'a> MR55_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR55_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR55_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR55_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR56_A = MR32_A;
#[doc = "Field `MR56` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR56_R = MR32_R;
#[doc = "Field `MR56` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR56_W<'a> {
    w: &'a mut W,
}
impl<'a> MR56_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR56_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR56_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR56_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR57_A = MR32_A;
#[doc = "Field `MR57` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR57_R = MR32_R;
#[doc = "Field `MR57` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR57_W<'a> {
    w: &'a mut W,
}
impl<'a> MR57_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR57_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR57_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR57_A::UNMASKED)
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
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR58_A = MR32_A;
#[doc = "Field `MR58` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR58_R = MR32_R;
#[doc = "Field `MR58` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR58_W<'a> {
    w: &'a mut W,
}
impl<'a> MR58_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR58_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR58_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR58_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR59_A = MR32_A;
#[doc = "Field `MR59` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR59_R = MR32_R;
#[doc = "Field `MR59` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR59_W<'a> {
    w: &'a mut W,
}
impl<'a> MR59_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR59_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR59_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR59_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR60_A = MR32_A;
#[doc = "Field `MR60` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR60_R = MR32_R;
#[doc = "Field `MR60` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR60_W<'a> {
    w: &'a mut W,
}
impl<'a> MR60_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR60_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR60_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR60_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR61_A = MR32_A;
#[doc = "Field `MR61` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR61_R = MR32_R;
#[doc = "Field `MR61` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR61_W<'a> {
    w: &'a mut W,
}
impl<'a> MR61_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR61_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR61_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR61_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR62_A = MR32_A;
#[doc = "Field `MR62` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR62_R = MR32_R;
#[doc = "Field `MR62` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR62_W<'a> {
    w: &'a mut W,
}
impl<'a> MR62_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR62_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR62_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR62_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR63_A = MR32_A;
#[doc = "Field `MR63` reader - CPU2 interrupt Mask on Direct Event input x+32"]
pub type MR63_R = MR32_R;
#[doc = "Field `MR63` writer - CPU2 interrupt Mask on Direct Event input x+32"]
pub struct MR63_W<'a> {
    w: &'a mut W,
}
impl<'a> MR63_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR63_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR63_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR63_A::UNMASKED)
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
impl R {
    #[doc = "Bit 0 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&mut self) -> MR32_W {
        MR32_W { w: self }
    }
    #[doc = "Bit 1 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&mut self) -> MR33_W {
        MR33_W { w: self }
    }
    #[doc = "Bit 2 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W {
        MR34_W { w: self }
    }
    #[doc = "Bit 3 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W {
        MR35_W { w: self }
    }
    #[doc = "Bit 4 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&mut self) -> MR36_W {
        MR36_W { w: self }
    }
    #[doc = "Bit 5 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&mut self) -> MR37_W {
        MR37_W { w: self }
    }
    #[doc = "Bit 6 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&mut self) -> MR38_W {
        MR38_W { w: self }
    }
    #[doc = "Bit 7 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&mut self) -> MR39_W {
        MR39_W { w: self }
    }
    #[doc = "Bit 8 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&mut self) -> MR40_W {
        MR40_W { w: self }
    }
    #[doc = "Bit 9 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&mut self) -> MR41_W {
        MR41_W { w: self }
    }
    #[doc = "Bit 10 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&mut self) -> MR42_W {
        MR42_W { w: self }
    }
    #[doc = "Bit 11 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&mut self) -> MR43_W {
        MR43_W { w: self }
    }
    #[doc = "Bit 12 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&mut self) -> MR44_W {
        MR44_W { w: self }
    }
    #[doc = "Bit 14 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&mut self) -> MR46_W {
        MR46_W { w: self }
    }
    #[doc = "Bit 15 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&mut self) -> MR47_W {
        MR47_W { w: self }
    }
    #[doc = "Bit 16 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&mut self) -> MR48_W {
        MR48_W { w: self }
    }
    #[doc = "Bit 17 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&mut self) -> MR49_W {
        MR49_W { w: self }
    }
    #[doc = "Bit 18 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&mut self) -> MR50_W {
        MR50_W { w: self }
    }
    #[doc = "Bit 19 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&mut self) -> MR51_W {
        MR51_W { w: self }
    }
    #[doc = "Bit 20 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&mut self) -> MR52_W {
        MR52_W { w: self }
    }
    #[doc = "Bit 21 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&mut self) -> MR53_W {
        MR53_W { w: self }
    }
    #[doc = "Bit 22 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&mut self) -> MR54_W {
        MR54_W { w: self }
    }
    #[doc = "Bit 23 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&mut self) -> MR55_W {
        MR55_W { w: self }
    }
    #[doc = "Bit 24 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&mut self) -> MR56_W {
        MR56_W { w: self }
    }
    #[doc = "Bit 25 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&mut self) -> MR57_W {
        MR57_W { w: self }
    }
    #[doc = "Bit 26 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&mut self) -> MR58_W {
        MR58_W { w: self }
    }
    #[doc = "Bit 27 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&mut self) -> MR59_W {
        MR59_W { w: self }
    }
    #[doc = "Bit 28 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&mut self) -> MR60_W {
        MR60_W { w: self }
    }
    #[doc = "Bit 29 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&mut self) -> MR61_W {
        MR61_W { w: self }
    }
    #[doc = "Bit 30 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&mut self) -> MR62_W {
        MR62_W { w: self }
    }
    #[doc = "Bit 31 - CPU2 interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&mut self) -> MR63_W {
        MR63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr2](index.html) module"]
pub struct C2EMR2_SPEC;
impl crate::RegisterSpec for C2EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2emr2::R](R) reader structure"]
impl crate::Readable for C2EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2emr2::W](W) writer structure"]
impl crate::Writable for C2EMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2EMR2 to value 0"]
impl crate::Resettable for C2EMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
