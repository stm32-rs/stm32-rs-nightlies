#[doc = "Register `EEFCR1` reader"]
pub struct R(crate::R<EEFCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFCR1` writer"]
pub struct W(crate::W<EEFCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFCR1_SPEC>;
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
impl From<crate::W<EEFCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Event 5 filter"]
pub type EE5FLTR_A = EE1FLTR_A;
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub type EE5FLTR_R = EE1FLTR_R;
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub struct EE5FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5FLTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE5FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE5FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE5FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE5FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE5FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "External Event 5 latch"]
pub type EE5LTCH_A = EE1LTCH_A;
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub type EE5LTCH_R = EE1LTCH_R;
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub struct EE5LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5LTCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE5LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE5LTCH_A::ENABLED)
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
#[doc = "External Event 4 filter"]
pub type EE4FLTR_A = EE1FLTR_A;
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub type EE4FLTR_R = EE1FLTR_R;
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub struct EE4FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4FLTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE4FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE4FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE4FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE4FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE4FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "External Event 4 latch"]
pub type EE4LTCH_A = EE1LTCH_A;
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub type EE4LTCH_R = EE1LTCH_R;
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub struct EE4LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4LTCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE4LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE4LTCH_A::ENABLED)
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
#[doc = "External Event 3 filter"]
pub type EE3FLTR_A = EE1FLTR_A;
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub type EE3FLTR_R = EE1FLTR_R;
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub struct EE3FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3FLTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE3FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE3FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE3FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE3FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE3FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "External Event 3 latch"]
pub type EE3LTCH_A = EE1LTCH_A;
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub type EE3LTCH_R = EE1LTCH_R;
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub struct EE3LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3LTCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE3LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE3LTCH_A::ENABLED)
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
#[doc = "External Event 2 filter"]
pub type EE2FLTR_A = EE1FLTR_A;
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub type EE2FLTR_R = EE1FLTR_R;
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub struct EE2FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2FLTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE2FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE2FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE2FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE2FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE2FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "External Event 2 latch"]
pub type EE2LTCH_A = EE1LTCH_A;
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub type EE2LTCH_R = EE1LTCH_R;
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub struct EE2LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2LTCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE2LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE2LTCH_A::ENABLED)
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
#[doc = "External Event 1 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE1FLTR_A {
    #[doc = "0: No filtering"]
    DISABLED = 0,
    #[doc = "1: Blanking from counter reset/roll-over to Compare 1"]
    BLANKRESETTOCOMPARE1 = 1,
    #[doc = "2: Blanking from counter reset/roll-over to Compare 2"]
    BLANKRESETTOCOMPARE2 = 2,
    #[doc = "3: Blanking from counter reset/roll-over to Compare 3"]
    BLANKRESETTOCOMPARE3 = 3,
    #[doc = "4: Blanking from counter reset/roll-over to Compare 4"]
    BLANKRESETTOCOMPARE4 = 4,
    #[doc = "5: Blanking from another timing unit: TIMFLTR1 source"]
    BLANKTIMFLTR1 = 5,
    #[doc = "6: Blanking from another timing unit: TIMFLTR2 source"]
    BLANKTIMFLTR2 = 6,
    #[doc = "7: Blanking from another timing unit: TIMFLTR3 source"]
    BLANKTIMFLTR3 = 7,
    #[doc = "8: Blanking from another timing unit: TIMFLTR4 source"]
    BLANKTIMFLTR4 = 8,
    #[doc = "9: Blanking from another timing unit: TIMFLTR5 source"]
    BLANKTIMFLTR5 = 9,
    #[doc = "10: Blanking from another timing unit: TIMFLTR6 source"]
    BLANKTIMFLTR6 = 10,
    #[doc = "11: Blanking from another timing unit: TIMFLTR7 source"]
    BLANKTIMFLTR7 = 11,
    #[doc = "12: Blanking from another timing unit: TIMFLTR8 source"]
    BLANKTIMFLTR8 = 12,
    #[doc = "13: Windowing from counter reset/roll-over to compare 2"]
    WINDOWRESETTOCOMPARE2 = 13,
    #[doc = "14: Windowing from counter reset/roll-over to compare 3"]
    WINDOWRESETTOCOMPARE3 = 14,
    #[doc = "15: Windowing from another timing unit: TIMWIN source"]
    WINDOWTIMWIN = 15,
}
impl From<EE1FLTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EE1FLTR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub struct EE1FLTR_R(crate::FieldReader<u8, EE1FLTR_A>);
impl EE1FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE1FLTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE1FLTR_A {
        match self.bits {
            0 => EE1FLTR_A::DISABLED,
            1 => EE1FLTR_A::BLANKRESETTOCOMPARE1,
            2 => EE1FLTR_A::BLANKRESETTOCOMPARE2,
            3 => EE1FLTR_A::BLANKRESETTOCOMPARE3,
            4 => EE1FLTR_A::BLANKRESETTOCOMPARE4,
            5 => EE1FLTR_A::BLANKTIMFLTR1,
            6 => EE1FLTR_A::BLANKTIMFLTR2,
            7 => EE1FLTR_A::BLANKTIMFLTR3,
            8 => EE1FLTR_A::BLANKTIMFLTR4,
            9 => EE1FLTR_A::BLANKTIMFLTR5,
            10 => EE1FLTR_A::BLANKTIMFLTR6,
            11 => EE1FLTR_A::BLANKTIMFLTR7,
            12 => EE1FLTR_A::BLANKTIMFLTR8,
            13 => EE1FLTR_A::WINDOWRESETTOCOMPARE2,
            14 => EE1FLTR_A::WINDOWRESETTOCOMPARE3,
            15 => EE1FLTR_A::WINDOWTIMWIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EE1FLTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE1`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        **self == EE1FLTR_A::BLANKRESETTOCOMPARE1
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE2`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        **self == EE1FLTR_A::BLANKRESETTOCOMPARE2
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE3`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        **self == EE1FLTR_A::BLANKRESETTOCOMPARE3
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE4`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        **self == EE1FLTR_A::BLANKRESETTOCOMPARE4
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR1`"]
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR1
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR2`"]
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR2
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR3`"]
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR3
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR4`"]
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR4
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR5`"]
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR5
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR6`"]
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR6
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR7`"]
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR7
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR8`"]
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        **self == EE1FLTR_A::BLANKTIMFLTR8
    }
    #[doc = "Checks if the value of the field is `WINDOWRESETTOCOMPARE2`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        **self == EE1FLTR_A::WINDOWRESETTOCOMPARE2
    }
    #[doc = "Checks if the value of the field is `WINDOWRESETTOCOMPARE3`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        **self == EE1FLTR_A::WINDOWRESETTOCOMPARE3
    }
    #[doc = "Checks if the value of the field is `WINDOWTIMWIN`"]
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        **self == EE1FLTR_A::WINDOWTIMWIN
    }
}
impl core::ops::Deref for EE1FLTR_R {
    type Target = crate::FieldReader<u8, EE1FLTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub struct EE1FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1FLTR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE1FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE1FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE1FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "External Event 1 latch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE1LTCH_A {
    #[doc = "0: Event is ignored if it happens during a blank, or passed through during a window"]
    DISABLED = 0,
    #[doc = "1: Event is latched and delayed till the end of the blanking or windowing period"]
    ENABLED = 1,
}
impl From<EE1LTCH_A> for bool {
    #[inline(always)]
    fn from(variant: EE1LTCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub struct EE1LTCH_R(crate::FieldReader<bool, EE1LTCH_A>);
impl EE1LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE1LTCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE1LTCH_A {
        match self.bits {
            false => EE1LTCH_A::DISABLED,
            true => EE1LTCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EE1LTCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EE1LTCH_A::ENABLED
    }
}
impl core::ops::Deref for EE1LTCH_R {
    type Target = crate::FieldReader<bool, EE1LTCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub struct EE1LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1LTCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE1LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE1LTCH_A::ENABLED)
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
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W {
        EE5FLTR_W { w: self }
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W {
        EE5LTCH_W { w: self }
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W {
        EE4FLTR_W { w: self }
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W {
        EE4LTCH_W { w: self }
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W {
        EE3FLTR_W { w: self }
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W {
        EE3LTCH_W { w: self }
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W {
        EE2FLTR_W { w: self }
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W {
        EE2LTCH_W { w: self }
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W {
        EE1FLTR_W { w: self }
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W {
        EE1LTCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefcr1](index.html) module"]
pub struct EEFCR1_SPEC;
impl crate::RegisterSpec for EEFCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefcr1::R](R) reader structure"]
impl crate::Readable for EEFCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefcr1::W](W) writer structure"]
impl crate::Writable for EEFCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFCR1 to value 0"]
impl crate::Resettable for EEFCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
