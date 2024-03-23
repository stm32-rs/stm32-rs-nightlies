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
    #[doc = "3: Low speed / ultra-low power"]
    LowSpeed = 3,
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
    pub const fn variant(&self) -> Option<PWRMODE> {
        match self.bits {
            0 => Some(PWRMODE::HighSpeed),
            1 => Some(PWRMODE::MediumSpeed),
            3 => Some(PWRMODE::LowSpeed),
            _ => None,
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
    #[doc = "Low speed / ultra-low power"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == PWRMODE::LowSpeed
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 1"]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRMODE>;
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
    #[doc = "Low speed / ultra-low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::LowSpeed)
    }
}
#[doc = "Comparator 1 Input Minus connection configuration bit\n\nValue on reset: 0"]
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
    #[doc = "5: DAC Channel 2"]
    DacCh2 = 5,
    #[doc = "6: PC4"]
    Pc4 = 6,
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
#[doc = "Field `INMSEL` reader - Comparator 1 Input Minus connection configuration bit"]
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
            5 => Some(INMSEL::DacCh2),
            6 => Some(INMSEL::Pc4),
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
    #[doc = "DAC Channel 2"]
    #[inline(always)]
    pub fn is_dac_ch2(&self) -> bool {
        *self == INMSEL::DacCh2
    }
    #[doc = "PC4"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == INMSEL::Pc4
    }
}
#[doc = "Field `INMSEL` writer - Comparator 1 Input Minus connection configuration bit"]
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
    #[doc = "DAC Channel 2"]
    #[inline(always)]
    pub fn dac_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::DacCh2)
    }
    #[doc = "PC4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(INMSEL::Pc4)
    }
}
#[doc = "Comparator1 input plus selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPSEL {
    #[doc = "0: PC5 connected to input plus"]
    Pc5 = 0,
    #[doc = "1: PB2 connected to input plus"]
    Pb2 = 1,
}
impl From<INPSEL> for bool {
    #[inline(always)]
    fn from(variant: INPSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPSEL` reader - Comparator1 input plus selection bit"]
pub type INPSEL_R = crate::BitReader<INPSEL>;
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INPSEL {
        match self.bits {
            false => INPSEL::Pc5,
            true => INPSEL::Pb2,
        }
    }
    #[doc = "PC5 connected to input plus"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == INPSEL::Pc5
    }
    #[doc = "PB2 connected to input plus"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == INPSEL::Pb2
    }
}
#[doc = "Field `INPSEL` writer - Comparator1 input plus selection bit"]
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG, INPSEL>;
impl<'a, REG> INPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PC5 connected to input plus"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pc5)
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
    #[doc = "4: TIM15 OC1 selected as blanking source"]
    Tim1oc5 = 4,
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
            4 => Some(BLANKING::Tim1oc5),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == BLANKING::NoBlanking
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == BLANKING::Tim1oc5
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
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim1oc5)
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
#[doc = "Field `LOCK` writer - COMP1_CSR register lock bit"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP1_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
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
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator 1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
