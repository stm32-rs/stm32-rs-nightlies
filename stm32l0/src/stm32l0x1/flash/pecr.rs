#[doc = "Register `PECR` reader"]
pub type R = crate::R<PECRrs>;
#[doc = "Register `PECR` writer"]
pub type W = crate::W<PECRrs>;
#[doc = "FLASH_PECR and data EEPROM lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PELOCK {
    #[doc = "0: The FLASH_PECR register is unlocked"]
    Unlocked = 0,
    #[doc = "1: The FLASH_PECR register is locked and no write/erase operation can start"]
    Locked = 1,
}
impl From<PELOCK> for bool {
    #[inline(always)]
    fn from(variant: PELOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PELOCK` reader - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_R = crate::BitReader<PELOCK>;
impl PELOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PELOCK {
        match self.bits {
            false => PELOCK::Unlocked,
            true => PELOCK::Locked,
        }
    }
    #[doc = "The FLASH_PECR register is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PELOCK::Unlocked
    }
    #[doc = "The FLASH_PECR register is locked and no write/erase operation can start"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PELOCK::Locked
    }
}
#[doc = "Field `PELOCK` writer - FLASH_PECR and data EEPROM lock"]
pub type PELOCK_W<'a, REG> = crate::BitWriter<'a, REG, PELOCK>;
impl<'a, REG> PELOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The FLASH_PECR register is unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(PELOCK::Unlocked)
    }
    #[doc = "The FLASH_PECR register is locked and no write/erase operation can start"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(PELOCK::Locked)
    }
}
#[doc = "Program memory lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGLOCK {
    #[doc = "0: The write and erase operations in the Flash program memory are disabled"]
    Unlocked = 0,
    #[doc = "1: The write and erase operations in the Flash program memory are enabled"]
    Locked = 1,
}
impl From<PRGLOCK> for bool {
    #[inline(always)]
    fn from(variant: PRGLOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGLOCK` reader - Program memory lock"]
pub type PRGLOCK_R = crate::BitReader<PRGLOCK>;
impl PRGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRGLOCK {
        match self.bits {
            false => PRGLOCK::Unlocked,
            true => PRGLOCK::Locked,
        }
    }
    #[doc = "The write and erase operations in the Flash program memory are disabled"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PRGLOCK::Unlocked
    }
    #[doc = "The write and erase operations in the Flash program memory are enabled"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PRGLOCK::Locked
    }
}
#[doc = "Field `PRGLOCK` writer - Program memory lock"]
pub type PRGLOCK_W<'a, REG> = crate::BitWriter<'a, REG, PRGLOCK>;
impl<'a, REG> PRGLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write and erase operations in the Flash program memory are disabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(PRGLOCK::Unlocked)
    }
    #[doc = "The write and erase operations in the Flash program memory are enabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(PRGLOCK::Locked)
    }
}
#[doc = "Option bytes block lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCK {
    #[doc = "0: The write and erase operations in the Option bytes area are disabled"]
    Unlocked = 0,
    #[doc = "1: The write and erase operations in the Option bytes area are enabled"]
    Locked = 1,
}
impl From<OPTLOCK> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTLOCK` reader - Option bytes block lock"]
pub type OPTLOCK_R = crate::BitReader<OPTLOCK>;
impl OPTLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTLOCK {
        match self.bits {
            false => OPTLOCK::Unlocked,
            true => OPTLOCK::Locked,
        }
    }
    #[doc = "The write and erase operations in the Option bytes area are disabled"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCK::Unlocked
    }
    #[doc = "The write and erase operations in the Option bytes area are enabled"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCK::Locked
    }
}
#[doc = "Field `OPTLOCK` writer - Option bytes block lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCK>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write and erase operations in the Option bytes area are disabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCK::Unlocked)
    }
    #[doc = "The write and erase operations in the Option bytes area are enabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCK::Locked)
    }
}
#[doc = "Program memory selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROG {
    #[doc = "0: The Flash program memory is not selected"]
    NotSelected = 0,
    #[doc = "1: The Flash program memory is selected"]
    Selected = 1,
}
impl From<PROG> for bool {
    #[inline(always)]
    fn from(variant: PROG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG` reader - Program memory selection"]
pub type PROG_R = crate::BitReader<PROG>;
impl PROG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROG {
        match self.bits {
            false => PROG::NotSelected,
            true => PROG::Selected,
        }
    }
    #[doc = "The Flash program memory is not selected"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PROG::NotSelected
    }
    #[doc = "The Flash program memory is selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PROG::Selected
    }
}
#[doc = "Field `PROG` writer - Program memory selection"]
pub type PROG_W<'a, REG> = crate::BitWriter<'a, REG, PROG>;
impl<'a, REG> PROG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Flash program memory is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PROG::NotSelected)
    }
    #[doc = "The Flash program memory is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PROG::Selected)
    }
}
#[doc = "Data EEPROM selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA {
    #[doc = "0: Data EEPROM not selected"]
    NotSelected = 0,
    #[doc = "1: Data memory selected"]
    Selected = 1,
}
impl From<DATA> for bool {
    #[inline(always)]
    fn from(variant: DATA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA` reader - Data EEPROM selection"]
pub type DATA_R = crate::BitReader<DATA>;
impl DATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA {
        match self.bits {
            false => DATA::NotSelected,
            true => DATA::Selected,
        }
    }
    #[doc = "Data EEPROM not selected"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == DATA::NotSelected
    }
    #[doc = "Data memory selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == DATA::Selected
    }
}
#[doc = "Field `DATA` writer - Data EEPROM selection"]
pub type DATA_W<'a, REG> = crate::BitWriter<'a, REG, DATA>;
impl<'a, REG> DATA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data EEPROM not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(DATA::NotSelected)
    }
    #[doc = "Data memory selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(DATA::Selected)
    }
}
#[doc = "Fixed time data write for Byte, Half Word and Word programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIX {
    #[doc = "0: An erase phase is automatically performed"]
    AutoErase = 0,
    #[doc = "1: The program operation is always performed with a preliminary erase"]
    PrelimErase = 1,
}
impl From<FIX> for bool {
    #[inline(always)]
    fn from(variant: FIX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIX` reader - Fixed time data write for Byte, Half Word and Word programming"]
pub type FIX_R = crate::BitReader<FIX>;
impl FIX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIX {
        match self.bits {
            false => FIX::AutoErase,
            true => FIX::PrelimErase,
        }
    }
    #[doc = "An erase phase is automatically performed"]
    #[inline(always)]
    pub fn is_auto_erase(&self) -> bool {
        *self == FIX::AutoErase
    }
    #[doc = "The program operation is always performed with a preliminary erase"]
    #[inline(always)]
    pub fn is_prelim_erase(&self) -> bool {
        *self == FIX::PrelimErase
    }
}
#[doc = "Field `FIX` writer - Fixed time data write for Byte, Half Word and Word programming"]
pub type FIX_W<'a, REG> = crate::BitWriter<'a, REG, FIX>;
impl<'a, REG> FIX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An erase phase is automatically performed"]
    #[inline(always)]
    pub fn auto_erase(self) -> &'a mut crate::W<REG> {
        self.variant(FIX::AutoErase)
    }
    #[doc = "The program operation is always performed with a preliminary erase"]
    #[inline(always)]
    pub fn prelim_erase(self) -> &'a mut crate::W<REG> {
        self.variant(FIX::PrelimErase)
    }
}
#[doc = "Page or Double Word erase mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASE {
    #[doc = "0: No erase operation requested"]
    NoErase = 0,
    #[doc = "1: Erase operation requested"]
    Erase = 1,
}
impl From<ERASE> for bool {
    #[inline(always)]
    fn from(variant: ERASE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` reader - Page or Double Word erase mode"]
pub type ERASE_R = crate::BitReader<ERASE>;
impl ERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERASE {
        match self.bits {
            false => ERASE::NoErase,
            true => ERASE::Erase,
        }
    }
    #[doc = "No erase operation requested"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == ERASE::NoErase
    }
    #[doc = "Erase operation requested"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASE::Erase
    }
}
#[doc = "Field `ERASE` writer - Page or Double Word erase mode"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG, ERASE>;
impl<'a, REG> ERASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No erase operation requested"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE::NoErase)
    }
    #[doc = "Erase operation requested"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE::Erase)
    }
}
#[doc = "Half Page/Double Word programming mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPRG {
    #[doc = "0: Half Page programming disabled"]
    Disabled = 0,
    #[doc = "1: Half Page programming enabled"]
    Enabled = 1,
}
impl From<FPRG> for bool {
    #[inline(always)]
    fn from(variant: FPRG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRG` reader - Half Page/Double Word programming mode"]
pub type FPRG_R = crate::BitReader<FPRG>;
impl FPRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPRG {
        match self.bits {
            false => FPRG::Disabled,
            true => FPRG::Enabled,
        }
    }
    #[doc = "Half Page programming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPRG::Disabled
    }
    #[doc = "Half Page programming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPRG::Enabled
    }
}
#[doc = "Field `FPRG` writer - Half Page/Double Word programming mode"]
pub type FPRG_W<'a, REG> = crate::BitWriter<'a, REG, FPRG>;
impl<'a, REG> FPRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half Page programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPRG::Disabled)
    }
    #[doc = "Half Page programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPRG::Enabled)
    }
}
#[doc = "Parallel bank mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARALLELBANK {
    #[doc = "0: Parallel bank mode disabled"]
    Disabled = 0,
    #[doc = "1: Parallel bank mode enabled"]
    Enabled = 1,
}
impl From<PARALLELBANK> for bool {
    #[inline(always)]
    fn from(variant: PARALLELBANK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARALLELBANK` reader - Parallel bank mode"]
pub type PARALLELBANK_R = crate::BitReader<PARALLELBANK>;
impl PARALLELBANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PARALLELBANK {
        match self.bits {
            false => PARALLELBANK::Disabled,
            true => PARALLELBANK::Enabled,
        }
    }
    #[doc = "Parallel bank mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PARALLELBANK::Disabled
    }
    #[doc = "Parallel bank mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PARALLELBANK::Enabled
    }
}
#[doc = "Field `PARALLELBANK` writer - Parallel bank mode"]
pub type PARALLELBANK_W<'a, REG> = crate::BitWriter<'a, REG, PARALLELBANK>;
impl<'a, REG> PARALLELBANK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parallel bank mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PARALLELBANK::Disabled)
    }
    #[doc = "Parallel bank mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PARALLELBANK::Enabled)
    }
}
#[doc = "End of programming interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE {
    #[doc = "0: End of program interrupt disable"]
    Disabled = 0,
    #[doc = "1: End of program interrupt enable"]
    Enabled = 1,
}
impl From<EOPIE> for bool {
    #[inline(always)]
    fn from(variant: EOPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - End of programming interrupt enable"]
pub type EOPIE_R = crate::BitReader<EOPIE>;
impl EOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOPIE {
        match self.bits {
            false => EOPIE::Disabled,
            true => EOPIE::Enabled,
        }
    }
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE::Enabled
    }
}
#[doc = "Field `EOPIE` writer - End of programming interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Error interrupt disable"]
    Disabled = 0,
    #[doc = "1: Error interrupt enable"]
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "Launch the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHR {
    #[doc = "0: Option byte loaded"]
    Complete = 0,
    #[doc = "1: Option byte loading to be done"]
    NotComplete = 1,
}
impl From<OBL_LAUNCHR> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` reader - Launch the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCHR>;
impl OBL_LAUNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBL_LAUNCHR {
        match self.bits {
            false => OBL_LAUNCHR::Complete,
            true => OBL_LAUNCHR::NotComplete,
        }
    }
    #[doc = "Option byte loaded"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR::Complete
    }
    #[doc = "Option byte loading to be done"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == OBL_LAUNCHR::NotComplete
    }
}
#[doc = "Launch the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW {
    #[doc = "1: Reload option byte"]
    Reload = 1,
}
impl From<OBL_LAUNCHW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` writer - Launch the option byte loading"]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG, OBL_LAUNCHW>;
impl<'a, REG> OBL_LAUNCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reload option byte"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCHW::Reload)
    }
}
impl R {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn fix(&self) -> FIX_R {
        FIX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    #[must_use]
    pub fn pelock(&mut self) -> PELOCK_W<PECRrs> {
        PELOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    #[must_use]
    pub fn prglock(&mut self) -> PRGLOCK_W<PECRrs> {
        PRGLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<PECRrs> {
        OPTLOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<PECRrs> {
        PROG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<PECRrs> {
        DATA_W::new(self, 4)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    #[must_use]
    pub fn fix(&mut self) -> FIX_W<PECRrs> {
        FIX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<PECRrs> {
        ERASE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    #[must_use]
    pub fn fprg(&mut self) -> FPRG_W<PECRrs> {
        FPRG_W::new(self, 10)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    #[must_use]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W<PECRrs> {
        PARALLELBANK_W::new(self, 15)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<PECRrs> {
        EOPIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<PECRrs> {
        ERRIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<PECRrs> {
        OBL_LAUNCH_W::new(self, 18)
    }
}
#[doc = "Program/erase control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PECRrs;
impl crate::RegisterSpec for PECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PECRrs {}
#[doc = "`write(|w| ..)` method takes [`pecr::W`](W) writer structure"]
impl crate::Writable for PECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PECR to value 0x07"]
impl crate::Resettable for PECRrs {
    const RESET_VALUE: u32 = 0x07;
}
