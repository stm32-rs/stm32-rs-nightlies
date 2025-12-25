///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
/**Option lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    ///0: The write and erase operations in the Option bytes area are disabled
    Unlocked = 0,
    ///1: The write and erase operations in the Option bytes area are enabled
    Locked = 1,
}
impl From<OPTLOCKR> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` reader - Option lock
pub type OPTLOCK_R = crate::BitReader<OPTLOCKR>;
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTLOCKR {
        match self.bits {
            false => OPTLOCKR::Unlocked,
            true => OPTLOCKR::Locked,
        }
    }
    ///The write and erase operations in the Option bytes area are disabled
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
    ///The write and erase operations in the Option bytes area are enabled
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCKR::Locked
    }
}
/**Option lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    ///1: Lock the FLASH_OPTCR register
    Set = 1,
}
impl From<OPTLOCKW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` writer - Option lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCKW>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Lock the FLASH_OPTCR register
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Set)
    }
}
/**Option start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR {
    ///0: Cleared when BSY bit is cleared in SR
    Complete = 0,
    ///1: Options modification requested
    Requested = 1,
}
impl From<OPTSTRTR> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` reader - Option start
pub type OPTSTRT_R = crate::BitReader<OPTSTRTR>;
impl OPTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTSTRTR {
        match self.bits {
            false => OPTSTRTR::Complete,
            true => OPTSTRTR::Requested,
        }
    }
    ///Cleared when BSY bit is cleared in SR
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OPTSTRTR::Complete
    }
    ///Options modification requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OPTSTRTR::Requested
    }
}
/**Option start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW {
    ///1: This bit triggers an options operation when set
    Set = 1,
}
impl From<OPTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` writer - Option start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, OPTSTRTW>;
impl<'a, REG> OPTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit triggers an options operation when set
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OPTSTRTW::Set)
    }
}
/**BOR reset Level

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOR_LEV {
    ///0: Reset threshold level for POR/PDR (around 1.7V)
    BorOff = 0,
    ///1: Reset threshold level for VBOR1 (around 2.2 V)
    BorLevel1 = 1,
    ///2: Reset threshold level for VBOR2 (around 2.5 V)
    BorLevel2 = 2,
    ///3: Reset threshold level for VBOR3 (around 2.8 V)
    BorLevel3 = 3,
}
impl From<BOR_LEV> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOR_LEV {
    type Ux = u8;
}
impl crate::IsEnum for BOR_LEV {}
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<BOR_LEV>;
impl BOR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOR_LEV {
        match self.bits {
            0 => BOR_LEV::BorOff,
            1 => BOR_LEV::BorLevel1,
            2 => BOR_LEV::BorLevel2,
            3 => BOR_LEV::BorLevel3,
            _ => unreachable!(),
        }
    }
    ///Reset threshold level for POR/PDR (around 1.7V)
    #[inline(always)]
    pub fn is_bor_off(&self) -> bool {
        *self == BOR_LEV::BorOff
    }
    ///Reset threshold level for VBOR1 (around 2.2 V)
    #[inline(always)]
    pub fn is_bor_level1(&self) -> bool {
        *self == BOR_LEV::BorLevel1
    }
    ///Reset threshold level for VBOR2 (around 2.5 V)
    #[inline(always)]
    pub fn is_bor_level2(&self) -> bool {
        *self == BOR_LEV::BorLevel2
    }
    ///Reset threshold level for VBOR3 (around 2.8 V)
    #[inline(always)]
    pub fn is_bor_level3(&self) -> bool {
        *self == BOR_LEV::BorLevel3
    }
}
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BOR_LEV, crate::Safe>;
impl<'a, REG> BOR_LEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset threshold level for POR/PDR (around 1.7V)
    #[inline(always)]
    pub fn bor_off(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorOff)
    }
    ///Reset threshold level for VBOR1 (around 2.2 V)
    #[inline(always)]
    pub fn bor_level1(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel1)
    }
    ///Reset threshold level for VBOR2 (around 2.5 V)
    #[inline(always)]
    pub fn bor_level2(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel2)
    }
    ///Reset threshold level for VBOR3 (around 2.8 V)
    #[inline(always)]
    pub fn bor_level3(self) -> &'a mut crate::W<REG> {
        self.variant(BOR_LEV::BorLevel3)
    }
}
/**User option bytes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDG_SW {
    ///0: Hardware window watchdog
    Hardware = 0,
    ///1: Software window watchdog
    Software = 1,
}
impl From<WWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDG_SW` reader - User option bytes
pub type WWDG_SW_R = crate::BitReader<WWDG_SW>;
impl WWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDG_SW {
        match self.bits {
            false => WWDG_SW::Hardware,
            true => WWDG_SW::Software,
        }
    }
    ///Hardware window watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WWDG_SW::Hardware
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WWDG_SW::Software
    }
}
///Field `WWDG_SW` writer - User option bytes
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, WWDG_SW>;
impl<'a, REG> WWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware window watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Hardware)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(WWDG_SW::Software)
    }
}
/**User option bytes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_SW {
    ///0: Hardware independant watchdog
    Hardware = 0,
    ///1: Software independant watchdog
    Software = 1,
}
impl From<IWDG_SW> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_SW` reader - User option bytes
pub type IWDG_SW_R = crate::BitReader<IWDG_SW>;
impl IWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_SW {
        match self.bits {
            false => IWDG_SW::Hardware,
            true => IWDG_SW::Software,
        }
    }
    ///Hardware independant watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IWDG_SW::Hardware
    }
    ///Software independant watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IWDG_SW::Software
    }
}
///Field `IWDG_SW` writer - User option bytes
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_SW>;
impl<'a, REG> IWDG_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware independant watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Hardware)
    }
    ///Software independant watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_SW::Software)
    }
}
/**User option bytes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP {
    ///0: Reset generated when entering Stop mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STOP> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STOP` reader - User option bytes
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP>;
impl N_RST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STOP {
        match self.bits {
            false => N_RST_STOP::Reset,
            true => N_RST_STOP::NoReset,
        }
    }
    ///Reset generated when entering Stop mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP::NoReset
    }
}
///Field `nRST_STOP` writer - User option bytes
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STOP>;
impl<'a, REG> N_RST_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset generated when entering Stop mode
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::Reset)
    }
    ///No reset generated
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STOP::NoReset)
    }
}
/**User option bytes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY {
    ///0: Reset generated when entering Standby mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STDBY> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STDBY` reader - User option bytes
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY>;
impl N_RST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STDBY {
        match self.bits {
            false => N_RST_STDBY::Reset,
            true => N_RST_STDBY::NoReset,
        }
    }
    ///Reset generated when entering Standby mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY::NoReset
    }
}
///Field `nRST_STDBY` writer - User option bytes
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, N_RST_STDBY>;
impl<'a, REG> N_RST_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset generated when entering Standby mode
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::Reset)
    }
    ///No reset generated
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(N_RST_STDBY::NoReset)
    }
}
/**Read protect

Value on reset: 170*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP {
    ///170: Read protection not active
    Level0 = 170,
    ///204: Chip read protection active
    Level2 = 204,
    ///0: Read protection of memories active
    Level1 = 0,
}
impl From<RDP> for u8 {
    #[inline(always)]
    fn from(variant: RDP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDP {
    type Ux = u8;
}
impl crate::IsEnum for RDP {}
///Field `RDP` reader - Read protect
pub type RDP_R = crate::FieldReader<RDP>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDP {
        match self.bits {
            170 => RDP::Level0,
            204 => RDP::Level2,
            _ => RDP::Level1,
        }
    }
    ///Read protection not active
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDP::Level0
    }
    ///Chip read protection active
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDP::Level2
    }
    ///Read protection of memories active
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        matches!(self.variant(), RDP::Level1)
    }
}
///Field `RDP` writer - Read protect
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RDP, crate::Safe>;
impl<'a, REG> RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Read protection not active
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level0)
    }
    ///Chip read protection active
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level2)
    }
    ///Read protection of memories active
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Level1)
    }
}
/**Not write protect

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_WRP0 {
    ///0: Write protection active on sector %s
    Active = 0,
    ///1: Write protection inactive on sector %s
    Inactive = 1,
}
impl From<N_WRP0> for bool {
    #[inline(always)]
    fn from(variant: N_WRP0) -> Self {
        variant as u8 != 0
    }
}
///Field `nWRP(0-11)` reader - Not write protect
pub type N_WRP_R = crate::BitReader<N_WRP0>;
impl N_WRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_WRP0 {
        match self.bits {
            false => N_WRP0::Active,
            true => N_WRP0::Inactive,
        }
    }
    ///Write protection active on sector %s
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == N_WRP0::Active
    }
    ///Write protection inactive on sector %s
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == N_WRP0::Inactive
    }
}
///Field `nWRP(0-11)` writer - Not write protect
pub type N_WRP_W<'a, REG> = crate::BitWriter<'a, REG, N_WRP0>;
impl<'a, REG> N_WRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection active on sector %s
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Active)
    }
    ///Write protection inactive on sector %s
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(N_WRP0::Inactive)
    }
}
/**Dual Boot mode (valid only when nDBANK=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_DBOOT {
    ///0: Boot always from system memory if boot address is in flash (Dual bank Boot mode), or RAM if Boot address option in RAM
    Enabled = 0,
    ///1: Boot according to boot address option
    Disabled = 1,
}
impl From<N_DBOOT> for bool {
    #[inline(always)]
    fn from(variant: N_DBOOT) -> Self {
        variant as u8 != 0
    }
}
///Field `nDBOOT` reader - Dual Boot mode (valid only when nDBANK=0)
pub type N_DBOOT_R = crate::BitReader<N_DBOOT>;
impl N_DBOOT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_DBOOT {
        match self.bits {
            false => N_DBOOT::Enabled,
            true => N_DBOOT::Disabled,
        }
    }
    ///Boot always from system memory if boot address is in flash (Dual bank Boot mode), or RAM if Boot address option in RAM
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_DBOOT::Enabled
    }
    ///Boot according to boot address option
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_DBOOT::Disabled
    }
}
///Field `nDBOOT` writer - Dual Boot mode (valid only when nDBANK=0)
pub type N_DBOOT_W<'a, REG> = crate::BitWriter<'a, REG, N_DBOOT>;
impl<'a, REG> N_DBOOT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Boot always from system memory if boot address is in flash (Dual bank Boot mode), or RAM if Boot address option in RAM
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_DBOOT::Enabled)
    }
    ///Boot according to boot address option
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_DBOOT::Disabled)
    }
}
/**Not dual bank mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_DBANK {
    ///0: The Flash user area is seen as a single bank with 256 bits read access
    DualBank = 0,
    ///1: The Flash user area is seen as a dual bank with 128 bits read access (dual bank mode feature active)
    SingleBank = 1,
}
impl From<N_DBANK> for bool {
    #[inline(always)]
    fn from(variant: N_DBANK) -> Self {
        variant as u8 != 0
    }
}
///Field `nDBANK` reader - Not dual bank mode
pub type N_DBANK_R = crate::BitReader<N_DBANK>;
impl N_DBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_DBANK {
        match self.bits {
            false => N_DBANK::DualBank,
            true => N_DBANK::SingleBank,
        }
    }
    ///The Flash user area is seen as a single bank with 256 bits read access
    #[inline(always)]
    pub fn is_dual_bank(&self) -> bool {
        *self == N_DBANK::DualBank
    }
    ///The Flash user area is seen as a dual bank with 128 bits read access (dual bank mode feature active)
    #[inline(always)]
    pub fn is_single_bank(&self) -> bool {
        *self == N_DBANK::SingleBank
    }
}
///Field `nDBANK` writer - Not dual bank mode
pub type N_DBANK_W<'a, REG> = crate::BitWriter<'a, REG, N_DBANK>;
impl<'a, REG> N_DBANK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The Flash user area is seen as a single bank with 256 bits read access
    #[inline(always)]
    pub fn dual_bank(self) -> &'a mut crate::W<REG> {
        self.variant(N_DBANK::DualBank)
    }
    ///The Flash user area is seen as a dual bank with 128 bits read access (dual bank mode feature active)
    #[inline(always)]
    pub fn single_bank(self) -> &'a mut crate::W<REG> {
        self.variant(N_DBANK::SingleBank)
    }
}
/**Independent watchdog counter freeze in standby mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STDBY {
    ///0: IWDG counter frozen in Standby mode
    Inactive = 0,
    ///1: IWDG counter active in Standby mode
    Active = 1,
}
impl From<IWDG_STDBY> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in standby mode
pub type IWDG_STDBY_R = crate::BitReader<IWDG_STDBY>;
impl IWDG_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STDBY {
        match self.bits {
            false => IWDG_STDBY::Inactive,
            true => IWDG_STDBY::Active,
        }
    }
    ///IWDG counter frozen in Standby mode
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IWDG_STDBY::Inactive
    }
    ///IWDG counter active in Standby mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IWDG_STDBY::Active
    }
}
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STDBY>;
impl<'a, REG> IWDG_STDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IWDG counter frozen in Standby mode
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Inactive)
    }
    ///IWDG counter active in Standby mode
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STDBY::Active)
    }
}
/**Independent watchdog counter freeze in Stop mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDG_STOP {
    ///0: IWDG counter frozen in Stop mode
    Inactive = 0,
    ///1: IWDG counter active in Stop mode
    Active = 1,
}
impl From<IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<IWDG_STOP>;
impl IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDG_STOP {
        match self.bits {
            false => IWDG_STOP::Inactive,
            true => IWDG_STOP::Active,
        }
    }
    ///IWDG counter frozen in Stop mode
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IWDG_STOP::Inactive
    }
    ///IWDG counter active in Stop mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IWDG_STOP::Active
    }
}
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, IWDG_STOP>;
impl<'a, REG> IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IWDG counter frozen in Stop mode
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Inactive)
    }
    ///IWDG counter active in Stop mode
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(IWDG_STOP::Active)
    }
}
impl R {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - User option bytes
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - User option bytes
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&self, n: u8) -> N_WRP_R {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Not write protect
    #[inline(always)]
    pub fn n_wrp_iter(&self) -> impl Iterator<Item = N_WRP_R> + '_ {
        (0..12).map(move |n| N_WRP_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Dual Boot mode (valid only when nDBANK=0)
    #[inline(always)]
    pub fn n_dboot(&self) -> N_DBOOT_R {
        N_DBOOT_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Not dual bank mode
    #[inline(always)]
    pub fn n_dbank(&self) -> N_DBANK_R {
        N_DBANK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Independent watchdog counter freeze in standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("optstrt", &self.optstrt())
            .field("bor_lev", &self.bor_lev())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("rdp", &self.rdp())
            .field("n_dboot", &self.n_dboot())
            .field("n_dbank", &self.n_dbank())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("n_wrp0", &self.n_wrp0())
            .field("n_wrp1", &self.n_wrp1())
            .field("n_wrp2", &self.n_wrp2())
            .field("n_wrp3", &self.n_wrp3())
            .field("n_wrp4", &self.n_wrp4())
            .field("n_wrp5", &self.n_wrp5())
            .field("n_wrp6", &self.n_wrp6())
            .field("n_wrp7", &self.n_wrp7())
            .field("n_wrp8", &self.n_wrp8())
            .field("n_wrp9", &self.n_wrp9())
            .field("n_wrp10", &self.n_wrp10())
            .field("n_wrp11", &self.n_wrp11())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, OPTCRrs> {
        OPTSTRT_W::new(self, 1)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTCRrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - User option bytes
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTCRrs> {
        WWDG_SW_W::new(self, 4)
    }
    ///Bit 5 - User option bytes
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTCRrs> {
        IWDG_SW_W::new(self, 5)
    }
    ///Bit 6 - User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTCRrs> {
        N_RST_STOP_W::new(self, 6)
    }
    ///Bit 7 - User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<'_, OPTCRrs> {
        N_RST_STDBY_W::new(self, 7)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTCRrs> {
        RDP_W::new(self, 8)
    }
    ///Not write protect
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `nWRP0` field.</div>
    #[inline(always)]
    pub fn n_wrp(&mut self, n: u8) -> N_WRP_W<'_, OPTCRrs> {
        #[allow(clippy::no_effect)]
        [(); 12][n as usize];
        N_WRP_W::new(self, n + 16)
    }
    ///Bit 16 - Not write protect
    #[inline(always)]
    pub fn n_wrp0(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 16)
    }
    ///Bit 17 - Not write protect
    #[inline(always)]
    pub fn n_wrp1(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 17)
    }
    ///Bit 18 - Not write protect
    #[inline(always)]
    pub fn n_wrp2(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 18)
    }
    ///Bit 19 - Not write protect
    #[inline(always)]
    pub fn n_wrp3(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 19)
    }
    ///Bit 20 - Not write protect
    #[inline(always)]
    pub fn n_wrp4(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 20)
    }
    ///Bit 21 - Not write protect
    #[inline(always)]
    pub fn n_wrp5(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 21)
    }
    ///Bit 22 - Not write protect
    #[inline(always)]
    pub fn n_wrp6(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 22)
    }
    ///Bit 23 - Not write protect
    #[inline(always)]
    pub fn n_wrp7(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 23)
    }
    ///Bit 24 - Not write protect
    #[inline(always)]
    pub fn n_wrp8(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 24)
    }
    ///Bit 25 - Not write protect
    #[inline(always)]
    pub fn n_wrp9(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 25)
    }
    ///Bit 26 - Not write protect
    #[inline(always)]
    pub fn n_wrp10(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 26)
    }
    ///Bit 27 - Not write protect
    #[inline(always)]
    pub fn n_wrp11(&mut self) -> N_WRP_W<'_, OPTCRrs> {
        N_WRP_W::new(self, 27)
    }
    ///Bit 28 - Dual Boot mode (valid only when nDBANK=0)
    #[inline(always)]
    pub fn n_dboot(&mut self) -> N_DBOOT_W<'_, OPTCRrs> {
        N_DBOOT_W::new(self, 28)
    }
    ///Bit 29 - Not dual bank mode
    #[inline(always)]
    pub fn n_dbank(&mut self) -> N_DBANK_W<'_, OPTCRrs> {
        N_DBANK_W::new(self, 29)
    }
    ///Bit 30 - Independent watchdog counter freeze in standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<'_, OPTCRrs> {
        IWDG_STDBY_W::new(self, 30)
    }
    ///Bit 31 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTCRrs> {
        IWDG_STOP_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#FLASH:OPTCR)*/
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`optcr::R`](R) reader structure
impl crate::Readable for OPTCRrs {}
///`write(|w| ..)` method takes [`optcr::W`](W) writer structure
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR to value 0x0fff_aaed
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x0fff_aaed;
}
