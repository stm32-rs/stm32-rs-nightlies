#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<COMP1_CSRrs>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<COMP1_CSRrs>;
#[doc = "Comparator 1 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Comparator 1 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 1 enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator 1 enable bit"]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - Comparator 1 enable bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Power Mode of the comparator 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "2: Low speed / low power"]
    LowSpeed = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
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
#[doc = "Field `PWRMODE` reader - Power Mode of the comparator 1"]
pub type PWRMODE_R = crate::FieldReader<PWRMODE>;
impl PWRMODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == PWRMODE::HighSpeed
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == PWRMODE::MediumSpeed
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == PWRMODE::LowSpeed
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == PWRMODE::VeryLowSpeed
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 1"]
pub type PWRMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PWRMODE>;
impl<'a, REG> PWRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::MediumSpeed)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::LowSpeed)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::VeryLowSpeed)
    }
}
#[doc = "Comparator 1 input minus selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMSEL {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: DAC Channel 1"]
    DacCh1 = 4,
    #[doc = "6: PB3"]
    Pb3 = 6,
    #[doc = "7: GPIO pin selected by INMESEL"]
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
#[doc = "Field `INMSEL` reader - Comparator 1 input minus selection bits"]
pub type INMSEL_R = crate::FieldReader<INMSEL>;
impl INMSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == INMSEL::OneQuarterVref
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == INMSEL::OneHalfVref
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == INMSEL::ThreeQuarterVref
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == INMSEL::Vref
    }
    #[doc = "DAC Channel 1"]
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        *self == INMSEL::DacCh1
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == INMSEL::Pb3
    }
    #[doc = "GPIO pin selected by INMESEL"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == INMSEL::Gpio
    }
}
#[doc = "Field `INMSEL` writer - Comparator 1 input minus selection bits"]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, INMSEL>;
impl<'a, REG> INMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Vref)
    }
    #[doc = "DAC Channel 1"]
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::DacCh1)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Pb3)
    }
    #[doc = "GPIO pin selected by INMESEL"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Gpio)
    }
}
#[doc = "Comparator1 input plus selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPSEL {
    #[doc = "0: PB4 connected to input plus"]
    Pb4 = 0,
    #[doc = "1: PB2 connected to input plus"]
    Pb2 = 1,
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
#[doc = "Field `INPSEL` reader - Comparator1 input plus selection bit"]
pub type INPSEL_R = crate::FieldReader<INPSEL>;
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPSEL> {
        match self.bits {
            0 => Some(INPSEL::Pb4),
            1 => Some(INPSEL::Pb2),
            _ => None,
        }
    }
    #[doc = "PB4 connected to input plus"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == INPSEL::Pb4
    }
    #[doc = "PB2 connected to input plus"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == INPSEL::Pb2
    }
}
#[doc = "Field `INPSEL` writer - Comparator1 input plus selection bit"]
pub type INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INPSEL>;
impl<'a, REG> INPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PB4 connected to input plus"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb4)
    }
    #[doc = "PB2 connected to input plus"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb2)
    }
}
#[doc = "Comparator 1 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<POLARITY> for bool {
    #[inline(always)]
    fn from(variant: POLARITY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - Comparator 1 polarity selection bit"]
pub type POLARITY_R = crate::BitReader<POLARITY>;
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY {
        match self.bits {
            false => POLARITY::NotInverted,
            true => POLARITY::Inverted,
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLARITY::NotInverted
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY::Inverted
    }
}
#[doc = "Field `POLARITY` writer - Comparator 1 polarity selection bit"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Inverted)
    }
}
#[doc = "Comparator 1 hysteresis selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
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
#[doc = "Field `HYST` reader - Comparator 1 hysteresis selection bits"]
pub type HYST_R = crate::FieldReader<HYST>;
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYST {
        match self.bits {
            0 => HYST::NoHysteresis,
            1 => HYST::LowHysteresis,
            2 => HYST::MediumHysteresis,
            3 => HYST::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == HYST::NoHysteresis
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == HYST::LowHysteresis
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == HYST::MediumHysteresis
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == HYST::HighHysteresis
    }
}
#[doc = "Field `HYST` writer - Comparator 1 hysteresis selection bits"]
pub type HYST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HYST>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::HighHysteresis)
    }
}
#[doc = "Comparator 1 blanking source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    Tim1oc5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
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
#[doc = "Field `BLANKING` reader - Comparator 1 blanking source selection bits"]
pub type BLANKING_R = crate::FieldReader<BLANKING>;
impl BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLANKING> {
        match self.bits {
            0 => Some(BLANKING::NoBlanking),
            1 => Some(BLANKING::Tim1oc5),
            2 => Some(BLANKING::Tim2oc3),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == BLANKING::NoBlanking
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == BLANKING::Tim1oc5
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == BLANKING::Tim2oc3
    }
}
#[doc = "Field `BLANKING` writer - Comparator 1 blanking source selection bits"]
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BLANKING>;
impl<'a, REG> BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::NoBlanking)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim1oc5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim2oc3)
    }
}
#[doc = "Scaler bridge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN {
    #[doc = "0: Scaler resistor bridge disabled"]
    Disabled = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    Enabled = 1,
}
impl From<BRGEN> for bool {
    #[inline(always)]
    fn from(variant: BRGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BRGEN_R = crate::BitReader<BRGEN>;
impl BRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRGEN {
        match self.bits {
            false => BRGEN::Disabled,
            true => BRGEN::Enabled,
        }
    }
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN::Disabled
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN::Enabled
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGEN>;
impl<'a, REG> BRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Disabled)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Enabled)
    }
}
#[doc = "Voltage scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN {
    #[doc = "0: Voltage scaler disabled"]
    Disabled = 0,
    #[doc = "1: Voltage scaler enabled"]
    Enabled = 1,
}
impl From<SCALEN> for bool {
    #[inline(always)]
    fn from(variant: SCALEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type SCALEN_R = crate::BitReader<SCALEN>;
impl SCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCALEN {
        match self.bits {
            false => SCALEN::Disabled,
            true => SCALEN::Enabled,
        }
    }
    #[doc = "Voltage scaler disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN::Disabled
    }
    #[doc = "Voltage scaler enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN::Enabled
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG, SCALEN>;
impl<'a, REG> SCALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage scaler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Disabled)
    }
    #[doc = "Voltage scaler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Enabled)
    }
}
#[doc = "comparator 1 input minus extended selection bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INMESEL {
    #[doc = "0: PA10 connected to input minus"]
    Pa10 = 0,
    #[doc = "1: PA11 connected to input minus"]
    Pa11 = 1,
    #[doc = "2: PA15 connected to input minus"]
    Pa15 = 2,
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
#[doc = "Field `INMESEL` reader - comparator 1 input minus extended selection bits."]
pub type INMESEL_R = crate::FieldReader<INMESEL>;
impl INMESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INMESEL> {
        match self.bits {
            0 => Some(INMESEL::Pa10),
            1 => Some(INMESEL::Pa11),
            2 => Some(INMESEL::Pa15),
            _ => None,
        }
    }
    #[doc = "PA10 connected to input minus"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == INMESEL::Pa10
    }
    #[doc = "PA11 connected to input minus"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == INMESEL::Pa11
    }
    #[doc = "PA15 connected to input minus"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == INMESEL::Pa15
    }
}
#[doc = "Field `INMESEL` writer - comparator 1 input minus extended selection bits."]
pub type INMESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INMESEL>;
impl<'a, REG> INMESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA10 connected to input minus"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pa10)
    }
    #[doc = "PA11 connected to input minus"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pa11)
    }
    #[doc = "PA15 connected to input minus"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(INMESEL::Pa15)
    }
}
#[doc = "Comparator 1 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALUE {
    #[doc = "0: Comparator output is low"]
    Low = 0,
    #[doc = "1: Comparator output is high"]
    High = 1,
}
impl From<VALUE> for bool {
    #[inline(always)]
    fn from(variant: VALUE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALUE` reader - Comparator 1 output status bit"]
pub type VALUE_R = crate::BitReader<VALUE>;
impl VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VALUE {
        match self.bits {
            false => VALUE::Low,
            true => VALUE::High,
        }
    }
    #[doc = "Comparator output is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VALUE::Low
    }
    #[doc = "Comparator output is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VALUE::High
    }
}
#[doc = "COMP1_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - COMP1_CSR register lock bit"]
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
#[doc = "Field `LOCK` writer - COMP1_CSR register lock bit"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 input minus selection bits"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:26 - comparator 1 input minus extended selection bits."]
    #[inline(always)]
    pub fn inmesel(&self) -> INMESEL_R {
        INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP1_CSRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP1_CSRrs> {
        PWRMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 input minus selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP1_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator1 input plus selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP1_CSRrs> {
        INPSEL_W::new(self, 7)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<COMP1_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP1_CSRrs> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<COMP1_CSRrs> {
        BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<COMP1_CSRrs> {
        BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<COMP1_CSRrs> {
        SCALEN_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - comparator 1 input minus extended selection bits."]
    #[inline(always)]
    #[must_use]
    pub fn inmesel(&mut self) -> INMESEL_W<COMP1_CSRrs> {
        INMESEL_W::new(self, 25)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "COMP1_CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for COMP1_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSRrs {
    const RESET_VALUE: u32 = 0;
}
