#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Comparator 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1EN {
    #[doc = "0: Comparator 1 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 1 enabled"]
    Enabled = 1,
}
impl From<COMP1EN> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub type COMP1EN_R = crate::BitReader<COMP1EN>;
impl COMP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1EN {
        match self.bits {
            false => COMP1EN::Disabled,
            true => COMP1EN::Enabled,
        }
    }
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN::Disabled
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN::Enabled
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub type COMP1EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP1EN>;
impl<'a, REG> COMP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Disabled)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Enabled)
    }
}
#[doc = "Comparator 1 non inverting input DAC switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1SW1 {
    #[doc = "0: Switch open"]
    Open = 0,
    #[doc = "1: Switch closed"]
    Closed = 1,
}
impl From<COMP1SW1> for bool {
    #[inline(always)]
    fn from(variant: COMP1SW1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1SW1` reader - Comparator 1 non inverting input DAC switch"]
pub type COMP1SW1_R = crate::BitReader<COMP1SW1>;
impl COMP1SW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1SW1 {
        match self.bits {
            false => COMP1SW1::Open,
            true => COMP1SW1::Closed,
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == COMP1SW1::Open
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == COMP1SW1::Closed
    }
}
#[doc = "Field `COMP1SW1` writer - Comparator 1 non inverting input DAC switch"]
pub type COMP1SW1_W<'a, REG> = crate::BitWriter<'a, REG, COMP1SW1>;
impl<'a, REG> COMP1SW1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1SW1::Open)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1SW1::Closed)
    }
}
#[doc = "Comparator 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1MODE {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<COMP1MODE> for u8 {
    #[inline(always)]
    fn from(variant: COMP1MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP1MODE {
    type Ux = u8;
}
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub type COMP1MODE_R = crate::FieldReader<COMP1MODE>;
impl COMP1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1MODE {
        match self.bits {
            0 => COMP1MODE::HighSpeed,
            1 => COMP1MODE::MediumSpeed,
            2 => COMP1MODE::LowSpeed,
            3 => COMP1MODE::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP1MODE::HighSpeed
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP1MODE::MediumSpeed
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP1MODE::LowSpeed
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP1MODE::VeryLowSpeed
    }
}
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub type COMP1MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP1MODE>;
impl<'a, REG> COMP1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::VeryLowSpeed)
    }
}
#[doc = "Comparator 1 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1INSEL {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    Comp1Inm4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    Comp1Inm5 = 5,
    #[doc = "6: COMP1_INM6 (PA0)"]
    Comp1Inm6 = 6,
}
impl From<COMP1INSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP1INSEL {
    type Ux = u8;
}
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub type COMP1INSEL_R = crate::FieldReader<COMP1INSEL>;
impl COMP1INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP1INSEL> {
        match self.bits {
            0 => Some(COMP1INSEL::OneQuarterVref),
            1 => Some(COMP1INSEL::OneHalfVref),
            2 => Some(COMP1INSEL::ThreeQuarterVref),
            3 => Some(COMP1INSEL::Vref),
            4 => Some(COMP1INSEL::Comp1Inm4),
            5 => Some(COMP1INSEL::Comp1Inm5),
            6 => Some(COMP1INSEL::Comp1Inm6),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP1INSEL::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP1INSEL::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP1INSEL::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP1INSEL::Vref
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn is_comp1_inm4(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm4
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn is_comp1_inm5(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm5
    }
    #[doc = "COMP1_INM6 (PA0)"]
    #[inline(always)]
    pub fn is_comp1_inm6(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm6
    }
}
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub type COMP1INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP1INSEL>;
impl<'a, REG> COMP1INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Vref)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp1_inm4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp1_inm5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm5)
    }
    #[doc = "COMP1_INM6 (PA0)"]
    #[inline(always)]
    pub fn comp1_inm6(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm6)
    }
}
#[doc = "Comparator 1 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1OUTSEL {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 Input capture 1"]
    Timer1inputCapture1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    Timer1ocrefClearInput = 3,
    #[doc = "4: Timer 2 input capture 4"]
    Timer2inputCapture4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    Timer2ocrefClearInput = 5,
    #[doc = "6: Timer 3 input capture 1"]
    Timer3inputCapture1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    Timer3ocrefClearInput = 7,
}
impl From<COMP1OUTSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP1OUTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP1OUTSEL {
    type Ux = u8;
}
#[doc = "Field `COMP1OUTSEL` reader - Comparator 1 output selection"]
pub type COMP1OUTSEL_R = crate::FieldReader<COMP1OUTSEL>;
impl COMP1OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1OUTSEL {
        match self.bits {
            0 => COMP1OUTSEL::NoSelection,
            1 => COMP1OUTSEL::Timer1breakInput,
            2 => COMP1OUTSEL::Timer1inputCapture1,
            3 => COMP1OUTSEL::Timer1ocrefClearInput,
            4 => COMP1OUTSEL::Timer2inputCapture4,
            5 => COMP1OUTSEL::Timer2ocrefClearInput,
            6 => COMP1OUTSEL::Timer3inputCapture1,
            7 => COMP1OUTSEL::Timer3ocrefClearInput,
            _ => unreachable!(),
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP1OUTSEL::NoSelection
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer1breakInput
    }
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP1OUTSEL::Timer1inputCapture1
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer1ocrefClearInput
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP1OUTSEL::Timer2inputCapture4
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer2ocrefClearInput
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP1OUTSEL::Timer3inputCapture1
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP1OUTSEL` writer - Comparator 1 output selection"]
pub type COMP1OUTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, COMP1OUTSEL>;
impl<'a, REG> COMP1OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1breakInput)
    }
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1inputCapture1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1ocrefClearInput)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer3ocrefClearInput)
    }
}
#[doc = "Comparator 1 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1POL {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP1POL> for bool {
    #[inline(always)]
    fn from(variant: COMP1POL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub type COMP1POL_R = crate::BitReader<COMP1POL>;
impl COMP1POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1POL {
        match self.bits {
            false => COMP1POL::NotInverted,
            true => COMP1POL::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POL::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POL::Inverted
    }
}
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub type COMP1POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP1POL>;
impl<'a, REG> COMP1POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POL::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POL::Inverted)
    }
}
#[doc = "Comparator 1 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1HYST {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<COMP1HYST> for u8 {
    #[inline(always)]
    fn from(variant: COMP1HYST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP1HYST {
    type Ux = u8;
}
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub type COMP1HYST_R = crate::FieldReader<COMP1HYST>;
impl COMP1HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1HYST {
        match self.bits {
            0 => COMP1HYST::NoHysteresis,
            1 => COMP1HYST::LowHysteresis,
            2 => COMP1HYST::MediumHysteresis,
            3 => COMP1HYST::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP1HYST::NoHysteresis
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP1HYST::LowHysteresis
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP1HYST::MediumHysteresis
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP1HYST::HighHysteresis
    }
}
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub type COMP1HYST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP1HYST>;
impl<'a, REG> COMP1HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::HighHysteresis)
    }
}
#[doc = "Comparator 1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1OUT {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP1OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP1OUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub type COMP1OUT_R = crate::BitReader<COMP1OUT>;
impl COMP1OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1OUT {
        match self.bits {
            false => COMP1OUT::Low,
            true => COMP1OUT::High,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP1OUT::Low
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP1OUT::High
    }
}
#[doc = "Comparator 1 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LOCK {
    #[doc = "0: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    Locked = 1,
}
impl From<COMP1LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub type COMP1LOCK_R = crate::BitReader<COMP1LOCK>;
impl COMP1LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1LOCK {
        match self.bits {
            false => COMP1LOCK::Unlocked,
            true => COMP1LOCK::Locked,
        }
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP1LOCK::Unlocked
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP1LOCK::Locked
    }
}
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub type COMP1LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP1LOCK>;
impl<'a, REG> COMP1LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LOCK::Unlocked)
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LOCK::Locked)
    }
}
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN {
    #[doc = "0: Comparator 2 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 2 enabled"]
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
    #[doc = "Comparator 2 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN::Disabled
    }
    #[doc = "Comparator 2 enabled"]
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
    #[doc = "Comparator 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Disabled)
    }
    #[doc = "Comparator 2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Enabled)
    }
}
#[doc = "Comparator 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2MODE {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VeryLowSpeed = 3,
}
impl From<COMP2MODE> for u8 {
    #[inline(always)]
    fn from(variant: COMP2MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2MODE {
    type Ux = u8;
}
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader<COMP2MODE>;
impl COMP2MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2MODE {
        match self.bits {
            0 => COMP2MODE::HighSpeed,
            1 => COMP2MODE::MediumSpeed,
            2 => COMP2MODE::LowSpeed,
            3 => COMP2MODE::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP2MODE::HighSpeed
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP2MODE::MediumSpeed
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP2MODE::LowSpeed
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP2MODE::VeryLowSpeed
    }
}
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP2MODE>;
impl<'a, REG> COMP2MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2MODE::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2MODE::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2MODE::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2MODE::VeryLowSpeed)
    }
}
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INSEL {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    Comp2Inm4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    Comp2Inm5 = 5,
    #[doc = "6: COMP1_INM6 (PA2)"]
    Comp2Inm6 = 6,
}
impl From<COMP2INSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2INSEL {
    type Ux = u8;
}
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INSEL_R = crate::FieldReader<COMP2INSEL>;
impl COMP2INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP2INSEL> {
        match self.bits {
            0 => Some(COMP2INSEL::OneQuarterVref),
            1 => Some(COMP2INSEL::OneHalfVref),
            2 => Some(COMP2INSEL::ThreeQuarterVref),
            3 => Some(COMP2INSEL::Vref),
            4 => Some(COMP2INSEL::Comp2Inm4),
            5 => Some(COMP2INSEL::Comp2Inm5),
            6 => Some(COMP2INSEL::Comp2Inm6),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INSEL::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INSEL::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INSEL::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INSEL::Vref
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn is_comp2_inm4(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm4
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn is_comp2_inm5(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm5
    }
    #[doc = "COMP1_INM6 (PA2)"]
    #[inline(always)]
    pub fn is_comp2_inm6(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm6
    }
}
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2INSEL>;
impl<'a, REG> COMP2INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Vref)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp2_inm4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp2_inm5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm5)
    }
    #[doc = "COMP1_INM6 (PA2)"]
    #[inline(always)]
    pub fn comp2_inm6(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm6)
    }
}
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WNDWEN {
    #[doc = "0: Window mode disabled"]
    Disabled = 0,
    #[doc = "1: Window mode enabled"]
    Enabled = 1,
}
impl From<WNDWEN> for bool {
    #[inline(always)]
    fn from(variant: WNDWEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WNDWEN` reader - Window mode enable"]
pub type WNDWEN_R = crate::BitReader<WNDWEN>;
impl WNDWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WNDWEN {
        match self.bits {
            false => WNDWEN::Disabled,
            true => WNDWEN::Enabled,
        }
    }
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDWEN::Disabled
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDWEN::Enabled
    }
}
#[doc = "Field `WNDWEN` writer - Window mode enable"]
pub type WNDWEN_W<'a, REG> = crate::BitWriter<'a, REG, WNDWEN>;
impl<'a, REG> WNDWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDWEN::Disabled)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDWEN::Enabled)
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
    #[doc = "2: Timer 1 Input capture 1"]
    Timer1inputCapture1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    Timer1ocrefClearInput = 3,
    #[doc = "4: Timer 2 input capture 4"]
    Timer2inputCapture4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    Timer2ocrefClearInput = 5,
    #[doc = "6: Timer 3 input capture 1"]
    Timer3inputCapture1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    Timer3ocrefClearInput = 7,
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
    pub const fn variant(&self) -> COMP2OUTSEL {
        match self.bits {
            0 => COMP2OUTSEL::NoSelection,
            1 => COMP2OUTSEL::Timer1breakInput,
            2 => COMP2OUTSEL::Timer1inputCapture1,
            3 => COMP2OUTSEL::Timer1ocrefClearInput,
            4 => COMP2OUTSEL::Timer2inputCapture4,
            5 => COMP2OUTSEL::Timer2ocrefClearInput,
            6 => COMP2OUTSEL::Timer3inputCapture1,
            7 => COMP2OUTSEL::Timer3ocrefClearInput,
            _ => unreachable!(),
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
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer1inputCapture1
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer1ocrefClearInput
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL::Timer2inputCapture4
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer2ocrefClearInput
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer3inputCapture1
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, COMP2OUTSEL>;
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
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1inputCapture1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1ocrefClearInput)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCrefclear input"]
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
#[doc = "Comparator 2 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2HYST {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<COMP2HYST> for u8 {
    #[inline(always)]
    fn from(variant: COMP2HYST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2HYST {
    type Ux = u8;
}
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader<COMP2HYST>;
impl COMP2HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2HYST {
        match self.bits {
            0 => COMP2HYST::NoHysteresis,
            1 => COMP2HYST::LowHysteresis,
            2 => COMP2HYST::MediumHysteresis,
            3 => COMP2HYST::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP2HYST::NoHysteresis
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP2HYST::LowHysteresis
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP2HYST::MediumHysteresis
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP2HYST::HighHysteresis
    }
}
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP2HYST>;
impl<'a, REG> COMP2HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2HYST::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2HYST::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2HYST::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2HYST::HighHysteresis)
    }
}
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
    #[doc = "0: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
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
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK::Unlocked
    }
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
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
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Unlocked)
    }
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    pub fn comp1sw1(&self) -> COMP1SW1_R {
        COMP1SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
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
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<CSRrs> {
        COMP1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    #[must_use]
    pub fn comp1sw1(&mut self) -> COMP1SW1_W<CSRrs> {
        COMP1SW1_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<CSRrs> {
        COMP1MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W<CSRrs> {
        COMP1INSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<CSRrs> {
        COMP1OUTSEL_W::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp1pol(&mut self) -> COMP1POL_W<CSRrs> {
        COMP1POL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<CSRrs> {
        COMP1HYST_W::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<CSRrs> {
        COMP1LOCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<CSRrs> {
        COMP2EN_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<CSRrs> {
        COMP2MODE_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<CSRrs> {
        COMP2INSEL_W::new(self, 20)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wndwen(&mut self) -> WNDWEN_W<CSRrs> {
        WNDWEN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<CSRrs> {
        COMP2OUTSEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<CSRrs> {
        COMP2POL_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<CSRrs> {
        COMP2HYST_W::new(self, 28)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<CSRrs> {
        COMP2LOCK_W::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
