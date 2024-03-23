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
#[doc = "Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    #[doc = "0: No mass erase"]
    NoErase = 0,
    #[doc = "1: Trigger mass erase"]
    MassErase = 1,
}
impl From<MER> for bool {
    #[inline(always)]
    fn from(variant: MER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Mass erase"]
pub type MER_R = crate::BitReader<MER>;
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MER {
        match self.bits {
            false => MER::NoErase,
            true => MER::MassErase,
        }
    }
    #[doc = "No mass erase"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == MER::NoErase
    }
    #[doc = "Trigger mass erase"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER::MassErase
    }
}
#[doc = "Field `MER` writer - Mass erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG, MER>;
impl<'a, REG> MER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No mass erase"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::NoErase)
    }
    #[doc = "Trigger mass erase"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
#[doc = "Field `PNB` reader - Page number"]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Page number"]
pub type PNB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    #[doc = "0: Options modification completed or idle"]
    Done = 0,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRTR> {
        match self.bits {
            false => Some(STRTR::Done),
            _ => None,
        }
    }
    #[doc = "Options modification completed or idle"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STRTR::Done
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    #[doc = "1: Trigger options programming operation"]
    Start = 1,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger options programming operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Start)
    }
}
#[doc = "Options modification start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR {
    #[doc = "0: Options modification completed or idle"]
    Done = 0,
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
    pub const fn variant(&self) -> Option<OPTSTRTR> {
        match self.bits {
            false => Some(OPTSTRTR::Done),
            _ => None,
        }
    }
    #[doc = "Options modification completed or idle"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == OPTSTRTR::Done
    }
}
#[doc = "Options modification start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW {
    #[doc = "1: Trigger options programming operation"]
    Start = 1,
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
    #[doc = "Trigger options programming operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(OPTSTRTW::Start)
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
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
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
    #[doc = "0: OPERR Error interrupt disable"]
    Disabled = 0,
    #[doc = "1: OPERR Error interrupt enable"]
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
    #[doc = "OPERR Error interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "OPERR Error interrupt enable"]
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
    #[doc = "OPERR Error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "OPERR Error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "PCROP read error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE {
    #[doc = "0: PCROP read error interrupt disable"]
    Disabled = 0,
    #[doc = "1: PCROP read error interrupt enable"]
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
    #[doc = "PCROP read error interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    #[doc = "PCROP read error interrupt enable"]
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
    #[doc = "PCROP read error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Enabled)
    }
}
#[doc = "Force the option byte loading\n\nValue on reset: 0"]
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
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
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
#[doc = "Force the option byte loading\n\nValue on reset: 0"]
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
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
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
#[doc = "Options Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    #[doc = "0: FLASH_CR options are unlocked"]
    Unlocked = 0,
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
    pub const fn variant(&self) -> Option<OPTLOCKR> {
        match self.bits {
            false => Some(OPTLOCKR::Unlocked),
            _ => None,
        }
    }
    #[doc = "FLASH_CR options are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
}
#[doc = "Options Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    #[doc = "1: FLASH_CR options are locked"]
    Locked = 1,
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
    #[doc = "FLASH_CR options are locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Locked)
    }
}
#[doc = "FLASH_CR Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    #[doc = "0: FLASH_CR is unlocked"]
    Unlocked = 0,
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
    pub const fn variant(&self) -> Option<LOCKR> {
        match self.bits {
            false => Some(LOCKR::Unlocked),
            _ => None,
        }
    }
    #[doc = "FLASH_CR is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::Unlocked
    }
}
#[doc = "FLASH_CR Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    #[doc = "1: FLASH_CR is locked"]
    Locked = 1,
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
    #[doc = "FLASH_CR is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Locked)
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
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Page number"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bits 3:9 - Page number"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<CRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
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
