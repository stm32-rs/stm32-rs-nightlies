#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    #[doc = "1: Flash programming activated"]
    Program = 1,
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
    pub const fn variant(&self) -> Option<PG> {
        match self.bits {
            true => Some(PG::Program),
            _ => None,
        }
    }
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG::Program
    }
}
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Program)
    }
}
#[doc = "Page erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    #[doc = "1: Erase activated for selected page"]
    PageErase = 1,
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
    pub const fn variant(&self) -> Option<PER> {
        match self.bits {
            true => Some(PER::PageErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == PER::PageErase
    }
}
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut crate::W<REG> {
        self.variant(PER::PageErase)
    }
}
#[doc = "Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    #[doc = "1: Erase activated for all user sectors"]
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
    pub const fn variant(&self) -> Option<MER> {
        match self.bits {
            true => Some(MER::MassErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for all user sectors"]
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
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
#[doc = "Option byte programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTPG {
    #[doc = "1: Program option byte activated"]
    OptionByteProgramming = 1,
}
impl From<OPTPG> for bool {
    #[inline(always)]
    fn from(variant: OPTPG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTPG` reader - Option byte programming"]
pub type OPTPG_R = crate::BitReader<OPTPG>;
impl OPTPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTPG> {
        match self.bits {
            true => Some(OPTPG::OptionByteProgramming),
            _ => None,
        }
    }
    #[doc = "Program option byte activated"]
    #[inline(always)]
    pub fn is_option_byte_programming(&self) -> bool {
        *self == OPTPG::OptionByteProgramming
    }
}
#[doc = "Field `OPTPG` writer - Option byte programming"]
pub type OPTPG_W<'a, REG> = crate::BitWriter<'a, REG, OPTPG>;
impl<'a, REG> OPTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Program option byte activated"]
    #[inline(always)]
    pub fn option_byte_programming(self) -> &'a mut crate::W<REG> {
        self.variant(OPTPG::OptionByteProgramming)
    }
}
#[doc = "Option byte erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTER {
    #[doc = "1: Erase option byte activated"]
    OptionByteErase = 1,
}
impl From<OPTER> for bool {
    #[inline(always)]
    fn from(variant: OPTER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTER` reader - Option byte erase"]
pub type OPTER_R = crate::BitReader<OPTER>;
impl OPTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTER> {
        match self.bits {
            true => Some(OPTER::OptionByteErase),
            _ => None,
        }
    }
    #[doc = "Erase option byte activated"]
    #[inline(always)]
    pub fn is_option_byte_erase(&self) -> bool {
        *self == OPTER::OptionByteErase
    }
}
#[doc = "Field `OPTER` writer - Option byte erase"]
pub type OPTER_W<'a, REG> = crate::BitWriter<'a, REG, OPTER>;
impl<'a, REG> OPTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase option byte activated"]
    #[inline(always)]
    pub fn option_byte_erase(self) -> &'a mut crate::W<REG> {
        self.variant(OPTER::OptionByteErase)
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT {
    #[doc = "1: Trigger an erase operation"]
    Start = 1,
}
impl From<STRT> for bool {
    #[inline(always)]
    fn from(variant: STRT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader<STRT>;
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRT> {
        match self.bits {
            true => Some(STRT::Start),
            _ => None,
        }
    }
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT::Start
    }
}
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRT>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRT::Start)
    }
}
#[doc = "Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    #[doc = "0: FLASH_CR register is unlocked"]
    Unlocked = 0,
    #[doc = "1: FLASH_CR register is locked"]
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    #[doc = "FLASH_CR register is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    #[doc = "FLASH_CR register is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLASH_CR register is unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    #[doc = "FLASH_CR register is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
#[doc = "Option bytes write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTWRE {
    #[doc = "0: Option byte write disabled"]
    Disabled = 0,
    #[doc = "1: Option byte write enabled"]
    Enabled = 1,
}
impl From<OPTWRE> for bool {
    #[inline(always)]
    fn from(variant: OPTWRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTWRE` reader - Option bytes write enable"]
pub type OPTWRE_R = crate::BitReader<OPTWRE>;
impl OPTWRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTWRE {
        match self.bits {
            false => OPTWRE::Disabled,
            true => OPTWRE::Enabled,
        }
    }
    #[doc = "Option byte write disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPTWRE::Disabled
    }
    #[doc = "Option byte write enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPTWRE::Enabled
    }
}
#[doc = "Field `OPTWRE` writer - Option bytes write enable"]
pub type OPTWRE_W<'a, REG> = crate::BitWriter<'a, REG, OPTWRE>;
impl<'a, REG> OPTWRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Option byte write disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPTWRE::Disabled)
    }
    #[doc = "Option byte write enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPTWRE::Enabled)
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
#[doc = "Force option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_OPTLOAD {
    #[doc = "0: Force option byte loading inactive"]
    Inactive = 0,
    #[doc = "1: Force option byte loading active"]
    Active = 1,
}
impl From<FORCE_OPTLOAD> for bool {
    #[inline(always)]
    fn from(variant: FORCE_OPTLOAD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_OPTLOAD` reader - Force option byte loading"]
pub type FORCE_OPTLOAD_R = crate::BitReader<FORCE_OPTLOAD>;
impl FORCE_OPTLOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_OPTLOAD {
        match self.bits {
            false => FORCE_OPTLOAD::Inactive,
            true => FORCE_OPTLOAD::Active,
        }
    }
    #[doc = "Force option byte loading inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == FORCE_OPTLOAD::Inactive
    }
    #[doc = "Force option byte loading active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FORCE_OPTLOAD::Active
    }
}
#[doc = "Field `FORCE_OPTLOAD` writer - Force option byte loading"]
pub type FORCE_OPTLOAD_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_OPTLOAD>;
impl<'a, REG> FORCE_OPTLOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force option byte loading inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_OPTLOAD::Inactive)
    }
    #[doc = "Force option byte loading active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_OPTLOAD::Active)
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
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn optwre(&self) -> OPTWRE_R {
        OPTWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force option byte loading"]
    #[inline(always)]
    pub fn force_optload(&self) -> FORCE_OPTLOAD_R {
        FORCE_OPTLOAD_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    #[must_use]
    pub fn optpg(&mut self) -> OPTPG_W<CRrs> {
        OPTPG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    #[must_use]
    pub fn opter(&mut self) -> OPTER_W<CRrs> {
        OPTER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 7)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    #[must_use]
    pub fn optwre(&mut self) -> OPTWRE_W<CRrs> {
        OPTWRE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Force option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn force_optload(&mut self) -> FORCE_OPTLOAD_W<CRrs> {
        FORCE_OPTLOAD_W::new(self, 13)
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
#[doc = "`reset()` method sets CR to value 0x80"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x80;
}
