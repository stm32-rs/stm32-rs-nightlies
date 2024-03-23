#[doc = "Register `OPAMP6_CSR` reader"]
pub type R = crate::R<OPAMP6_CSRrs>;
#[doc = "Register `OPAMP6_CSR` writer"]
pub type W = crate::W<OPAMP6_CSRrs>;
#[doc = "Operational amplifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAEN {
    #[doc = "0: OpAmp disabled"]
    Disabled = 0,
    #[doc = "1: OpAmp enabled"]
    Enabled = 1,
}
impl From<OPAEN> for bool {
    #[inline(always)]
    fn from(variant: OPAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader<OPAEN>;
impl OPAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAEN {
        match self.bits {
            false => OPAEN::Disabled,
            true => OPAEN::Enabled,
        }
    }
    #[doc = "OpAmp disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAEN::Disabled
    }
    #[doc = "OpAmp enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAEN::Enabled
    }
}
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAEN>;
impl<'a, REG> OPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OpAmp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN::Disabled)
    }
    #[doc = "OpAmp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN::Enabled)
    }
}
#[doc = "FORCE_VP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP {
    #[doc = "0: Non-inverting input connected configured inputs"]
    Normal = 0,
    #[doc = "1: Non-inverting input connected to calibration reference voltage"]
    CalibrationVerification = 1,
}
impl From<FORCE_VP> for bool {
    #[inline(always)]
    fn from(variant: FORCE_VP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type FORCE_VP_R = crate::BitReader<FORCE_VP>;
impl FORCE_VP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_VP {
        match self.bits {
            false => FORCE_VP::Normal,
            true => FORCE_VP::CalibrationVerification,
        }
    }
    #[doc = "Non-inverting input connected configured inputs"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP::Normal
    }
    #[doc = "Non-inverting input connected to calibration reference voltage"]
    #[inline(always)]
    pub fn is_calibration_verification(&self) -> bool {
        *self == FORCE_VP::CalibrationVerification
    }
}
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_VP>;
impl<'a, REG> FORCE_VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-inverting input connected configured inputs"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::Normal)
    }
    #[doc = "Non-inverting input connected to calibration reference voltage"]
    #[inline(always)]
    pub fn calibration_verification(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::CalibrationVerification)
    }
}
#[doc = "VP_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL {
    #[doc = "0: VINP0 connected to VINP input"]
    Vinp0 = 0,
    #[doc = "1: VINP1 connected to VINP input"]
    Vinp1 = 1,
    #[doc = "2: VINP2 connected to VINP input"]
    Vinp2 = 2,
    #[doc = "3: DAC3_CH1 connected to VINP input"]
    Dac3Ch1 = 3,
}
impl From<VP_SEL> for u8 {
    #[inline(always)]
    fn from(variant: VP_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VP_SEL {
    type Ux = u8;
}
#[doc = "Field `VP_SEL` reader - VP_SEL"]
pub type VP_SEL_R = crate::FieldReader<VP_SEL>;
impl VP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VP_SEL {
        match self.bits {
            0 => VP_SEL::Vinp0,
            1 => VP_SEL::Vinp1,
            2 => VP_SEL::Vinp2,
            3 => VP_SEL::Dac3Ch1,
            _ => unreachable!(),
        }
    }
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VP_SEL::Vinp0
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VP_SEL::Vinp1
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VP_SEL::Vinp2
    }
    #[doc = "DAC3_CH1 connected to VINP input"]
    #[inline(always)]
    pub fn is_dac3_ch1(&self) -> bool {
        *self == VP_SEL::Dac3Ch1
    }
}
#[doc = "Field `VP_SEL` writer - VP_SEL"]
pub type VP_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VP_SEL>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp0)
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp1)
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp2)
    }
    #[doc = "DAC3_CH1 connected to VINP input"]
    #[inline(always)]
    pub fn dac3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Dac3Ch1)
    }
}
#[doc = "USERTRIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERTRIM {
    #[doc = "0: Factory trim used"]
    Factory = 0,
    #[doc = "1: User trim used"]
    User = 1,
}
impl From<USERTRIM> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERTRIM` reader - USERTRIM"]
pub type USERTRIM_R = crate::BitReader<USERTRIM>;
impl USERTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USERTRIM {
        match self.bits {
            false => USERTRIM::Factory,
            true => USERTRIM::User,
        }
    }
    #[doc = "Factory trim used"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == USERTRIM::Factory
    }
    #[doc = "User trim used"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == USERTRIM::User
    }
}
#[doc = "Field `USERTRIM` writer - USERTRIM"]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG, USERTRIM>;
impl<'a, REG> USERTRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Factory trim used"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM::Factory)
    }
    #[doc = "User trim used"]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM::User)
    }
}
#[doc = "VM_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL {
    #[doc = "0: VINM0 connected to VINM input"]
    Vinm0 = 0,
    #[doc = "1: VINM1 connected to VINM input"]
    Vinm1 = 1,
    #[doc = "2: Feedback resistor connected to VINM (PGA mode)"]
    Pga = 2,
    #[doc = "3: OpAmp output connected to VINM (Follower mode)"]
    Output = 3,
}
impl From<VM_SEL> for u8 {
    #[inline(always)]
    fn from(variant: VM_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VM_SEL {
    type Ux = u8;
}
#[doc = "Field `VM_SEL` reader - VM_SEL"]
pub type VM_SEL_R = crate::FieldReader<VM_SEL>;
impl VM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VM_SEL {
        match self.bits {
            0 => VM_SEL::Vinm0,
            1 => VM_SEL::Vinm1,
            2 => VM_SEL::Pga,
            3 => VM_SEL::Output,
            _ => unreachable!(),
        }
    }
    #[doc = "VINM0 connected to VINM input"]
    #[inline(always)]
    pub fn is_vinm0(&self) -> bool {
        *self == VM_SEL::Vinm0
    }
    #[doc = "VINM1 connected to VINM input"]
    #[inline(always)]
    pub fn is_vinm1(&self) -> bool {
        *self == VM_SEL::Vinm1
    }
    #[doc = "Feedback resistor connected to VINM (PGA mode)"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL::Pga
    }
    #[doc = "OpAmp output connected to VINM (Follower mode)"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == VM_SEL::Output
    }
}
#[doc = "Field `VM_SEL` writer - VM_SEL"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VM_SEL>;
impl<'a, REG> VM_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VINM0 connected to VINM input"]
    #[inline(always)]
    pub fn vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Vinm0)
    }
    #[doc = "VINM1 connected to VINM input"]
    #[inline(always)]
    pub fn vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Vinm1)
    }
    #[doc = "Feedback resistor connected to VINM (PGA mode)"]
    #[inline(always)]
    pub fn pga(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pga)
    }
    #[doc = "OpAmp output connected to VINM (Follower mode)"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Output)
    }
}
#[doc = "OPAHSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAHSM {
    #[doc = "0: OpAmp in normal mode"]
    Normal = 0,
    #[doc = "1: OpAmp in high speed mode"]
    HighSpeed = 1,
}
impl From<OPAHSM> for bool {
    #[inline(always)]
    fn from(variant: OPAHSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAHSM` reader - OPAHSM"]
pub type OPAHSM_R = crate::BitReader<OPAHSM>;
impl OPAHSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAHSM {
        match self.bits {
            false => OPAHSM::Normal,
            true => OPAHSM::HighSpeed,
        }
    }
    #[doc = "OpAmp in normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPAHSM::Normal
    }
    #[doc = "OpAmp in high speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OPAHSM::HighSpeed
    }
}
#[doc = "Field `OPAHSM` writer - OPAHSM"]
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG, OPAHSM>;
impl<'a, REG> OPAHSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OpAmp in normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OPAHSM::Normal)
    }
    #[doc = "OpAmp in high speed mode"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OPAHSM::HighSpeed)
    }
}
#[doc = "OPAINTOEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAINTOEN {
    #[doc = "0: Output is connected to the output Pin"]
    OutputPin = 0,
    #[doc = "1: Output is connected internally to ADC channel"]
    Adcchannel = 1,
}
impl From<OPAINTOEN> for bool {
    #[inline(always)]
    fn from(variant: OPAINTOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAINTOEN` reader - OPAINTOEN"]
pub type OPAINTOEN_R = crate::BitReader<OPAINTOEN>;
impl OPAINTOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAINTOEN {
        match self.bits {
            false => OPAINTOEN::OutputPin,
            true => OPAINTOEN::Adcchannel,
        }
    }
    #[doc = "Output is connected to the output Pin"]
    #[inline(always)]
    pub fn is_output_pin(&self) -> bool {
        *self == OPAINTOEN::OutputPin
    }
    #[doc = "Output is connected internally to ADC channel"]
    #[inline(always)]
    pub fn is_adcchannel(&self) -> bool {
        *self == OPAINTOEN::Adcchannel
    }
}
#[doc = "Field `OPAINTOEN` writer - OPAINTOEN"]
pub type OPAINTOEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAINTOEN>;
impl<'a, REG> OPAINTOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is connected to the output Pin"]
    #[inline(always)]
    pub fn output_pin(self) -> &'a mut crate::W<REG> {
        self.variant(OPAINTOEN::OutputPin)
    }
    #[doc = "Output is connected internally to ADC channel"]
    #[inline(always)]
    pub fn adcchannel(self) -> &'a mut crate::W<REG> {
        self.variant(OPAINTOEN::Adcchannel)
    }
}
#[doc = "CALON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON {
    #[doc = "0: Calibration mode disabled"]
    Disabled = 0,
    #[doc = "1: Calibration mode enabled"]
    Enabled = 1,
}
impl From<CALON> for bool {
    #[inline(always)]
    fn from(variant: CALON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` reader - CALON"]
pub type CALON_R = crate::BitReader<CALON>;
impl CALON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALON {
        match self.bits {
            false => CALON::Disabled,
            true => CALON::Enabled,
        }
    }
    #[doc = "Calibration mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON::Disabled
    }
    #[doc = "Calibration mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON::Enabled
    }
}
#[doc = "Field `CALON` writer - CALON"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG, CALON>;
impl<'a, REG> CALON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Disabled)
    }
    #[doc = "Calibration mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Enabled)
    }
}
#[doc = "CALSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL {
    #[doc = "0: 0.033*VDDA applied to OPAMP inputs during calibration"]
    Percent3_3 = 0,
    #[doc = "1: 0.1*VDDA applied to OPAMP inputs during calibration"]
    Percent10 = 1,
    #[doc = "2: 0.5*VDDA applied to OPAMP inputs during calibration"]
    Percent50 = 2,
    #[doc = "3: 0.9*VDDA applied to OPAMP inputs during calibration"]
    Percent90 = 3,
}
impl From<CALSEL> for u8 {
    #[inline(always)]
    fn from(variant: CALSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALSEL {
    type Ux = u8;
}
#[doc = "Field `CALSEL` reader - CALSEL"]
pub type CALSEL_R = crate::FieldReader<CALSEL>;
impl CALSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALSEL {
        match self.bits {
            0 => CALSEL::Percent3_3,
            1 => CALSEL::Percent10,
            2 => CALSEL::Percent50,
            3 => CALSEL::Percent90,
            _ => unreachable!(),
        }
    }
    #[doc = "0.033*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn is_percent3_3(&self) -> bool {
        *self == CALSEL::Percent3_3
    }
    #[doc = "0.1*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        *self == CALSEL::Percent10
    }
    #[doc = "0.5*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        *self == CALSEL::Percent50
    }
    #[doc = "0.9*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        *self == CALSEL::Percent90
    }
}
#[doc = "Field `CALSEL` writer - CALSEL"]
pub type CALSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CALSEL>;
impl<'a, REG> CALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.033*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent3_3)
    }
    #[doc = "0.1*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent10(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent10)
    }
    #[doc = "0.5*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent50(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent50)
    }
    #[doc = "0.9*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent90(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent90)
    }
}
#[doc = "PGA_GAIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN {
    #[doc = "0: Gain 2"]
    Gain2 = 0,
    #[doc = "1: Gain 4"]
    Gain4 = 1,
    #[doc = "2: Gain 8"]
    Gain8 = 2,
    #[doc = "3: Gain 16"]
    Gain16 = 3,
    #[doc = "4: Gain 32"]
    Gain32 = 4,
    #[doc = "5: Gain 64"]
    Gain64 = 5,
    #[doc = "8: Gain 2, input/bias connected to VINM0 or inverting gain"]
    Gain2InputVinm0 = 8,
    #[doc = "9: Gain 4, input/bias connected to VINM0 or inverting gain"]
    Gain4InputVinm0 = 9,
    #[doc = "10: Gain 8, input/bias connected to VINM0 or inverting gain"]
    Gain8InputVinm0 = 10,
    #[doc = "11: Gain 16, input/bias connected to VINM0 or inverting gain"]
    Gain16InputVinm0 = 11,
    #[doc = "12: Gain 32, input/bias connected to VINM0 or inverting gain"]
    Gain32InputVinm0 = 12,
    #[doc = "13: Gain 64, input/bias connected to VINM0 or inverting gain"]
    Gain64InputVinm0 = 13,
    #[doc = "16: Gain 2, with filtering on VINM0"]
    Gain2FilteringVinm0 = 16,
    #[doc = "17: Gain 4, with filtering on VINM0"]
    Gain4FilteringVinm0 = 17,
    #[doc = "18: Gain 8, with filtering on VINM0"]
    Gain8FilteringVinm0 = 18,
    #[doc = "19: Gain 16, with filtering on VINM0"]
    Gain16FilteringVinm0 = 19,
    #[doc = "20: Gain 32, with filtering on VINM0"]
    Gain32FilteringVinm0 = 20,
    #[doc = "21: Gain 64, with filtering on VINM0"]
    Gain64FilteringVinm0 = 21,
    #[doc = "24: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain2InputVinm0filteringVinm1 = 24,
    #[doc = "25: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain4InputVinm0filteringVinm1 = 25,
    #[doc = "26: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain8InputVinm0filteringVinm1 = 26,
    #[doc = "27: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain16InputVinm0filteringVinm1 = 27,
    #[doc = "28: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain32InputVinm0filteringVinm1 = 28,
    #[doc = "29: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    Gain64InputVinm0filteringVinm1 = 29,
}
impl From<PGA_GAIN> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PGA_GAIN {
    type Ux = u8;
}
#[doc = "Field `PGA_GAIN` reader - PGA_GAIN"]
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN>;
impl PGA_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PGA_GAIN> {
        match self.bits {
            0 => Some(PGA_GAIN::Gain2),
            1 => Some(PGA_GAIN::Gain4),
            2 => Some(PGA_GAIN::Gain8),
            3 => Some(PGA_GAIN::Gain16),
            4 => Some(PGA_GAIN::Gain32),
            5 => Some(PGA_GAIN::Gain64),
            8 => Some(PGA_GAIN::Gain2InputVinm0),
            9 => Some(PGA_GAIN::Gain4InputVinm0),
            10 => Some(PGA_GAIN::Gain8InputVinm0),
            11 => Some(PGA_GAIN::Gain16InputVinm0),
            12 => Some(PGA_GAIN::Gain32InputVinm0),
            13 => Some(PGA_GAIN::Gain64InputVinm0),
            16 => Some(PGA_GAIN::Gain2FilteringVinm0),
            17 => Some(PGA_GAIN::Gain4FilteringVinm0),
            18 => Some(PGA_GAIN::Gain8FilteringVinm0),
            19 => Some(PGA_GAIN::Gain16FilteringVinm0),
            20 => Some(PGA_GAIN::Gain32FilteringVinm0),
            21 => Some(PGA_GAIN::Gain64FilteringVinm0),
            24 => Some(PGA_GAIN::Gain2InputVinm0filteringVinm1),
            25 => Some(PGA_GAIN::Gain4InputVinm0filteringVinm1),
            26 => Some(PGA_GAIN::Gain8InputVinm0filteringVinm1),
            27 => Some(PGA_GAIN::Gain16InputVinm0filteringVinm1),
            28 => Some(PGA_GAIN::Gain32InputVinm0filteringVinm1),
            29 => Some(PGA_GAIN::Gain64InputVinm0filteringVinm1),
            _ => None,
        }
    }
    #[doc = "Gain 2"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN::Gain2
    }
    #[doc = "Gain 4"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN::Gain4
    }
    #[doc = "Gain 8"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN::Gain8
    }
    #[doc = "Gain 16"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN::Gain16
    }
    #[doc = "Gain 32"]
    #[inline(always)]
    pub fn is_gain32(&self) -> bool {
        *self == PGA_GAIN::Gain32
    }
    #[doc = "Gain 64"]
    #[inline(always)]
    pub fn is_gain64(&self) -> bool {
        *self == PGA_GAIN::Gain64
    }
    #[doc = "Gain 2, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain2_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain2InputVinm0
    }
    #[doc = "Gain 4, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain4_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain4InputVinm0
    }
    #[doc = "Gain 8, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain8_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain8InputVinm0
    }
    #[doc = "Gain 16, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain16_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain16InputVinm0
    }
    #[doc = "Gain 32, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain32_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain32InputVinm0
    }
    #[doc = "Gain 64, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn is_gain64_input_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain64InputVinm0
    }
    #[doc = "Gain 2, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain2_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain2FilteringVinm0
    }
    #[doc = "Gain 4, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain4_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain4FilteringVinm0
    }
    #[doc = "Gain 8, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain8_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain8FilteringVinm0
    }
    #[doc = "Gain 16, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain16_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain16FilteringVinm0
    }
    #[doc = "Gain 32, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain32_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain32FilteringVinm0
    }
    #[doc = "Gain 64, with filtering on VINM0"]
    #[inline(always)]
    pub fn is_gain64_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN::Gain64FilteringVinm0
    }
    #[doc = "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain2_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain2InputVinm0filteringVinm1
    }
    #[doc = "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain4_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain4InputVinm0filteringVinm1
    }
    #[doc = "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain8_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain8InputVinm0filteringVinm1
    }
    #[doc = "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain16_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain16InputVinm0filteringVinm1
    }
    #[doc = "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain32_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain32InputVinm0filteringVinm1
    }
    #[doc = "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn is_gain64_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN::Gain64InputVinm0filteringVinm1
    }
}
#[doc = "Field `PGA_GAIN` writer - PGA_GAIN"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PGA_GAIN>;
impl<'a, REG> PGA_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Gain 2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2)
    }
    #[doc = "Gain 4"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4)
    }
    #[doc = "Gain 8"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8)
    }
    #[doc = "Gain 16"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16)
    }
    #[doc = "Gain 32"]
    #[inline(always)]
    pub fn gain32(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain32)
    }
    #[doc = "Gain 64"]
    #[inline(always)]
    pub fn gain64(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain64)
    }
    #[doc = "Gain 2, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain2_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2InputVinm0)
    }
    #[doc = "Gain 4, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain4_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4InputVinm0)
    }
    #[doc = "Gain 8, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain8_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8InputVinm0)
    }
    #[doc = "Gain 16, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain16_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16InputVinm0)
    }
    #[doc = "Gain 32, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain32_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain32InputVinm0)
    }
    #[doc = "Gain 64, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain64_input_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain64InputVinm0)
    }
    #[doc = "Gain 2, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain2_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2FilteringVinm0)
    }
    #[doc = "Gain 4, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain4_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4FilteringVinm0)
    }
    #[doc = "Gain 8, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain8_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8FilteringVinm0)
    }
    #[doc = "Gain 16, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain16_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16FilteringVinm0)
    }
    #[doc = "Gain 32, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain32_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain32FilteringVinm0)
    }
    #[doc = "Gain 64, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain64_filtering_vinm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain64FilteringVinm0)
    }
    #[doc = "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain2_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2InputVinm0filteringVinm1)
    }
    #[doc = "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain4_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4InputVinm0filteringVinm1)
    }
    #[doc = "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain8_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8InputVinm0filteringVinm1)
    }
    #[doc = "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain16_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16InputVinm0filteringVinm1)
    }
    #[doc = "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain32_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain32InputVinm0filteringVinm1)
    }
    #[doc = "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain64_input_vinm0filtering_vinm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain64InputVinm0filteringVinm1)
    }
}
#[doc = "Field `TRIMOFFSETP` reader - TRIMOFFSETP"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - TRIMOFFSETP"]
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETN` reader - TRIMOFFSETN"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - TRIMOFFSETN"]
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `CALOUT` reader - CALOUT"]
pub type CALOUT_R = crate::BitReader;
#[doc = "Field `CALOUT` writer - CALOUT"]
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    #[doc = "0: CSR is read-write"]
    ReadWrite = 0,
    #[doc = "1: CSR is read-only, can only be cleared by system reset"]
    ReadOnly = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::ReadWrite,
            true => LOCK::ReadOnly,
        }
    }
    #[doc = "CSR is read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK::ReadWrite
    }
    #[doc = "CSR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK::ReadOnly
    }
}
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSR is read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadWrite)
    }
    #[doc = "CSR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP6_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<OPAMP6_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP6_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP6_CSRrs> {
        USERTRIM_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP6_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<OPAMP6_CSRrs> {
        OPAHSM_W::new(self, 7)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W<OPAMP6_CSRrs> {
        OPAINTOEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP6_CSRrs> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP6_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP6_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<OPAMP6_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<OPAMP6_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<OPAMP6_CSRrs> {
        CALOUT_W::new(self, 30)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<OPAMP6_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "OPAMP6 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp6_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp6_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP6_CSRrs;
impl crate::RegisterSpec for OPAMP6_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp6_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP6_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp6_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP6_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP6_CSR to value 0"]
impl crate::Resettable for OPAMP6_CSRrs {
    const RESET_VALUE: u32 = 0;
}
