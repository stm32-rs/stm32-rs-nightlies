#[doc = "Register `CHSELR` reader"]
pub struct R(crate::R<CHSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR` writer"]
pub struct W(crate::W<CHSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR_SPEC>;
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
impl From<crate::W<CHSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel-x selection"]
pub type CHSEL18_A = CHSEL0_A;
#[doc = "Field `CHSEL18` reader - Channel-x selection"]
pub type CHSEL18_R = CHSEL0_R;
#[doc = "Field `CHSEL18` writer - Channel-x selection"]
pub struct CHSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL18_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL18_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL17_A = CHSEL0_A;
#[doc = "Field `CHSEL17` reader - Channel-x selection"]
pub type CHSEL17_R = CHSEL0_R;
#[doc = "Field `CHSEL17` writer - Channel-x selection"]
pub struct CHSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL17_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL17_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL16_A = CHSEL0_A;
#[doc = "Field `CHSEL16` reader - Channel-x selection"]
pub type CHSEL16_R = CHSEL0_R;
#[doc = "Field `CHSEL16` writer - Channel-x selection"]
pub struct CHSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL16_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL16_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL15_A = CHSEL0_A;
#[doc = "Field `CHSEL15` reader - Channel-x selection"]
pub type CHSEL15_R = CHSEL0_R;
#[doc = "Field `CHSEL15` writer - Channel-x selection"]
pub struct CHSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL15_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL15_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL14_A = CHSEL0_A;
#[doc = "Field `CHSEL14` reader - Channel-x selection"]
pub type CHSEL14_R = CHSEL0_R;
#[doc = "Field `CHSEL14` writer - Channel-x selection"]
pub struct CHSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL14_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL14_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL13_A = CHSEL0_A;
#[doc = "Field `CHSEL13` reader - Channel-x selection"]
pub type CHSEL13_R = CHSEL0_R;
#[doc = "Field `CHSEL13` writer - Channel-x selection"]
pub struct CHSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL13_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL13_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL12_A = CHSEL0_A;
#[doc = "Field `CHSEL12` reader - Channel-x selection"]
pub type CHSEL12_R = CHSEL0_R;
#[doc = "Field `CHSEL12` writer - Channel-x selection"]
pub struct CHSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL12_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL12_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL11_A = CHSEL0_A;
#[doc = "Field `CHSEL11` reader - Channel-x selection"]
pub type CHSEL11_R = CHSEL0_R;
#[doc = "Field `CHSEL11` writer - Channel-x selection"]
pub struct CHSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL11_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL11_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL10_A = CHSEL0_A;
#[doc = "Field `CHSEL10` reader - Channel-x selection"]
pub type CHSEL10_R = CHSEL0_R;
#[doc = "Field `CHSEL10` writer - Channel-x selection"]
pub struct CHSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL10_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL10_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL9_A = CHSEL0_A;
#[doc = "Field `CHSEL9` reader - Channel-x selection"]
pub type CHSEL9_R = CHSEL0_R;
#[doc = "Field `CHSEL9` writer - Channel-x selection"]
pub struct CHSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL9_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL9_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL8_A = CHSEL0_A;
#[doc = "Field `CHSEL8` reader - Channel-x selection"]
pub type CHSEL8_R = CHSEL0_R;
#[doc = "Field `CHSEL8` writer - Channel-x selection"]
pub struct CHSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL8_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL8_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL7_A = CHSEL0_A;
#[doc = "Field `CHSEL7` reader - Channel-x selection"]
pub type CHSEL7_R = CHSEL0_R;
#[doc = "Field `CHSEL7` writer - Channel-x selection"]
pub struct CHSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL7_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL7_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL6_A = CHSEL0_A;
#[doc = "Field `CHSEL6` reader - Channel-x selection"]
pub type CHSEL6_R = CHSEL0_R;
#[doc = "Field `CHSEL6` writer - Channel-x selection"]
pub struct CHSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL6_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL6_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL5_A = CHSEL0_A;
#[doc = "Field `CHSEL5` reader - Channel-x selection"]
pub type CHSEL5_R = CHSEL0_R;
#[doc = "Field `CHSEL5` writer - Channel-x selection"]
pub struct CHSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL5_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL5_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL4_A = CHSEL0_A;
#[doc = "Field `CHSEL4` reader - Channel-x selection"]
pub type CHSEL4_R = CHSEL0_R;
#[doc = "Field `CHSEL4` writer - Channel-x selection"]
pub struct CHSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL4_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL4_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL3_A = CHSEL0_A;
#[doc = "Field `CHSEL3` reader - Channel-x selection"]
pub type CHSEL3_R = CHSEL0_R;
#[doc = "Field `CHSEL3` writer - Channel-x selection"]
pub struct CHSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL3_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL3_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL2_A = CHSEL0_A;
#[doc = "Field `CHSEL2` reader - Channel-x selection"]
pub type CHSEL2_R = CHSEL0_R;
#[doc = "Field `CHSEL2` writer - Channel-x selection"]
pub struct CHSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL2_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL2_A::SELECTED)
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
#[doc = "Channel-x selection"]
pub type CHSEL1_A = CHSEL0_A;
#[doc = "Field `CHSEL1` reader - Channel-x selection"]
pub type CHSEL1_R = CHSEL0_R;
#[doc = "Field `CHSEL1` writer - Channel-x selection"]
pub struct CHSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL1_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL1_A::SELECTED)
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
#[doc = "Channel-x selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSEL0_A {
    #[doc = "0: Input Channel is not selected for conversion"]
    NOTSELECTED = 0,
    #[doc = "1: Input Channel is selected for conversion"]
    SELECTED = 1,
}
impl From<CHSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHSEL0` reader - Channel-x selection"]
pub struct CHSEL0_R(crate::FieldReader<bool, CHSEL0_A>);
impl CHSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL0_A {
        match self.bits {
            false => CHSEL0_A::NOTSELECTED,
            true => CHSEL0_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        **self == CHSEL0_A::NOTSELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        **self == CHSEL0_A::SELECTED
    }
}
impl core::ops::Deref for CHSEL0_R {
    type Target = crate::FieldReader<bool, CHSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL0` writer - Channel-x selection"]
pub struct CHSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL0_A::NOTSELECTED)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL0_A::SELECTED)
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
    #[doc = "Bit 18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL18_W {
        CHSEL18_W { w: self }
    }
    #[doc = "Bit 17 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL17_W {
        CHSEL17_W { w: self }
    }
    #[doc = "Bit 16 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL16_W {
        CHSEL16_W { w: self }
    }
    #[doc = "Bit 15 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL15_W {
        CHSEL15_W { w: self }
    }
    #[doc = "Bit 14 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL14_W {
        CHSEL14_W { w: self }
    }
    #[doc = "Bit 13 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL13_W {
        CHSEL13_W { w: self }
    }
    #[doc = "Bit 12 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL12_W {
        CHSEL12_W { w: self }
    }
    #[doc = "Bit 11 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL11_W {
        CHSEL11_W { w: self }
    }
    #[doc = "Bit 10 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W {
        CHSEL10_W { w: self }
    }
    #[doc = "Bit 9 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W {
        CHSEL9_W { w: self }
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W {
        CHSEL8_W { w: self }
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W {
        CHSEL7_W { w: self }
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W {
        CHSEL6_W { w: self }
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W {
        CHSEL5_W { w: self }
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W {
        CHSEL4_W { w: self }
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W {
        CHSEL3_W { w: self }
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W {
        CHSEL2_W { w: self }
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W {
        CHSEL1_W { w: self }
    }
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W {
        CHSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr](index.html) module"]
pub struct CHSELR_SPEC;
impl crate::RegisterSpec for CHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr::R](R) reader structure"]
impl crate::Readable for CHSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr::W](W) writer structure"]
impl crate::Writable for CHSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR to value 0"]
impl crate::Resettable for CHSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
