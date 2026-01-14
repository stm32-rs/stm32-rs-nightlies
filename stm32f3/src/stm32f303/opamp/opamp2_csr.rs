///Register `OPAMP2_CSR` reader
pub type R = crate::R<OPAMP2_CSRrs>;
///Register `OPAMP2_CSR` writer
pub type W = crate::W<OPAMP2_CSRrs>;
/**OPAMP2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMP2EN {
    ///0: OPAMP2 is disabled
    Disabled = 0,
    ///1: OPAMP2 is enabled
    Enabled = 1,
}
impl From<OPAMP2EN> for bool {
    #[inline(always)]
    fn from(variant: OPAMP2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OPAMP2EN` reader - OPAMP2 enable
pub type OPAMP2EN_R = crate::BitReader<OPAMP2EN>;
impl OPAMP2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPAMP2EN {
        match self.bits {
            false => OPAMP2EN::Disabled,
            true => OPAMP2EN::Enabled,
        }
    }
    ///OPAMP2 is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMP2EN::Disabled
    }
    ///OPAMP2 is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMP2EN::Enabled
    }
}
///Field `OPAMP2EN` writer - OPAMP2 enable
pub type OPAMP2EN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMP2EN>;
impl<'a, REG> OPAMP2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OPAMP2 is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMP2EN::Disabled)
    }
    ///OPAMP2 is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMP2EN::Enabled)
    }
}
/**FORCE_VP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP {
    ///0: Normal operating mode
    Normal = 0,
    ///1: Calibration mode. Non-inverting input connected to calibration reference
    Calibration = 1,
}
impl From<FORCE_VP> for bool {
    #[inline(always)]
    fn from(variant: FORCE_VP) -> Self {
        variant as u8 != 0
    }
}
///Field `FORCE_VP` reader - FORCE_VP
pub type FORCE_VP_R = crate::BitReader<FORCE_VP>;
impl FORCE_VP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_VP {
        match self.bits {
            false => FORCE_VP::Normal,
            true => FORCE_VP::Calibration,
        }
    }
    ///Normal operating mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP::Normal
    }
    ///Calibration mode. Non-inverting input connected to calibration reference
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == FORCE_VP::Calibration
    }
}
///Field `FORCE_VP` writer - FORCE_VP
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_VP>;
impl<'a, REG> FORCE_VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operating mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::Normal)
    }
    ///Calibration mode. Non-inverting input connected to calibration reference
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP::Calibration)
    }
}
/**OPAMP Non inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL {
    ///1: PB14 used as OPAMP2 non-inverting input
    Pb14 = 1,
    ///2: PB0 used as OPAMP2 non-inverting input
    Pb0 = 2,
    ///3: PA7 used as OPAMP2 non-inverting input
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
impl crate::IsEnum for VP_SEL {}
///Field `VP_SEL` reader - OPAMP Non inverting input selection
pub type VP_SEL_R = crate::FieldReader<VP_SEL>;
impl VP_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VP_SEL> {
        match self.bits {
            1 => Some(VP_SEL::Pb14),
            2 => Some(VP_SEL::Pb0),
            3 => Some(VP_SEL::Pa7),
            _ => None,
        }
    }
    ///PB14 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VP_SEL::Pb14
    }
    ///PB0 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VP_SEL::Pb0
    }
    ///PA7 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VP_SEL::Pa7
    }
}
///Field `VP_SEL` writer - OPAMP Non inverting input selection
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VP_SEL>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PB14 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pb14)
    }
    ///PB0 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pb0)
    }
    ///PA7 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Pa7)
    }
}
/**OPAMP inverting input selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL {
    ///0: PC5 (VM0) used as OPAMP2 inverting input
    Pc5 = 0,
    ///1: PA5 (VM1) used as OPAMP2 inverting input
    Pa5 = 1,
    ///2: Resistor feedback output (PGA mode)
    Pga = 2,
    ///3: Follower mode
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
impl crate::IsEnum for VM_SEL {}
///Field `VM_SEL` reader - OPAMP inverting input selection
pub type VM_SEL_R = crate::FieldReader<VM_SEL>;
impl VM_SEL_R {
    ///Get enumerated values variant
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
    ///PC5 (VM0) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VM_SEL::Pc5
    }
    ///PA5 (VM1) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VM_SEL::Pa5
    }
    ///Resistor feedback output (PGA mode)
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL::Pga
    }
    ///Follower mode
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == VM_SEL::Follower
    }
}
///Field `VM_SEL` writer - OPAMP inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VM_SEL, crate::Safe>;
impl<'a, REG> VM_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PC5 (VM0) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pc5)
    }
    ///PA5 (VM1) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pa5)
    }
    ///Resistor feedback output (PGA mode)
    #[inline(always)]
    pub fn pga(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Pga)
    }
    ///Follower mode
    #[inline(always)]
    pub fn follower(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL::Follower)
    }
}
/**Timer controlled Mux mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_EN {
    ///0: Timer controlled mux disabled
    Disabled = 0,
    ///1: Timer controlled mux enabled
    Enabled = 1,
}
impl From<TCM_EN> for bool {
    #[inline(always)]
    fn from(variant: TCM_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCM_EN` reader - Timer controlled Mux mode enable
pub type TCM_EN_R = crate::BitReader<TCM_EN>;
impl TCM_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCM_EN {
        match self.bits {
            false => TCM_EN::Disabled,
            true => TCM_EN::Enabled,
        }
    }
    ///Timer controlled mux disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCM_EN::Disabled
    }
    ///Timer controlled mux enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCM_EN::Enabled
    }
}
///Field `TCM_EN` writer - Timer controlled Mux mode enable
pub type TCM_EN_W<'a, REG> = crate::BitWriter<'a, REG, TCM_EN>;
impl<'a, REG> TCM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer controlled mux disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_EN::Disabled)
    }
    ///Timer controlled mux enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCM_EN::Enabled)
    }
}
/**OPAMP inverting input secondary selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMS_SEL {
    ///0: PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
    Pc5 = 0,
    ///1: PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
    Pa5 = 1,
}
impl From<VMS_SEL> for bool {
    #[inline(always)]
    fn from(variant: VMS_SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `VMS_SEL` reader - OPAMP inverting input secondary selection
pub type VMS_SEL_R = crate::BitReader<VMS_SEL>;
impl VMS_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VMS_SEL {
        match self.bits {
            false => VMS_SEL::Pc5,
            true => VMS_SEL::Pa5,
        }
    }
    ///PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VMS_SEL::Pc5
    }
    ///PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VMS_SEL::Pa5
    }
}
///Field `VMS_SEL` writer - OPAMP inverting input secondary selection
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG, VMS_SEL>;
impl<'a, REG> VMS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(VMS_SEL::Pc5)
    }
    ///PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(VMS_SEL::Pa5)
    }
}
/**OPAMP Non inverting input secondary selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPS_SEL {
    ///1: PB14 used as OPAMP2 non-inverting input when TCM_EN=1
    Pb14 = 1,
    ///2: PB0 used as OPAMP2 non-inverting input when TCM_EN=1
    Pb0 = 2,
    ///3: PA7 used as OPAMP2 non-inverting input when TCM_EN=1
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
impl crate::IsEnum for VPS_SEL {}
///Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection
pub type VPS_SEL_R = crate::FieldReader<VPS_SEL>;
impl VPS_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VPS_SEL> {
        match self.bits {
            1 => Some(VPS_SEL::Pb14),
            2 => Some(VPS_SEL::Pb0),
            3 => Some(VPS_SEL::Pa7),
            _ => None,
        }
    }
    ///PB14 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VPS_SEL::Pb14
    }
    ///PB0 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VPS_SEL::Pb0
    }
    ///PA7 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VPS_SEL::Pa7
    }
}
///Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection
pub type VPS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VPS_SEL>;
impl<'a, REG> VPS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PB14 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pb14)
    }
    ///PB0 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pb0)
    }
    ///PA7 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Pa7)
    }
}
/**Calibration mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON {
    ///0: Calibration mode disabled
    Disabled = 0,
    ///1: Calibration mode enabled
    Enabled = 1,
}
impl From<CALON> for bool {
    #[inline(always)]
    fn from(variant: CALON) -> Self {
        variant as u8 != 0
    }
}
///Field `CALON` reader - Calibration mode enable
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
    ///Calibration mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON::Disabled
    }
    ///Calibration mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON::Enabled
    }
}
///Field `CALON` writer - Calibration mode enable
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG, CALON>;
impl<'a, REG> CALON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Disabled)
    }
    ///Calibration mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALON::Enabled)
    }
}
/**Calibration selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL {
    ///0: VREFOPAMP=3.3% VDDA
    Percent3_3 = 0,
    ///1: VREFOPAMP=10% VDDA
    Percent10 = 1,
    ///2: VREFOPAMP=50% VDDA
    Percent50 = 2,
    ///3: VREFOPAMP=90% VDDA
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
impl crate::IsEnum for CALSEL {}
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::FieldReader<CALSEL>;
impl CALSEL_R {
    ///Get enumerated values variant
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
    ///VREFOPAMP=3.3% VDDA
    #[inline(always)]
    pub fn is_percent3_3(&self) -> bool {
        *self == CALSEL::Percent3_3
    }
    ///VREFOPAMP=10% VDDA
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        *self == CALSEL::Percent10
    }
    ///VREFOPAMP=50% VDDA
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        *self == CALSEL::Percent50
    }
    ///VREFOPAMP=90% VDDA
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        *self == CALSEL::Percent90
    }
}
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CALSEL, crate::Safe>;
impl<'a, REG> CALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VREFOPAMP=3.3% VDDA
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent3_3)
    }
    ///VREFOPAMP=10% VDDA
    #[inline(always)]
    pub fn percent10(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent10)
    }
    ///VREFOPAMP=50% VDDA
    #[inline(always)]
    pub fn percent50(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent50)
    }
    ///VREFOPAMP=90% VDDA
    #[inline(always)]
    pub fn percent90(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL::Percent90)
    }
}
/**Gain in PGA mode

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
    ///4: Gain 16
    Gain16 = 4,
    ///8: Gain 2, feedback connected to VM0
    Gain2Vm0 = 8,
    ///9: Gain 4, feedback connected to VM0
    Gain4Vm0 = 9,
    ///10: Gain 8, feedback connected to VM0
    Gain8Vm0 = 10,
    ///11: Gain 16, feedback connected to VM0
    Gain16Vm0 = 11,
    ///12: Gain 2, feedback connected to VM1
    Gain2Vm1 = 12,
    ///13: Gain 4, feedback connected to VM1
    Gain4Vm1 = 13,
    ///14: Gain 8, feedback connected to VM1
    Gain8Vm1 = 14,
    ///15: Gain 16, feedback connected to VM1
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
impl crate::IsEnum for PGA_GAIN {}
///Field `PGA_GAIN` reader - Gain in PGA mode
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN>;
impl PGA_GAIN_R {
    ///Get enumerated values variant
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
    ///Gain 2, feedback connected to VM0
    #[inline(always)]
    pub fn is_gain2_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain2Vm0
    }
    ///Gain 4, feedback connected to VM0
    #[inline(always)]
    pub fn is_gain4_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain4Vm0
    }
    ///Gain 8, feedback connected to VM0
    #[inline(always)]
    pub fn is_gain8_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain8Vm0
    }
    ///Gain 16, feedback connected to VM0
    #[inline(always)]
    pub fn is_gain16_vm0(&self) -> bool {
        *self == PGA_GAIN::Gain16Vm0
    }
    ///Gain 2, feedback connected to VM1
    #[inline(always)]
    pub fn is_gain2_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain2Vm1
    }
    ///Gain 4, feedback connected to VM1
    #[inline(always)]
    pub fn is_gain4_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain4Vm1
    }
    ///Gain 8, feedback connected to VM1
    #[inline(always)]
    pub fn is_gain8_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain8Vm1
    }
    ///Gain 16, feedback connected to VM1
    #[inline(always)]
    pub fn is_gain16_vm1(&self) -> bool {
        *self == PGA_GAIN::Gain16Vm1
    }
}
///Field `PGA_GAIN` writer - Gain in PGA mode
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PGA_GAIN>;
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
    ///Gain 2, feedback connected to VM0
    #[inline(always)]
    pub fn gain2_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2Vm0)
    }
    ///Gain 4, feedback connected to VM0
    #[inline(always)]
    pub fn gain4_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4Vm0)
    }
    ///Gain 8, feedback connected to VM0
    #[inline(always)]
    pub fn gain8_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8Vm0)
    }
    ///Gain 16, feedback connected to VM0
    #[inline(always)]
    pub fn gain16_vm0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16Vm0)
    }
    ///Gain 2, feedback connected to VM1
    #[inline(always)]
    pub fn gain2_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2Vm1)
    }
    ///Gain 4, feedback connected to VM1
    #[inline(always)]
    pub fn gain4_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4Vm1)
    }
    ///Gain 8, feedback connected to VM1
    #[inline(always)]
    pub fn gain8_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8Vm1)
    }
    ///Gain 16, feedback connected to VM1
    #[inline(always)]
    pub fn gain16_vm1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16Vm1)
    }
}
/**User trimming enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USER_TRIM {
    ///0: User trimming disabled
    Disabled = 0,
    ///1: User trimming enabled
    Enabled = 1,
}
impl From<USER_TRIM> for bool {
    #[inline(always)]
    fn from(variant: USER_TRIM) -> Self {
        variant as u8 != 0
    }
}
///Field `USER_TRIM` reader - User trimming enable
pub type USER_TRIM_R = crate::BitReader<USER_TRIM>;
impl USER_TRIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USER_TRIM {
        match self.bits {
            false => USER_TRIM::Disabled,
            true => USER_TRIM::Enabled,
        }
    }
    ///User trimming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USER_TRIM::Disabled
    }
    ///User trimming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USER_TRIM::Enabled
    }
}
///Field `USER_TRIM` writer - User trimming enable
pub type USER_TRIM_W<'a, REG> = crate::BitWriter<'a, REG, USER_TRIM>;
impl<'a, REG> USER_TRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///User trimming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USER_TRIM::Disabled)
    }
    ///User trimming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USER_TRIM::Enabled)
    }
}
///Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)
pub type TRIMOFFSETP_R = crate::FieldReader;
///Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)
pub type TRIMOFFSETN_R = crate::FieldReader;
///Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**TSTREF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTREF {
    ///0: VREFOPAMP2 is output
    Output = 0,
    ///1: VREFOPAMP2 is not output
    NotOutput = 1,
}
impl From<TSTREF> for bool {
    #[inline(always)]
    fn from(variant: TSTREF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTREF` reader - TSTREF
pub type TSTREF_R = crate::BitReader<TSTREF>;
impl TSTREF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTREF {
        match self.bits {
            false => TSTREF::Output,
            true => TSTREF::NotOutput,
        }
    }
    ///VREFOPAMP2 is output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == TSTREF::Output
    }
    ///VREFOPAMP2 is not output
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == TSTREF::NotOutput
    }
}
///Field `TSTREF` writer - TSTREF
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG, TSTREF>;
impl<'a, REG> TSTREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREFOPAMP2 is output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF::Output)
    }
    ///VREFOPAMP2 is not output
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF::NotOutput)
    }
}
/**OPAMP ouput status flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTCAL {
    ///0: Non-inverting < inverting
    Low = 0,
    ///1: Non-inverting > inverting
    High = 1,
}
impl From<OUTCAL> for bool {
    #[inline(always)]
    fn from(variant: OUTCAL) -> Self {
        variant as u8 != 0
    }
}
///Field `OUTCAL` reader - OPAMP ouput status flag
pub type OUTCAL_R = crate::BitReader<OUTCAL>;
impl OUTCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTCAL {
        match self.bits {
            false => OUTCAL::Low,
            true => OUTCAL::High,
        }
    }
    ///Non-inverting < inverting
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTCAL::Low
    }
    ///Non-inverting > inverting
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTCAL::High
    }
}
/**OPAMP lock

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
///Field `LOCK` reader - OPAMP lock
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
///Field `LOCK` writer - OPAMP lock
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
    ///Bit 0 - OPAMP2 enable
    #[inline(always)]
    pub fn opamp2en(&self) -> OPAMP2EN_R {
        OPAMP2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP ouput status flag
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OPAMP lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP2_CSR")
            .field("opamp2en", &self.opamp2en())
            .field("force_vp", &self.force_vp())
            .field("vp_sel", &self.vp_sel())
            .field("vm_sel", &self.vm_sel())
            .field("tcm_en", &self.tcm_en())
            .field("vms_sel", &self.vms_sel())
            .field("vps_sel", &self.vps_sel())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("pga_gain", &self.pga_gain())
            .field("user_trim", &self.user_trim())
            .field("trimoffsetp", &self.trimoffsetp())
            .field("trimoffsetn", &self.trimoffsetn())
            .field("tstref", &self.tstref())
            .field("outcal", &self.outcal())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - OPAMP2 enable
    #[inline(always)]
    pub fn opamp2en(&mut self) -> OPAMP2EN_W<'_, OPAMP2_CSRrs> {
        OPAMP2EN_W::new(self, 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W<'_, OPAMP2_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP2_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TCM_EN_W<'_, OPAMP2_CSRrs> {
        TCM_EN_W::new(self, 7)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<'_, OPAMP2_CSRrs> {
        VMS_SEL_W::new(self, 8)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<'_, OPAMP2_CSRrs> {
        VPS_SEL_W::new(self, 9)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP2_CSRrs> {
        CALON_W::new(self, 11)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP2_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn user_trim(&mut self) -> USER_TRIM_W<'_, OPAMP2_CSRrs> {
        USER_TRIM_W::new(self, 18)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<'_, OPAMP2_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<'_, OPAMP2_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W<'_, OPAMP2_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    ///Bit 31 - OPAMP lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OPAMP2_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**OPAMP2 control register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#OPAMP:OPAMP2_CSR)*/
pub struct OPAMP2_CSRrs;
impl crate::RegisterSpec for OPAMP2_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp2_csr::R`](R) reader structure
impl crate::Readable for OPAMP2_CSRrs {}
///`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure
impl crate::Writable for OPAMP2_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP2_CSR to value 0
impl crate::Resettable for OPAMP2_CSRrs {}
