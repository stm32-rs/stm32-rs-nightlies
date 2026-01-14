///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Low-power deepsleep/Sleep/Low-power run

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSDSR {
    ///0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode
    MainMode = 0,
    ///1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
    LowPowerMode = 1,
}
impl From<LPSDSR> for bool {
    #[inline(always)]
    fn from(variant: LPSDSR) -> Self {
        variant as u8 != 0
    }
}
///Field `LPSDSR` reader - Low-power deepsleep/Sleep/Low-power run
pub type LPSDSR_R = crate::BitReader<LPSDSR>;
impl LPSDSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPSDSR {
        match self.bits {
            false => LPSDSR::MainMode,
            true => LPSDSR::LowPowerMode,
        }
    }
    ///Voltage regulator on during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPSDSR::MainMode
    }
    ///Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPSDSR::LowPowerMode
    }
}
///Field `LPSDSR` writer - Low-power deepsleep/Sleep/Low-power run
pub type LPSDSR_W<'a, REG> = crate::BitWriter<'a, REG, LPSDSR>;
impl<'a, REG> LPSDSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage regulator on during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPSDSR::MainMode)
    }
    ///Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPSDSR::LowPowerMode)
    }
}
/**Power down deepsleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    ///0: Enter Stop mode when the CPU enters deepsleep
    StopMode = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    StandbyMode = 1,
}
impl From<PDDS> for bool {
    #[inline(always)]
    fn from(variant: PDDS) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader<PDDS>;
impl PDDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDS {
        match self.bits {
            false => PDDS::StopMode,
            true => PDDS::StandbyMode,
        }
    }
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS::StopMode
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS::StandbyMode
    }
}
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StopMode)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StandbyMode)
    }
}
/**Clear wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFW {
    ///1: Clear the WUF Wakeup flag after 2 system clock cycles
    Clear = 1,
}
impl From<CWUFW> for bool {
    #[inline(always)]
    fn from(variant: CWUFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader<CWUFW>;
impl CWUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CWUFW> {
        match self.bits {
            true => Some(CWUFW::Clear),
            _ => None,
        }
    }
    ///Clear the WUF Wakeup flag after 2 system clock cycles
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CWUFW::Clear
    }
}
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG, CWUFW>;
impl<'a, REG> CWUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the WUF Wakeup flag after 2 system clock cycles
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUFW::Clear)
    }
}
/**Clear standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFW {
    ///1: Clear the SBF Standby flag
    Clear = 1,
}
impl From<CSBFW> for bool {
    #[inline(always)]
    fn from(variant: CSBFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader<CSBFW>;
impl CSBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSBFW> {
        match self.bits {
            true => Some(CSBFW::Clear),
            _ => None,
        }
    }
    ///Clear the SBF Standby flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSBFW::Clear
    }
}
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBFW>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the SBF Standby flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSBFW::Clear)
    }
}
/**Power voltage detector enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    ///0: PVD Disabled
    Disabled = 0,
    ///1: PVD Enabled
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    ///PVD Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///PVD Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///PVD Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
/**PVD level selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    ///0: 1.9 V
    V1_9 = 0,
    ///1: 2.1 V
    V2_1 = 1,
    ///2: 2.3 V
    V2_3 = 2,
    ///3: 2.5 V
    V2_5 = 3,
    ///4: 2.7 V
    V2_7 = 4,
    ///5: 2.9 V
    V2_9 = 5,
    ///6: 3.1 V
    V3_1 = 6,
    ///7: External input analog voltage (Compare internally to VREFINT)
    External = 7,
}
impl From<PLS> for u8 {
    #[inline(always)]
    fn from(variant: PLS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLS {
    type Ux = u8;
}
impl crate::IsEnum for PLS {}
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLS {
        match self.bits {
            0 => PLS::V1_9,
            1 => PLS::V2_1,
            2 => PLS::V2_3,
            3 => PLS::V2_5,
            4 => PLS::V2_7,
            5 => PLS::V2_9,
            6 => PLS::V3_1,
            7 => PLS::External,
            _ => unreachable!(),
        }
    }
    ///1.9 V
    #[inline(always)]
    pub fn is_v1_9(&self) -> bool {
        *self == PLS::V1_9
    }
    ///2.1 V
    #[inline(always)]
    pub fn is_v2_1(&self) -> bool {
        *self == PLS::V2_1
    }
    ///2.3 V
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == PLS::V2_3
    }
    ///2.5 V
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PLS::V2_5
    }
    ///2.7 V
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == PLS::V2_7
    }
    ///2.9 V
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PLS::V2_9
    }
    ///3.1 V
    #[inline(always)]
    pub fn is_v3_1(&self) -> bool {
        *self == PLS::V3_1
    }
    ///External input analog voltage (Compare internally to VREFINT)
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLS::External
    }
}
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLS, crate::Safe>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.9 V
    #[inline(always)]
    pub fn v1_9(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V1_9)
    }
    ///2.1 V
    #[inline(always)]
    pub fn v2_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_1)
    }
    ///2.3 V
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_3)
    }
    ///2.5 V
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_5)
    }
    ///2.7 V
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_7)
    }
    ///2.9 V
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_9)
    }
    ///3.1 V
    #[inline(always)]
    pub fn v3_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V3_1)
    }
    ///External input analog voltage (Compare internally to VREFINT)
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::External)
    }
}
/**Disable backup domain write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Access to RTC, RTC Backup and RCC CSR registers disabled
    Disabled = 0,
    ///1: Access to RTC, RTC Backup and RCC CSR registers enabled
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    ///Access to RTC, RTC Backup and RCC CSR registers disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    ///Access to RTC, RTC Backup and RCC CSR registers enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access to RTC, RTC Backup and RCC CSR registers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    ///Access to RTC, RTC Backup and RCC CSR registers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
/**Ultra-low-power mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULP {
    ///0: VREFINT is on in low-power mode
    Enabled = 0,
    ///1: VREFINT is off in low-power mode
    Disabled = 1,
}
impl From<ULP> for bool {
    #[inline(always)]
    fn from(variant: ULP) -> Self {
        variant as u8 != 0
    }
}
///Field `ULP` reader - Ultra-low-power mode
pub type ULP_R = crate::BitReader<ULP>;
impl ULP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ULP {
        match self.bits {
            false => ULP::Enabled,
            true => ULP::Disabled,
        }
    }
    ///VREFINT is on in low-power mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ULP::Enabled
    }
    ///VREFINT is off in low-power mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ULP::Disabled
    }
}
///Field `ULP` writer - Ultra-low-power mode
pub type ULP_W<'a, REG> = crate::BitWriter<'a, REG, ULP>;
impl<'a, REG> ULP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREFINT is on in low-power mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULP::Enabled)
    }
    ///VREFINT is off in low-power mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULP::Disabled)
    }
}
/**Fast wakeup

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWU {
    ///0: Low-power modes exit occurs only when VREFINT is ready
    Disabled = 0,
    ///1: VREFINT start up time is ignored when exiting low-power modes
    Enabled = 1,
}
impl From<FWU> for bool {
    #[inline(always)]
    fn from(variant: FWU) -> Self {
        variant as u8 != 0
    }
}
///Field `FWU` reader - Fast wakeup
pub type FWU_R = crate::BitReader<FWU>;
impl FWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FWU {
        match self.bits {
            false => FWU::Disabled,
            true => FWU::Enabled,
        }
    }
    ///Low-power modes exit occurs only when VREFINT is ready
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWU::Disabled
    }
    ///VREFINT start up time is ignored when exiting low-power modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWU::Enabled
    }
}
///Field `FWU` writer - Fast wakeup
pub type FWU_W<'a, REG> = crate::BitWriter<'a, REG, FWU>;
impl<'a, REG> FWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low-power modes exit occurs only when VREFINT is ready
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWU::Disabled)
    }
    ///VREFINT start up time is ignored when exiting low-power modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWU::Enabled)
    }
}
/**Voltage scaling range selection

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///1: 1.8 V (range 1)
    V1_8 = 1,
    ///2: 1.5 V (range 2)
    V1_5 = 2,
    ///3: 1.2 V (range 3)
    V1_2 = 3,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
impl crate::IsEnum for VOS {}
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::V1_8),
            2 => Some(VOS::V1_5),
            3 => Some(VOS::V1_2),
            _ => None,
        }
    }
    ///1.8 V (range 1)
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == VOS::V1_8
    }
    ///1.5 V (range 2)
    #[inline(always)]
    pub fn is_v1_5(&self) -> bool {
        *self == VOS::V1_5
    }
    ///1.2 V (range 3)
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS::V1_2
    }
}
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.8 V (range 1)
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_8)
    }
    ///1.5 V (range 2)
    #[inline(always)]
    pub fn v1_5(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_5)
    }
    ///1.2 V (range 3)
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_2)
    }
}
/**Deep sleep mode with Flash memory kept off

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS_EE_KOFF {
    ///0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
    NvmwakeUp = 0,
    ///1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
    Nvmsleep = 1,
}
impl From<DS_EE_KOFF> for bool {
    #[inline(always)]
    fn from(variant: DS_EE_KOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `DS_EE_KOFF` reader - Deep sleep mode with Flash memory kept off
pub type DS_EE_KOFF_R = crate::BitReader<DS_EE_KOFF>;
impl DS_EE_KOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DS_EE_KOFF {
        match self.bits {
            false => DS_EE_KOFF::NvmwakeUp,
            true => DS_EE_KOFF::Nvmsleep,
        }
    }
    ///NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
    #[inline(always)]
    pub fn is_nvmwake_up(&self) -> bool {
        *self == DS_EE_KOFF::NvmwakeUp
    }
    ///NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
    #[inline(always)]
    pub fn is_nvmsleep(&self) -> bool {
        *self == DS_EE_KOFF::Nvmsleep
    }
}
///Field `DS_EE_KOFF` writer - Deep sleep mode with Flash memory kept off
pub type DS_EE_KOFF_W<'a, REG> = crate::BitWriter<'a, REG, DS_EE_KOFF>;
impl<'a, REG> DS_EE_KOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
    #[inline(always)]
    pub fn nvmwake_up(self) -> &'a mut crate::W<REG> {
        self.variant(DS_EE_KOFF::NvmwakeUp)
    }
    ///NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
    #[inline(always)]
    pub fn nvmsleep(self) -> &'a mut crate::W<REG> {
        self.variant(DS_EE_KOFF::Nvmsleep)
    }
}
/**Low power run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRUN {
    ///0: Voltage regulator in Main mode in Low-power run mode
    MainMode = 0,
    ///1: Voltage regulator in low-power mode in Low-power run mode
    LowPowerMode = 1,
}
impl From<LPRUN> for bool {
    #[inline(always)]
    fn from(variant: LPRUN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPRUN` reader - Low power run mode
pub type LPRUN_R = crate::BitReader<LPRUN>;
impl LPRUN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPRUN {
        match self.bits {
            false => LPRUN::MainMode,
            true => LPRUN::LowPowerMode,
        }
    }
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPRUN::MainMode
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPRUN::LowPowerMode
    }
}
///Field `LPRUN` writer - Low power run mode
pub type LPRUN_W<'a, REG> = crate::BitWriter<'a, REG, LPRUN>;
impl<'a, REG> LPRUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPRUN::MainMode)
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPRUN::LowPowerMode)
    }
}
impl R {
    ///Bit 0 - Low-power deepsleep/Sleep/Low-power run
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Ultra-low-power mode
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - Deep sleep mode with Flash memory kept off
    #[inline(always)]
    pub fn ds_ee_koff(&self) -> DS_EE_KOFF_R {
        DS_EE_KOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lpsdsr", &self.lpsdsr())
            .field("pdds", &self.pdds())
            .field("cwuf", &self.cwuf())
            .field("csbf", &self.csbf())
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("ulp", &self.ulp())
            .field("fwu", &self.fwu())
            .field("vos", &self.vos())
            .field("ds_ee_koff", &self.ds_ee_koff())
            .field("lprun", &self.lprun())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deepsleep/Sleep/Low-power run
    #[inline(always)]
    pub fn lpsdsr(&mut self) -> LPSDSR_W<'_, CRrs> {
        LPSDSR_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CRrs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<'_, CRrs> {
        CWUF_W::new(self, 2)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CRrs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CRrs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CRrs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Ultra-low-power mode
    #[inline(always)]
    pub fn ulp(&mut self) -> ULP_W<'_, CRrs> {
        ULP_W::new(self, 9)
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&mut self) -> FWU_W<'_, CRrs> {
        FWU_W::new(self, 10)
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CRrs> {
        VOS_W::new(self, 11)
    }
    ///Bit 13 - Deep sleep mode with Flash memory kept off
    #[inline(always)]
    pub fn ds_ee_koff(&mut self) -> DS_EE_KOFF_W<'_, CRrs> {
        DS_EE_KOFF_W::new(self, 13)
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&mut self) -> LPRUN_W<'_, CRrs> {
        LPRUN_W::new(self, 14)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#PWR:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x1000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x1000;
}
