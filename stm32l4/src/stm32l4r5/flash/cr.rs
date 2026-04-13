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
/**Bank 1 Mass erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER1W {
    ///1: This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
    MassErase = 1,
}
impl From<MER1W> for bool {
    #[inline(always)]
    fn from(variant: MER1W) -> Self {
        variant as u8 != 0
    }
}
///Field `MER1` reader - Bank 1 Mass erase
pub type MER1_R = crate::BitReader<MER1W>;
impl MER1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER1W> {
        match self.bits {
            true => Some(MER1W::MassErase),
            _ => None,
        }
    }
    ///This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1W::MassErase
    }
}
///Field `MER1` writer - Bank 1 Mass erase
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG, MER1W>;
impl<'a, REG> MER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit triggers the bank 1 mass erase (all bank 1 user pages) when set
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER1W::MassErase)
    }
}
///Field `PNB` reader - Page number
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page number
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Bank erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKER {
    ///0: Bank 1 is selected for page erase
    Bank1 = 0,
    ///1: Bank 2 is selected for page erase
    Bank2 = 1,
}
impl From<BKER> for bool {
    #[inline(always)]
    fn from(variant: BKER) -> Self {
        variant as u8 != 0
    }
}
///Field `BKER` reader - Bank erase
pub type BKER_R = crate::BitReader<BKER>;
impl BKER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKER {
        match self.bits {
            false => BKER::Bank1,
            true => BKER::Bank2,
        }
    }
    ///Bank 1 is selected for page erase
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BKER::Bank1
    }
    ///Bank 2 is selected for page erase
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BKER::Bank2
    }
}
///Field `BKER` writer - Bank erase
pub type BKER_W<'a, REG> = crate::BitWriter<'a, REG, BKER>;
impl<'a, REG> BKER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bank 1 is selected for page erase
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(BKER::Bank1)
    }
    ///Bank 2 is selected for page erase
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(BKER::Bank2)
    }
}
/**Bank 2 Mass erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER2W {
    ///1: This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
    MassErase = 1,
}
impl From<MER2W> for bool {
    #[inline(always)]
    fn from(variant: MER2W) -> Self {
        variant as u8 != 0
    }
}
///Field `MER2` reader - Bank 2 Mass erase
pub type MER2_R = crate::BitReader<MER2W>;
impl MER2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER2W> {
        match self.bits {
            true => Some(MER2W::MassErase),
            _ => None,
        }
    }
    ///This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2W::MassErase
    }
}
///Field `MER2` writer - Bank 2 Mass erase
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG, MER2W>;
impl<'a, REG> MER2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit triggers the bank 2 mass erase (all bank 2 user pages) when set
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER2W::MassErase)
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR {
    ///0: Cleared when BSY bit is cleared in SR
    Complete = 0,
    ///1: Erase operation requested
    Requested = 1,
}
impl From<STARTR> for bool {
    #[inline(always)]
    fn from(variant: STARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<STARTR>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STARTR {
        match self.bits {
            false => STARTR::Complete,
            true => STARTR::Requested,
        }
    }
    ///Cleared when BSY bit is cleared in SR
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == STARTR::Complete
    }
    ///Erase operation requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == STARTR::Requested
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    ///1: Trigger an erase operation
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
/**Options modification start

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
///Field `OPTSTRT` reader - Options modification start
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
/**Options modification start

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
///Field `OPTSTRT` writer - Options modification start
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
    ///0: End of operation interrupt disabled
    Disabled = 0,
    ///1: End of operation interrupt enabled
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
    ///End of operation interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    ///End of operation interrupt enabled
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
    ///End of operation interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    ///End of operation interrupt enabled
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
    ///0: Error interrupt generation disabled
    Disabled = 0,
    ///1: Error interrupt generation enabled
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
    ///Error interrupt generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    ///Error interrupt generation enabled
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
    ///Error interrupt generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    ///Error interrupt generation enabled
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
    ///0: PCROP read error interrupt disabled
    Disabled = 0,
    ///1: PCROP read error interrupt enabled
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
    ///PCROP read error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    ///PCROP read error interrupt enabled
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
    ///PCROP read error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    ///PCROP read error interrupt enabled
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
    ///0: Option byte loading complete
    Complete = 0,
    ///1: Option byte loading requested
    Requested = 1,
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
            true => OBL_LAUNCHR::Requested,
        }
    }
    ///Option byte loading complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == OBL_LAUNCHR::Complete
    }
    ///Option byte loading requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == OBL_LAUNCHR::Requested
    }
}
/**Force the option byte loading

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCHW {
    ///1: Force option byte reloading
    Set = 1,
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
    ///Force option byte reloading
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCHW::Set)
    }
}
/**Options Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKR {
    ///0: Option page is unlocked
    Unlocked = 0,
    ///1: All bits concerning user option in FLASH_CR register and so option page are locked
    Locked = 1,
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
    pub const fn variant(&self) -> OPTLOCKR {
        match self.bits {
            false => OPTLOCKR::Unlocked,
            true => OPTLOCKR::Locked,
        }
    }
    ///Option page is unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCKR::Unlocked
    }
    ///All bits concerning user option in FLASH_CR register and so option page are locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCKR::Locked
    }
}
/**Options Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCKW {
    ///1: This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked
    Set = 1,
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
    ///This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCKW::Set)
    }
}
/**FLASH_CR Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    ///0: FLASH_CR register is unlocked
    Unlocked = 0,
    ///1: FLASH_CR register is locked
    Locked = 1,
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
    pub const fn variant(&self) -> LOCKR {
        match self.bits {
            false => LOCKR::Unlocked,
            true => LOCKR::Locked,
        }
    }
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::Unlocked
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::Locked
    }
}
/**FLASH_CR Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    ///1: This bit is set only. When set, the FLASH_CR register is locked
    Set = 1,
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
    ///This bit is set only. When set, the FLASH_CR register is locked
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Set)
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
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:10 - Page number
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("mer1", &self.mer1())
            .field("pnb", &self.pnb())
            .field("bker", &self.bker())
            .field("mer2", &self.mer2())
            .field("start", &self.start())
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
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<'_, CRrs> {
        MER1_W::new(self, 2)
    }
    ///Bits 3:10 - Page number
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    pub fn bker(&mut self) -> BKER_W<'_, CRrs> {
        BKER_W::new(self, 11)
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W<'_, CRrs> {
        MER2_W::new(self, 15)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 16)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#FLASH:CR)*/
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
