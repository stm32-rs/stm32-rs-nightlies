///Register `OPAMP1_CSR` reader
pub type R = crate::R<OPAMP1_CSRrs>;
///Register `OPAMP1_CSR` writer
pub type W = crate::W<OPAMP1_CSRrs>;
/**Operational amplifier Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAEN {
    ///0: OpAmp disabled
    Disabled = 0,
    ///1: OpAmp enabled
    Enabled = 1,
}
impl From<OPAEN> for bool {
    #[inline(always)]
    fn from(variant: OPAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OPAEN` reader - Operational amplifier Enable
pub type OPAEN_R = crate::BitReader<OPAEN>;
impl OPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPAEN {
        match self.bits {
            false => OPAEN::Disabled,
            true => OPAEN::Enabled,
        }
    }
    ///OpAmp disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAEN::Disabled
    }
    ///OpAmp enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAEN::Enabled
    }
}
///Field `OPAEN` writer - Operational amplifier Enable
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAEN>;
impl<'a, REG> OPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OpAmp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN::Disabled)
    }
    ///OpAmp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN::Enabled)
    }
}
/**Operational amplifier Low Power Mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPALPM {
    ///0: OpAmp in normal mode
    Normal = 0,
    ///1: OpAmp in low power mode
    Low = 1,
}
impl From<OPALPM> for bool {
    #[inline(always)]
    fn from(variant: OPALPM) -> Self {
        variant as u8 != 0
    }
}
///Field `OPALPM` reader - Operational amplifier Low Power Mode
pub type OPALPM_R = crate::BitReader<OPALPM>;
impl OPALPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPALPM {
        match self.bits {
            false => OPALPM::Normal,
            true => OPALPM::Low,
        }
    }
    ///OpAmp in normal mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPALPM::Normal
    }
    ///OpAmp in low power mode
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPALPM::Low
    }
}
///Field `OPALPM` writer - Operational amplifier Low Power Mode
pub type OPALPM_W<'a, REG> = crate::BitWriter<'a, REG, OPALPM>;
impl<'a, REG> OPALPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OpAmp in normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(OPALPM::Normal)
    }
    ///OpAmp in low power mode
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OPALPM::Low)
    }
}
/**Operational amplifier PGA mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAMODE {
    ///0: internal PGA diabled
    PgaDisabled = 0,
    ///2: internal PGA enabled, gain programmed in PGA_GAIN
    PgaEnabled = 2,
    ///3: internal follower
    Follower = 3,
}
impl From<OPAMODE> for u8 {
    #[inline(always)]
    fn from(variant: OPAMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPAMODE {
    type Ux = u8;
}
impl crate::IsEnum for OPAMODE {}
///Field `OPAMODE` reader - Operational amplifier PGA mode
pub type OPAMODE_R = crate::FieldReader<OPAMODE>;
impl OPAMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPAMODE> {
        match self.bits {
            0 => Some(OPAMODE::PgaDisabled),
            2 => Some(OPAMODE::PgaEnabled),
            3 => Some(OPAMODE::Follower),
            _ => None,
        }
    }
    ///internal PGA diabled
    #[inline(always)]
    pub fn is_pga_disabled(&self) -> bool {
        *self == OPAMODE::PgaDisabled
    }
    ///internal PGA enabled, gain programmed in PGA_GAIN
    #[inline(always)]
    pub fn is_pga_enabled(&self) -> bool {
        *self == OPAMODE::PgaEnabled
    }
    ///internal follower
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == OPAMODE::Follower
    }
}
///Field `OPAMODE` writer - Operational amplifier PGA mode
pub type OPAMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OPAMODE>;
impl<'a, REG> OPAMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///internal PGA diabled
    #[inline(always)]
    pub fn pga_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMODE::PgaDisabled)
    }
    ///internal PGA enabled, gain programmed in PGA_GAIN
    #[inline(always)]
    pub fn pga_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMODE::PgaEnabled)
    }
    ///internal follower
    #[inline(always)]
    pub fn follower(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMODE::Follower)
    }
}
/**Operational amplifier Programmable amplifier gain value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN {
    ///0: Gain 2
    Gain2 = 0,
    ///1: Gain 4
    Gain4 = 1,
    ///2: Gain 8
    Gain8 = 2,
    ///3: Gain 16
    Gain16 = 3,
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
impl crate::IsEnum for PGA_GAIN {}
///Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN>;
impl PGA_GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGA_GAIN {
        match self.bits {
            0 => PGA_GAIN::Gain2,
            1 => PGA_GAIN::Gain4,
            2 => PGA_GAIN::Gain8,
            3 => PGA_GAIN::Gain16,
            _ => unreachable!(),
        }
    }
    ///Gain 2
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN::Gain2
    }
    ///Gain 4
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN::Gain4
    }
    ///Gain 8
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN::Gain8
    }
    ///Gain 16
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN::Gain16
    }
}
///Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PGA_GAIN, crate::Safe>;
impl<'a, REG> PGA_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Gain 2
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2)
    }
    ///Gain 4
    #[inline(always)]
    pub fn gain4(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4)
    }
    ///Gain 8
    #[inline(always)]
    pub fn gain8(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8)
    }
    ///Gain 16
    #[inline(always)]
    pub fn gain16(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16)
    }
}
/**Inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL {
    ///0: GPIO connectet to VINM
    Gpio = 0,
    ///1: Low leakage inputs connected (only available in certein BGA cases
    LowLeakage = 1,
    ///2: OPAMP in PGA mode
    PgaMode = 2,
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
impl crate::IsEnum for VM_SEL {}
///Field `VM_SEL` reader - Inverting input selection
pub type VM_SEL_R = crate::FieldReader<VM_SEL>;
impl VM_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VM_SEL> {
        match self.bits {
            0 => Some(VM_SEL::Gpio),
            1 => Some(VM_SEL::LowLeakage),
            2 => Some(VM_SEL::PgaMode),
            _ => None,
        }
    }
    ///GPIO connectet to VINM
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VM_SEL::Gpio
    }
    ///Low leakage inputs connected (only available in certein BGA cases
    #[inline(always)]
    pub fn is_low_leakage(&self) -> bool {
        *self == VM_SEL::LowLeakage
    }
    ///OPAMP in PGA mode
    #[inline(always)]
    pub fn is_pga_mode(&self) -> bool {
        *self == VM_SEL::PgaMode
    }
}
///Field `VM_SEL` writer - Inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VM_SEL>;
impl<'a, REG> VM_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GPIO connectet to VINM
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Gpio)
    }
    ///Low leakage inputs connected (only available in certein BGA cases
    #[inline(always)]
    pub fn low_leakage(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::LowLeakage)
    }
    ///OPAMP in PGA mode
    #[inline(always)]
    pub fn pga_mode(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::PgaMode)
    }
}
/**Non inverted input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VP_SEL {
    ///0: GPIO connected to VINP
    Gpio = 0,
    ///1: DAC connected to VPINP
    Dac = 1,
}
impl From<VP_SEL> for bool {
    #[inline(always)]
    fn from(variant: VP_SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `VP_SEL` reader - Non inverted input selection
pub type VP_SEL_R = crate::BitReader<VP_SEL>;
impl VP_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VP_SEL {
        match self.bits {
            false => VP_SEL::Gpio,
            true => VP_SEL::Dac,
        }
    }
    ///GPIO connected to VINP
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VP_SEL::Gpio
    }
    ///DAC connected to VPINP
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == VP_SEL::Dac
    }
}
///Field `VP_SEL` writer - Non inverted input selection
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VP_SEL>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GPIO connected to VINP
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Gpio)
    }
    ///DAC connected to VPINP
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Dac)
    }
}
/**Calibration mode enabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON {
    ///0: Normal mode
    Disabled = 0,
    ///1: Calibration mode
    Enabled = 1,
}
impl From<CALON> for bool {
    #[inline(always)]
    fn from(variant: CALON) -> Self {
        variant as u8 != 0
    }
}
///Field `CALON` reader - Calibration mode enabled
pub type CALON_R = crate::BitReader<CALON>;
impl CALON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALON {
        match self.bits {
            false => CALON::Disabled,
            true => CALON::Enabled,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON::Disabled
    }
    ///Calibration mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON::Enabled
    }
}
///Field `CALON` writer - Calibration mode enabled
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG, CALON>;
impl<'a, REG> CALON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Disabled)
    }
    ///Calibration mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Enabled)
    }
}
/**Calibration selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALSEL {
    ///0: 0.2V applied to OPAMP inputs during calibration
    Nmos = 0,
    ///1: VDDA-0.2V applied to OPAMP inputs during calibration"
    Pmos = 1,
}
impl From<CALSEL> for bool {
    #[inline(always)]
    fn from(variant: CALSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::BitReader<CALSEL>;
impl CALSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALSEL {
        match self.bits {
            false => CALSEL::Nmos,
            true => CALSEL::Pmos,
        }
    }
    ///0.2V applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn is_nmos(&self) -> bool {
        *self == CALSEL::Nmos
    }
    ///VDDA-0.2V applied to OPAMP inputs during calibration"
    #[inline(always)]
    pub fn is_pmos(&self) -> bool {
        *self == CALSEL::Pmos
    }
}
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG, CALSEL>;
impl<'a, REG> CALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///0.2V applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn nmos(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Nmos)
    }
    ///VDDA-0.2V applied to OPAMP inputs during calibration"
    #[inline(always)]
    pub fn pmos(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Pmos)
    }
}
/**allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERTRIM {
    ///0: Factory trim used
    Factory = 0,
    ///1: User trim used
    User = 1,
}
impl From<USERTRIM> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM) -> Self {
        variant as u8 != 0
    }
}
///Field `USERTRIM` reader - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values
pub type USERTRIM_R = crate::BitReader<USERTRIM>;
impl USERTRIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USERTRIM {
        match self.bits {
            false => USERTRIM::Factory,
            true => USERTRIM::User,
        }
    }
    ///Factory trim used
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == USERTRIM::Factory
    }
    ///User trim used
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == USERTRIM::User
    }
}
///Field `USERTRIM` writer - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG, USERTRIM>;
impl<'a, REG> USERTRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Factory trim used
    #[inline(always)]
    pub fn factory(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM::Factory)
    }
    ///User trim used
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM::User)
    }
}
///Field `CALOUT` reader - Operational amplifier calibration output
pub type CALOUT_R = crate::BitReader;
///Field `CALOUT` writer - Operational amplifier calibration output
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Operational amplifier power supply range for stability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPA_RANGE {
    ///0: low range (VDDA < 2.4V
    Low = 0,
    ///1: low range (VDDA >2.4V
    High = 1,
}
impl From<OPA_RANGE> for bool {
    #[inline(always)]
    fn from(variant: OPA_RANGE) -> Self {
        variant as u8 != 0
    }
}
///Field `OPA_RANGE` reader - Operational amplifier power supply range for stability
pub type OPA_RANGE_R = crate::BitReader<OPA_RANGE>;
impl OPA_RANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPA_RANGE {
        match self.bits {
            false => OPA_RANGE::Low,
            true => OPA_RANGE::High,
        }
    }
    ///low range (VDDA < 2.4V
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPA_RANGE::Low
    }
    ///low range (VDDA >2.4V
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OPA_RANGE::High
    }
}
///Field `OPA_RANGE` writer - Operational amplifier power supply range for stability
pub type OPA_RANGE_W<'a, REG> = crate::BitWriter<'a, REG, OPA_RANGE>;
impl<'a, REG> OPA_RANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///low range (VDDA < 2.4V
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OPA_RANGE::Low)
    }
    ///low range (VDDA >2.4V
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OPA_RANGE::High)
    }
}
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31 - Operational amplifier power supply range for stability
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_CSR")
            .field("opaen", &self.opaen())
            .field("opalpm", &self.opalpm())
            .field("opamode", &self.opamode())
            .field("pga_gain", &self.pga_gain())
            .field("vm_sel", &self.vm_sel())
            .field("vp_sel", &self.vp_sel())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("usertrim", &self.usertrim())
            .field("calout", &self.calout())
            .field("opa_range", &self.opa_range())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<'_, OPAMP1_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<'_, OPAMP1_CSRrs> {
        OPALPM_W::new(self, 1)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<'_, OPAMP1_CSRrs> {
        OPAMODE_W::new(self, 2)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP1_CSRrs> {
        PGA_GAIN_W::new(self, 4)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP1_CSRrs> {
        VM_SEL_W::new(self, 8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP1_CSRrs> {
        VP_SEL_W::new(self, 10)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP1_CSRrs> {
        CALON_W::new(self, 12)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP1_CSRrs> {
        CALSEL_W::new(self, 13)
    }
    ///Bit 14 - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<'_, OPAMP1_CSRrs> {
        USERTRIM_W::new(self, 14)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<'_, OPAMP1_CSRrs> {
        CALOUT_W::new(self, 15)
    }
    ///Bit 31 - Operational amplifier power supply range for stability
    #[inline(always)]
    pub fn opa_range(&mut self) -> OPA_RANGE_W<'_, OPAMP1_CSRrs> {
        OPA_RANGE_W::new(self, 31)
    }
}
/**OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#OPAMP:OPAMP1_CSR)*/
pub struct OPAMP1_CSRrs;
impl crate::RegisterSpec for OPAMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_csr::R`](R) reader structure
impl crate::Readable for OPAMP1_CSRrs {}
///`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure
impl crate::Writable for OPAMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_CSR to value 0
impl crate::Resettable for OPAMP1_CSRrs {}
