#[doc = "Register `C1IMR3` reader"]
pub struct R(crate::R<C1IMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IMR3` writer"]
pub struct W(crate::W<C1IMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IMR3_SPEC>;
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
impl From<crate::W<C1IMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU Interrupt Mask on Direct Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR64_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR64_A> for bool {
    #[inline(always)]
    fn from(variant: MR64_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR64` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR64_R(crate::FieldReader<bool, MR64_A>);
impl MR64_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR64_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR64_A {
        match self.bits {
            false => MR64_A::MASKED,
            true => MR64_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == MR64_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == MR64_A::UNMASKED
    }
}
impl core::ops::Deref for MR64_R {
    type Target = crate::FieldReader<bool, MR64_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR64` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR64_W<'a> {
    w: &'a mut W,
}
impl<'a> MR64_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR64_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR64_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR64_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR65_A = MR64_A;
#[doc = "Field `MR65` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR65_R = MR64_R;
#[doc = "Field `MR65` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR65_W<'a> {
    w: &'a mut W,
}
impl<'a> MR65_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR65_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR65_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR65_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR66_A = MR64_A;
#[doc = "Field `MR66` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR66_R = MR64_R;
#[doc = "Field `MR66` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR66_W<'a> {
    w: &'a mut W,
}
impl<'a> MR66_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR66_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR66_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR66_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR67_A = MR64_A;
#[doc = "Field `MR67` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR67_R = MR64_R;
#[doc = "Field `MR67` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR67_W<'a> {
    w: &'a mut W,
}
impl<'a> MR67_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR67_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR67_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR67_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR68_A = MR64_A;
#[doc = "Field `MR68` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR68_R = MR64_R;
#[doc = "Field `MR68` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR68_W<'a> {
    w: &'a mut W,
}
impl<'a> MR68_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR68_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR68_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR68_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR69_A = MR64_A;
#[doc = "Field `MR69` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR69_R = MR64_R;
#[doc = "Field `MR69` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR69_W<'a> {
    w: &'a mut W,
}
impl<'a> MR69_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR69_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR69_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR69_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR70_A = MR64_A;
#[doc = "Field `MR70` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR70_R = MR64_R;
#[doc = "Field `MR70` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR70_W<'a> {
    w: &'a mut W,
}
impl<'a> MR70_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR70_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR70_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR70_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR71_A = MR64_A;
#[doc = "Field `MR71` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR71_R = MR64_R;
#[doc = "Field `MR71` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR71_W<'a> {
    w: &'a mut W,
}
impl<'a> MR71_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR71_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR71_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR71_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR72_A = MR64_A;
#[doc = "Field `MR72` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR72_R = MR64_R;
#[doc = "Field `MR72` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR72_W<'a> {
    w: &'a mut W,
}
impl<'a> MR72_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR72_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR72_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR72_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR73_A = MR64_A;
#[doc = "Field `MR73` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR73_R = MR64_R;
#[doc = "Field `MR73` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR73_W<'a> {
    w: &'a mut W,
}
impl<'a> MR73_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR73_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR73_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR73_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR74_A = MR64_A;
#[doc = "Field `MR74` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR74_R = MR64_R;
#[doc = "Field `MR74` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR74_W<'a> {
    w: &'a mut W,
}
impl<'a> MR74_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR74_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR74_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR74_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR75_A = MR64_A;
#[doc = "Field `MR75` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR75_R = MR64_R;
#[doc = "Field `MR75` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR75_W<'a> {
    w: &'a mut W,
}
impl<'a> MR75_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR75_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR75_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR75_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR76_A = MR64_A;
#[doc = "Field `MR76` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR76_R = MR64_R;
#[doc = "Field `MR76` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR76_W<'a> {
    w: &'a mut W,
}
impl<'a> MR76_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR76_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR76_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR76_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR77_A = MR64_A;
#[doc = "Field `MR77` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR77_R = MR64_R;
#[doc = "Field `MR77` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR77_W<'a> {
    w: &'a mut W,
}
impl<'a> MR77_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR77_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR77_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR77_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR78_A = MR64_A;
#[doc = "Field `MR78` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR78_R = MR64_R;
#[doc = "Field `MR78` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR78_W<'a> {
    w: &'a mut W,
}
impl<'a> MR78_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR78_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR78_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR78_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR79_A = MR64_A;
#[doc = "Field `MR79` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR79_R = MR64_R;
#[doc = "Field `MR79` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR79_W<'a> {
    w: &'a mut W,
}
impl<'a> MR79_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR79_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR79_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR79_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR80_A = MR64_A;
#[doc = "Field `MR80` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR80_R = MR64_R;
#[doc = "Field `MR80` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR80_W<'a> {
    w: &'a mut W,
}
impl<'a> MR80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR80_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR80_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR80_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR82_A = MR64_A;
#[doc = "Field `MR82` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR82_R = MR64_R;
#[doc = "Field `MR82` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR82_W<'a> {
    w: &'a mut W,
}
impl<'a> MR82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR82_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR82_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR82_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR84_A = MR64_A;
#[doc = "Field `MR84` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR84_R = MR64_R;
#[doc = "Field `MR84` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR84_W<'a> {
    w: &'a mut W,
}
impl<'a> MR84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR84_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR84_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR84_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR85_A = MR64_A;
#[doc = "Field `MR85` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR85_R = MR64_R;
#[doc = "Field `MR85` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR85_W<'a> {
    w: &'a mut W,
}
impl<'a> MR85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR85_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR85_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR85_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR86_A = MR64_A;
#[doc = "Field `MR86` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR86_R = MR64_R;
#[doc = "Field `MR86` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR86_W<'a> {
    w: &'a mut W,
}
impl<'a> MR86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR86_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR86_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR86_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR87_A = MR64_A;
#[doc = "Field `MR87` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR87_R = MR64_R;
#[doc = "Field `MR87` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR87_W<'a> {
    w: &'a mut W,
}
impl<'a> MR87_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR87_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR87_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR87_A::UNMASKED)
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
#[doc = "CPU Interrupt Mask on Direct Event input x+64"]
pub type MR88_A = MR64_A;
#[doc = "Field `MR88` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR88_R = MR64_R;
#[doc = "Field `MR88` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub struct MR88_W<'a> {
    w: &'a mut W,
}
impl<'a> MR88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR88_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR88_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR88_A::UNMASKED)
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
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&mut self) -> MR64_W {
        MR64_W { w: self }
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&mut self) -> MR65_W {
        MR65_W { w: self }
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&mut self) -> MR66_W {
        MR66_W { w: self }
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&mut self) -> MR67_W {
        MR67_W { w: self }
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&mut self) -> MR68_W {
        MR68_W { w: self }
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&mut self) -> MR69_W {
        MR69_W { w: self }
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&mut self) -> MR70_W {
        MR70_W { w: self }
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&mut self) -> MR71_W {
        MR71_W { w: self }
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&mut self) -> MR72_W {
        MR72_W { w: self }
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&mut self) -> MR73_W {
        MR73_W { w: self }
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&mut self) -> MR74_W {
        MR74_W { w: self }
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&mut self) -> MR75_W {
        MR75_W { w: self }
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&mut self) -> MR76_W {
        MR76_W { w: self }
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&mut self) -> MR77_W {
        MR77_W { w: self }
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&mut self) -> MR78_W {
        MR78_W { w: self }
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&mut self) -> MR79_W {
        MR79_W { w: self }
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&mut self) -> MR80_W {
        MR80_W { w: self }
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&mut self) -> MR82_W {
        MR82_W { w: self }
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&mut self) -> MR84_W {
        MR84_W { w: self }
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&mut self) -> MR85_W {
        MR85_W { w: self }
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&mut self) -> MR86_W {
        MR86_W { w: self }
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&mut self) -> MR87_W {
        MR87_W { w: self }
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W {
        MR88_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr3](index.html) module"]
pub struct C1IMR3_SPEC;
impl crate::RegisterSpec for C1IMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1imr3::R](R) reader structure"]
impl crate::Readable for C1IMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1imr3::W](W) writer structure"]
impl crate::Writable for C1IMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1IMR3 to value 0"]
impl crate::Resettable for C1IMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
