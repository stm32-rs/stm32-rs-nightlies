#[doc = "Register `ADC3R` reader"]
pub struct R(crate::R<ADC3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC3R` writer"]
pub struct W(crate::W<ADC3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC3R_SPEC>;
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
impl From<crate::W<ADC3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AD1TEPER"]
pub type AD1TEPER_A = AD1MPER_A;
#[doc = "Field `AD1TEPER` reader - AD1TEPER"]
pub type AD1TEPER_R = AD1MPER_R;
#[doc = "Field `AD1TEPER` writer - AD1TEPER"]
pub struct AD1TEPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TEPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TEPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TEPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TEPER_A::ENABLED)
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
#[doc = "AD1TEC4"]
pub type AD1TEC4_A = AD1TAC2_A;
#[doc = "Field `AD1TEC4` reader - AD1TEC4"]
pub type AD1TEC4_R = AD1TAC2_R;
#[doc = "Field `AD1TEC4` writer - AD1TEC4"]
pub struct AD1TEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TEC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TEC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TEC4_A::ENABLED)
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
#[doc = "AD1TEC3"]
pub type AD1TEC3_A = AD1TAC2_A;
#[doc = "Field `AD1TEC3` reader - AD1TEC3"]
pub type AD1TEC3_R = AD1TAC2_R;
#[doc = "Field `AD1TEC3` writer - AD1TEC3"]
pub struct AD1TEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TEC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TEC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TEC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TEC3_A::ENABLED)
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
#[doc = "AD1TEC2"]
pub type AD1TEC2_A = AD1TAC2_A;
#[doc = "Field `AD1TEC2` reader - AD1TEC2"]
pub type AD1TEC2_R = AD1TAC2_R;
#[doc = "Field `AD1TEC2` writer - AD1TEC2"]
pub struct AD1TEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TEC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TEC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TEC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TEC2_A::ENABLED)
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
#[doc = "AD1TDPER"]
pub type AD1TDPER_A = AD1MPER_A;
#[doc = "Field `AD1TDPER` reader - AD1TDPER"]
pub type AD1TDPER_R = AD1MPER_R;
#[doc = "Field `AD1TDPER` writer - AD1TDPER"]
pub struct AD1TDPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TDPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TDPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TDPER_A::ENABLED)
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
#[doc = "AD1TDC4"]
pub type AD1TDC4_A = AD1TAC2_A;
#[doc = "Field `AD1TDC4` reader - AD1TDC4"]
pub type AD1TDC4_R = AD1TAC2_R;
#[doc = "Field `AD1TDC4` writer - AD1TDC4"]
pub struct AD1TDC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TDC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TDC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TDC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TDC4_A::ENABLED)
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
#[doc = "AD1TDC3"]
pub type AD1TDC3_A = AD1TAC2_A;
#[doc = "Field `AD1TDC3` reader - AD1TDC3"]
pub type AD1TDC3_R = AD1TAC2_R;
#[doc = "Field `AD1TDC3` writer - AD1TDC3"]
pub struct AD1TDC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TDC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TDC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TDC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TDC3_A::ENABLED)
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
#[doc = "AD1TDC2"]
pub type AD1TDC2_A = AD1TAC2_A;
#[doc = "Field `AD1TDC2` reader - AD1TDC2"]
pub type AD1TDC2_R = AD1TAC2_R;
#[doc = "Field `AD1TDC2` writer - AD1TDC2"]
pub struct AD1TDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TDC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TDC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TDC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TDC2_A::ENABLED)
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
#[doc = "AD1TCPER"]
pub type AD1TCPER_A = AD1MPER_A;
#[doc = "Field `AD1TCPER` reader - AD1TCPER"]
pub type AD1TCPER_R = AD1MPER_R;
#[doc = "Field `AD1TCPER` writer - AD1TCPER"]
pub struct AD1TCPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TCPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TCPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TCPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TCPER_A::ENABLED)
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
#[doc = "AD1TCC4"]
pub type AD1TCC4_A = AD1TAC2_A;
#[doc = "Field `AD1TCC4` reader - AD1TCC4"]
pub type AD1TCC4_R = AD1TAC2_R;
#[doc = "Field `AD1TCC4` writer - AD1TCC4"]
pub struct AD1TCC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TCC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TCC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TCC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TCC4_A::ENABLED)
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
#[doc = "AD1TCC3"]
pub type AD1TCC3_A = AD1TAC2_A;
#[doc = "Field `AD1TCC3` reader - AD1TCC3"]
pub type AD1TCC3_R = AD1TAC2_R;
#[doc = "Field `AD1TCC3` writer - AD1TCC3"]
pub struct AD1TCC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TCC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TCC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TCC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TCC3_A::ENABLED)
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
#[doc = "AD1TCC2"]
pub type AD1TCC2_A = AD1TAC2_A;
#[doc = "Field `AD1TCC2` reader - AD1TCC2"]
pub type AD1TCC2_R = AD1TAC2_R;
#[doc = "Field `AD1TCC2` writer - AD1TCC2"]
pub struct AD1TCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TCC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TCC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TCC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TCC2_A::ENABLED)
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
#[doc = "AD1TBRST"]
pub type AD1TBRST_A = AD1TARST_A;
#[doc = "Field `AD1TBRST` reader - AD1TBRST"]
pub type AD1TBRST_R = AD1TARST_R;
#[doc = "Field `AD1TBRST` writer - AD1TBRST"]
pub struct AD1TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TBRST_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TBRST_A::ENABLED)
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
#[doc = "AD1TBPER"]
pub type AD1TBPER_A = AD1MPER_A;
#[doc = "Field `AD1TBPER` reader - AD1TBPER"]
pub type AD1TBPER_R = AD1MPER_R;
#[doc = "Field `AD1TBPER` writer - AD1TBPER"]
pub struct AD1TBPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TBPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TBPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TBPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TBPER_A::ENABLED)
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
#[doc = "AD1TBC4"]
pub type AD1TBC4_A = AD1TAC2_A;
#[doc = "Field `AD1TBC4` reader - AD1TBC4"]
pub type AD1TBC4_R = AD1TAC2_R;
#[doc = "Field `AD1TBC4` writer - AD1TBC4"]
pub struct AD1TBC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TBC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TBC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TBC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TBC4_A::ENABLED)
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
#[doc = "AD1TBC3"]
pub type AD1TBC3_A = AD1TAC2_A;
#[doc = "Field `AD1TBC3` reader - AD1TBC3"]
pub type AD1TBC3_R = AD1TAC2_R;
#[doc = "Field `AD1TBC3` writer - AD1TBC3"]
pub struct AD1TBC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TBC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TBC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TBC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TBC3_A::ENABLED)
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
#[doc = "AD1TBC2"]
pub type AD1TBC2_A = AD1TAC2_A;
#[doc = "Field `AD1TBC2` reader - AD1TBC2"]
pub type AD1TBC2_R = AD1TAC2_R;
#[doc = "Field `AD1TBC2` writer - AD1TBC2"]
pub struct AD1TBC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TBC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TBC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TBC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TBC2_A::ENABLED)
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
#[doc = "AD1TARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1TARST_A {
    #[doc = "0: No generation of ADC trigger on timer reset and roll-over"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer reset and roll-over"]
    ENABLED = 1,
}
impl From<AD1TARST_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1TARST` reader - AD1TARST"]
pub struct AD1TARST_R(crate::FieldReader<bool, AD1TARST_A>);
impl AD1TARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1TARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1TARST_A {
        match self.bits {
            false => AD1TARST_A::DISABLED,
            true => AD1TARST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AD1TARST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AD1TARST_A::ENABLED
    }
}
impl core::ops::Deref for AD1TARST_R {
    type Target = crate::FieldReader<bool, AD1TARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1TARST` writer - AD1TARST"]
pub struct AD1TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::ENABLED)
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
#[doc = "AD1TAPER"]
pub type AD1TAPER_A = AD1MPER_A;
#[doc = "Field `AD1TAPER` reader - AD1TAPER"]
pub type AD1TAPER_R = AD1MPER_R;
#[doc = "Field `AD1TAPER` writer - AD1TAPER"]
pub struct AD1TAPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TAPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TAPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAPER_A::ENABLED)
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
#[doc = "AD1TAC4"]
pub type AD1TAC4_A = AD1TAC2_A;
#[doc = "Field `AD1TAC4` reader - AD1TAC4"]
pub type AD1TAC4_R = AD1TAC2_R;
#[doc = "Field `AD1TAC4` writer - AD1TAC4"]
pub struct AD1TAC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TAC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TAC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAC4_A::ENABLED)
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
#[doc = "AD1TAC3"]
pub type AD1TAC3_A = AD1TAC2_A;
#[doc = "Field `AD1TAC3` reader - AD1TAC3"]
pub type AD1TAC3_R = AD1TAC2_R;
#[doc = "Field `AD1TAC3` writer - AD1TAC3"]
pub struct AD1TAC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TAC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TAC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAC3_A::ENABLED)
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
#[doc = "AD1TAC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1TAC2_A {
    #[doc = "0: No generation of ADC trigger on timer compare event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer compare event"]
    ENABLED = 1,
}
impl From<AD1TAC2_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TAC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1TAC2` reader - AD1TAC2"]
pub struct AD1TAC2_R(crate::FieldReader<bool, AD1TAC2_A>);
impl AD1TAC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1TAC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1TAC2_A {
        match self.bits {
            false => AD1TAC2_A::DISABLED,
            true => AD1TAC2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AD1TAC2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AD1TAC2_A::ENABLED
    }
}
impl core::ops::Deref for AD1TAC2_R {
    type Target = crate::FieldReader<bool, AD1TAC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1TAC2` writer - AD1TAC2"]
pub struct AD1TAC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1TAC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1TAC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::ENABLED)
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
#[doc = "AD1EEV5"]
pub type AD1EEV5_A = AD1EEV1_A;
#[doc = "Field `AD1EEV5` reader - AD1EEV5"]
pub type AD1EEV5_R = AD1EEV1_R;
#[doc = "Field `AD1EEV5` writer - AD1EEV5"]
pub struct AD1EEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1EEV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1EEV5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV5_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV5_A::ENABLED)
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
#[doc = "AD1EEV4"]
pub type AD1EEV4_A = AD1EEV1_A;
#[doc = "Field `AD1EEV4` reader - AD1EEV4"]
pub type AD1EEV4_R = AD1EEV1_R;
#[doc = "Field `AD1EEV4` writer - AD1EEV4"]
pub struct AD1EEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1EEV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1EEV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV4_A::ENABLED)
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
#[doc = "AD1EEV3"]
pub type AD1EEV3_A = AD1EEV1_A;
#[doc = "Field `AD1EEV3` reader - AD1EEV3"]
pub type AD1EEV3_R = AD1EEV1_R;
#[doc = "Field `AD1EEV3` writer - AD1EEV3"]
pub struct AD1EEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1EEV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1EEV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV3_A::ENABLED)
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
#[doc = "AD1EEV2"]
pub type AD1EEV2_A = AD1EEV1_A;
#[doc = "Field `AD1EEV2` reader - AD1EEV2"]
pub type AD1EEV2_R = AD1EEV1_R;
#[doc = "Field `AD1EEV2` writer - AD1EEV2"]
pub struct AD1EEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1EEV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1EEV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV2_A::ENABLED)
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
#[doc = "AD1EEV1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1EEV1_A {
    #[doc = "0: No generation of ADC trigger on external event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on external event"]
    ENABLED = 1,
}
impl From<AD1EEV1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1EEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1EEV1` reader - AD1EEV1"]
pub struct AD1EEV1_R(crate::FieldReader<bool, AD1EEV1_A>);
impl AD1EEV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1EEV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1EEV1_A {
        match self.bits {
            false => AD1EEV1_A::DISABLED,
            true => AD1EEV1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AD1EEV1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AD1EEV1_A::ENABLED
    }
}
impl core::ops::Deref for AD1EEV1_R {
    type Target = crate::FieldReader<bool, AD1EEV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1EEV1` writer - AD1EEV1"]
pub struct AD1EEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1EEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1EEV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::ENABLED)
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
#[doc = "AD1MPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1MPER_A {
    #[doc = "0: No generation of ADC trigger on timer period event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer period event"]
    ENABLED = 1,
}
impl From<AD1MPER_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1MPER` reader - AD1MPER"]
pub struct AD1MPER_R(crate::FieldReader<bool, AD1MPER_A>);
impl AD1MPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1MPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1MPER_A {
        match self.bits {
            false => AD1MPER_A::DISABLED,
            true => AD1MPER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AD1MPER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AD1MPER_A::ENABLED
    }
}
impl core::ops::Deref for AD1MPER_R {
    type Target = crate::FieldReader<bool, AD1MPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1MPER` writer - AD1MPER"]
pub struct AD1MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1MPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::ENABLED)
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
#[doc = "AD1MC4"]
pub type AD1MC4_A = AD1MC1_A;
#[doc = "Field `AD1MC4` reader - AD1MC4"]
pub type AD1MC4_R = AD1MC1_R;
#[doc = "Field `AD1MC4` writer - AD1MC4"]
pub struct AD1MC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1MC4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC4_A::ENABLED)
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
#[doc = "AD1MC3"]
pub type AD1MC3_A = AD1MC1_A;
#[doc = "Field `AD1MC3` reader - AD1MC3"]
pub type AD1MC3_R = AD1MC1_R;
#[doc = "Field `AD1MC3` writer - AD1MC3"]
pub struct AD1MC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1MC3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC3_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC3_A::ENABLED)
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
#[doc = "AD1MC2"]
pub type AD1MC2_A = AD1MC1_A;
#[doc = "Field `AD1MC2` reader - AD1MC2"]
pub type AD1MC2_R = AD1MC1_R;
#[doc = "Field `AD1MC2` writer - AD1MC2"]
pub struct AD1MC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1MC2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC2_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC2_A::ENABLED)
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
#[doc = "AD1MC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1MC1_A {
    #[doc = "0: No generation of ADC trigger on master compare event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on master compare event"]
    ENABLED = 1,
}
impl From<AD1MC1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1MC1` reader - AD1MC1"]
pub struct AD1MC1_R(crate::FieldReader<bool, AD1MC1_A>);
impl AD1MC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1MC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1MC1_A {
        match self.bits {
            false => AD1MC1_A::DISABLED,
            true => AD1MC1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AD1MC1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AD1MC1_A::ENABLED
    }
}
impl core::ops::Deref for AD1MC1_R {
    type Target = crate::FieldReader<bool, AD1MC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1MC1` writer - AD1MC1"]
pub struct AD1MC1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1MC1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::ENABLED)
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
    #[doc = "Bit 31 - AD1TEPER"]
    #[inline(always)]
    pub fn ad1teper(&self) -> AD1TEPER_R {
        AD1TEPER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - AD1TEC4"]
    #[inline(always)]
    pub fn ad1tec4(&self) -> AD1TEC4_R {
        AD1TEC4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AD1TEC3"]
    #[inline(always)]
    pub fn ad1tec3(&self) -> AD1TEC3_R {
        AD1TEC3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - AD1TEC2"]
    #[inline(always)]
    pub fn ad1tec2(&self) -> AD1TEC2_R {
        AD1TEC2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - AD1TDPER"]
    #[inline(always)]
    pub fn ad1tdper(&self) -> AD1TDPER_R {
        AD1TDPER_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - AD1TDC4"]
    #[inline(always)]
    pub fn ad1tdc4(&self) -> AD1TDC4_R {
        AD1TDC4_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AD1TDC3"]
    #[inline(always)]
    pub fn ad1tdc3(&self) -> AD1TDC3_R {
        AD1TDC3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AD1TDC2"]
    #[inline(always)]
    pub fn ad1tdc2(&self) -> AD1TDC2_R {
        AD1TDC2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AD1TCPER"]
    #[inline(always)]
    pub fn ad1tcper(&self) -> AD1TCPER_R {
        AD1TCPER_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - AD1TCC4"]
    #[inline(always)]
    pub fn ad1tcc4(&self) -> AD1TCC4_R {
        AD1TCC4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - AD1TCC3"]
    #[inline(always)]
    pub fn ad1tcc3(&self) -> AD1TCC3_R {
        AD1TCC3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AD1TCC2"]
    #[inline(always)]
    pub fn ad1tcc2(&self) -> AD1TCC2_R {
        AD1TCC2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - AD1TBRST"]
    #[inline(always)]
    pub fn ad1tbrst(&self) -> AD1TBRST_R {
        AD1TBRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AD1TBPER"]
    #[inline(always)]
    pub fn ad1tbper(&self) -> AD1TBPER_R {
        AD1TBPER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AD1TBC4"]
    #[inline(always)]
    pub fn ad1tbc4(&self) -> AD1TBC4_R {
        AD1TBC4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AD1TBC3"]
    #[inline(always)]
    pub fn ad1tbc3(&self) -> AD1TBC3_R {
        AD1TBC3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AD1TBC2"]
    #[inline(always)]
    pub fn ad1tbc2(&self) -> AD1TBC2_R {
        AD1TBC2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AD1TARST"]
    #[inline(always)]
    pub fn ad1tarst(&self) -> AD1TARST_R {
        AD1TARST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AD1TAPER"]
    #[inline(always)]
    pub fn ad1taper(&self) -> AD1TAPER_R {
        AD1TAPER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AD1TAC4"]
    #[inline(always)]
    pub fn ad1tac4(&self) -> AD1TAC4_R {
        AD1TAC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AD1TAC3"]
    #[inline(always)]
    pub fn ad1tac3(&self) -> AD1TAC3_R {
        AD1TAC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AD1TAC2"]
    #[inline(always)]
    pub fn ad1tac2(&self) -> AD1TAC2_R {
        AD1TAC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AD1EEV5"]
    #[inline(always)]
    pub fn ad1eev5(&self) -> AD1EEV5_R {
        AD1EEV5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AD1EEV4"]
    #[inline(always)]
    pub fn ad1eev4(&self) -> AD1EEV4_R {
        AD1EEV4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AD1EEV3"]
    #[inline(always)]
    pub fn ad1eev3(&self) -> AD1EEV3_R {
        AD1EEV3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AD1EEV2"]
    #[inline(always)]
    pub fn ad1eev2(&self) -> AD1EEV2_R {
        AD1EEV2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AD1EEV1"]
    #[inline(always)]
    pub fn ad1eev1(&self) -> AD1EEV1_R {
        AD1EEV1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AD1MPER"]
    #[inline(always)]
    pub fn ad1mper(&self) -> AD1MPER_R {
        AD1MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AD1MC4"]
    #[inline(always)]
    pub fn ad1mc4(&self) -> AD1MC4_R {
        AD1MC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AD1MC3"]
    #[inline(always)]
    pub fn ad1mc3(&self) -> AD1MC3_R {
        AD1MC3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AD1MC2"]
    #[inline(always)]
    pub fn ad1mc2(&self) -> AD1MC2_R {
        AD1MC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AD1MC1"]
    #[inline(always)]
    pub fn ad1mc1(&self) -> AD1MC1_R {
        AD1MC1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - AD1TEPER"]
    #[inline(always)]
    pub fn ad1teper(&mut self) -> AD1TEPER_W {
        AD1TEPER_W { w: self }
    }
    #[doc = "Bit 30 - AD1TEC4"]
    #[inline(always)]
    pub fn ad1tec4(&mut self) -> AD1TEC4_W {
        AD1TEC4_W { w: self }
    }
    #[doc = "Bit 29 - AD1TEC3"]
    #[inline(always)]
    pub fn ad1tec3(&mut self) -> AD1TEC3_W {
        AD1TEC3_W { w: self }
    }
    #[doc = "Bit 28 - AD1TEC2"]
    #[inline(always)]
    pub fn ad1tec2(&mut self) -> AD1TEC2_W {
        AD1TEC2_W { w: self }
    }
    #[doc = "Bit 27 - AD1TDPER"]
    #[inline(always)]
    pub fn ad1tdper(&mut self) -> AD1TDPER_W {
        AD1TDPER_W { w: self }
    }
    #[doc = "Bit 26 - AD1TDC4"]
    #[inline(always)]
    pub fn ad1tdc4(&mut self) -> AD1TDC4_W {
        AD1TDC4_W { w: self }
    }
    #[doc = "Bit 25 - AD1TDC3"]
    #[inline(always)]
    pub fn ad1tdc3(&mut self) -> AD1TDC3_W {
        AD1TDC3_W { w: self }
    }
    #[doc = "Bit 24 - AD1TDC2"]
    #[inline(always)]
    pub fn ad1tdc2(&mut self) -> AD1TDC2_W {
        AD1TDC2_W { w: self }
    }
    #[doc = "Bit 23 - AD1TCPER"]
    #[inline(always)]
    pub fn ad1tcper(&mut self) -> AD1TCPER_W {
        AD1TCPER_W { w: self }
    }
    #[doc = "Bit 22 - AD1TCC4"]
    #[inline(always)]
    pub fn ad1tcc4(&mut self) -> AD1TCC4_W {
        AD1TCC4_W { w: self }
    }
    #[doc = "Bit 21 - AD1TCC3"]
    #[inline(always)]
    pub fn ad1tcc3(&mut self) -> AD1TCC3_W {
        AD1TCC3_W { w: self }
    }
    #[doc = "Bit 20 - AD1TCC2"]
    #[inline(always)]
    pub fn ad1tcc2(&mut self) -> AD1TCC2_W {
        AD1TCC2_W { w: self }
    }
    #[doc = "Bit 19 - AD1TBRST"]
    #[inline(always)]
    pub fn ad1tbrst(&mut self) -> AD1TBRST_W {
        AD1TBRST_W { w: self }
    }
    #[doc = "Bit 18 - AD1TBPER"]
    #[inline(always)]
    pub fn ad1tbper(&mut self) -> AD1TBPER_W {
        AD1TBPER_W { w: self }
    }
    #[doc = "Bit 17 - AD1TBC4"]
    #[inline(always)]
    pub fn ad1tbc4(&mut self) -> AD1TBC4_W {
        AD1TBC4_W { w: self }
    }
    #[doc = "Bit 16 - AD1TBC3"]
    #[inline(always)]
    pub fn ad1tbc3(&mut self) -> AD1TBC3_W {
        AD1TBC3_W { w: self }
    }
    #[doc = "Bit 15 - AD1TBC2"]
    #[inline(always)]
    pub fn ad1tbc2(&mut self) -> AD1TBC2_W {
        AD1TBC2_W { w: self }
    }
    #[doc = "Bit 14 - AD1TARST"]
    #[inline(always)]
    pub fn ad1tarst(&mut self) -> AD1TARST_W {
        AD1TARST_W { w: self }
    }
    #[doc = "Bit 13 - AD1TAPER"]
    #[inline(always)]
    pub fn ad1taper(&mut self) -> AD1TAPER_W {
        AD1TAPER_W { w: self }
    }
    #[doc = "Bit 12 - AD1TAC4"]
    #[inline(always)]
    pub fn ad1tac4(&mut self) -> AD1TAC4_W {
        AD1TAC4_W { w: self }
    }
    #[doc = "Bit 11 - AD1TAC3"]
    #[inline(always)]
    pub fn ad1tac3(&mut self) -> AD1TAC3_W {
        AD1TAC3_W { w: self }
    }
    #[doc = "Bit 10 - AD1TAC2"]
    #[inline(always)]
    pub fn ad1tac2(&mut self) -> AD1TAC2_W {
        AD1TAC2_W { w: self }
    }
    #[doc = "Bit 9 - AD1EEV5"]
    #[inline(always)]
    pub fn ad1eev5(&mut self) -> AD1EEV5_W {
        AD1EEV5_W { w: self }
    }
    #[doc = "Bit 8 - AD1EEV4"]
    #[inline(always)]
    pub fn ad1eev4(&mut self) -> AD1EEV4_W {
        AD1EEV4_W { w: self }
    }
    #[doc = "Bit 7 - AD1EEV3"]
    #[inline(always)]
    pub fn ad1eev3(&mut self) -> AD1EEV3_W {
        AD1EEV3_W { w: self }
    }
    #[doc = "Bit 6 - AD1EEV2"]
    #[inline(always)]
    pub fn ad1eev2(&mut self) -> AD1EEV2_W {
        AD1EEV2_W { w: self }
    }
    #[doc = "Bit 5 - AD1EEV1"]
    #[inline(always)]
    pub fn ad1eev1(&mut self) -> AD1EEV1_W {
        AD1EEV1_W { w: self }
    }
    #[doc = "Bit 4 - AD1MPER"]
    #[inline(always)]
    pub fn ad1mper(&mut self) -> AD1MPER_W {
        AD1MPER_W { w: self }
    }
    #[doc = "Bit 3 - AD1MC4"]
    #[inline(always)]
    pub fn ad1mc4(&mut self) -> AD1MC4_W {
        AD1MC4_W { w: self }
    }
    #[doc = "Bit 2 - AD1MC3"]
    #[inline(always)]
    pub fn ad1mc3(&mut self) -> AD1MC3_W {
        AD1MC3_W { w: self }
    }
    #[doc = "Bit 1 - AD1MC2"]
    #[inline(always)]
    pub fn ad1mc2(&mut self) -> AD1MC2_W {
        AD1MC2_W { w: self }
    }
    #[doc = "Bit 0 - AD1MC1"]
    #[inline(always)]
    pub fn ad1mc1(&mut self) -> AD1MC1_W {
        AD1MC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trigger 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc3r](index.html) module"]
pub struct ADC3R_SPEC;
impl crate::RegisterSpec for ADC3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc3r::R](R) reader structure"]
impl crate::Readable for ADC3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc3r::W](W) writer structure"]
impl crate::Writable for ADC3R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC3R to value 0"]
impl crate::Resettable for ADC3R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
