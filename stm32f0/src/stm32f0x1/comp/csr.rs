///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Comparator 1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1EN {
    ///0: Comparator X disabled
    Disabled = 0,
    ///1: Comparator X enabled
    Enabled = 1,
}
impl From<COMP1EN> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP1EN` reader - Comparator 1 enable
pub type COMP1EN_R = crate::BitReader<COMP1EN>;
impl COMP1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP1EN {
        match self.bits {
            false => COMP1EN::Disabled,
            true => COMP1EN::Enabled,
        }
    }
    ///Comparator X disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN::Disabled
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN::Enabled
    }
}
///Field `COMP1EN` writer - Comparator 1 enable
pub type COMP1EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP1EN>;
impl<'a, REG> COMP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Disabled)
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Enabled)
    }
}
/**Comparator 1 non inverting input DAC switch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1SW1 {
    ///0: Switch open
    Open = 0,
    ///1: Switch closed
    Closed = 1,
}
impl From<COMP1SW1> for bool {
    #[inline(always)]
    fn from(variant: COMP1SW1) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP1SW1` reader - Comparator 1 non inverting input DAC switch
pub type COMP1SW1_R = crate::BitReader<COMP1SW1>;
impl COMP1SW1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP1SW1 {
        match self.bits {
            false => COMP1SW1::Open,
            true => COMP1SW1::Closed,
        }
    }
    ///Switch open
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == COMP1SW1::Open
    }
    ///Switch closed
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == COMP1SW1::Closed
    }
}
///Field `COMP1SW1` writer - Comparator 1 non inverting input DAC switch
pub type COMP1SW1_W<'a, REG> = crate::BitWriter<'a, REG, COMP1SW1>;
impl<'a, REG> COMP1SW1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Switch open
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1SW1::Open)
    }
    ///Switch closed
    #[inline(always)]
    pub fn closed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1SW1::Closed)
    }
}
/**Comparator 1 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1MODE {
    ///0: High speed / full power
    HighSpeed = 0,
    ///1: Medium speed / medium power
    MediumSpeed = 1,
    ///2: Low speed / low power
    LowSpeed = 2,
    ///3: Very-low speed / ultra-low power
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
impl crate::IsEnum for COMP1MODE {}
///Field `COMP1MODE` reader - Comparator 1 mode
pub type COMP1MODE_R = crate::FieldReader<COMP1MODE>;
impl COMP1MODE_R {
    ///Get enumerated values variant
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
    ///High speed / full power
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP1MODE::HighSpeed
    }
    ///Medium speed / medium power
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP1MODE::MediumSpeed
    }
    ///Low speed / low power
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP1MODE::LowSpeed
    }
    ///Very-low speed / ultra-low power
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP1MODE::VeryLowSpeed
    }
}
///Field `COMP1MODE` writer - Comparator 1 mode
pub type COMP1MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COMP1MODE, crate::Safe>;
impl<'a, REG> COMP1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///High speed / full power
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::HighSpeed)
    }
    ///Medium speed / medium power
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::MediumSpeed)
    }
    ///Low speed / low power
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::LowSpeed)
    }
    ///Very-low speed / ultra-low power
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1MODE::VeryLowSpeed)
    }
}
/**Comparator 1 inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1INSEL {
    ///0: 1/4 of VRefint
    OneQuarterVref = 0,
    ///1: 1/2 of VRefint
    OneHalfVref = 1,
    ///2: 3/4 of VRefint
    ThreeQuarterVref = 2,
    ///3: VRefint
    Vref = 3,
    ///4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    Comp1Inm4 = 4,
    ///5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    Comp1Inm5 = 5,
    ///6: COMP1_INM6 (PA0)
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
impl crate::IsEnum for COMP1INSEL {}
///Field `COMP1INSEL` reader - Comparator 1 inverting input selection
pub type COMP1INSEL_R = crate::FieldReader<COMP1INSEL>;
impl COMP1INSEL_R {
    ///Get enumerated values variant
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
    ///1/4 of VRefint
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP1INSEL::OneQuarterVref
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP1INSEL::OneHalfVref
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP1INSEL::ThreeQuarterVref
    }
    ///VRefint
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP1INSEL::Vref
    }
    ///COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    #[inline(always)]
    pub fn is_comp1_inm4(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm4
    }
    ///COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    #[inline(always)]
    pub fn is_comp1_inm5(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm5
    }
    ///COMP1_INM6 (PA0)
    #[inline(always)]
    pub fn is_comp1_inm6(&self) -> bool {
        *self == COMP1INSEL::Comp1Inm6
    }
}
///Field `COMP1INSEL` writer - Comparator 1 inverting input selection
pub type COMP1INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP1INSEL>;
impl<'a, REG> COMP1INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Vref)
    }
    ///COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    #[inline(always)]
    pub fn comp1_inm4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm4)
    }
    ///COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    #[inline(always)]
    pub fn comp1_inm5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm5)
    }
    ///COMP1_INM6 (PA0)
    #[inline(always)]
    pub fn comp1_inm6(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INSEL::Comp1Inm6)
    }
}
/**Comparator 1 output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1OUTSEL {
    ///0: No selection
    NoSelection = 0,
    ///1: Timer 1 break input
    Timer1breakInput = 1,
    ///2: Timer 1 Input capture 1
    Timer1inputCapture1 = 2,
    ///3: Timer 1 OCrefclear input
    Timer1ocrefClearInput = 3,
    ///4: Timer 2 input capture 4
    Timer2inputCapture4 = 4,
    ///5: Timer 2 OCrefclear input
    Timer2ocrefClearInput = 5,
    ///6: Timer 3 input capture 1
    Timer3inputCapture1 = 6,
    ///7: Timer 3 OCrefclear input
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
impl crate::IsEnum for COMP1OUTSEL {}
///Field `COMP1OUTSEL` reader - Comparator 1 output selection
pub type COMP1OUTSEL_R = crate::FieldReader<COMP1OUTSEL>;
impl COMP1OUTSEL_R {
    ///Get enumerated values variant
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
    ///No selection
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP1OUTSEL::NoSelection
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer1breakInput
    }
    ///Timer 1 Input capture 1
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP1OUTSEL::Timer1inputCapture1
    }
    ///Timer 1 OCrefclear input
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer1ocrefClearInput
    }
    ///Timer 2 input capture 4
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP1OUTSEL::Timer2inputCapture4
    }
    ///Timer 2 OCrefclear input
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer2ocrefClearInput
    }
    ///Timer 3 input capture 1
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP1OUTSEL::Timer3inputCapture1
    }
    ///Timer 3 OCrefclear input
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL::Timer3ocrefClearInput
    }
}
///Field `COMP1OUTSEL` writer - Comparator 1 output selection
pub type COMP1OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP1OUTSEL, crate::Safe>;
impl<'a, REG> COMP1OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No selection
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::NoSelection)
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1breakInput)
    }
    ///Timer 1 Input capture 1
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1inputCapture1)
    }
    ///Timer 1 OCrefclear input
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer1ocrefClearInput)
    }
    ///Timer 2 input capture 4
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer2inputCapture4)
    }
    ///Timer 2 OCrefclear input
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer2ocrefClearInput)
    }
    ///Timer 3 input capture 1
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer3inputCapture1)
    }
    ///Timer 3 OCrefclear input
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1OUTSEL::Timer3ocrefClearInput)
    }
}
/**Comparator 1 output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1POL {
    ///0: Output is not inverted
    NotInverted = 0,
    ///1: Output is inverted
    Inverted = 1,
}
impl From<COMP1POL> for bool {
    #[inline(always)]
    fn from(variant: COMP1POL) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP1POL` reader - Comparator 1 output polarity
pub type COMP1POL_R = crate::BitReader<COMP1POL>;
impl COMP1POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP1POL {
        match self.bits {
            false => COMP1POL::NotInverted,
            true => COMP1POL::Inverted,
        }
    }
    ///Output is not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POL::NotInverted
    }
    ///Output is inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POL::Inverted
    }
}
///Field `COMP1POL` writer - Comparator 1 output polarity
pub type COMP1POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP1POL>;
impl<'a, REG> COMP1POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output is not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POL::NotInverted)
    }
    ///Output is inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POL::Inverted)
    }
}
/**Comparator 1 hysteresis

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1HYST {
    ///0: No hysteresis
    NoHysteresis = 0,
    ///1: Low hysteresis
    LowHysteresis = 1,
    ///2: Medium hysteresis
    MediumHysteresis = 2,
    ///3: High hysteresis
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
impl crate::IsEnum for COMP1HYST {}
///Field `COMP1HYST` reader - Comparator 1 hysteresis
pub type COMP1HYST_R = crate::FieldReader<COMP1HYST>;
impl COMP1HYST_R {
    ///Get enumerated values variant
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
    ///No hysteresis
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP1HYST::NoHysteresis
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP1HYST::LowHysteresis
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP1HYST::MediumHysteresis
    }
    ///High hysteresis
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP1HYST::HighHysteresis
    }
}
///Field `COMP1HYST` writer - Comparator 1 hysteresis
pub type COMP1HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COMP1HYST, crate::Safe>;
impl<'a, REG> COMP1HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No hysteresis
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::NoHysteresis)
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::LowHysteresis)
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::MediumHysteresis)
    }
    ///High hysteresis
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1HYST::HighHysteresis)
    }
}
/**Comparator 1 output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1OUT {
    ///0: Non-inverting input below inverting input
    Low = 0,
    ///1: Non-inverting input above inverting input
    High = 1,
}
impl From<COMP1OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP1OUT) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP1OUT` reader - Comparator 1 output
pub type COMP1OUT_R = crate::BitReader<COMP1OUT>;
impl COMP1OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP1OUT {
        match self.bits {
            false => COMP1OUT::Low,
            true => COMP1OUT::High,
        }
    }
    ///Non-inverting input below inverting input
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP1OUT::Low
    }
    ///Non-inverting input above inverting input
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP1OUT::High
    }
}
/**Comparator 1 lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LOCK {
    ///0: Comparator X CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator X CSR bits are read-only
    Locked = 1,
}
impl From<COMP1LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP1LOCK` reader - Comparator 1 lock
pub type COMP1LOCK_R = crate::BitReader<COMP1LOCK>;
impl COMP1LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP1LOCK {
        match self.bits {
            false => COMP1LOCK::Unlocked,
            true => COMP1LOCK::Locked,
        }
    }
    ///Comparator X CSR bits are read-write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP1LOCK::Unlocked
    }
    ///Comparator X CSR bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP1LOCK::Locked
    }
}
///Field `COMP1LOCK` writer - Comparator 1 lock
pub type COMP1LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP1LOCK>;
impl<'a, REG> COMP1LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LOCK::Unlocked)
    }
    ///Comparator X CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LOCK::Locked)
    }
}
///Field `COMP2EN` reader - Comparator 2 enable
pub use COMP1EN_R as COMP2EN_R;
///Field `COMP2EN` writer - Comparator 2 enable
pub use COMP1EN_W as COMP2EN_W;
///Field `COMP2MODE` reader - Comparator 2 mode
pub use COMP1MODE_R as COMP2MODE_R;
///Field `COMP2MODE` writer - Comparator 2 mode
pub use COMP1MODE_W as COMP2MODE_W;
/**Comparator 2 inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INSEL {
    ///0: 1/4 of VRefint
    OneQuarterVref = 0,
    ///1: 1/2 of VRefint
    OneHalfVref = 1,
    ///2: 3/4 of VRefint
    ThreeQuarterVref = 2,
    ///3: VRefint
    Vref = 3,
    ///4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    Comp2Inm4 = 4,
    ///5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    Comp2Inm5 = 5,
    ///6: COMP1_INM6 (PA2)
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
impl crate::IsEnum for COMP2INSEL {}
///Field `COMP2INSEL` reader - Comparator 2 inverting input selection
pub type COMP2INSEL_R = crate::FieldReader<COMP2INSEL>;
impl COMP2INSEL_R {
    ///Get enumerated values variant
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
    ///1/4 of VRefint
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INSEL::OneQuarterVref
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INSEL::OneHalfVref
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INSEL::ThreeQuarterVref
    }
    ///VRefint
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INSEL::Vref
    }
    ///COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    #[inline(always)]
    pub fn is_comp2_inm4(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm4
    }
    ///COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    #[inline(always)]
    pub fn is_comp2_inm5(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm5
    }
    ///COMP1_INM6 (PA2)
    #[inline(always)]
    pub fn is_comp2_inm6(&self) -> bool {
        *self == COMP2INSEL::Comp2Inm6
    }
}
///Field `COMP2INSEL` writer - Comparator 2 inverting input selection
pub type COMP2INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2INSEL>;
impl<'a, REG> COMP2INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Vref)
    }
    ///COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
    #[inline(always)]
    pub fn comp2_inm4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm4)
    }
    ///COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
    #[inline(always)]
    pub fn comp2_inm5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm5)
    }
    ///COMP1_INM6 (PA2)
    #[inline(always)]
    pub fn comp2_inm6(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INSEL::Comp2Inm6)
    }
}
/**Window mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WNDWEN {
    ///0: Window mode disabled
    Disabled = 0,
    ///1: Window mode enabled
    Enabled = 1,
}
impl From<WNDWEN> for bool {
    #[inline(always)]
    fn from(variant: WNDWEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WNDWEN` reader - Window mode enable
pub type WNDWEN_R = crate::BitReader<WNDWEN>;
impl WNDWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WNDWEN {
        match self.bits {
            false => WNDWEN::Disabled,
            true => WNDWEN::Enabled,
        }
    }
    ///Window mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDWEN::Disabled
    }
    ///Window mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDWEN::Enabled
    }
}
///Field `WNDWEN` writer - Window mode enable
pub type WNDWEN_W<'a, REG> = crate::BitWriter<'a, REG, WNDWEN>;
impl<'a, REG> WNDWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Window mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDWEN::Disabled)
    }
    ///Window mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WNDWEN::Enabled)
    }
}
///Field `COMP2HYST` reader - Comparator 2 hysteresis
pub use COMP1HYST_R as COMP2HYST_R;
///Field `COMP2HYST` writer - Comparator 2 hysteresis
pub use COMP1HYST_W as COMP2HYST_W;
///Field `COMP2LOCK` reader - Comparator 2 lock
pub use COMP1LOCK_R as COMP2LOCK_R;
///Field `COMP2LOCK` writer - Comparator 2 lock
pub use COMP1LOCK_W as COMP2LOCK_W;
///Field `COMP2OUTSEL` reader - Comparator 2 output selection
pub use COMP1OUTSEL_R as COMP2OUTSEL_R;
///Field `COMP2OUTSEL` writer - Comparator 2 output selection
pub use COMP1OUTSEL_W as COMP2OUTSEL_W;
///Field `COMP2OUT` reader - Comparator 2 output
pub use COMP1OUT_R as COMP2OUT_R;
///Field `COMP2POL` reader - Comparator 2 output polarity
pub use COMP1POL_R as COMP2POL_R;
///Field `COMP2POL` writer - Comparator 2 output polarity
pub use COMP1POL_W as COMP2POL_W;
impl R {
    ///Bit 0 - Comparator 1 enable
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator 1 non inverting input DAC switch
    #[inline(always)]
    pub fn comp1sw1(&self) -> COMP1SW1_R {
        COMP1SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Comparator 1 mode
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 1 inverting input selection
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Comparator 1 output selection
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Comparator 1 output polarity
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Comparator 1 hysteresis
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Comparator 1 output
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 1 lock
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Comparator 2 enable
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:19 - Comparator 2 mode
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:22 - Comparator 2 inverting input selection
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - Window mode enable
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Comparator 2 output selection
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - Comparator 2 output polarity
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - Comparator 2 hysteresis
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Comparator 2 output
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 2 lock
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("comp1en", &self.comp1en())
            .field("comp1mode", &self.comp1mode())
            .field("comp1insel", &self.comp1insel())
            .field("comp1outsel", &self.comp1outsel())
            .field("comp1pol", &self.comp1pol())
            .field("comp1hyst", &self.comp1hyst())
            .field("comp1out", &self.comp1out())
            .field("comp1lock", &self.comp1lock())
            .field("comp2en", &self.comp2en())
            .field("comp2mode", &self.comp2mode())
            .field("comp2insel", &self.comp2insel())
            .field("wndwen", &self.wndwen())
            .field("comp2outsel", &self.comp2outsel())
            .field("comp2pol", &self.comp2pol())
            .field("comp2hyst", &self.comp2hyst())
            .field("comp2out", &self.comp2out())
            .field("comp2lock", &self.comp2lock())
            .field("comp1sw1", &self.comp1sw1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W<'_, CSRrs> {
        COMP1EN_W::new(self, 0)
    }
    ///Bit 1 - Comparator 1 non inverting input DAC switch
    #[inline(always)]
    pub fn comp1sw1(&mut self) -> COMP1SW1_W<'_, CSRrs> {
        COMP1SW1_W::new(self, 1)
    }
    ///Bits 2:3 - Comparator 1 mode
    #[inline(always)]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<'_, CSRrs> {
        COMP1MODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 1 inverting input selection
    #[inline(always)]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W<'_, CSRrs> {
        COMP1INSEL_W::new(self, 4)
    }
    ///Bits 8:10 - Comparator 1 output selection
    #[inline(always)]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<'_, CSRrs> {
        COMP1OUTSEL_W::new(self, 8)
    }
    ///Bit 11 - Comparator 1 output polarity
    #[inline(always)]
    pub fn comp1pol(&mut self) -> COMP1POL_W<'_, CSRrs> {
        COMP1POL_W::new(self, 11)
    }
    ///Bits 12:13 - Comparator 1 hysteresis
    #[inline(always)]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<'_, CSRrs> {
        COMP1HYST_W::new(self, 12)
    }
    ///Bit 15 - Comparator 1 lock
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<'_, CSRrs> {
        COMP1LOCK_W::new(self, 15)
    }
    ///Bit 16 - Comparator 2 enable
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W<'_, CSRrs> {
        COMP2EN_W::new(self, 16)
    }
    ///Bits 18:19 - Comparator 2 mode
    #[inline(always)]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<'_, CSRrs> {
        COMP2MODE_W::new(self, 18)
    }
    ///Bits 20:22 - Comparator 2 inverting input selection
    #[inline(always)]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<'_, CSRrs> {
        COMP2INSEL_W::new(self, 20)
    }
    ///Bit 23 - Window mode enable
    #[inline(always)]
    pub fn wndwen(&mut self) -> WNDWEN_W<'_, CSRrs> {
        WNDWEN_W::new(self, 23)
    }
    ///Bits 24:26 - Comparator 2 output selection
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<'_, CSRrs> {
        COMP2OUTSEL_W::new(self, 24)
    }
    ///Bit 27 - Comparator 2 output polarity
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W<'_, CSRrs> {
        COMP2POL_W::new(self, 27)
    }
    ///Bits 28:29 - Comparator 2 hysteresis
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<'_, CSRrs> {
        COMP2HYST_W::new(self, 28)
    }
    ///Bit 31 - Comparator 2 lock
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<'_, CSRrs> {
        COMP2LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#COMP:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
