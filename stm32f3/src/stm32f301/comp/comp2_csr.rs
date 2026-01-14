///Register `COMP2_CSR` reader
pub type R = crate::R<COMP2_CSRrs>;
///Register `COMP2_CSR` writer
pub type W = crate::W<COMP2_CSRrs>;
/**Comparator 2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN {
    ///0: Comparator disabled
    Disabled = 0,
    ///1: Comparator enabled
    Enabled = 1,
}
impl From<COMP2EN> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP2EN` reader - Comparator 2 enable
pub type COMP2EN_R = crate::BitReader<COMP2EN>;
impl COMP2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP2EN {
        match self.bits {
            false => COMP2EN::Disabled,
            true => COMP2EN::Enabled,
        }
    }
    ///Comparator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN::Disabled
    }
    ///Comparator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN::Enabled
    }
}
///Field `COMP2EN` writer - Comparator 2 enable
pub type COMP2EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP2EN>;
impl<'a, REG> COMP2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Disabled)
    }
    ///Comparator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Enabled)
    }
}
///Field `COMP2_INP_DAC` reader - Comparator 2 non inverting input connection to DAC output
pub type COMP2_INP_DAC_R = crate::BitReader;
///Field `COMP2_INP_DAC` writer - Comparator 2 non inverting input connection to DAC output
pub type COMP2_INP_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Comparator 2 inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INMSEL {
    ///0: 1/4 of VRefint
    OneQuarterVref = 0,
    ///1: 1/2 of VRefint
    OneHalfVref = 1,
    ///2: 3/4 of VRefint
    ThreeQuarterVref = 2,
    ///3: VRefint
    Vref = 3,
    ///4: PA4 or DAC1_CH1 output if enabled
    Pa4Dac1Ch1 = 4,
    ///5: DAC1_CH2
    Dac1Ch2 = 5,
    ///6: PA2
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
impl crate::IsEnum for COMP2INMSEL {}
///Field `COMP2INMSEL` reader - Comparator 2 inverting input selection
pub type COMP2INMSEL_R = crate::FieldReader<COMP2INMSEL>;
impl COMP2INMSEL_R {
    ///Get enumerated values variant
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
    ///1/4 of VRefint
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL::OneQuarterVref
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INMSEL::OneHalfVref
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL::ThreeQuarterVref
    }
    ///VRefint
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INMSEL::Vref
    }
    ///PA4 or DAC1_CH1 output if enabled
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP2INMSEL::Pa4Dac1Ch1
    }
    ///DAC1_CH2
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP2INMSEL::Dac1Ch2
    }
    ///PA2
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INMSEL::Pa2
    }
}
///Field `COMP2INMSEL` writer - Comparator 2 inverting input selection
pub type COMP2INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2INMSEL>;
impl<'a, REG> COMP2INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Vref)
    }
    ///PA4 or DAC1_CH1 output if enabled
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Pa4Dac1Ch1)
    }
    ///DAC1_CH2
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Dac1Ch2)
    }
    ///PA2
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INMSEL::Pa2)
    }
}
/**Comparator 2 output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2OUTSEL {
    ///0: No selection
    NoSelection = 0,
    ///1: Timer 1 break input
    Timer1breakInput = 1,
    ///2: Timer 1 break input 2
    Timer1breakInput2 = 2,
    ///6: Timer 1 OCREF_CLR input
    Timer1ocrefClearInput = 6,
    ///7: Timer 1 input capture 1
    Timer1inputCapture1 = 7,
    ///8: Timer 2 input capture 4
    Timer2inputCapture4 = 8,
    ///9: Timer 2 OCREF_CLR input
    Timer2ocrefClearInput = 9,
    ///10: Timer 3 input capture 1
    Timer3inputCapture1 = 10,
    ///11: Timer 3 OCREF_CLR input
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
impl crate::IsEnum for COMP2OUTSEL {}
///Field `COMP2OUTSEL` reader - Comparator 2 output selection
pub type COMP2OUTSEL_R = crate::FieldReader<COMP2OUTSEL>;
impl COMP2OUTSEL_R {
    ///Get enumerated values variant
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
    ///No selection
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL::NoSelection
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer1breakInput
    }
    ///Timer 1 break input 2
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP2OUTSEL::Timer1breakInput2
    }
    ///Timer 1 OCREF_CLR input
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer1ocrefClearInput
    }
    ///Timer 1 input capture 1
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer1inputCapture1
    }
    ///Timer 2 input capture 4
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL::Timer2inputCapture4
    }
    ///Timer 2 OCREF_CLR input
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer2ocrefClearInput
    }
    ///Timer 3 input capture 1
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL::Timer3inputCapture1
    }
    ///Timer 3 OCREF_CLR input
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL::Timer3ocrefClearInput
    }
}
///Field `COMP2OUTSEL` writer - Comparator 2 output selection
pub type COMP2OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COMP2OUTSEL>;
impl<'a, REG> COMP2OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No selection
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::NoSelection)
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1breakInput)
    }
    ///Timer 1 break input 2
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1breakInput2)
    }
    ///Timer 1 OCREF_CLR input
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1ocrefClearInput)
    }
    ///Timer 1 input capture 1
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer1inputCapture1)
    }
    ///Timer 2 input capture 4
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2inputCapture4)
    }
    ///Timer 2 OCREF_CLR input
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer2ocrefClearInput)
    }
    ///Timer 3 input capture 1
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer3inputCapture1)
    }
    ///Timer 3 OCREF_CLR input
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2OUTSEL::Timer3ocrefClearInput)
    }
}
/**Comparator 2 output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POL {
    ///0: Output is not inverted
    NotInverted = 0,
    ///1: Output is inverted
    Inverted = 1,
}
impl From<COMP2POL> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP2POL` reader - Comparator 2 output polarity
pub type COMP2POL_R = crate::BitReader<COMP2POL>;
impl COMP2POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP2POL {
        match self.bits {
            false => COMP2POL::NotInverted,
            true => COMP2POL::Inverted,
        }
    }
    ///Output is not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POL::NotInverted
    }
    ///Output is inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL::Inverted
    }
}
///Field `COMP2POL` writer - Comparator 2 output polarity
pub type COMP2POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP2POL>;
impl<'a, REG> COMP2POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output is not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POL::NotInverted)
    }
    ///Output is inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POL::Inverted)
    }
}
/**Comparator 2 blanking source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2_BLANKING {
    ///0: No blanking
    NoBlanking = 0,
    ///1: TIM1 OC5 selected as blanking source
    Tim1oc5 = 1,
    ///2: TIM2 OC3 selected as blanking source
    Tim2oc3 = 2,
    ///3: TIM3 OC3 selected as blanking source
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
impl crate::IsEnum for COMP2_BLANKING {}
///Field `COMP2_BLANKING` reader - Comparator 2 blanking source
pub type COMP2_BLANKING_R = crate::FieldReader<COMP2_BLANKING>;
impl COMP2_BLANKING_R {
    ///Get enumerated values variant
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
    ///No blanking
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP2_BLANKING::NoBlanking
    }
    ///TIM1 OC5 selected as blanking source
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == COMP2_BLANKING::Tim1oc5
    }
    ///TIM2 OC3 selected as blanking source
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == COMP2_BLANKING::Tim2oc3
    }
    ///TIM3 OC3 selected as blanking source
    #[inline(always)]
    pub fn is_tim3oc3(&self) -> bool {
        *self == COMP2_BLANKING::Tim3oc3
    }
}
///Field `COMP2_BLANKING` writer - Comparator 2 blanking source
pub type COMP2_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2_BLANKING>;
impl<'a, REG> COMP2_BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No blanking
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::NoBlanking)
    }
    ///TIM1 OC5 selected as blanking source
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim1oc5)
    }
    ///TIM2 OC3 selected as blanking source
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim2oc3)
    }
    ///TIM3 OC3 selected as blanking source
    #[inline(always)]
    pub fn tim3oc3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2_BLANKING::Tim3oc3)
    }
}
/**Comparator 2 output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2OUT {
    ///0: Non-inverting input below inverting input
    Low = 0,
    ///1: Non-inverting input above inverting input
    High = 1,
}
impl From<COMP2OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP2OUT` reader - Comparator 2 output
pub type COMP2OUT_R = crate::BitReader<COMP2OUT>;
impl COMP2OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP2OUT {
        match self.bits {
            false => COMP2OUT::Low,
            true => COMP2OUT::High,
        }
    }
    ///Non-inverting input below inverting input
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP2OUT::Low
    }
    ///Non-inverting input above inverting input
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT::High
    }
}
/**Comparator 2 lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
}
impl From<COMP2LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP2LOCK` reader - Comparator 2 lock
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK>;
impl COMP2LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP2LOCK {
        match self.bits {
            false => COMP2LOCK::Unlocked,
            true => COMP2LOCK::Locked,
        }
    }
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK::Unlocked
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK::Locked
    }
}
///Field `COMP2LOCK` writer - Comparator 2 lock
pub type COMP2LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP2LOCK>;
impl<'a, REG> COMP2LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LOCK::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 2 enable
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator 2 non inverting input connection to DAC output
    #[inline(always)]
    pub fn comp2_inp_dac(&self) -> COMP2_INP_DAC_R {
        COMP2_INP_DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - Comparator 2 inverting input selection
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 10:13 - Comparator 2 output selection
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 2 output polarity
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 18:20 - Comparator 2 blanking source
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
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
        f.debug_struct("COMP2_CSR")
            .field("comp2en", &self.comp2en())
            .field("comp2inmsel", &self.comp2inmsel())
            .field("comp2outsel", &self.comp2outsel())
            .field("comp2pol", &self.comp2pol())
            .field("comp2_blanking", &self.comp2_blanking())
            .field("comp2out", &self.comp2out())
            .field("comp2lock", &self.comp2lock())
            .field("comp2_inp_dac", &self.comp2_inp_dac())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W<'_, COMP2_CSRrs> {
        COMP2EN_W::new(self, 0)
    }
    ///Bit 1 - Comparator 2 non inverting input connection to DAC output
    #[inline(always)]
    pub fn comp2_inp_dac(&mut self) -> COMP2_INP_DAC_W<'_, COMP2_CSRrs> {
        COMP2_INP_DAC_W::new(self, 1)
    }
    ///Bits 4:6 - Comparator 2 inverting input selection
    #[inline(always)]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W<'_, COMP2_CSRrs> {
        COMP2INMSEL_W::new(self, 4)
    }
    ///Bits 10:13 - Comparator 2 output selection
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<'_, COMP2_CSRrs> {
        COMP2OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 2 output polarity
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W<'_, COMP2_CSRrs> {
        COMP2POL_W::new(self, 15)
    }
    ///Bits 18:20 - Comparator 2 blanking source
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<'_, COMP2_CSRrs> {
        COMP2_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 2 lock
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<'_, COMP2_CSRrs> {
        COMP2LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#COMP:COMP2_CSR)*/
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp2_csr::R`](R) reader structure
impl crate::Readable for COMP2_CSRrs {}
///`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSRrs {}
