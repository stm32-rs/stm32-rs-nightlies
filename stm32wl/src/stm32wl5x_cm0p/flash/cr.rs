///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    ///0: Flash programming disabled
    Disabled = 0,
    ///1: Flash programming enabled
    Enabled = 1,
}
impl From<PG> for bool {
    #[inline(always)]
    fn from(variant: PG) -> Self {
        variant as u8 != 0
    }
}
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader<PG>;
impl PG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PG {
        match self.bits {
            false => PG::Disabled,
            true => PG::Enabled,
        }
    }
    ///Flash programming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG::Disabled
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG::Enabled
    }
}
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Disabled)
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Enabled)
    }
}
/**Page erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    ///0: Page erase disabled
    Disabled = 0,
    ///1: Page erase enabled
    Enabled = 1,
}
impl From<PER> for bool {
    #[inline(always)]
    fn from(variant: PER) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader<PER>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PER {
        match self.bits {
            false => PER::Disabled,
            true => PER::Enabled,
        }
    }
    ///Page erase disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER::Disabled
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER::Enabled
    }
}
///Field `PER` writer - Page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Page erase disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Disabled)
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Enabled)
    }
}
/**Mass erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    ///0: No mass erase
    NoErase = 0,
    ///1: Trigger mass erase
    MassErase = 1,
}
impl From<MER> for bool {
    #[inline(always)]
    fn from(variant: MER) -> Self {
        variant as u8 != 0
    }
}
///Field `MER` reader - Mass erase
pub type MER_R = crate::BitReader<MER>;
impl MER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MER {
        match self.bits {
            false => MER::NoErase,
            true => MER::MassErase,
        }
    }
    ///No mass erase
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == MER::NoErase
    }
    ///Trigger mass erase
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER::MassErase
    }
}
///Field `MER` writer - Mass erase
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG, MER>;
impl<'a, REG> MER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No mass erase
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::NoErase)
    }
    ///Trigger mass erase
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
///Field `PNB` reader - Page number
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page number
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    ///0: Options modification completed or idle
    Done = 0,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRTR> {
        match self.bits {
            false => Some(STRTR::Done),
            _ => None,
        }
    }
    ///Options modification completed or idle
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STRTR::Done
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    ///1: Trigger options programming operation
    Start = 1,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger options programming operation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Start)
    }
}
/**Options modification start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTR {
    ///0: Options modification completed or idle
    Done = 0,
}
impl From<OPTSTRTR> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` reader - Options modification start
pub type OPTSTRT_R = crate::BitReader<OPTSTRTR>;
impl OPTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTSTRTR> {
        match self.bits {
            false => Some(OPTSTRTR::Done),
            _ => None,
        }
    }
    ///Options modification completed or idle
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == OPTSTRTR::Done
    }
}
/**Options modification start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTSTRTW {
    ///1: Trigger options programming operation
    Start = 1,
}
impl From<OPTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` writer - Options modification start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, OPTSTRTW>;
impl<'a, REG> OPTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger options programming operation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(OPTSTRTW::Start)
    }
}
/**Fast programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG {
    ///0: Fast programming disabled
    Disabled = 0,
    ///1: Fast programming enabled
    Enabled = 1,
}
impl From<FSTPG> for bool {
    #[inline(always)]
    fn from(variant: FSTPG) -> Self {
        variant as u8 != 0
    }
}
///Field `FSTPG` reader - Fast programming
pub type FSTPG_R = crate::BitReader<FSTPG>;
impl FSTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSTPG {
        match self.bits {
            false => FSTPG::Disabled,
            true => FSTPG::Enabled,
        }
    }
    ///Fast programming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG::Disabled
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG::Enabled
    }
}
///Field `FSTPG` writer - Fast programming
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG, FSTPG>;
impl<'a, REG> FSTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Disabled)
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Enabled)
    }
}
/**End of operation interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE {
    ///0: End of program interrupt disable
    Disabled = 0,
    ///1: End of program interrupt enable
    Enabled = 1,
}
impl From<EOPIE> for bool {
    #[inline(always)]
    fn from(variant: EOPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOPIE` reader - End of operation interrupt enable
pub type EOPIE_R = crate::BitReader<EOPIE>;
impl EOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOPIE {
        match self.bits {
            false => EOPIE::Disabled,
            true => EOPIE::Enabled,
        }
    }
    ///End of program interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE::Enabled
    }
}
///Field `EOPIE` writer - End of operation interrupt enable
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of program interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Enabled)
    }
}
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: OPERR Error interrupt disable
    Disabled = 0,
    ///1: OPERR Error interrupt enable
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    ///OPERR Error interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    ///OPERR Error interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OPERR Error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    ///OPERR Error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
/**PCROP read error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE {
    ///0: PCROP read error interrupt disable
    Disabled = 0,
    ///1: PCROP read error interrupt enable
    Enabled = 1,
}
impl From<RDERRIE> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERRIE` reader - PCROP read error interrupt enable
pub type RDERRIE_R = crate::BitReader<RDERRIE>;
impl RDERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDERRIE {
        match self.bits {
            false => RDERRIE::Disabled,
            true => RDERRIE::Enabled,
        }
    }
    ///PCROP read error interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    ///PCROP read error interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE::Enabled
    }
}
///Field `RDERRIE` writer - PCROP read error interrupt enable
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RDERRIE>;
impl<'a, REG> RDERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCROP read error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    ///PCROP read error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Enabled)
    }
}
/**Force the option byte loading

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHR {
    ///0: Option byte loaded
    Complete = 0,
    ///1: Option byte loading to be done
    NotComplete = 1,
}
impl From<OBL_LAUNCHR> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHR) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` reader - Force the option byte loading
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCHR>;
impl OBL_LAUNCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBL_LAUNCHR {
        match self.bits {
            false => OBL_LAUNCHR::Complete,
            true => OBL_LAUNCHR::NotComplete,
        }
    }
    ///Option byte loaded
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR::Complete
    }
    ///Option byte loading to be done
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == OBL_LAUNCHR::NotComplete
    }
}
/**Force the option byte loading

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW {
    ///1: Reload option byte
    Reload = 1,
}
impl From<OBL_LAUNCHW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCHW) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` writer - Force the option byte loading
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG, OBL_LAUNCHW>;
impl<'a, REG> OBL_LAUNCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reload option byte
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCHW::Reload)
    }
}
/**Options Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    ///0: FLASH_CR options are unlocked
    Unlocked = 0,
}
impl From<OPTLOCKR> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` reader - Options Lock
pub type OPTLOCK_R = crate::BitReader<OPTLOCKR>;
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTLOCKR> {
        match self.bits {
            false => Some(OPTLOCKR::Unlocked),
            _ => None,
        }
    }
    ///FLASH_CR options are unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
}
/**Options Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    ///1: FLASH_CR options are locked
    Locked = 1,
}
impl From<OPTLOCKW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` writer - Options Lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCKW>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLASH_CR options are locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Locked)
    }
}
/**FLASH_CR Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    ///0: FLASH_CR is unlocked
    Unlocked = 0,
}
impl From<LOCKR> for bool {
    #[inline(always)]
    fn from(variant: LOCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - FLASH_CR Lock
pub type LOCK_R = crate::BitReader<LOCKR>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCKR> {
        match self.bits {
            false => Some(LOCKR::Unlocked),
            _ => None,
        }
    }
    ///FLASH_CR is unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::Unlocked
    }
}
/**FLASH_CR Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    ///1: FLASH_CR is locked
    Locked = 1,
}
impl From<LOCKW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - FLASH_CR Lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKW>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLASH_CR is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Locked)
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("strt", &self.strt())
            .field("optstrt", &self.optstrt())
            .field("fstpg", &self.fstpg())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .field("obl_launch", &self.obl_launch())
            .field("optlock", &self.optlock())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, CRrs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, CRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, CRrs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<'_, CRrs> {
        FSTPG_W::new(self, 18)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, CRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<'_, CRrs> {
        RDERRIE_W::new(self, 26)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<'_, CRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, CRrs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Flash control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:CR)*/
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
///`reset()` method sets CR to value 0xc000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
