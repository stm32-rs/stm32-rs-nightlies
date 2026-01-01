///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    ///1: Flash programming activated
    Program = 1,
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
    pub const fn variant(&self) -> Option<PG> {
        match self.bits {
            true => Some(PG::Program),
            _ => None,
        }
    }
    ///Flash programming activated
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG::Program
    }
}
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash programming activated
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Program)
    }
}
/**Page erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    ///1: Erase activated for selected page
    PageErase = 1,
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
    pub const fn variant(&self) -> Option<PER> {
        match self.bits {
            true => Some(PER::PageErase),
            _ => None,
        }
    }
    ///Erase activated for selected page
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == PER::PageErase
    }
}
///Field `PER` writer - Page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Erase activated for selected page
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut crate::W<REG> {
        self.variant(PER::PageErase)
    }
}
/**Mass erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    ///1: Erase activated for all user sectors
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
    pub const fn variant(&self) -> Option<MER> {
        match self.bits {
            true => Some(MER::MassErase),
            _ => None,
        }
    }
    ///Erase activated for all user sectors
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
    ///Erase activated for all user sectors
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
/**Option byte programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTPG {
    ///1: Program option byte activated
    OptionByteProgramming = 1,
}
impl From<OPTPG> for bool {
    #[inline(always)]
    fn from(variant: OPTPG) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTPG` reader - Option byte programming
pub type OPTPG_R = crate::BitReader<OPTPG>;
impl OPTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTPG> {
        match self.bits {
            true => Some(OPTPG::OptionByteProgramming),
            _ => None,
        }
    }
    ///Program option byte activated
    #[inline(always)]
    pub fn is_option_byte_programming(&self) -> bool {
        *self == OPTPG::OptionByteProgramming
    }
}
///Field `OPTPG` writer - Option byte programming
pub type OPTPG_W<'a, REG> = crate::BitWriter<'a, REG, OPTPG>;
impl<'a, REG> OPTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Program option byte activated
    #[inline(always)]
    pub fn option_byte_programming(self) -> &'a mut crate::W<REG> {
        self.variant(OPTPG::OptionByteProgramming)
    }
}
/**Option byte erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTER {
    ///1: Erase option byte activated
    OptionByteErase = 1,
}
impl From<OPTER> for bool {
    #[inline(always)]
    fn from(variant: OPTER) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTER` reader - Option byte erase
pub type OPTER_R = crate::BitReader<OPTER>;
impl OPTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTER> {
        match self.bits {
            true => Some(OPTER::OptionByteErase),
            _ => None,
        }
    }
    ///Erase option byte activated
    #[inline(always)]
    pub fn is_option_byte_erase(&self) -> bool {
        *self == OPTER::OptionByteErase
    }
}
///Field `OPTER` writer - Option byte erase
pub type OPTER_W<'a, REG> = crate::BitWriter<'a, REG, OPTER>;
impl<'a, REG> OPTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Erase option byte activated
    #[inline(always)]
    pub fn option_byte_erase(self) -> &'a mut crate::W<REG> {
        self.variant(OPTER::OptionByteErase)
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT {
    ///1: Trigger an erase operation
    Start = 1,
}
impl From<STRT> for bool {
    #[inline(always)]
    fn from(variant: STRT) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader<STRT>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRT> {
        match self.bits {
            true => Some(STRT::Start),
            _ => None,
        }
    }
    ///Trigger an erase operation
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT::Start
    }
}
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRT>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRT::Start)
    }
}
/**Lock

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
///Field `LOCK` reader - Lock
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
/**Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    ///1: Lock the FLASH_CR register
    Lock = 1,
}
impl From<LOCKW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - Lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKW>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Lock the FLASH_CR register
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Lock)
    }
}
/**Option bytes write enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTWRE {
    ///0: Option byte write enabled
    Disabled = 0,
    ///1: Option byte write disabled
    Enabled = 1,
}
impl From<OPTWRE> for bool {
    #[inline(always)]
    fn from(variant: OPTWRE) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTWRE` reader - Option bytes write enable
pub type OPTWRE_R = crate::BitReader<OPTWRE>;
impl OPTWRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPTWRE {
        match self.bits {
            false => OPTWRE::Disabled,
            true => OPTWRE::Enabled,
        }
    }
    ///Option byte write enabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPTWRE::Disabled
    }
    ///Option byte write disabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPTWRE::Enabled
    }
}
///Field `OPTWRE` writer - Option bytes write enable
pub type OPTWRE_W<'a, REG> = crate::BitWriter<'a, REG, OPTWRE>;
impl<'a, REG> OPTWRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Option byte write enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPTWRE::Disabled)
    }
    ///Option byte write disabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPTWRE::Enabled)
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
/**Force option byte loading

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBL_LAUNCH {
    ///0: Force option byte loading inactive
    Inactive = 0,
    ///1: Force option byte loading active
    Active = 1,
}
impl From<OBL_LAUNCH> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` reader - Force option byte loading
pub type OBL_LAUNCH_R = crate::BitReader<OBL_LAUNCH>;
impl OBL_LAUNCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBL_LAUNCH {
        match self.bits {
            false => OBL_LAUNCH::Inactive,
            true => OBL_LAUNCH::Active,
        }
    }
    ///Force option byte loading inactive
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OBL_LAUNCH::Inactive
    }
    ///Force option byte loading active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OBL_LAUNCH::Active
    }
}
///Field `OBL_LAUNCH` writer - Force option byte loading
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG, OBL_LAUNCH>;
impl<'a, REG> OBL_LAUNCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Force option byte loading inactive
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCH::Inactive)
    }
    ///Force option byte loading active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(OBL_LAUNCH::Active)
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
    ///Bit 4 - Option byte programming
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    pub fn optwre(&self) -> OPTWRE_R {
        OPTWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("obl_launch", &self.obl_launch())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("optwre", &self.optwre())
            .field("lock", &self.lock())
            .field("strt", &self.strt())
            .field("opter", &self.opter())
            .field("optpg", &self.optpg())
            .field("mer", &self.mer())
            .field("per", &self.per())
            .field("pg", &self.pg())
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
    ///Bit 4 - Option byte programming
    #[inline(always)]
    pub fn optpg(&mut self) -> OPTPG_W<'_, CRrs> {
        OPTPG_W::new(self, 4)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    pub fn opter(&mut self) -> OPTER_W<'_, CRrs> {
        OPTER_W::new(self, 5)
    }
    ///Bit 6 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, CRrs> {
        STRT_W::new(self, 6)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 7)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    pub fn optwre(&mut self) -> OPTWRE_W<'_, CRrs> {
        OPTWRE_W::new(self, 9)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, CRrs> {
        EOPIE_W::new(self, 12)
    }
    ///Bit 13 - Force option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<'_, CRrs> {
        OBL_LAUNCH_W::new(self, 13)
    }
}
/**Flash control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#FLASH:CR)*/
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
///`reset()` method sets CR to value 0x80
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x80;
}
