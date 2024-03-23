#[doc = "Register `COMP4_CSR` reader"]
pub type R = crate::R<COMP4_CSRrs>;
#[doc = "Register `COMP4_CSR` writer"]
pub type W = crate::W<COMP4_CSRrs>;
#[doc = "Comparator 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP4EN {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<COMP4EN> for bool {
    #[inline(always)]
    fn from(variant: COMP4EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub type COMP4EN_R = crate::BitReader<COMP4EN>;
impl COMP4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP4EN {
        match self.bits {
            false => COMP4EN::Disabled,
            true => COMP4EN::Enabled,
        }
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP4EN::Disabled
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP4EN::Enabled
    }
}
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub type COMP4EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP4EN>;
impl<'a, REG> COMP4EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4EN::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4EN::Enabled)
    }
}
#[doc = "Field `COMP4MODE` reader - Comparator 4 mode"]
pub type COMP4MODE_R = crate::FieldReader;
#[doc = "Field `COMP4MODE` writer - Comparator 4 mode"]
pub type COMP4MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Comparator 4 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP4INMSEL {
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
    #[doc = "7: PB2"]
    Pb2 = 7,
}
impl From<COMP4INMSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP4INMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP4INMSEL {
    type Ux = u8;
}
#[doc = "Field `COMP4INMSEL` reader - Comparator 4 inverting input selection"]
pub type COMP4INMSEL_R = crate::FieldReader<COMP4INMSEL>;
impl COMP4INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP4INMSEL> {
        match self.bits {
            0 => Some(COMP4INMSEL::OneQuarterVref),
            1 => Some(COMP4INMSEL::OneHalfVref),
            2 => Some(COMP4INMSEL::ThreeQuarterVref),
            3 => Some(COMP4INMSEL::Vref),
            4 => Some(COMP4INMSEL::Pa4Dac1Ch1),
            5 => Some(COMP4INMSEL::Dac1Ch2),
            7 => Some(COMP4INMSEL::Pb2),
            _ => None,
        }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP4INMSEL::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP4INMSEL::Vref
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP4INMSEL::Pa4Dac1Ch1
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP4INMSEL::Dac1Ch2
    }
    #[doc = "PB2"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == COMP4INMSEL::Pb2
    }
}
#[doc = "Field `COMP4INMSEL` writer - Comparator 4 inverting input selection"]
pub type COMP4INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP4INMSEL>;
impl<'a, REG> COMP4INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::Vref)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::Pa4Dac1Ch1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::Dac1Ch2)
    }
    #[doc = "PB2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4INMSEL::Pb2)
    }
}
#[doc = "Field `COMP4INPSEL` reader - Comparator 4 non inverted input"]
pub type COMP4INPSEL_R = crate::BitReader;
#[doc = "Field `COMP4INPSEL` writer - Comparator 4 non inverted input"]
pub type COMP4INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator 4 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP4OUTSEL {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 break input 2"]
    Timer1breakInput2 = 2,
    #[doc = "6: Timer 3 input capture 3"]
    Timer3inputCapture3 = 6,
    #[doc = "8: Timer 15 input capture 2"]
    Timer15inputCapture2 = 8,
    #[doc = "10: Timer 15 OCREF_CLR input"]
    Timer15ocrefClearInput = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    Timer3ocrefClearInput = 11,
}
impl From<COMP4OUTSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP4OUTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP4OUTSEL {
    type Ux = u8;
}
#[doc = "Field `COMP4OUTSEL` reader - Comparator 4 output selection"]
pub type COMP4OUTSEL_R = crate::FieldReader<COMP4OUTSEL>;
impl COMP4OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP4OUTSEL> {
        match self.bits {
            0 => Some(COMP4OUTSEL::NoSelection),
            1 => Some(COMP4OUTSEL::Timer1breakInput),
            2 => Some(COMP4OUTSEL::Timer1breakInput2),
            6 => Some(COMP4OUTSEL::Timer3inputCapture3),
            8 => Some(COMP4OUTSEL::Timer15inputCapture2),
            10 => Some(COMP4OUTSEL::Timer15ocrefClearInput),
            11 => Some(COMP4OUTSEL::Timer3ocrefClearInput),
            _ => None,
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP4OUTSEL::NoSelection
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP4OUTSEL::Timer1breakInput
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP4OUTSEL::Timer1breakInput2
    }
    #[doc = "Timer 3 input capture 3"]
    #[inline(always)]
    pub fn is_timer3input_capture3(&self) -> bool {
        *self == COMP4OUTSEL::Timer3inputCapture3
    }
    #[doc = "Timer 15 input capture 2"]
    #[inline(always)]
    pub fn is_timer15input_capture2(&self) -> bool {
        *self == COMP4OUTSEL::Timer15inputCapture2
    }
    #[doc = "Timer 15 OCREF_CLR input"]
    #[inline(always)]
    pub fn is_timer15ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL::Timer15ocrefClearInput
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP4OUTSEL` writer - Comparator 4 output selection"]
pub type COMP4OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COMP4OUTSEL>;
impl<'a, REG> COMP4OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer1breakInput)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer1breakInput2)
    }
    #[doc = "Timer 3 input capture 3"]
    #[inline(always)]
    pub fn timer3input_capture3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer3inputCapture3)
    }
    #[doc = "Timer 15 input capture 2"]
    #[inline(always)]
    pub fn timer15input_capture2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer15inputCapture2)
    }
    #[doc = "Timer 15 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer15ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer15ocrefClearInput)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4OUTSEL::Timer3ocrefClearInput)
    }
}
#[doc = "Comparator 4 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP4POL {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP4POL> for bool {
    #[inline(always)]
    fn from(variant: COMP4POL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub type COMP4POL_R = crate::BitReader<COMP4POL>;
impl COMP4POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP4POL {
        match self.bits {
            false => COMP4POL::NotInverted,
            true => COMP4POL::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP4POL::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP4POL::Inverted
    }
}
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub type COMP4POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP4POL>;
impl<'a, REG> COMP4POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4POL::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4POL::Inverted)
    }
}
#[doc = "Field `COMP4HYST` reader - Comparator 4 hysteresis"]
pub type COMP4HYST_R = crate::FieldReader;
#[doc = "Field `COMP4HYST` writer - Comparator 4 hysteresis"]
pub type COMP4HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Comparator 4 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP4_BLANKING {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "1: TIM3 OC4 selected as blanking source"]
    Tim3oc4 = 1,
    #[doc = "3: TIM15 OC1 selected as blanking source"]
    Tim15oc1 = 3,
}
impl From<COMP4_BLANKING> for u8 {
    #[inline(always)]
    fn from(variant: COMP4_BLANKING) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP4_BLANKING {
    type Ux = u8;
}
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub type COMP4_BLANKING_R = crate::FieldReader<COMP4_BLANKING>;
impl COMP4_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP4_BLANKING> {
        match self.bits {
            0 => Some(COMP4_BLANKING::NoBlanking),
            1 => Some(COMP4_BLANKING::Tim3oc4),
            3 => Some(COMP4_BLANKING::Tim15oc1),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP4_BLANKING::NoBlanking
    }
    #[doc = "TIM3 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim3oc4(&self) -> bool {
        *self == COMP4_BLANKING::Tim3oc4
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        *self == COMP4_BLANKING::Tim15oc1
    }
}
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub type COMP4_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP4_BLANKING>;
impl<'a, REG> COMP4_BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4_BLANKING::NoBlanking)
    }
    #[doc = "TIM3 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4_BLANKING::Tim3oc4)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4_BLANKING::Tim15oc1)
    }
}
#[doc = "Comparator 4 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP4OUT {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP4OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP4OUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub type COMP4OUT_R = crate::BitReader<COMP4OUT>;
impl COMP4OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP4OUT {
        match self.bits {
            false => COMP4OUT::Low,
            true => COMP4OUT::High,
        }
    }
    #[doc = "Non-inverting input below inverting input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP4OUT::Low
    }
    #[doc = "Non-inverting input above inverting input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP4OUT::High
    }
}
#[doc = "Comparator 4 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP4LOCK {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<COMP4LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP4LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub type COMP4LOCK_R = crate::BitReader<COMP4LOCK>;
impl COMP4LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP4LOCK {
        match self.bits {
            false => COMP4LOCK::Unlocked,
            true => COMP4LOCK::Locked,
        }
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP4LOCK::Unlocked
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP4LOCK::Locked
    }
}
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub type COMP4LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP4LOCK>;
impl<'a, REG> COMP4LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4LOCK::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP4LOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> COMP4EN_R {
        COMP4EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> COMP4MODE_R {
        COMP4MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel(&self) -> COMP4INMSEL_R {
        COMP4INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> COMP4INPSEL_R {
        COMP4INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4outsel(&self) -> COMP4OUTSEL_R {
        COMP4OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> COMP4POL_R {
        COMP4POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> COMP4HYST_R {
        COMP4HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> COMP4_BLANKING_R {
        COMP4_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> COMP4OUT_R {
        COMP4OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> COMP4LOCK_R {
        COMP4LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp4en(&mut self) -> COMP4EN_W<COMP4_CSRrs> {
        COMP4EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp4mode(&mut self) -> COMP4MODE_W<COMP4_CSRrs> {
        COMP4MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp4inmsel(&mut self) -> COMP4INMSEL_W<COMP4_CSRrs> {
        COMP4INMSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    #[must_use]
    pub fn comp4inpsel(&mut self) -> COMP4INPSEL_W<COMP4_CSRrs> {
        COMP4INPSEL_W::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp4outsel(&mut self) -> COMP4OUTSEL_W<COMP4_CSRrs> {
        COMP4OUTSEL_W::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp4pol(&mut self) -> COMP4POL_W<COMP4_CSRrs> {
        COMP4POL_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp4hyst(&mut self) -> COMP4HYST_W<COMP4_CSRrs> {
        COMP4HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp4_blanking(&mut self) -> COMP4_BLANKING_W<COMP4_CSRrs> {
        COMP4_BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp4lock(&mut self) -> COMP4LOCK_W<COMP4_CSRrs> {
        COMP4LOCK_W::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp4_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp4_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP4_CSRrs;
impl crate::RegisterSpec for COMP4_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp4_csr::R`](R) reader structure"]
impl crate::Readable for COMP4_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp4_csr::W`](W) writer structure"]
impl crate::Writable for COMP4_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for COMP4_CSRrs {
    const RESET_VALUE: u32 = 0;
}
