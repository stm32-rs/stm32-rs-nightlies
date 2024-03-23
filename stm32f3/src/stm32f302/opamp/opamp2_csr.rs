#[doc = "Register `OPAMP2_CSR` reader"]
pub type R = crate::R<OPAMP2_CSRrs>;
#[doc = "Register `OPAMP2_CSR` writer"]
pub type W = crate::W<OPAMP2_CSRrs>;
#[doc = "OPAMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMP2EN {
    #[doc = "0: OPAMP2 is disabled"]
    Disabled = 0,
    #[doc = "1: OPAMP2 is enabled"]
    Enabled = 1,
}
impl From<OPAMP2EN> for bool {
    #[inline(always)]
    fn from(variant: OPAMP2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMP2EN` reader - OPAMP2 enable"]
pub type OPAMP2EN_R = crate::BitReader<OPAMP2EN>;
impl OPAMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMP2EN {
        match self.bits {
            false => OPAMP2EN::Disabled,
            true => OPAMP2EN::Enabled,
        }
    }
    #[doc = "OPAMP2 is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMP2EN::Disabled
    }
    #[doc = "OPAMP2 is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMP2EN::Enabled
    }
}
#[doc = "Field `OPAMP2EN` writer - OPAMP2 enable"]
pub type OPAMP2EN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMP2EN>;
impl<'a, REG> OPAMP2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPAMP2 is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMP2EN::Disabled)
    }
    #[doc = "OPAMP2 is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMP2EN::Enabled)
    }
}
#[doc = "FORCE_VP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP {
    #[doc = "0: Normal operating mode"]
    Normal = 0,
    #[doc = "1: Calibration mode. Non-inverting input connected to calibration reference"]
    Calibration = 1,
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
            true => FORCE_VP::Calibration,
        }
    }
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP::Normal
    }
    #[doc = "Calibration mode. Non-inverting input connected to calibration reference"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == FORCE_VP::Calibration
    }
}
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_VP>;
impl<'a, REG> FORCE_VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::Normal)
    }
    #[doc = "Calibration mode. Non-inverting input connected to calibration reference"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::Calibration)
    }
}
#[doc = "OPAMP Non inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL {
    #[doc = "1: PB14 used as OPAMP2 non-inverting input"]
    Pb14 = 1,
    #[doc = "2: PB0 used as OPAMP2 non-inverting input"]
    Pb0 = 2,
    #[doc = "3: PA7 used as OPAMP2 non-inverting input"]
    Pa7 = 3,
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
#[doc = "Field `VP_SEL` reader - OPAMP Non inverting input selection"]
pub type VP_SEL_R = crate::FieldReader<VP_SEL>;
impl VP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VP_SEL> {
        match self.bits {
            1 => Some(VP_SEL::Pb14),
            2 => Some(VP_SEL::Pb0),
            3 => Some(VP_SEL::Pa7),
            _ => None,
        }
    }
    #[doc = "PB14 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VP_SEL::Pb14
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VP_SEL::Pb0
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VP_SEL::Pa7
    }
}
#[doc = "Field `VP_SEL` writer - OPAMP Non inverting input selection"]
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VP_SEL>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PB14 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pb14)
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pb0)
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pa7)
    }
}
#[doc = "OPAMP inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL {
    #[doc = "0: PC5 (VM0) used as OPAMP2 inverting input"]
    Pc5 = 0,
    #[doc = "1: PA5 (VM1) used as OPAMP2 inverting input"]
    Pa5 = 1,
    #[doc = "2: Resistor feedback output (PGA mode)"]
    Pga = 2,
    #[doc = "3: Follower mode"]
    Follower = 3,
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
#[doc = "Field `VM_SEL` reader - OPAMP inverting input selection"]
pub type VM_SEL_R = crate::FieldReader<VM_SEL>;
impl VM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VM_SEL {
        match self.bits {
            0 => VM_SEL::Pc5,
            1 => VM_SEL::Pa5,
            2 => VM_SEL::Pga,
            3 => VM_SEL::Follower,
            _ => unreachable!(),
        }
    }
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VM_SEL::Pc5
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VM_SEL::Pa5
    }
    #[doc = "Resistor feedback output (PGA mode)"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL::Pga
    }
    #[doc = "Follower mode"]
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == VM_SEL::Follower
    }
}
#[doc = "Field `VM_SEL` writer - OPAMP inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VM_SEL>;
impl<'a, REG> VM_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pc5)
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pa5)
    }
    #[doc = "Resistor feedback output (PGA mode)"]
    #[inline(always)]
    pub fn pga(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pga)
    }
    #[doc = "Follower mode"]
    #[inline(always)]
    pub fn follower(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Follower)
    }
}
#[doc = "Timer controlled Mux mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_EN {
    #[doc = "0: Timer controlled mux disabled"]
    Disabled = 0,
    #[doc = "1: Timer controlled mux enabled"]
    Enabled = 1,
}
impl From<TCM_EN> for bool {
    #[inline(always)]
    fn from(variant: TCM_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub type TCM_EN_R = crate::BitReader<TCM_EN>;
impl TCM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCM_EN {
        match self.bits {
            false => TCM_EN::Disabled,
            true => TCM_EN::Enabled,
        }
    }
    #[doc = "Timer controlled mux disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCM_EN::Disabled
    }
    #[doc = "Timer controlled mux enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCM_EN::Enabled
    }
}
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub type TCM_EN_W<'a, REG> = crate::BitWriter<'a, REG, TCM_EN>;
impl<'a, REG> TCM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer controlled mux disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_EN::Disabled)
    }
    #[doc = "Timer controlled mux enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_EN::Enabled)
    }
}
#[doc = "OPAMP inverting input secondary selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMS_SEL {
    #[doc = "0: PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"]
    Pc5 = 0,
    #[doc = "1: PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"]
    Pa5 = 1,
}
impl From<VMS_SEL> for bool {
    #[inline(always)]
    fn from(variant: VMS_SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMS_SEL` reader - OPAMP inverting input secondary selection"]
pub type VMS_SEL_R = crate::BitReader<VMS_SEL>;
impl VMS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VMS_SEL {
        match self.bits {
            false => VMS_SEL::Pc5,
            true => VMS_SEL::Pa5,
        }
    }
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VMS_SEL::Pc5
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VMS_SEL::Pa5
    }
}
#[doc = "Field `VMS_SEL` writer - OPAMP inverting input secondary selection"]
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VMS_SEL>;
impl<'a, REG> VMS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(VMS_SEL::Pc5)
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(VMS_SEL::Pa5)
    }
}
#[doc = "OPAMP Non inverting input secondary selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPS_SEL {
    #[doc = "1: PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
    Pb14 = 1,
    #[doc = "2: PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
    Pb0 = 2,
    #[doc = "3: PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
    Pa7 = 3,
}
impl From<VPS_SEL> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VPS_SEL {
    type Ux = u8;
}
#[doc = "Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_R = crate::FieldReader<VPS_SEL>;
impl VPS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VPS_SEL> {
        match self.bits {
            1 => Some(VPS_SEL::Pb14),
            2 => Some(VPS_SEL::Pb0),
            3 => Some(VPS_SEL::Pa7),
            _ => None,
        }
    }
    #[doc = "PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VPS_SEL::Pb14
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VPS_SEL::Pb0
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VPS_SEL::Pa7
    }
}
#[doc = "Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VPS_SEL>;
impl<'a, REG> VPS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pb14)
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pb0)
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pa7)
    }
}
#[doc = "Calibration mode enable\n\nValue on reset: 0"]
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
#[doc = "Field `CALON` reader - Calibration mode enable"]
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
#[doc = "Field `CALON` writer - Calibration mode enable"]
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
#[doc = "Calibration selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL {
    #[doc = "0: VREFOPAMP=3.3% VDDA"]
    Percent3_3 = 0,
    #[doc = "1: VREFOPAMP=10% VDDA"]
    Percent10 = 1,
    #[doc = "2: VREFOPAMP=50% VDDA"]
    Percent50 = 2,
    #[doc = "3: VREFOPAMP=90% VDDA"]
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
#[doc = "Field `CALSEL` reader - Calibration selection"]
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
    #[doc = "VREFOPAMP=3.3% VDDA"]
    #[inline(always)]
    pub fn is_percent3_3(&self) -> bool {
        *self == CALSEL::Percent3_3
    }
    #[doc = "VREFOPAMP=10% VDDA"]
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        *self == CALSEL::Percent10
    }
    #[doc = "VREFOPAMP=50% VDDA"]
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        *self == CALSEL::Percent50
    }
    #[doc = "VREFOPAMP=90% VDDA"]
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        *self == CALSEL::Percent90
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CALSEL>;
impl<'a, REG> CALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREFOPAMP=3.3% VDDA"]
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent3_3)
    }
    #[doc = "VREFOPAMP=10% VDDA"]
    #[inline(always)]
    pub fn percent10(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent10)
    }
    #[doc = "VREFOPAMP=50% VDDA"]
    #[inline(always)]
    pub fn percent50(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent50)
    }
    #[doc = "VREFOPAMP=90% VDDA"]
    #[inline(always)]
    pub fn percent90(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent90)
    }
}
#[doc = "Gain in PGA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN {
    #[doc = "0: Gain 2"]
    Gain2 = 0,
    #[doc = "1: Gain 4"]
    Gain4 = 1,
    #[doc = "2: Gain 8"]
    Gain8 = 2,
    #[doc = "4: Gain 16"]
    Gain16 = 4,
    #[doc = "8: Gain 2, feedback connected to VM0"]
    Gain2Vm0 = 8,
    #[doc = "9: Gain 4, feedback connected to VM0"]
    Gain4Vm0 = 9,
    #[doc = "10: Gain 8, feedback connected to VM0"]
    Gain8Vm0 = 10,
    #[doc = "11: Gain 16, feedback connected to VM0"]
    Gain16Vm0 = 11,
    #[doc = "12: Gain 2, feedback connected to VM1"]
    Gain2Vm1 = 12,
    #[doc = "13: Gain 4, feedback connected to VM1"]
    Gain4Vm1 = 13,
    #[doc = "14: Gain 8, feedback connected to VM1"]
    Gain8Vm1 = 14,
    #[doc = "15: Gain 16, feedback connected to VM1"]
    Gain16Vm1 = 15,
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
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN>;
impl PGA_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PGA_GAIN> {
        match self.bits {
            0 => Some(PGA_GAIN::Gain2),
            1 => Some(PGA_GAIN::Gain4),
            2 => Some(PGA_GAIN::Gain8),
            4 => Some(PGA_GAIN::Gain16),
            8 => Some(PGA_GAIN::Gain2Vm0),
            9 => Some(PGA_GAIN::Gain4Vm0),
            10 => Some(PGA_GAIN::Gain8Vm0),
            11 => Some(PGA_GAIN::Gain16Vm0),
            12 => Some(PGA_GAIN::Gain2Vm1),
            13 => Some(PGA_GAIN::Gain4Vm1),
            14 => Some(PGA_GAIN::Gain8Vm1),
            15 => Some(PGA_GAIN::Gain16Vm1),
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
    #[doc = "Gain 2, feedback connected to VM0"]
    #[inline(always)]
    pub fn is_gain2_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain2Vm0
    }
    #[doc = "Gain 4, feedback connected to VM0"]
    #[inline(always)]
    pub fn is_gain4_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain4Vm0
    }
    #[doc = "Gain 8, feedback connected to VM0"]
    #[inline(always)]
    pub fn is_gain8_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain8Vm0
    }
    #[doc = "Gain 16, feedback connected to VM0"]
    #[inline(always)]
    pub fn is_gain16_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain16Vm0
    }
    #[doc = "Gain 2, feedback connected to VM1"]
    #[inline(always)]
    pub fn is_gain2_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain2Vm1
    }
    #[doc = "Gain 4, feedback connected to VM1"]
    #[inline(always)]
    pub fn is_gain4_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain4Vm1
    }
    #[doc = "Gain 8, feedback connected to VM1"]
    #[inline(always)]
    pub fn is_gain8_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain8Vm1
    }
    #[doc = "Gain 16, feedback connected to VM1"]
    #[inline(always)]
    pub fn is_gain16_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain16Vm1
    }
}
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PGA_GAIN>;
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
    #[doc = "Gain 2, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain2_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2Vm0)
    }
    #[doc = "Gain 4, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain4_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4Vm0)
    }
    #[doc = "Gain 8, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain8_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8Vm0)
    }
    #[doc = "Gain 16, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain16_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16Vm0)
    }
    #[doc = "Gain 2, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain2_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2Vm1)
    }
    #[doc = "Gain 4, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain4_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4Vm1)
    }
    #[doc = "Gain 8, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain8_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8Vm1)
    }
    #[doc = "Gain 16, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain16_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16Vm1)
    }
}
#[doc = "User trimming enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USER_TRIM {
    #[doc = "0: User trimming disabled"]
    Disabled = 0,
    #[doc = "1: User trimming enabled"]
    Enabled = 1,
}
impl From<USER_TRIM> for bool {
    #[inline(always)]
    fn from(variant: USER_TRIM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub type USER_TRIM_R = crate::BitReader<USER_TRIM>;
impl USER_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USER_TRIM {
        match self.bits {
            false => USER_TRIM::Disabled,
            true => USER_TRIM::Enabled,
        }
    }
    #[doc = "User trimming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USER_TRIM::Disabled
    }
    #[doc = "User trimming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USER_TRIM::Enabled
    }
}
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub type USER_TRIM_W<'a, REG> = crate::BitWriter<'a, REG, USER_TRIM>;
impl<'a, REG> USER_TRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User trimming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USER_TRIM::Disabled)
    }
    #[doc = "User trimming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USER_TRIM::Enabled)
    }
}
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "TSTREF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTREF {
    #[doc = "0: VREFOPAMP2 is output"]
    Output = 0,
    #[doc = "1: VREFOPAMP2 is not output"]
    NotOutput = 1,
}
impl From<TSTREF> for bool {
    #[inline(always)]
    fn from(variant: TSTREF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTREF` reader - TSTREF"]
pub type TSTREF_R = crate::BitReader<TSTREF>;
impl TSTREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTREF {
        match self.bits {
            false => TSTREF::Output,
            true => TSTREF::NotOutput,
        }
    }
    #[doc = "VREFOPAMP2 is output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == TSTREF::Output
    }
    #[doc = "VREFOPAMP2 is not output"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == TSTREF::NotOutput
    }
}
#[doc = "Field `TSTREF` writer - TSTREF"]
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG, TSTREF>;
impl<'a, REG> TSTREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFOPAMP2 is output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF::Output)
    }
    #[doc = "VREFOPAMP2 is not output"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF::NotOutput)
    }
}
#[doc = "OPAMP ouput status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTCAL {
    #[doc = "0: Non-inverting &lt; inverting"]
    Low = 0,
    #[doc = "1: Non-inverting > inverting"]
    High = 1,
}
impl From<OUTCAL> for bool {
    #[inline(always)]
    fn from(variant: OUTCAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTCAL` reader - OPAMP ouput status flag"]
pub type OUTCAL_R = crate::BitReader<OUTCAL>;
impl OUTCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTCAL {
        match self.bits {
            false => OUTCAL::Low,
            true => OUTCAL::High,
        }
    }
    #[doc = "Non-inverting &lt; inverting"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTCAL::Low
    }
    #[doc = "Non-inverting > inverting"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTCAL::High
    }
}
#[doc = "OPAMP lock\n\nValue on reset: 0"]
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
#[doc = "Field `LOCK` reader - OPAMP lock"]
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
#[doc = "Field `LOCK` writer - OPAMP lock"]
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
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    pub fn opamp2en(&self) -> OPAMP2EN_R {
        OPAMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn opamp2en(&mut self) -> OPAMP2EN_W<OPAMP2_CSRrs> {
        OPAMP2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<OPAMP2_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP2_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcm_en(&mut self) -> TCM_EN_W<OPAMP2_CSRrs> {
        TCM_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<OPAMP2_CSRrs> {
        VMS_SEL_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<OPAMP2_CSRrs> {
        VPS_SEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP2_CSRrs> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP2_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn user_trim(&mut self) -> USER_TRIM_W<OPAMP2_CSRrs> {
        USER_TRIM_W::new(self, 18)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<OPAMP2_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<OPAMP2_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<OPAMP2_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<OPAMP2_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "OPAMP2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_CSRrs;
impl crate::RegisterSpec for OPAMP2_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP2_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP2_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSRrs {
    const RESET_VALUE: u32 = 0;
}
