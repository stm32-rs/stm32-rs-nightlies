///Register `COMP6_CSR` reader
pub type R = crate::R<COMP6_CSRrs>;
///Register `COMP6_CSR` writer
pub type W = crate::W<COMP6_CSRrs>;
/**Comparator 6 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6EN {
    ///0: Comparator disabled
    Disabled = 0,
    ///1: Comparator enabled
    Enabled = 1,
}
impl From<COMP6EN> for bool {
    #[inline(always)]
    fn from(variant: COMP6EN) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP6EN` reader - Comparator 6 enable
pub type COMP6EN_R = crate::BitReader<COMP6EN>;
impl COMP6EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP6EN {
        match self.bits {
            false => COMP6EN::Disabled,
            true => COMP6EN::Enabled,
        }
    }
    ///Comparator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP6EN::Disabled
    }
    ///Comparator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP6EN::Enabled
    }
}
///Field `COMP6EN` writer - Comparator 6 enable
pub type COMP6EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP6EN>;
impl<'a, REG> COMP6EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6EN::Disabled)
    }
    ///Comparator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6EN::Enabled)
    }
}
/**Comparator 6 inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6INMSEL {
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
    ///7: PB15
    Pb15 = 7,
}
impl From<COMP6INMSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP6INMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6INMSEL {
    type Ux = u8;
}
impl crate::IsEnum for COMP6INMSEL {}
///Field `COMP6INMSEL` reader - Comparator 6 inverting input selection
pub type COMP6INMSEL_R = crate::FieldReader<COMP6INMSEL>;
impl COMP6INMSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP6INMSEL> {
        match self.bits {
            0 => Some(COMP6INMSEL::OneQuarterVref),
            1 => Some(COMP6INMSEL::OneHalfVref),
            2 => Some(COMP6INMSEL::ThreeQuarterVref),
            3 => Some(COMP6INMSEL::Vref),
            4 => Some(COMP6INMSEL::Pa4Dac1Ch1),
            5 => Some(COMP6INMSEL::Dac1Ch2),
            7 => Some(COMP6INMSEL::Pb15),
            _ => None,
        }
    }
    ///1/4 of VRefint
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL::OneQuarterVref
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP6INMSEL::OneHalfVref
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL::ThreeQuarterVref
    }
    ///VRefint
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP6INMSEL::Vref
    }
    ///PA4 or DAC1_CH1 output if enabled
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP6INMSEL::Pa4Dac1Ch1
    }
    ///DAC1_CH2
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP6INMSEL::Dac1Ch2
    }
    ///PB15
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == COMP6INMSEL::Pb15
    }
}
///Field `COMP6INMSEL` writer - Comparator 6 inverting input selection
pub type COMP6INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP6INMSEL>;
impl<'a, REG> COMP6INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::Vref)
    }
    ///PA4 or DAC1_CH1 output if enabled
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::Pa4Dac1Ch1)
    }
    ///DAC1_CH2
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::Dac1Ch2)
    }
    ///PB15
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6INMSEL::Pb15)
    }
}
/**Comparator 6 output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6OUTSEL {
    ///0: No selection
    NoSelection = 0,
    ///1: Timer 1 break input
    Timer1breakInput = 1,
    ///2: Timer 1 break input 2
    Timer1breakInput2 = 2,
    ///6: Timer 2 input capture 2
    Timer2inputCapture2 = 6,
    ///8: Timer 2 OCREF_CLR input
    Timer2ocrefClearInput = 8,
    ///9: Timer 16 OCREF_CLR input
    Timer16ocrefClearInput = 9,
    ///10: Timer 16 input capture 1
    Timer16inputCapture1 = 10,
}
impl From<COMP6OUTSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP6OUTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6OUTSEL {
    type Ux = u8;
}
impl crate::IsEnum for COMP6OUTSEL {}
///Field `COMP6OUTSEL` reader - Comparator 6 output selection
pub type COMP6OUTSEL_R = crate::FieldReader<COMP6OUTSEL>;
impl COMP6OUTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP6OUTSEL> {
        match self.bits {
            0 => Some(COMP6OUTSEL::NoSelection),
            1 => Some(COMP6OUTSEL::Timer1breakInput),
            2 => Some(COMP6OUTSEL::Timer1breakInput2),
            6 => Some(COMP6OUTSEL::Timer2inputCapture2),
            8 => Some(COMP6OUTSEL::Timer2ocrefClearInput),
            9 => Some(COMP6OUTSEL::Timer16ocrefClearInput),
            10 => Some(COMP6OUTSEL::Timer16inputCapture1),
            _ => None,
        }
    }
    ///No selection
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP6OUTSEL::NoSelection
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP6OUTSEL::Timer1breakInput
    }
    ///Timer 1 break input 2
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP6OUTSEL::Timer1breakInput2
    }
    ///Timer 2 input capture 2
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        *self == COMP6OUTSEL::Timer2inputCapture2
    }
    ///Timer 2 OCREF_CLR input
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL::Timer2ocrefClearInput
    }
    ///Timer 16 OCREF_CLR input
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL::Timer16ocrefClearInput
    }
    ///Timer 16 input capture 1
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        *self == COMP6OUTSEL::Timer16inputCapture1
    }
}
///Field `COMP6OUTSEL` writer - Comparator 6 output selection
pub type COMP6OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COMP6OUTSEL>;
impl<'a, REG> COMP6OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No selection
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::NoSelection)
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer1breakInput)
    }
    ///Timer 1 break input 2
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer1breakInput2)
    }
    ///Timer 2 input capture 2
    #[inline(always)]
    pub fn timer2input_capture2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer2inputCapture2)
    }
    ///Timer 2 OCREF_CLR input
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer2ocrefClearInput)
    }
    ///Timer 16 OCREF_CLR input
    #[inline(always)]
    pub fn timer16ocref_clear_input(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer16ocrefClearInput)
    }
    ///Timer 16 input capture 1
    #[inline(always)]
    pub fn timer16input_capture1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6OUTSEL::Timer16inputCapture1)
    }
}
/**Comparator 6 output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6POL {
    ///0: Output is not inverted
    NotInverted = 0,
    ///1: Output is inverted
    Inverted = 1,
}
impl From<COMP6POL> for bool {
    #[inline(always)]
    fn from(variant: COMP6POL) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP6POL` reader - Comparator 6 output polarity
pub type COMP6POL_R = crate::BitReader<COMP6POL>;
impl COMP6POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP6POL {
        match self.bits {
            false => COMP6POL::NotInverted,
            true => COMP6POL::Inverted,
        }
    }
    ///Output is not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP6POL::NotInverted
    }
    ///Output is inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP6POL::Inverted
    }
}
///Field `COMP6POL` writer - Comparator 6 output polarity
pub type COMP6POL_W<'a, REG> = crate::BitWriter<'a, REG, COMP6POL>;
impl<'a, REG> COMP6POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output is not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6POL::NotInverted)
    }
    ///Output is inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6POL::Inverted)
    }
}
/**Comparator 6 blanking source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6_BLANKING {
    ///0: No blanking
    NoBlanking = 0,
    ///3: TIM2 OC4 selected as blanking source
    Tim2oc4 = 3,
    ///4: TIM15 OC2 selected as blanking source
    Tim15oc2 = 4,
}
impl From<COMP6_BLANKING> for u8 {
    #[inline(always)]
    fn from(variant: COMP6_BLANKING) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6_BLANKING {
    type Ux = u8;
}
impl crate::IsEnum for COMP6_BLANKING {}
///Field `COMP6_BLANKING` reader - Comparator 6 blanking source
pub type COMP6_BLANKING_R = crate::FieldReader<COMP6_BLANKING>;
impl COMP6_BLANKING_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP6_BLANKING> {
        match self.bits {
            0 => Some(COMP6_BLANKING::NoBlanking),
            3 => Some(COMP6_BLANKING::Tim2oc4),
            4 => Some(COMP6_BLANKING::Tim15oc2),
            _ => None,
        }
    }
    ///No blanking
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP6_BLANKING::NoBlanking
    }
    ///TIM2 OC4 selected as blanking source
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        *self == COMP6_BLANKING::Tim2oc4
    }
    ///TIM15 OC2 selected as blanking source
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        *self == COMP6_BLANKING::Tim15oc2
    }
}
///Field `COMP6_BLANKING` writer - Comparator 6 blanking source
pub type COMP6_BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP6_BLANKING>;
impl<'a, REG> COMP6_BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No blanking
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6_BLANKING::NoBlanking)
    }
    ///TIM2 OC4 selected as blanking source
    #[inline(always)]
    pub fn tim2oc4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6_BLANKING::Tim2oc4)
    }
    ///TIM15 OC2 selected as blanking source
    #[inline(always)]
    pub fn tim15oc2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6_BLANKING::Tim15oc2)
    }
}
/**Comparator 6 output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6OUT {
    ///0: Non-inverting input below inverting input
    Low = 0,
    ///1: Non-inverting input above inverting input
    High = 1,
}
impl From<COMP6OUT> for bool {
    #[inline(always)]
    fn from(variant: COMP6OUT) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP6OUT` reader - Comparator 6 output
pub type COMP6OUT_R = crate::BitReader<COMP6OUT>;
impl COMP6OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP6OUT {
        match self.bits {
            false => COMP6OUT::Low,
            true => COMP6OUT::High,
        }
    }
    ///Non-inverting input below inverting input
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP6OUT::Low
    }
    ///Non-inverting input above inverting input
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP6OUT::High
    }
}
/**Comparator 6 lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6LOCK {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
}
impl From<COMP6LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP6LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `COMP6LOCK` reader - Comparator 6 lock
pub type COMP6LOCK_R = crate::BitReader<COMP6LOCK>;
impl COMP6LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMP6LOCK {
        match self.bits {
            false => COMP6LOCK::Unlocked,
            true => COMP6LOCK::Locked,
        }
    }
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP6LOCK::Unlocked
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP6LOCK::Locked
    }
}
///Field `COMP6LOCK` writer - Comparator 6 lock
pub type COMP6LOCK_W<'a, REG> = crate::BitWriter<'a, REG, COMP6LOCK>;
impl<'a, REG> COMP6LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6LOCK::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(COMP6LOCK::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 6 enable
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - Comparator 6 inverting input selection
    #[inline(always)]
    pub fn comp6inmsel(&self) -> COMP6INMSEL_R {
        COMP6INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 10:13 - Comparator 6 output selection
    #[inline(always)]
    pub fn comp6outsel(&self) -> COMP6OUTSEL_R {
        COMP6OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 6 output polarity
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 18:20 - Comparator 6 blanking source
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 6 output
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 6 lock
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP6_CSR")
            .field("comp6en", &self.comp6en())
            .field("comp6inmsel", &self.comp6inmsel())
            .field("comp6outsel", &self.comp6outsel())
            .field("comp6pol", &self.comp6pol())
            .field("comp6_blanking", &self.comp6_blanking())
            .field("comp6out", &self.comp6out())
            .field("comp6lock", &self.comp6lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 6 enable
    #[inline(always)]
    pub fn comp6en(&mut self) -> COMP6EN_W<COMP6_CSRrs> {
        COMP6EN_W::new(self, 0)
    }
    ///Bits 4:6 - Comparator 6 inverting input selection
    #[inline(always)]
    pub fn comp6inmsel(&mut self) -> COMP6INMSEL_W<COMP6_CSRrs> {
        COMP6INMSEL_W::new(self, 4)
    }
    ///Bits 10:13 - Comparator 6 output selection
    #[inline(always)]
    pub fn comp6outsel(&mut self) -> COMP6OUTSEL_W<COMP6_CSRrs> {
        COMP6OUTSEL_W::new(self, 10)
    }
    ///Bit 15 - Comparator 6 output polarity
    #[inline(always)]
    pub fn comp6pol(&mut self) -> COMP6POL_W<COMP6_CSRrs> {
        COMP6POL_W::new(self, 15)
    }
    ///Bits 18:20 - Comparator 6 blanking source
    #[inline(always)]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W<COMP6_CSRrs> {
        COMP6_BLANKING_W::new(self, 18)
    }
    ///Bit 31 - Comparator 6 lock
    #[inline(always)]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W<COMP6_CSRrs> {
        COMP6LOCK_W::new(self, 31)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`comp6_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp6_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#COMP:COMP6_CSR)*/
pub struct COMP6_CSRrs;
impl crate::RegisterSpec for COMP6_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp6_csr::R`](R) reader structure
impl crate::Readable for COMP6_CSRrs {}
///`write(|w| ..)` method takes [`comp6_csr::W`](W) writer structure
impl crate::Writable for COMP6_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP6_CSR to value 0
impl crate::Resettable for COMP6_CSRrs {}
