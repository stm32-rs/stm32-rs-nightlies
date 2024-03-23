#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    #[doc = "0: Flash programming disabled"]
    Disabled = 0,
    #[doc = "1: Flash programming enabled"]
    Enabled = 1,
}
impl From<PG> for bool {
    #[inline(always)]
    fn from(variant: PG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader<PG>;
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PG {
        match self.bits {
            false => PG::Disabled,
            true => PG::Enabled,
        }
    }
    #[doc = "Flash programming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG::Disabled
    }
    #[doc = "Flash programming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG::Enabled
    }
}
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Disabled)
    }
    #[doc = "Flash programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Enabled)
    }
}
#[doc = "Page erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    #[doc = "0: Page erase disabled"]
    Disabled = 0,
    #[doc = "1: Page erase enabled"]
    Enabled = 1,
}
impl From<PER> for bool {
    #[inline(always)]
    fn from(variant: PER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader<PER>;
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PER {
        match self.bits {
            false => PER::Disabled,
            true => PER::Enabled,
        }
    }
    #[doc = "Page erase disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER::Disabled
    }
    #[doc = "Page erase enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER::Enabled
    }
}
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Page erase disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Disabled)
    }
    #[doc = "Page erase enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Enabled)
    }
}
#[doc = "Bank 1 Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER1W {
    #[doc = "1: This bit triggers the bank 1 mass erase (all bank 1 user pages) when set"]
    MassErase = 1,
}
impl From<MER1W> for bool {
    #[inline(always)]
    fn from(variant: MER1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER1` reader - Bank 1 Mass erase"]
pub type MER1_R = crate::BitReader<MER1W>;
impl MER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER1W> {
        match self.bits {
            true => Some(MER1W::MassErase),
            _ => None,
        }
    }
    #[doc = "This bit triggers the bank 1 mass erase (all bank 1 user pages) when set"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1W::MassErase
    }
}
#[doc = "Field `MER1` writer - Bank 1 Mass erase"]
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG, MER1W>;
impl<'a, REG> MER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit triggers the bank 1 mass erase (all bank 1 user pages) when set"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER1W::MassErase)
    }
}
#[doc = "Field `PNB` reader - Page number"]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Page number"]
pub type PNB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Bank erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKER {
    #[doc = "0: Bank 1 is selected for page erase"]
    Bank1 = 0,
    #[doc = "1: Bank 2 is selected for page erase"]
    Bank2 = 1,
}
impl From<BKER> for bool {
    #[inline(always)]
    fn from(variant: BKER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKER` reader - Bank erase"]
pub type BKER_R = crate::BitReader<BKER>;
impl BKER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKER {
        match self.bits {
            false => BKER::Bank1,
            true => BKER::Bank2,
        }
    }
    #[doc = "Bank 1 is selected for page erase"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BKER::Bank1
    }
    #[doc = "Bank 2 is selected for page erase"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BKER::Bank2
    }
}
#[doc = "Field `BKER` writer - Bank erase"]
pub type BKER_W<'a, REG> = crate::BitWriter<'a, REG, BKER>;
impl<'a, REG> BKER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank 1 is selected for page erase"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(BKER::Bank1)
    }
    #[doc = "Bank 2 is selected for page erase"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(BKER::Bank2)
    }
}
#[doc = "Bank 2 Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER2W {
    #[doc = "1: This bit triggers the bank 2 mass erase (all bank 2 user pages) when set"]
    MassErase = 1,
}
impl From<MER2W> for bool {
    #[inline(always)]
    fn from(variant: MER2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER2` reader - Bank 2 Mass erase"]
pub type MER2_R = crate::BitReader<MER2W>;
impl MER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER2W> {
        match self.bits {
            true => Some(MER2W::MassErase),
            _ => None,
        }
    }
    #[doc = "This bit triggers the bank 2 mass erase (all bank 2 user pages) when set"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2W::MassErase
    }
}
#[doc = "Field `MER2` writer - Bank 2 Mass erase"]
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG, MER2W>;
impl<'a, REG> MER2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit triggers the bank 2 mass erase (all bank 2 user pages) when set"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER2W::MassErase)
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR {
    #[doc = "0: Cleared when BSY bit is cleared in SR"]
    Complete = 0,
    #[doc = "1: Erase operation requested"]
    Requested = 1,
}
impl From<STARTR> for bool {
    #[inline(always)]
    fn from(variant: STARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<STARTR>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTR {
        match self.bits {
            false => STARTR::Complete,
            true => STARTR::Requested,
        }
    }
    #[doc = "Cleared when BSY bit is cleared in SR"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == STARTR::Complete
    }
    #[doc = "Erase operation requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == STARTR::Requested
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    #[doc = "1: Trigger an erase operation"]
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
#[doc = "Options modification start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR {
    #[doc = "0: Cleared when BSY bit is cleared in SR"]
    Complete = 0,
    #[doc = "1: Options modification requested"]
    Requested = 1,
}
impl From<OPTSTRTR> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OPTSTRT_R = crate::BitReader<OPTSTRTR>;
impl OPTSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTSTRTR {
        match self.bits {
            false => OPTSTRTR::Complete,
            true => OPTSTRTR::Requested,
        }
    }
    #[doc = "Cleared when BSY bit is cleared in SR"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OPTSTRTR::Complete
    }
    #[doc = "Options modification requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OPTSTRTR::Requested
    }
}
#[doc = "Options modification start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW {
    #[doc = "1: This bit triggers an options operation when set"]
    Set = 1,
}
impl From<OPTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, OPTSTRTW>;
impl<'a, REG> OPTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit triggers an options operation when set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OPTSTRTW::Set)
    }
}
#[doc = "Fast programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG {
    #[doc = "0: Fast programming disabled"]
    Disabled = 0,
    #[doc = "1: Fast programming enabled"]
    Enabled = 1,
}
impl From<FSTPG> for bool {
    #[inline(always)]
    fn from(variant: FSTPG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FSTPG_R = crate::BitReader<FSTPG>;
impl FSTPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSTPG {
        match self.bits {
            false => FSTPG::Disabled,
            true => FSTPG::Enabled,
        }
    }
    #[doc = "Fast programming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG::Disabled
    }
    #[doc = "Fast programming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG::Enabled
    }
}
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG, FSTPG>;
impl<'a, REG> FSTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Disabled)
    }
    #[doc = "Fast programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Enabled)
    }
}
#[doc = "End of operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE {
    #[doc = "0: End of operation interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of operation interrupt enabled"]
    Enabled = 1,
}
impl From<EOPIE> for bool {
    #[inline(always)]
    fn from(variant: EOPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
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
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE::Enabled
    }
}
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Error interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt generation enabled"]
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
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "Error interrupt generation enabled"]
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
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "PCROP read error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE {
    #[doc = "0: PCROP read error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PCROP read error interrupt enabled"]
    Enabled = 1,
}
impl From<RDERRIE> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERRIE` reader - PCROP read error interrupt enable"]
pub type RDERRIE_R = crate::BitReader<RDERRIE>;
impl RDERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDERRIE {
        match self.bits {
            false => RDERRIE::Disabled,
            true => RDERRIE::Enabled,
        }
    }
    #[doc = "PCROP read error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    #[doc = "PCROP read error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE::Enabled
    }
}
#[doc = "Field `RDERRIE` writer - PCROP read error interrupt enable"]
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RDERRIE>;
impl<'a, REG> RDERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCROP read error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    #[doc = "PCROP read error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Enabled)
    }
}
#[doc = "Force the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHR {
    #[doc = "0: Option byte loading complete"]
    Complete = 0,
    #[doc = "1: Option byte loading requested"]
    Requested = 1,
}
impl From<OBL_LAUNCHR> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCHR>;
impl OBL_LAUNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBL_LAUNCHR {
        match self.bits {
            false => OBL_LAUNCHR::Complete,
            true => OBL_LAUNCHR::Requested,
        }
    }
    #[doc = "Option byte loading complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR::Complete
    }
    #[doc = "Option byte loading requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OBL_LAUNCHR::Requested
    }
}
#[doc = "Force the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW {
    #[doc = "1: Force option byte reloading"]
    Set = 1,
}
impl From<OBL_LAUNCHW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG, OBL_LAUNCHW>;
impl<'a, REG> OBL_LAUNCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force option byte reloading"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCHW::Set)
    }
}
#[doc = "Options Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    #[doc = "0: Option page is unlocked"]
    Unlocked = 0,
    #[doc = "1: All bits concerning user option in FLASH_CR register and so option page are locked"]
    Locked = 1,
}
impl From<OPTLOCKR> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OPTLOCK_R = crate::BitReader<OPTLOCKR>;
impl OPTLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTLOCKR {
        match self.bits {
            false => OPTLOCKR::Unlocked,
            true => OPTLOCKR::Locked,
        }
    }
    #[doc = "Option page is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
    #[doc = "All bits concerning user option in FLASH_CR register and so option page are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCKR::Locked
    }
}
#[doc = "Options Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    #[doc = "1: This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked"]
    Set = 1,
}
impl From<OPTLOCKW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCKW>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Set)
    }
}
#[doc = "FLASH_CR Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    #[doc = "0: FLASH_CR register is unlocked"]
    Unlocked = 0,
    #[doc = "1: FLASH_CR register is locked"]
    Locked = 1,
}
impl From<LOCKR> for bool {
    #[inline(always)]
    fn from(variant: LOCKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - FLASH_CR Lock"]
pub type LOCK_R = crate::BitReader<LOCKR>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKR {
        match self.bits {
            false => LOCKR::Unlocked,
            true => LOCKR::Locked,
        }
    }
    #[doc = "FLASH_CR register is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::Unlocked
    }
    #[doc = "FLASH_CR register is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::Locked
    }
}
#[doc = "FLASH_CR Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    #[doc = "1: This bit is set only. When set, the FLASH_CR register is locked"]
    Set = 1,
}
impl From<LOCKW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` writer - FLASH_CR Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKW>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set only. When set, the FLASH_CR register is locked"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<CRrs> {
        MER1_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<CRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    #[must_use]
    pub fn bker(&mut self) -> BKER_W<CRrs> {
        BKER_W::new(self, 11)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<CRrs> {
        MER2_W::new(self, 15)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 16)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<CRrs> {
        OPTSTRT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<CRrs> {
        FSTPG_W::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<CRrs> {
        RDERRIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<CRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<CRrs> {
        OPTLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0xc000_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
