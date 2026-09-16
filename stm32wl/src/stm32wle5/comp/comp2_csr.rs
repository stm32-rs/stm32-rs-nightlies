///Register `COMP2_CSR` reader
pub type R = crate::R<COMP2_CSRrs>;
///Register `COMP2_CSR` writer
pub type W = crate::W<COMP2_CSRrs>;
/**Comparator 2 enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Comparator X disabled
    Disabled = 0,
    ///1: Comparator X enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Comparator 2 enable bit
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Comparator X disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Comparator 2 enable bit
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Power Mode of the comparator 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE {
    ///0: High speed / full power
    HighSpeed = 0,
    ///1: Medium speed / medium power
    MediumSpeed = 1,
    ///2: Low speed / low power
    LowSpeed = 2,
    ///3: Very-low speed / ultra-low power
    VeryLowSpeed = 3,
}
impl From<PWRMODE> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRMODE {
    type Ux = u8;
}
impl crate::IsEnum for PWRMODE {}
///Field `PWRMODE` reader - Power Mode of the comparator 2
pub type PWRMODE_R = crate::FieldReader<PWRMODE>;
impl PWRMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRMODE {
        match self.bits {
            0 => PWRMODE::HighSpeed,
            1 => PWRMODE::MediumSpeed,
            2 => PWRMODE::LowSpeed,
            3 => PWRMODE::VeryLowSpeed,
            _ => unreachable!(),
        }
    }
    ///High speed / full power
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == PWRMODE::HighSpeed
    }
    ///Medium speed / medium power
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == PWRMODE::MediumSpeed
    }
    ///Low speed / low power
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == PWRMODE::LowSpeed
    }
    ///Very-low speed / ultra-low power
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == PWRMODE::VeryLowSpeed
    }
}
///Field `PWRMODE` writer - Power Mode of the comparator 2
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRMODE, crate::Safe>;
impl<'a, REG> PWRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///High speed / full power
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::HighSpeed)
    }
    ///Medium speed / medium power
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::MediumSpeed)
    }
    ///Low speed / low power
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::LowSpeed)
    }
    ///Very-low speed / ultra-low power
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::VeryLowSpeed)
    }
}
/**Comparator 2 input minus selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMSEL {
    ///0: 1/4 of VRefint
    OneQuarterVref = 0,
    ///1: 1/2 of VRefint
    OneHalfVref = 1,
    ///2: 3/4 of VRefint
    ThreeQuarterVref = 2,
    ///3: VRefint
    Vref = 3,
    ///4: DAC Channel 1
    DacCh1 = 4,
    ///6: PB3
    Pb3 = 6,
    ///7: GPIO pin selected by INMESEL
    Gpio = 7,
}
impl From<INMSEL> for u8 {
    #[inline(always)]
    fn from(variant: INMSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INMSEL {
    type Ux = u8;
}
impl crate::IsEnum for INMSEL {}
///Field `INMSEL` reader - Comparator 2 input minus selection bits
pub type INMSEL_R = crate::FieldReader<INMSEL>;
impl INMSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<INMSEL> {
        match self.bits {
            0 => Some(INMSEL::OneQuarterVref),
            1 => Some(INMSEL::OneHalfVref),
            2 => Some(INMSEL::ThreeQuarterVref),
            3 => Some(INMSEL::Vref),
            4 => Some(INMSEL::DacCh1),
            6 => Some(INMSEL::Pb3),
            7 => Some(INMSEL::Gpio),
            _ => None,
        }
    }
    ///1/4 of VRefint
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == INMSEL::OneQuarterVref
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == INMSEL::OneHalfVref
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == INMSEL::ThreeQuarterVref
    }
    ///VRefint
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == INMSEL::Vref
    }
    ///DAC Channel 1
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        *self == INMSEL::DacCh1
    }
    ///PB3
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == INMSEL::Pb3
    }
    ///GPIO pin selected by INMESEL
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == INMSEL::Gpio
    }
}
///Field `INMSEL` writer - Comparator 2 input minus selection bits
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, INMSEL>;
impl<'a, REG> INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Vref)
    }
    ///DAC Channel 1
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::DacCh1)
    }
    ///PB3
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Pb3)
    }
    ///GPIO pin selected by INMESEL
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Gpio)
    }
}
/**Comparator 1 input plus selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPSEL {
    ///0: PB4 connected to input plus
    Pb4 = 0,
    ///1: PB1 connected to input plus
    Pb1 = 1,
    ///2: PA15 connected to input plus
    Pa15 = 2,
}
impl From<INPSEL> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPSEL {
    type Ux = u8;
}
impl crate::IsEnum for INPSEL {}
///Field `INPSEL` reader - Comparator 1 input plus selection bit
pub type INPSEL_R = crate::FieldReader<INPSEL>;
impl INPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPSEL> {
        match self.bits {
            0 => Some(INPSEL::Pb4),
            1 => Some(INPSEL::Pb1),
            2 => Some(INPSEL::Pa15),
            _ => None,
        }
    }
    ///PB4 connected to input plus
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == INPSEL::Pb4
    }
    ///PB1 connected to input plus
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == INPSEL::Pb1
    }
    ///PA15 connected to input plus
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == INPSEL::Pa15
    }
}
///Field `INPSEL` writer - Comparator 1 input plus selection bit
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INPSEL>;
impl<'a, REG> INPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PB4 connected to input plus
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb4)
    }
    ///PB1 connected to input plus
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb1)
    }
    ///PA15 connected to input plus
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pa15)
    }
}
/**Windows mode selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WINMODE {
    ///0: COMP2 input plus is not connected to COMP1
    Disabled = 0,
    ///1: COMP2 input plus is connected to COMP1
    Enabled = 1,
}
impl From<WINMODE> for bool {
    #[inline(always)]
    fn from(variant: WINMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `WINMODE` reader - Windows mode selection bit
pub type WINMODE_R = crate::BitReader<WINMODE>;
impl WINMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WINMODE {
        match self.bits {
            false => WINMODE::Disabled,
            true => WINMODE::Enabled,
        }
    }
    ///COMP2 input plus is not connected to COMP1
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WINMODE::Disabled
    }
    ///COMP2 input plus is connected to COMP1
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WINMODE::Enabled
    }
}
///Field `WINMODE` writer - Windows mode selection bit
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG, WINMODE>;
impl<'a, REG> WINMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COMP2 input plus is not connected to COMP1
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODE::Disabled)
    }
    ///COMP2 input plus is connected to COMP1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODE::Enabled)
    }
}
/**Comparator 2 polarity selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY {
    ///0: Comparator X output value not inverted
    Normal = 0,
    ///1: Comparator X output value inverted
    Inverted = 1,
}
impl From<POLARITY> for bool {
    #[inline(always)]
    fn from(variant: POLARITY) -> Self {
        variant as u8 != 0
    }
}
///Field `POLARITY` reader - Comparator 2 polarity selection bit
pub type POLARITY_R = crate::BitReader<POLARITY>;
impl POLARITY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY {
        match self.bits {
            false => POLARITY::Normal,
            true => POLARITY::Inverted,
        }
    }
    ///Comparator X output value not inverted
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == POLARITY::Normal
    }
    ///Comparator X output value inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY::Inverted
    }
}
///Field `POLARITY` writer - Comparator 2 polarity selection bit
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X output value not inverted
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Normal)
    }
    ///Comparator X output value inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Inverted)
    }
}
/**Comparator 2 hysteresis selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST {
    ///0: No hysteresis
    None = 0,
    ///1: Low hysteresis
    Low = 1,
    ///2: Medium hysteresis
    Medium = 2,
    ///3: High hysteresis
    High = 3,
}
impl From<HYST> for u8 {
    #[inline(always)]
    fn from(variant: HYST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYST {
    type Ux = u8;
}
impl crate::IsEnum for HYST {}
///Field `HYST` reader - Comparator 2 hysteresis selection bits
pub type HYST_R = crate::FieldReader<HYST>;
impl HYST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HYST {
        match self.bits {
            0 => HYST::None,
            1 => HYST::Low,
            2 => HYST::Medium,
            3 => HYST::High,
            _ => unreachable!(),
        }
    }
    ///No hysteresis
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYST::None
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HYST::Low
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYST::Medium
    }
    ///High hysteresis
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HYST::High
    }
}
///Field `HYST` writer - Comparator 2 hysteresis selection bits
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HYST, crate::Safe>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No hysteresis
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::None)
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Low)
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Medium)
    }
    ///High hysteresis
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::High)
    }
}
/**Comparator 2 blanking source selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING {
    ///0: No blanking
    NoBlanking = 0,
    ///1: TIM1 OC5 selected as blanking source
    Tim1oc5 = 1,
    ///2: TIM2 OC3 selected as blanking source
    Tim2oc3 = 2,
}
impl From<BLANKING> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLANKING {
    type Ux = u8;
}
impl crate::IsEnum for BLANKING {}
///Field `BLANKING` reader - Comparator 2 blanking source selection bits
pub type BLANKING_R = crate::FieldReader<BLANKING>;
impl BLANKING_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLANKING> {
        match self.bits {
            0 => Some(BLANKING::NoBlanking),
            1 => Some(BLANKING::Tim1oc5),
            2 => Some(BLANKING::Tim2oc3),
            _ => None,
        }
    }
    ///No blanking
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == BLANKING::NoBlanking
    }
    ///TIM1 OC5 selected as blanking source
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == BLANKING::Tim1oc5
    }
    ///TIM2 OC3 selected as blanking source
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == BLANKING::Tim2oc3
    }
}
///Field `BLANKING` writer - Comparator 2 blanking source selection bits
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BLANKING>;
impl<'a, REG> BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No blanking
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::NoBlanking)
    }
    ///TIM1 OC5 selected as blanking source
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim1oc5)
    }
    ///TIM2 OC3 selected as blanking source
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim2oc3)
    }
}
/**Scaler bridge enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN {
    ///0: Scaler resistor bridge disabled
    Disabled = 0,
    ///1: Scaler resistor bridge enabled
    Enabled = 1,
}
impl From<BRGEN> for bool {
    #[inline(always)]
    fn from(variant: BRGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BRGEN` reader - Scaler bridge enable
pub type BRGEN_R = crate::BitReader<BRGEN>;
impl BRGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRGEN {
        match self.bits {
            false => BRGEN::Disabled,
            true => BRGEN::Enabled,
        }
    }
    ///Scaler resistor bridge disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN::Disabled
    }
    ///Scaler resistor bridge enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN::Enabled
    }
}
///Field `BRGEN` writer - Scaler bridge enable
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGEN>;
impl<'a, REG> BRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Scaler resistor bridge disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Disabled)
    }
    ///Scaler resistor bridge enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Enabled)
    }
}
/**Voltage scaler enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN {
    ///0: Voltage scaler disabled
    Disabled = 0,
    ///1: Voltage scaler enabled
    Enabled = 1,
}
impl From<SCALEN> for bool {
    #[inline(always)]
    fn from(variant: SCALEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCALEN` reader - Voltage scaler enable bit
pub type SCALEN_R = crate::BitReader<SCALEN>;
impl SCALEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCALEN {
        match self.bits {
            false => SCALEN::Disabled,
            true => SCALEN::Enabled,
        }
    }
    ///Voltage scaler disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN::Disabled
    }
    ///Voltage scaler enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN::Enabled
    }
}
///Field `SCALEN` writer - Voltage scaler enable bit
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG, SCALEN>;
impl<'a, REG> SCALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage scaler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Disabled)
    }
    ///Voltage scaler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Enabled)
    }
}
/**comparator 2 input minus extended selection bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMESEL {
    ///0: PB2 connected to input minus
    Pb2 = 0,
    ///1: PA10 connected to input minus
    Pa10 = 1,
    ///2: PA11 connected to input minus
    Pa11 = 2,
}
impl From<INMESEL> for u8 {
    #[inline(always)]
    fn from(variant: INMESEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INMESEL {
    type Ux = u8;
}
impl crate::IsEnum for INMESEL {}
///Field `INMESEL` reader - comparator 2 input minus extended selection bits.
pub type INMESEL_R = crate::FieldReader<INMESEL>;
impl INMESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<INMESEL> {
        match self.bits {
            0 => Some(INMESEL::Pb2),
            1 => Some(INMESEL::Pa10),
            2 => Some(INMESEL::Pa11),
            _ => None,
        }
    }
    ///PB2 connected to input minus
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == INMESEL::Pb2
    }
    ///PA10 connected to input minus
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == INMESEL::Pa10
    }
    ///PA11 connected to input minus
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == INMESEL::Pa11
    }
}
///Field `INMESEL` writer - comparator 2 input minus extended selection bits.
pub type INMESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INMESEL>;
impl<'a, REG> INMESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PB2 connected to input minus
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pb2)
    }
    ///PA10 connected to input minus
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pa10)
    }
    ///PA11 connected to input minus
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pa11)
    }
}
/**Comparator 2 output status bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALUE {
    ///0: Comparator output is low
    Low = 0,
    ///1: Comparator output is high
    High = 1,
}
impl From<VALUE> for bool {
    #[inline(always)]
    fn from(variant: VALUE) -> Self {
        variant as u8 != 0
    }
}
///Field `VALUE` reader - Comparator 2 output status bit
pub type VALUE_R = crate::BitReader<VALUE>;
impl VALUE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VALUE {
        match self.bits {
            false => VALUE::Low,
            true => VALUE::High,
        }
    }
    ///Comparator output is low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VALUE::Low
    }
    ///Comparator output is high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VALUE::High
    }
}
/**CSR register lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - CSR register lock bit
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
///Field `LOCK` writer - CSR register lock bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 2 input minus selection bits
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    pub fn inmesel(&self) -> INMESEL_R {
        INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2_CSR")
            .field("lock", &self.lock())
            .field("value", &self.value())
            .field("inmesel", &self.inmesel())
            .field("scalen", &self.scalen())
            .field("brgen", &self.brgen())
            .field("blanking", &self.blanking())
            .field("hyst", &self.hyst())
            .field("polarity", &self.polarity())
            .field("winmode", &self.winmode())
            .field("inpsel", &self.inpsel())
            .field("inmsel", &self.inmsel())
            .field("pwrmode", &self.pwrmode())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, COMP2_CSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<'_, COMP2_CSRrs> {
        PWRMODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 2 input minus selection bits
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, COMP2_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<'_, COMP2_CSRrs> {
        INPSEL_W::new(self, 7)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<'_, COMP2_CSRrs> {
        WINMODE_W::new(self, 9)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<'_, COMP2_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, COMP2_CSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<'_, COMP2_CSRrs> {
        BLANKING_W::new(self, 18)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, COMP2_CSRrs> {
        BRGEN_W::new(self, 22)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, COMP2_CSRrs> {
        SCALEN_W::new(self, 23)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    pub fn inmesel(&mut self) -> INMESEL_W<'_, COMP2_CSRrs> {
        INMESEL_W::new(self, 25)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, COMP2_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**COMP2_CSR

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#COMP:COMP2_CSR)*/
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
