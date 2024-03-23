#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `LPDS` reader - Low-power deep sleep"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - Low-power deep sleep"]
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-power deepsleep/Sleep/Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSDSR {
    #[doc = "0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    MainMode = 0,
    #[doc = "1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    LowPowerMode = 1,
}
impl From<LPSDSR> for bool {
    #[inline(always)]
    fn from(variant: LPSDSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSDSR` reader - Low-power deepsleep/Sleep/Low-power run"]
pub type LPSDSR_R = crate::BitReader<LPSDSR>;
impl LPSDSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPSDSR {
        match self.bits {
            false => LPSDSR::MainMode,
            true => LPSDSR::LowPowerMode,
        }
    }
    #[doc = "Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPSDSR::MainMode
    }
    #[doc = "Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPSDSR::LowPowerMode
    }
}
#[doc = "Field `LPSDSR` writer - Low-power deepsleep/Sleep/Low-power run"]
pub type LPSDSR_W<'a, REG> = crate::BitWriter<'a, REG, LPSDSR>;
impl<'a, REG> LPSDSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPSDSR::MainMode)
    }
    #[doc = "Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPSDSR::LowPowerMode)
    }
}
#[doc = "Power down deepsleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    #[doc = "0: Enter Stop mode when the CPU enters deepsleep"]
    StopMode = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    StandbyMode = 1,
}
impl From<PDDS> for bool {
    #[inline(always)]
    fn from(variant: PDDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PDDS_R = crate::BitReader<PDDS>;
impl PDDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDDS {
        match self.bits {
            false => PDDS::StopMode,
            true => PDDS::StandbyMode,
        }
    }
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS::StopMode
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS::StandbyMode
    }
}
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StopMode)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::StandbyMode)
    }
}
#[doc = "Clear wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFW {
    #[doc = "1: Clear the WUF Wakeup flag after 2 system clock cycles"]
    Clear = 1,
}
impl From<CWUFW> for bool {
    #[inline(always)]
    fn from(variant: CWUFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub type CWUF_R = crate::BitReader<CWUFW>;
impl CWUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CWUFW> {
        match self.bits {
            true => Some(CWUFW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the WUF Wakeup flag after 2 system clock cycles"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CWUFW::Clear
    }
}
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG, CWUFW>;
impl<'a, REG> CWUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the WUF Wakeup flag after 2 system clock cycles"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUFW::Clear)
    }
}
#[doc = "Clear standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFW {
    #[doc = "1: Clear the SBF Standby flag"]
    Clear = 1,
}
impl From<CSBFW> for bool {
    #[inline(always)]
    fn from(variant: CSBFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CSBF_R = crate::BitReader<CSBFW>;
impl CSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSBFW> {
        match self.bits {
            true => Some(CSBFW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the SBF Standby flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSBFW::Clear
    }
}
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBFW>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the SBF Standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSBFW::Clear)
    }
}
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    #[doc = "0: PVD Disabled"]
    Disabled = 0,
    #[doc = "1: PVD Enabled"]
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
#[doc = "PVD level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    #[doc = "0: 1.9 V"]
    V1_9 = 0,
    #[doc = "1: 2.1 V"]
    V2_1 = 1,
    #[doc = "2: 2.3 V"]
    V2_3 = 2,
    #[doc = "3: 2.5 V"]
    V2_5 = 3,
    #[doc = "4: 2.7 V"]
    V2_7 = 4,
    #[doc = "5: 2.9 V"]
    V2_9 = 5,
    #[doc = "6: 3.1 V"]
    V3_1 = 6,
    #[doc = "7: External input analog voltage (Compare internally to VREFINT)"]
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
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn is_v1_9(&self) -> bool {
        *self == PLS::V1_9
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_v2_1(&self) -> bool {
        *self == PLS::V2_1
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == PLS::V2_3
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PLS::V2_5
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == PLS::V2_7
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PLS::V2_9
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn is_v3_1(&self) -> bool {
        *self == PLS::V3_1
    }
    #[doc = "External input analog voltage (Compare internally to VREFINT)"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLS::External
    }
}
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PLS>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn v1_9(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V1_9)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn v2_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_1)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_3)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_5)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_7)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V2_9)
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn v3_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::V3_1)
    }
    #[doc = "External input analog voltage (Compare internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::External)
    }
}
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    #[doc = "0: Access to RTC, RTC Backup and RCC CSR registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to RTC, RTC Backup and RCC CSR registers enabled"]
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
#[doc = "Ultra-low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULP {
    #[doc = "0: VREFINT is on in low-power mode"]
    Enabled = 0,
    #[doc = "1: VREFINT is off in low-power mode"]
    Disabled = 1,
}
impl From<ULP> for bool {
    #[inline(always)]
    fn from(variant: ULP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULP` reader - Ultra-low-power mode"]
pub type ULP_R = crate::BitReader<ULP>;
impl ULP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULP {
        match self.bits {
            false => ULP::Enabled,
            true => ULP::Disabled,
        }
    }
    #[doc = "VREFINT is on in low-power mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ULP::Enabled
    }
    #[doc = "VREFINT is off in low-power mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ULP::Disabled
    }
}
#[doc = "Field `ULP` writer - Ultra-low-power mode"]
pub type ULP_W<'a, REG> = crate::BitWriter<'a, REG, ULP>;
impl<'a, REG> ULP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT is on in low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULP::Enabled)
    }
    #[doc = "VREFINT is off in low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ULP::Disabled)
    }
}
#[doc = "Fast wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWU {
    #[doc = "0: Low-power modes exit occurs only when VREFINT is ready"]
    Disabled = 0,
    #[doc = "1: VREFINT start up time is ignored when exiting low-power modes"]
    Enabled = 1,
}
impl From<FWU> for bool {
    #[inline(always)]
    fn from(variant: FWU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWU` reader - Fast wakeup"]
pub type FWU_R = crate::BitReader<FWU>;
impl FWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWU {
        match self.bits {
            false => FWU::Disabled,
            true => FWU::Enabled,
        }
    }
    #[doc = "Low-power modes exit occurs only when VREFINT is ready"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWU::Disabled
    }
    #[doc = "VREFINT start up time is ignored when exiting low-power modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWU::Enabled
    }
}
#[doc = "Field `FWU` writer - Fast wakeup"]
pub type FWU_W<'a, REG> = crate::BitWriter<'a, REG, FWU>;
impl<'a, REG> FWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-power modes exit occurs only when VREFINT is ready"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWU::Disabled)
    }
    #[doc = "VREFINT start up time is ignored when exiting low-power modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWU::Enabled)
    }
}
#[doc = "Voltage scaling range selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    #[doc = "1: 1.8 V (range 1)"]
    V1_8 = 1,
    #[doc = "2: 1.5 V (range 2)"]
    V1_5 = 2,
    #[doc = "3: 1.2 V (range 3)"]
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
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::V1_8),
            2 => Some(VOS::V1_5),
            3 => Some(VOS::V1_2),
            _ => None,
        }
    }
    #[doc = "1.8 V (range 1)"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == VOS::V1_8
    }
    #[doc = "1.5 V (range 2)"]
    #[inline(always)]
    pub fn is_v1_5(&self) -> bool {
        *self == VOS::V1_5
    }
    #[doc = "1.2 V (range 3)"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS::V1_2
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.8 V (range 1)"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_8)
    }
    #[doc = "1.5 V (range 2)"]
    #[inline(always)]
    pub fn v1_5(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_5)
    }
    #[doc = "1.2 V (range 3)"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_2)
    }
}
#[doc = "Deep sleep mode with Flash memory kept off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS_EE_KOFF {
    #[doc = "0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    NvmwakeUp = 0,
    #[doc = "1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    Nvmsleep = 1,
}
impl From<DS_EE_KOFF> for bool {
    #[inline(always)]
    fn from(variant: DS_EE_KOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS_EE_KOFF` reader - Deep sleep mode with Flash memory kept off"]
pub type DS_EE_KOFF_R = crate::BitReader<DS_EE_KOFF>;
impl DS_EE_KOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DS_EE_KOFF {
        match self.bits {
            false => DS_EE_KOFF::NvmwakeUp,
            true => DS_EE_KOFF::Nvmsleep,
        }
    }
    #[doc = "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    #[inline(always)]
    pub fn is_nvmwake_up(&self) -> bool {
        *self == DS_EE_KOFF::NvmwakeUp
    }
    #[doc = "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    #[inline(always)]
    pub fn is_nvmsleep(&self) -> bool {
        *self == DS_EE_KOFF::Nvmsleep
    }
}
#[doc = "Field `DS_EE_KOFF` writer - Deep sleep mode with Flash memory kept off"]
pub type DS_EE_KOFF_W<'a, REG> = crate::BitWriter<'a, REG, DS_EE_KOFF>;
impl<'a, REG> DS_EE_KOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    #[inline(always)]
    pub fn nvmwake_up(self) -> &'a mut crate::W<REG> {
        self.variant(DS_EE_KOFF::NvmwakeUp)
    }
    #[doc = "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    #[inline(always)]
    pub fn nvmsleep(self) -> &'a mut crate::W<REG> {
        self.variant(DS_EE_KOFF::Nvmsleep)
    }
}
#[doc = "Low power run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRUN {
    #[doc = "0: Voltage regulator in Main mode in Low-power run mode"]
    MainMode = 0,
    #[doc = "1: Voltage regulator in low-power mode in Low-power run mode"]
    LowPowerMode = 1,
}
impl From<LPRUN> for bool {
    #[inline(always)]
    fn from(variant: LPRUN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRUN` reader - Low power run mode"]
pub type LPRUN_R = crate::BitReader<LPRUN>;
impl LPRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPRUN {
        match self.bits {
            false => LPRUN::MainMode,
            true => LPRUN::LowPowerMode,
        }
    }
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPRUN::MainMode
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPRUN::LowPowerMode
    }
}
#[doc = "Field `LPRUN` writer - Low power run mode"]
pub type LPRUN_W<'a, REG> = crate::BitWriter<'a, REG, LPRUN>;
impl<'a, REG> LPRUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPRUN::MainMode)
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPRUN::LowPowerMode)
    }
}
impl R {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    pub fn ds_ee_koff(&self) -> DS_EE_KOFF_R {
        DS_EE_KOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<CRrs> {
        LPDS_W::new(self, 0)
    }
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpsdsr(&mut self) -> LPSDSR_W<CRrs> {
        LPSDSR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<CRrs> {
        PDDS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<CRrs> {
        CWUF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<CRrs> {
        CSBF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CRrs> {
        PVDE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CRrs> {
        PLS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CRrs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ulp(&mut self) -> ULP_W<CRrs> {
        ULP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fwu(&mut self) -> FWU_W<CRrs> {
        FWU_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CRrs> {
        VOS_W::new(self, 11)
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    #[must_use]
    pub fn ds_ee_koff(&mut self) -> DS_EE_KOFF_W<CRrs> {
        DS_EE_KOFF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn lprun(&mut self) -> LPRUN_W<CRrs> {
        LPRUN_W::new(self, 14)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x1000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x1000;
}
