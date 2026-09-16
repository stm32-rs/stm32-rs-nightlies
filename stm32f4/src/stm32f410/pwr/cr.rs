///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Low-power deep sleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDS {
    ///0: Main voltage regulator ON during Stop mode
    Main = 0,
    ///1: Low-power voltage regulator ON during Stop mode
    LowPower = 1,
}
impl From<LPDS> for bool {
    #[inline(always)]
    fn from(variant: LPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader<LPDS>;
impl LPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPDS {
        match self.bits {
            false => LPDS::Main,
            true => LPDS::LowPower,
        }
    }
    ///Main voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == LPDS::Main
    }
    ///Low-power voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LPDS::LowPower
    }
}
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG, LPDS>;
impl<'a, REG> LPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn main(self) -> &'a mut crate::W<REG> {
        self.variant(LPDS::Main)
    }
    ///Low-power voltage regulator ON during Stop mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(LPDS::LowPower)
    }
}
/**Power down deepsleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS {
    ///0: Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    EnterStop = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    EnterStandby = 1,
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
            false => PDDS::EnterStop,
            true => PDDS::EnterStandby,
        }
    }
    ///Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    #[inline(always)]
    pub fn is_enter_stop(&self) -> bool {
        *self == PDDS::EnterStop
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn is_enter_standby(&self) -> bool {
        *self == PDDS::EnterStandby
    }
}
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG, PDDS>;
impl<'a, REG> PDDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enter Stop mode when the CPU enters deepsleep. The regulator status depends on the LPDS bit
    #[inline(always)]
    pub fn enter_stop(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::EnterStop)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn enter_standby(self) -> &'a mut crate::W<REG> {
        self.variant(PDDS::EnterStandby)
    }
}
/**Clear wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFR {
    ///0: This bit is always read as 0
    Zero = 0,
}
impl From<CWUFR> for bool {
    #[inline(always)]
    fn from(variant: CWUFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader<CWUFR>;
impl CWUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CWUFR> {
        match self.bits {
            false => Some(CWUFR::Zero),
            _ => None,
        }
    }
    ///This bit is always read as 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CWUFR::Zero
    }
}
/**Clear wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFW {
    ///1: Clear the WUPF Wakeup Flag **after 2 System clock cycles**
    Clear = 1,
}
impl From<CWUFW> for bool {
    #[inline(always)]
    fn from(variant: CWUFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG, CWUFW>;
impl<'a, REG> CWUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the WUPF Wakeup Flag **after 2 System clock cycles**
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUFW::Clear)
    }
}
/**Clear standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFR {
    ///0: This bit is always read as 0
    Zero = 0,
}
impl From<CSBFR> for bool {
    #[inline(always)]
    fn from(variant: CSBFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader<CSBFR>;
impl CSBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSBFR> {
        match self.bits {
            false => Some(CSBFR::Zero),
            _ => None,
        }
    }
    ///This bit is always read as 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSBFR::Zero
    }
}
/**Clear standby flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFW {
    ///1: Clear the SBF Standby Flag
    Clear = 1,
}
impl From<CSBFW> for bool {
    #[inline(always)]
    fn from(variant: CSBFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBFW>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the SBF Standby Flag
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
    ///0: PVD disabled
    Disabled = 0,
    ///1: PVD enabled
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
    ///PVD disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///PVD enabled
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
    ///PVD disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///PVD enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Disable backup domain write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Access to RTC and RTC Backup registers and backup SRAM disabled
    Protected = 0,
    ///1: Access to RTC and RTC Backup registers and backup SRAM enabled
    Writable = 1,
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
            false => DBP::Protected,
            true => DBP::Writable,
        }
    }
    ///Access to RTC and RTC Backup registers and backup SRAM disabled
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == DBP::Protected
    }
    ///Access to RTC and RTC Backup registers and backup SRAM enabled
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == DBP::Writable
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access to RTC and RTC Backup registers and backup SRAM disabled
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Protected)
    }
    ///Access to RTC and RTC Backup registers and backup SRAM enabled
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Writable)
    }
}
/**Flash power down in Stop mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS {
    ///0: Flash memory not in power-down when the device is in Stop mode
    Idle = 0,
    ///1: Flash memory in power-down when the device is in Stop mode
    PowerDown = 1,
}
impl From<FPDS> for bool {
    #[inline(always)]
    fn from(variant: FPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `FPDS` reader - Flash power down in Stop mode
pub type FPDS_R = crate::BitReader<FPDS>;
impl FPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPDS {
        match self.bits {
            false => FPDS::Idle,
            true => FPDS::PowerDown,
        }
    }
    ///Flash memory not in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS::Idle
    }
    ///Flash memory in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS::PowerDown
    }
}
///Field `FPDS` writer - Flash power down in Stop mode
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG, FPDS>;
impl<'a, REG> FPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory not in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::Idle)
    }
    ///Flash memory in power-down when the device is in Stop mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::PowerDown)
    }
}
/**Low-power regulator Low Voltage in Deep Sleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPLVDS {
    ///0: Low-power regulator on if LPDS bit is set when the device is in Stop mode
    On = 0,
    ///1: Low-power regulator in Low Voltage and Flash memory in Deep Sleep mode if LPDS bit is set when device is in Stop mode
    UnderDrive = 1,
}
impl From<LPLVDS> for bool {
    #[inline(always)]
    fn from(variant: LPLVDS) -> Self {
        variant as u8 != 0
    }
}
///Field `LPLVDS` reader - Low-power regulator Low Voltage in Deep Sleep
pub type LPLVDS_R = crate::BitReader<LPLVDS>;
impl LPLVDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPLVDS {
        match self.bits {
            false => LPLVDS::On,
            true => LPLVDS::UnderDrive,
        }
    }
    ///Low-power regulator on if LPDS bit is set when the device is in Stop mode
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LPLVDS::On
    }
    ///Low-power regulator in Low Voltage and Flash memory in Deep Sleep mode if LPDS bit is set when device is in Stop mode
    #[inline(always)]
    pub fn is_under_drive(&self) -> bool {
        *self == LPLVDS::UnderDrive
    }
}
///Field `LPLVDS` writer - Low-power regulator Low Voltage in Deep Sleep
pub type LPLVDS_W<'a, REG> = crate::BitWriter<'a, REG, LPLVDS>;
impl<'a, REG> LPLVDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low-power regulator on if LPDS bit is set when the device is in Stop mode
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LPLVDS::On)
    }
    ///Low-power regulator in Low Voltage and Flash memory in Deep Sleep mode if LPDS bit is set when device is in Stop mode
    #[inline(always)]
    pub fn under_drive(self) -> &'a mut crate::W<REG> {
        self.variant(LPLVDS::UnderDrive)
    }
}
/**Main regulator Low Voltage in Deep Sleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRLVDS {
    ///0: Main regulator in Voltage scale 3 when the device is in Stop mode
    Scale3 = 0,
    ///1: Main regulator in Low Voltage and Flash memory in Deep Sleep mode when the device is in Stop mode
    LowVoltage = 1,
}
impl From<MRLVDS> for bool {
    #[inline(always)]
    fn from(variant: MRLVDS) -> Self {
        variant as u8 != 0
    }
}
///Field `MRLVDS` reader - Main regulator Low Voltage in Deep Sleep
pub type MRLVDS_R = crate::BitReader<MRLVDS>;
impl MRLVDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MRLVDS {
        match self.bits {
            false => MRLVDS::Scale3,
            true => MRLVDS::LowVoltage,
        }
    }
    ///Main regulator in Voltage scale 3 when the device is in Stop mode
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == MRLVDS::Scale3
    }
    ///Main regulator in Low Voltage and Flash memory in Deep Sleep mode when the device is in Stop mode
    #[inline(always)]
    pub fn is_low_voltage(&self) -> bool {
        *self == MRLVDS::LowVoltage
    }
}
///Field `MRLVDS` writer - Main regulator Low Voltage in Deep Sleep
pub type MRLVDS_W<'a, REG> = crate::BitWriter<'a, REG, MRLVDS>;
impl<'a, REG> MRLVDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main regulator in Voltage scale 3 when the device is in Stop mode
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(MRLVDS::Scale3)
    }
    ///Main regulator in Low Voltage and Flash memory in Deep Sleep mode when the device is in Stop mode
    #[inline(always)]
    pub fn low_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(MRLVDS::LowVoltage)
    }
}
///Field `ADCDC1` reader - ADCDC1
pub type ADCDC1_R = crate::BitReader;
///Field `ADCDC1` writer - ADCDC1
pub type ADCDC1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Regulator voltage scaling output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///1: Scale 3 mode <= 64 MHz
    Scale3 = 1,
    ///2: Scale 2 mode (reset value) <= 84 MHz
    Scale2 = 2,
    ///3: Scale 1 mode <= 100 MHz
    Scale1 = 3,
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
///Field `VOS` reader - Regulator voltage scaling output selection
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::Scale3),
            2 => Some(VOS::Scale2),
            3 => Some(VOS::Scale1),
            _ => None,
        }
    }
    ///Scale 3 mode <= 64 MHz
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == VOS::Scale3
    }
    ///Scale 2 mode (reset value) <= 84 MHz
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == VOS::Scale2
    }
    ///Scale 1 mode <= 100 MHz
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == VOS::Scale1
    }
}
///Field `VOS` writer - Regulator voltage scaling output selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Scale 3 mode <= 64 MHz
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale3)
    }
    ///Scale 2 mode (reset value) <= 84 MHz
    #[inline(always)]
    pub fn scale2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale2)
    }
    ///Scale 1 mode <= 100 MHz
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Scale1)
    }
}
/**Flash Memory Sleep System Run

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMSSR {
    ///0: Flash standard mode (Default value)
    Standard = 0,
    ///1: Flash forced to be in STOP or DeepPower Down mode (depending of FPDS value bit) by hardware
    Forced = 1,
}
impl From<FMSSR> for bool {
    #[inline(always)]
    fn from(variant: FMSSR) -> Self {
        variant as u8 != 0
    }
}
///Field `FMSSR` reader - Flash Memory Sleep System Run
pub type FMSSR_R = crate::BitReader<FMSSR>;
impl FMSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMSSR {
        match self.bits {
            false => FMSSR::Standard,
            true => FMSSR::Forced,
        }
    }
    ///Flash standard mode (Default value)
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FMSSR::Standard
    }
    ///Flash forced to be in STOP or DeepPower Down mode (depending of FPDS value bit) by hardware
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == FMSSR::Forced
    }
}
///Field `FMSSR` writer - Flash Memory Sleep System Run
pub type FMSSR_W<'a, REG> = crate::BitWriter<'a, REG, FMSSR>;
impl<'a, REG> FMSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash standard mode (Default value)
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(FMSSR::Standard)
    }
    ///Flash forced to be in STOP or DeepPower Down mode (depending of FPDS value bit) by hardware
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(FMSSR::Forced)
    }
}
/**Flash Interface Stop while System Run

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FISSR {
    ///0: Flash Interface clock run (Default value)
    Run = 0,
    ///1: Flash Interface clock off
    Off = 1,
}
impl From<FISSR> for bool {
    #[inline(always)]
    fn from(variant: FISSR) -> Self {
        variant as u8 != 0
    }
}
///Field `FISSR` reader - Flash Interface Stop while System Run
pub type FISSR_R = crate::BitReader<FISSR>;
impl FISSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FISSR {
        match self.bits {
            false => FISSR::Run,
            true => FISSR::Off,
        }
    }
    ///Flash Interface clock run (Default value)
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == FISSR::Run
    }
    ///Flash Interface clock off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FISSR::Off
    }
}
///Field `FISSR` writer - Flash Interface Stop while System Run
pub type FISSR_W<'a, REG> = crate::BitWriter<'a, REG, FISSR>;
impl<'a, REG> FISSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash Interface clock run (Default value)
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(FISSR::Run)
    }
    ///Flash Interface clock off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FISSR::Off)
    }
}
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
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
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-power regulator Low Voltage in Deep Sleep
    #[inline(always)]
    pub fn lplvds(&self) -> LPLVDS_R {
        LPLVDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Main regulator Low Voltage in Deep Sleep
    #[inline(always)]
    pub fn mrlvds(&self) -> MRLVDS_R {
        MRLVDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&self) -> ADCDC1_R {
        ADCDC1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 20 - Flash Memory Sleep System Run
    #[inline(always)]
    pub fn fmssr(&self) -> FMSSR_R {
        FMSSR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Flash Interface Stop while System Run
    #[inline(always)]
    pub fn fissr(&self) -> FISSR_R {
        FISSR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("vos", &self.vos())
            .field("adcdc1", &self.adcdc1())
            .field("fpds", &self.fpds())
            .field("dbp", &self.dbp())
            .field("pls", &self.pls())
            .field("pvde", &self.pvde())
            .field("csbf", &self.csbf())
            .field("cwuf", &self.cwuf())
            .field("pdds", &self.pdds())
            .field("lpds", &self.lpds())
            .field("fissr", &self.fissr())
            .field("fmssr", &self.fmssr())
            .field("mrlvds", &self.mrlvds())
            .field("lplvds", &self.lplvds())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CRrs> {
        LPDS_W::new(self, 0)
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
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CRrs> {
        FPDS_W::new(self, 9)
    }
    ///Bit 10 - Low-power regulator Low Voltage in Deep Sleep
    #[inline(always)]
    pub fn lplvds(&mut self) -> LPLVDS_W<'_, CRrs> {
        LPLVDS_W::new(self, 10)
    }
    ///Bit 11 - Main regulator Low Voltage in Deep Sleep
    #[inline(always)]
    pub fn mrlvds(&mut self) -> MRLVDS_W<'_, CRrs> {
        MRLVDS_W::new(self, 11)
    }
    ///Bit 13 - ADCDC1
    #[inline(always)]
    pub fn adcdc1(&mut self) -> ADCDC1_W<'_, CRrs> {
        ADCDC1_W::new(self, 13)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CRrs> {
        VOS_W::new(self, 14)
    }
    ///Bit 20 - Flash Memory Sleep System Run
    #[inline(always)]
    pub fn fmssr(&mut self) -> FMSSR_W<'_, CRrs> {
        FMSSR_W::new(self, 20)
    }
    ///Bit 21 - Flash Interface Stop while System Run
    #[inline(always)]
    pub fn fissr(&mut self) -> FISSR_W<'_, CRrs> {
        FISSR_W::new(self, 21)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#PWR:CR)*/
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
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
