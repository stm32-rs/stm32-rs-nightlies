#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<COMP2_CSRrs>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<COMP2_CSRrs>;
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<COMP2EN> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader<COMP2EN>;
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2EN {
        match self.bits {
            false => COMP2EN::Disabled,
            true => COMP2EN::Enabled,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN::Disabled
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN::Enabled
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP2EN>;
impl<'a, REG> COMP2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Enabled)
    }
}
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INMSEL {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: PA4 or DAC1_CH1 output if enabled"]
    Pa4Dac1Ch1 = 4,
    #[doc = "5: DAC1_CH2"]
    Dac1Ch2 = 5,
    #[doc = "6: PA2"]
    Pa2 = 6,
}
impl From<COMP2INMSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2INMSEL {
    type Ux = u8;
}
#[doc = "Field `COMP2INMSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_R = crate::FieldReader<COMP2INMSEL>;
impl COMP2INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP2INMSEL> {
        match self.bits {
            0 => Some(COMP2INMSEL::OneQuarterVref),
            1 => Some(COMP2INMSEL::OneHalfVref),
            2 => Some(COMP2INMSEL::ThreeQuarterVref),
            3 => Some(COMP2INMSEL::Vref),
            4 => Some(COMP2INMSEL::Pa4Dac1Ch1),
            5 => Some(COMP2INMSEL::Dac1Ch2),
            6 => Some(COMP2INMSEL::Pa2),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INMSEL::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INMSEL::Vref
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP2INMSEL::Pa4Dac1Ch1
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP2INMSEL::Dac1Ch2
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INMSEL::Pa2
    }
}
#[doc = "Field `COMP2INMSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2INMSEL>;
impl<'a, REG> COMP2INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Vref)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Pa4Dac1Ch1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Dac1Ch2)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Pa2)
    }
}
#[doc = "Comparator 2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2OUTSEL {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 break input 2"]
    Timer1breakInput2 = 2,
    #[doc = "6: Timer 1 OCREF_CLR input"]
    Timer1ocrefClearInput = 6,
    #[doc = "7: Timer 1 input capture 1"]
    Timer1inputCapture1 = 7,
    #[doc = "8: Timer 2 input capture 4"]
    Timer2inputCapture4 = 8,
    #[doc = "9: Timer 2 OCREF_CLR input"]
    Timer2ocrefClearInput = 9,
    #[doc = "10: Timer 3 input capture 1"]
    Timer3inputCapture1 = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    Timer3ocrefClearInput = 11,
}
impl From<COMP2OUTSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2OUTSEL {
    type Ux = u8;
}
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader<COMP2OUTSEL>;
impl COMP2OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP2OUTSEL> {
        match self.bits {
            0 => Some(COMP2OUTSEL::NoSelection),
            1 => Some(COMP2OUTSEL::Timer1breakInput),
            2 => Some(COMP2OUTSEL::Timer1breakInput2),
            6 => Some(COMP2OUTSEL::Timer1ocrefClearInput),
            7 => Some(COMP2OUTSEL::Timer1inputCapture1),
            8 => Some(COMP2OUTSEL::Timer2inputCapture4),
            9 => Some(COMP2OUTSEL::Timer2ocrefClearInput),
            10 => Some(COMP2OUTSEL::Timer3inputCapture1),
            11 => Some(COMP2OUTSEL::Timer3ocrefClearInput),
            _ => None,
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL::NoSelection
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer1breakInput
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP2OUTSEL::Timer1breakInput2
    }
    #[doc = "Timer 1 OCREF_CLR input"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer1ocrefClearInput
    }
    #[doc = "Timer 1 input capture 1"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer1inputCapture1
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL::Timer2inputCapture4
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer2ocrefClearInput
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer3inputCapture1
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COMP2OUTSEL>;
impl<'a, REG> COMP2OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1breakInput)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1breakInput2)
    }
    #[doc = "Timer 1 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1ocrefClearInput)
    }
    #[doc = "Timer 1 input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1inputCapture1)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer3ocrefClearInput)
    }
}
#[doc = "Comparator 2 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POL {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP2POL> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader<COMP2POL>;
impl COMP2POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2POL {
        match self.bits {
            false => COMP2POL::NotInverted,
            true => COMP2POL::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POL::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL::Inverted
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP2POL>;
impl<'a, REG> COMP2POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POL::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POL::Inverted)
    }
}
#[doc = "Comparator 2 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2_BLANKING {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    Tim1oc5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
    Tim2oc3 = 2,
    #[doc = "3: TIM3 OC3 selected as blanking source"]
    Tim3oc3 = 3,
}
impl From<COMP2_BLANKING> for u8 {
    #[inline(always)]
    fn from(variant: COMP2_BLANKING) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2_BLANKING {
    type Ux = u8;
}
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub type COMP2_BLANKING_R = crate::FieldReader<COMP2_BLANKING>;
impl COMP2_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP2_BLANKING> {
        match self.bits {
            0 => Some(COMP2_BLANKING::NoBlanking),
            1 => Some(COMP2_BLANKING::Tim1oc5),
            2 => Some(COMP2_BLANKING::Tim2oc3),
            3 => Some(COMP2_BLANKING::Tim3oc3),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP2_BLANKING::NoBlanking
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == COMP2_BLANKING::Tim1oc5
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == COMP2_BLANKING::Tim2oc3
    }
    #[doc = "TIM3 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim3oc3(&self) -> bool {
        *self == COMP2_BLANKING::Tim3oc3
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub type COMP2_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2_BLANKING>;
impl<'a, REG> COMP2_BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::NoBlanking)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim1oc5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim2oc3)
    }
    #[doc = "TIM3 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim3oc3)
    }
}
#[doc = "Field `COMP2INMSEL3` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_R = crate::BitReader;
#[doc = "Field `COMP2INMSEL3` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2OUT {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP2OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader<COMP2OUT>;
impl COMP2OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2OUT {
        match self.bits {
            false => COMP2OUT::Low,
            true => COMP2OUT::High,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP2OUT::Low
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT::High
    }
}
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<COMP2LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK>;
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2LOCK {
        match self.bits {
            false => COMP2LOCK::Unlocked,
            true => COMP2LOCK::Locked,
        }
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK::Unlocked
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK::Locked
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP2LOCK>;
impl<'a, REG> COMP2LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&self) -> COMP2INMSEL3_R {
        COMP2INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<COMP2_CSRrs> {
        COMP2EN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W<COMP2_CSRrs> {
        COMP2INMSEL_W::new(self, 4)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<COMP2_CSRrs> {
        COMP2OUTSEL_W::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<COMP2_CSRrs> {
        COMP2POL_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<COMP2_CSRrs> {
        COMP2_BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inmsel3(&mut self) -> COMP2INMSEL3_W<COMP2_CSRrs> {
        COMP2INMSEL3_W::new(self, 22)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<COMP2_CSRrs> {
        COMP2LOCK_W::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for COMP2_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSRrs {
    const RESET_VALUE: u32 = 0;
}
