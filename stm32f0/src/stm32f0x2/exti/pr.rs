#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pending interrupt flag on line 31"]
pub type PIF31_A = PIF0_A;
#[doc = "Field `PIF31` reader - Pending interrupt flag on line 31"]
pub type PIF31_R = PIF0_R;
#[doc = "Pending interrupt flag on line 31"]
pub type PIF31_AW = PIF0_AW;
#[doc = "Field `PIF31` writer - Pending interrupt flag on line 31"]
pub struct PIF31_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF31_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF31_AW::CLEAR)
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
#[doc = "Pending interrupt flag on line 22"]
pub type PIF22_A = PIF0_A;
#[doc = "Field `PIF22` reader - Pending interrupt flag on line 22"]
pub type PIF22_R = PIF0_R;
#[doc = "Pending interrupt flag on line 22"]
pub type PIF22_AW = PIF0_AW;
#[doc = "Field `PIF22` writer - Pending interrupt flag on line 22"]
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
#[doc = "Pending interrupt flag on line 21"]
pub type PIF21_A = PIF0_A;
#[doc = "Field `PIF21` reader - Pending interrupt flag on line 21"]
pub type PIF21_R = PIF0_R;
#[doc = "Pending interrupt flag on line 21"]
pub type PIF21_AW = PIF0_AW;
#[doc = "Field `PIF21` writer - Pending interrupt flag on line 21"]
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
#[doc = "Pending interrupt flag on line 20"]
pub type PIF20_A = PIF0_A;
#[doc = "Field `PIF20` reader - Pending interrupt flag on line 20"]
pub type PIF20_R = PIF0_R;
#[doc = "Pending interrupt flag on line 20"]
pub type PIF20_AW = PIF0_AW;
#[doc = "Field `PIF20` writer - Pending interrupt flag on line 20"]
pub struct PIF20_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF20_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF20_AW::CLEAR)
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
#[doc = "Pending interrupt flag on line 19"]
pub type PIF19_A = PIF0_A;
#[doc = "Field `PIF19` reader - Pending interrupt flag on line 19"]
pub type PIF19_R = PIF0_R;
#[doc = "Pending interrupt flag on line 19"]
pub type PIF19_AW = PIF0_AW;
#[doc = "Field `PIF19` writer - Pending interrupt flag on line 19"]
pub struct PIF19_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF19_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF19_AW::CLEAR)
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
#[doc = "Pending interrupt flag on line 17"]
pub type PIF17_A = PIF0_A;
#[doc = "Field `PIF17` reader - Pending interrupt flag on line 17"]
pub type PIF17_R = PIF0_R;
#[doc = "Pending interrupt flag on line 17"]
pub type PIF17_AW = PIF0_AW;
#[doc = "Field `PIF17` writer - Pending interrupt flag on line 17"]
pub struct PIF17_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF17_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF17_AW::CLEAR)
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
#[doc = "Pending interrupt flag on line 16"]
pub type PIF16_A = PIF0_A;
#[doc = "Field `PIF16` reader - Pending interrupt flag on line 16"]
pub type PIF16_R = PIF0_R;
#[doc = "Pending interrupt flag on line 16"]
pub type PIF16_AW = PIF0_AW;
#[doc = "Field `PIF16` writer - Pending interrupt flag on line 16"]
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
#[doc = "Pending interrupt flag on line 15"]
pub type PIF15_A = PIF0_A;
#[doc = "Field `PIF15` reader - Pending interrupt flag on line 15"]
pub type PIF15_R = PIF0_R;
#[doc = "Pending interrupt flag on line 15"]
pub type PIF15_AW = PIF0_AW;
#[doc = "Field `PIF15` writer - Pending interrupt flag on line 15"]
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
#[doc = "Pending interrupt flag on line 14"]
pub type PIF14_A = PIF0_A;
#[doc = "Field `PIF14` reader - Pending interrupt flag on line 14"]
pub type PIF14_R = PIF0_R;
#[doc = "Pending interrupt flag on line 14"]
pub type PIF14_AW = PIF0_AW;
#[doc = "Field `PIF14` writer - Pending interrupt flag on line 14"]
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
#[doc = "Pending interrupt flag on line 13"]
pub type PIF13_A = PIF0_A;
#[doc = "Field `PIF13` reader - Pending interrupt flag on line 13"]
pub type PIF13_R = PIF0_R;
#[doc = "Pending interrupt flag on line 13"]
pub type PIF13_AW = PIF0_AW;
#[doc = "Field `PIF13` writer - Pending interrupt flag on line 13"]
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
#[doc = "Pending interrupt flag on line 12"]
pub type PIF12_A = PIF0_A;
#[doc = "Field `PIF12` reader - Pending interrupt flag on line 12"]
pub type PIF12_R = PIF0_R;
#[doc = "Pending interrupt flag on line 12"]
pub type PIF12_AW = PIF0_AW;
#[doc = "Field `PIF12` writer - Pending interrupt flag on line 12"]
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
#[doc = "Pending interrupt flag on line 11"]
pub type PIF11_A = PIF0_A;
#[doc = "Field `PIF11` reader - Pending interrupt flag on line 11"]
pub type PIF11_R = PIF0_R;
#[doc = "Pending interrupt flag on line 11"]
pub type PIF11_AW = PIF0_AW;
#[doc = "Field `PIF11` writer - Pending interrupt flag on line 11"]
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
#[doc = "Pending interrupt flag on line 10"]
pub type PIF10_A = PIF0_A;
#[doc = "Field `PIF10` reader - Pending interrupt flag on line 10"]
pub type PIF10_R = PIF0_R;
#[doc = "Pending interrupt flag on line 10"]
pub type PIF10_AW = PIF0_AW;
#[doc = "Field `PIF10` writer - Pending interrupt flag on line 10"]
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
#[doc = "Pending interrupt flag on line 9"]
pub type PIF9_A = PIF0_A;
#[doc = "Field `PIF9` reader - Pending interrupt flag on line 9"]
pub type PIF9_R = PIF0_R;
#[doc = "Pending interrupt flag on line 9"]
pub type PIF9_AW = PIF0_AW;
#[doc = "Field `PIF9` writer - Pending interrupt flag on line 9"]
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
#[doc = "Pending interrupt flag on line 8"]
pub type PIF8_A = PIF0_A;
#[doc = "Field `PIF8` reader - Pending interrupt flag on line 8"]
pub type PIF8_R = PIF0_R;
#[doc = "Pending interrupt flag on line 8"]
pub type PIF8_AW = PIF0_AW;
#[doc = "Field `PIF8` writer - Pending interrupt flag on line 8"]
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
#[doc = "Pending interrupt flag on line 7"]
pub type PIF7_A = PIF0_A;
#[doc = "Field `PIF7` reader - Pending interrupt flag on line 7"]
pub type PIF7_R = PIF0_R;
#[doc = "Pending interrupt flag on line 7"]
pub type PIF7_AW = PIF0_AW;
#[doc = "Field `PIF7` writer - Pending interrupt flag on line 7"]
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
#[doc = "Pending interrupt flag on line 6"]
pub type PIF6_A = PIF0_A;
#[doc = "Field `PIF6` reader - Pending interrupt flag on line 6"]
pub type PIF6_R = PIF0_R;
#[doc = "Pending interrupt flag on line 6"]
pub type PIF6_AW = PIF0_AW;
#[doc = "Field `PIF6` writer - Pending interrupt flag on line 6"]
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
#[doc = "Pending interrupt flag on line 5"]
pub type PIF5_A = PIF0_A;
#[doc = "Field `PIF5` reader - Pending interrupt flag on line 5"]
pub type PIF5_R = PIF0_R;
#[doc = "Pending interrupt flag on line 5"]
pub type PIF5_AW = PIF0_AW;
#[doc = "Field `PIF5` writer - Pending interrupt flag on line 5"]
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
#[doc = "Pending interrupt flag on line 4"]
pub type PIF4_A = PIF0_A;
#[doc = "Field `PIF4` reader - Pending interrupt flag on line 4"]
pub type PIF4_R = PIF0_R;
#[doc = "Pending interrupt flag on line 4"]
pub type PIF4_AW = PIF0_AW;
#[doc = "Field `PIF4` writer - Pending interrupt flag on line 4"]
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
#[doc = "Pending interrupt flag on line 3"]
pub type PIF3_A = PIF0_A;
#[doc = "Field `PIF3` reader - Pending interrupt flag on line 3"]
pub type PIF3_R = PIF0_R;
#[doc = "Pending interrupt flag on line 3"]
pub type PIF3_AW = PIF0_AW;
#[doc = "Field `PIF3` writer - Pending interrupt flag on line 3"]
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
#[doc = "Pending interrupt flag on line 2"]
pub type PIF2_A = PIF0_A;
#[doc = "Field `PIF2` reader - Pending interrupt flag on line 2"]
pub type PIF2_R = PIF0_R;
#[doc = "Pending interrupt flag on line 2"]
pub type PIF2_AW = PIF0_AW;
#[doc = "Field `PIF2` writer - Pending interrupt flag on line 2"]
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
#[doc = "Pending interrupt flag on line 1"]
pub type PIF1_A = PIF0_A;
#[doc = "Field `PIF1` reader - Pending interrupt flag on line 1"]
pub type PIF1_R = PIF0_R;
#[doc = "Pending interrupt flag on line 1"]
pub type PIF1_AW = PIF0_AW;
#[doc = "Field `PIF1` writer - Pending interrupt flag on line 1"]
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
#[doc = "Pending interrupt flag on line 0\n\nValue on reset: 0"]
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
#[doc = "Field `PIF0` reader - Pending interrupt flag on line 0"]
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
#[doc = "Pending interrupt flag on line 0\n\nValue on reset: 0"]
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
#[doc = "Field `PIF0` writer - Pending interrupt flag on line 0"]
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
impl R {
    #[doc = "Bit 31 - Pending interrupt flag on line 31"]
    #[inline(always)]
    pub fn pif31(&self) -> PIF31_R {
        PIF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pending interrupt flag on line 22"]
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pending interrupt flag on line 21"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pending interrupt flag on line 20"]
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pending interrupt flag on line 19"]
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pending interrupt flag on line 17"]
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pending interrupt flag on line 16"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pending interrupt flag on line 15"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pending interrupt flag on line 14"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pending interrupt flag on line 13"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pending interrupt flag on line 12"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pending interrupt flag on line 11"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pending interrupt flag on line 10"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pending interrupt flag on line 9"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending interrupt flag on line 8"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pending interrupt flag on line 7"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 6"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 5"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 4"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending interrupt flag on line 3"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending interrupt flag on line 2"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending interrupt flag on line 1"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pending interrupt flag on line 0"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Pending interrupt flag on line 31"]
    #[inline(always)]
    pub fn pif31(&mut self) -> PIF31_W {
        PIF31_W { w: self }
    }
    #[doc = "Bit 22 - Pending interrupt flag on line 22"]
    #[inline(always)]
    pub fn pif22(&mut self) -> PIF22_W {
        PIF22_W { w: self }
    }
    #[doc = "Bit 21 - Pending interrupt flag on line 21"]
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W {
        PIF21_W { w: self }
    }
    #[doc = "Bit 20 - Pending interrupt flag on line 20"]
    #[inline(always)]
    pub fn pif20(&mut self) -> PIF20_W {
        PIF20_W { w: self }
    }
    #[doc = "Bit 19 - Pending interrupt flag on line 19"]
    #[inline(always)]
    pub fn pif19(&mut self) -> PIF19_W {
        PIF19_W { w: self }
    }
    #[doc = "Bit 17 - Pending interrupt flag on line 17"]
    #[inline(always)]
    pub fn pif17(&mut self) -> PIF17_W {
        PIF17_W { w: self }
    }
    #[doc = "Bit 16 - Pending interrupt flag on line 16"]
    #[inline(always)]
    pub fn pif16(&mut self) -> PIF16_W {
        PIF16_W { w: self }
    }
    #[doc = "Bit 15 - Pending interrupt flag on line 15"]
    #[inline(always)]
    pub fn pif15(&mut self) -> PIF15_W {
        PIF15_W { w: self }
    }
    #[doc = "Bit 14 - Pending interrupt flag on line 14"]
    #[inline(always)]
    pub fn pif14(&mut self) -> PIF14_W {
        PIF14_W { w: self }
    }
    #[doc = "Bit 13 - Pending interrupt flag on line 13"]
    #[inline(always)]
    pub fn pif13(&mut self) -> PIF13_W {
        PIF13_W { w: self }
    }
    #[doc = "Bit 12 - Pending interrupt flag on line 12"]
    #[inline(always)]
    pub fn pif12(&mut self) -> PIF12_W {
        PIF12_W { w: self }
    }
    #[doc = "Bit 11 - Pending interrupt flag on line 11"]
    #[inline(always)]
    pub fn pif11(&mut self) -> PIF11_W {
        PIF11_W { w: self }
    }
    #[doc = "Bit 10 - Pending interrupt flag on line 10"]
    #[inline(always)]
    pub fn pif10(&mut self) -> PIF10_W {
        PIF10_W { w: self }
    }
    #[doc = "Bit 9 - Pending interrupt flag on line 9"]
    #[inline(always)]
    pub fn pif9(&mut self) -> PIF9_W {
        PIF9_W { w: self }
    }
    #[doc = "Bit 8 - Pending interrupt flag on line 8"]
    #[inline(always)]
    pub fn pif8(&mut self) -> PIF8_W {
        PIF8_W { w: self }
    }
    #[doc = "Bit 7 - Pending interrupt flag on line 7"]
    #[inline(always)]
    pub fn pif7(&mut self) -> PIF7_W {
        PIF7_W { w: self }
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 6"]
    #[inline(always)]
    pub fn pif6(&mut self) -> PIF6_W {
        PIF6_W { w: self }
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 5"]
    #[inline(always)]
    pub fn pif5(&mut self) -> PIF5_W {
        PIF5_W { w: self }
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 4"]
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W {
        PIF4_W { w: self }
    }
    #[doc = "Bit 3 - Pending interrupt flag on line 3"]
    #[inline(always)]
    pub fn pif3(&mut self) -> PIF3_W {
        PIF3_W { w: self }
    }
    #[doc = "Bit 2 - Pending interrupt flag on line 2"]
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W {
        PIF2_W { w: self }
    }
    #[doc = "Bit 1 - Pending interrupt flag on line 1"]
    #[inline(always)]
    pub fn pif1(&mut self) -> PIF1_W {
        PIF1_W { w: self }
    }
    #[doc = "Bit 0 - Pending interrupt flag on line 0"]
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W {
        PIF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_PR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
