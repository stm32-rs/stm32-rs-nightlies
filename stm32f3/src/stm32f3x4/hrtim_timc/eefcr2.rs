#[doc = "Reader of register EEFCR2"]
pub type R = crate::R<u32, super::EEFCR2>;
#[doc = "Writer for register EEFCR2"]
pub type W = crate::W<u32, super::EEFCR2>;
#[doc = "Register EEFCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EEFCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Event 10 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE10FLTR_A {
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
impl From<EE10FLTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EE10FLTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EE10FLTR`"]
pub type EE10FLTR_R = crate::R<u8, EE10FLTR_A>;
impl EE10FLTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE10FLTR_A {
        match self.bits {
            0 => EE10FLTR_A::DISABLED,
            1 => EE10FLTR_A::BLANKRESETTOCOMPARE1,
            2 => EE10FLTR_A::BLANKRESETTOCOMPARE2,
            3 => EE10FLTR_A::BLANKRESETTOCOMPARE3,
            4 => EE10FLTR_A::BLANKRESETTOCOMPARE4,
            5 => EE10FLTR_A::BLANKTIMFLTR1,
            6 => EE10FLTR_A::BLANKTIMFLTR2,
            7 => EE10FLTR_A::BLANKTIMFLTR3,
            8 => EE10FLTR_A::BLANKTIMFLTR4,
            9 => EE10FLTR_A::BLANKTIMFLTR5,
            10 => EE10FLTR_A::BLANKTIMFLTR6,
            11 => EE10FLTR_A::BLANKTIMFLTR7,
            12 => EE10FLTR_A::BLANKTIMFLTR8,
            13 => EE10FLTR_A::WINDOWRESETTOCOMPARE2,
            14 => EE10FLTR_A::WINDOWRESETTOCOMPARE3,
            15 => EE10FLTR_A::WINDOWTIMWIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE10FLTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE1`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE10FLTR_A::BLANKRESETTOCOMPARE1
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE2`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE10FLTR_A::BLANKRESETTOCOMPARE2
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE3`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE10FLTR_A::BLANKRESETTOCOMPARE3
    }
    #[doc = "Checks if the value of the field is `BLANKRESETTOCOMPARE4`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE10FLTR_A::BLANKRESETTOCOMPARE4
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR1`"]
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR1
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR2`"]
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR2
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR3`"]
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR3
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR4`"]
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR4
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR5`"]
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR5
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR6`"]
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR6
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR7`"]
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR7
    }
    #[doc = "Checks if the value of the field is `BLANKTIMFLTR8`"]
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE10FLTR_A::BLANKTIMFLTR8
    }
    #[doc = "Checks if the value of the field is `WINDOWRESETTOCOMPARE2`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE10FLTR_A::WINDOWRESETTOCOMPARE2
    }
    #[doc = "Checks if the value of the field is `WINDOWRESETTOCOMPARE3`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE10FLTR_A::WINDOWRESETTOCOMPARE3
    }
    #[doc = "Checks if the value of the field is `WINDOWTIMWIN`"]
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE10FLTR_A::WINDOWTIMWIN
    }
}
#[doc = "Write proxy for field `EE10FLTR`"]
pub struct EE10FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10FLTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "External Event 10 latch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE10LTCH_A {
    #[doc = "0: Event is ignored if it happens during a blank, or passed through during a window"]
    DISABLED = 0,
    #[doc = "1: Event is latched and delayed till the end of the blanking or windowing period"]
    ENABLED = 1,
}
impl From<EE10LTCH_A> for bool {
    #[inline(always)]
    fn from(variant: EE10LTCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EE10LTCH`"]
pub type EE10LTCH_R = crate::R<bool, EE10LTCH_A>;
impl EE10LTCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE10LTCH_A {
        match self.bits {
            false => EE10LTCH_A::DISABLED,
            true => EE10LTCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE10LTCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE10LTCH_A::ENABLED
    }
}
#[doc = "Write proxy for field `EE10LTCH`"]
pub struct EE10LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10LTCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "External Event 9 filter"]
pub type EE9FLTR_A = EE10FLTR_A;
#[doc = "Reader of field `EE9FLTR`"]
pub type EE9FLTR_R = crate::R<u8, EE10FLTR_A>;
#[doc = "Write proxy for field `EE9FLTR`"]
pub struct EE9FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9FLTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "External Event 9 latch"]
pub type EE9LTCH_A = EE10LTCH_A;
#[doc = "Reader of field `EE9LTCH`"]
pub type EE9LTCH_R = crate::R<bool, EE10LTCH_A>;
#[doc = "Write proxy for field `EE9LTCH`"]
pub struct EE9LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9LTCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "External Event 8 filter"]
pub type EE8FLTR_A = EE10FLTR_A;
#[doc = "Reader of field `EE8FLTR`"]
pub type EE8FLTR_R = crate::R<u8, EE10FLTR_A>;
#[doc = "Write proxy for field `EE8FLTR`"]
pub struct EE8FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8FLTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
#[doc = "External Event 8 latch"]
pub type EE8LTCH_A = EE10LTCH_A;
#[doc = "Reader of field `EE8LTCH`"]
pub type EE8LTCH_R = crate::R<bool, EE10LTCH_A>;
#[doc = "Write proxy for field `EE8LTCH`"]
pub struct EE8LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8LTCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "External Event 7 filter"]
pub type EE7FLTR_A = EE10FLTR_A;
#[doc = "Reader of field `EE7FLTR`"]
pub type EE7FLTR_R = crate::R<u8, EE10FLTR_A>;
#[doc = "Write proxy for field `EE7FLTR`"]
pub struct EE7FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7FLTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "External Event 7 latch"]
pub type EE7LTCH_A = EE10LTCH_A;
#[doc = "Reader of field `EE7LTCH`"]
pub type EE7LTCH_R = crate::R<bool, EE10LTCH_A>;
#[doc = "Write proxy for field `EE7LTCH`"]
pub struct EE7LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7LTCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "External Event 6 filter"]
pub type EE6FLTR_A = EE10FLTR_A;
#[doc = "Reader of field `EE6FLTR`"]
pub type EE6FLTR_R = crate::R<u8, EE10FLTR_A>;
#[doc = "Write proxy for field `EE6FLTR`"]
pub struct EE6FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6FLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6FLTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10FLTR_A::DISABLED)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKRESETTOCOMPARE4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE10FLTR_A::BLANKTIMFLTR8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWRESETTOCOMPARE3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE10FLTR_A::WINDOWTIMWIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "External Event 6 latch"]
pub type EE6LTCH_A = EE10LTCH_A;
#[doc = "Reader of field `EE6LTCH`"]
pub type EE6LTCH_R = crate::R<bool, EE10LTCH_A>;
#[doc = "Write proxy for field `EE6LTCH`"]
pub struct EE6LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6LTCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6LTCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::DISABLED)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE10LTCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W {
        EE10FLTR_W { w: self }
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W {
        EE10LTCH_W { w: self }
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W {
        EE9FLTR_W { w: self }
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W {
        EE9LTCH_W { w: self }
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W {
        EE8FLTR_W { w: self }
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W {
        EE8LTCH_W { w: self }
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W {
        EE7FLTR_W { w: self }
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W {
        EE7LTCH_W { w: self }
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W {
        EE6FLTR_W { w: self }
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W {
        EE6LTCH_W { w: self }
    }
}
