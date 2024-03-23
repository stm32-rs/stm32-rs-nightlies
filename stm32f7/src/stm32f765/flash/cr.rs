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
#[doc = "Sector Erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SER {
    #[doc = "1: Erase activated for selected sector"]
    SectorErase = 1,
}
impl From<SER> for bool {
    #[inline(always)]
    fn from(variant: SER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SER` reader - Sector Erase"]
pub type SER_R = crate::BitReader<SER>;
impl SER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SER> {
        match self.bits {
            true => Some(SER::SectorErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for selected sector"]
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SER::SectorErase
    }
}
#[doc = "Field `SER` writer - Sector Erase"]
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG, SER>;
impl<'a, REG> SER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for selected sector"]
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut crate::W<REG> {
        self.variant(SER::SectorErase)
    }
}
#[doc = "Mass Erase of sectors 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER1 {
    #[doc = "1: Erase activated for all user sectors or bank 1 in dual bank mode"]
    MassErase = 1,
}
impl From<MER1> for bool {
    #[inline(always)]
    fn from(variant: MER1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER1` reader - Mass Erase of sectors 0 to 11"]
pub type MER1_R = crate::BitReader<MER1>;
impl MER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER1> {
        match self.bits {
            true => Some(MER1::MassErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for all user sectors or bank 1 in dual bank mode"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1::MassErase
    }
}
#[doc = "Field `MER1` writer - Mass Erase of sectors 0 to 11"]
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG, MER1>;
impl<'a, REG> MER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for all user sectors or bank 1 in dual bank mode"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER1::MassErase)
    }
}
#[doc = "Field `SNB` reader - Sector number"]
pub type SNB_R = crate::FieldReader;
#[doc = "Field `SNB` writer - Sector number"]
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Program size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    #[doc = "0: Program x8"]
    Psize8 = 0,
    #[doc = "1: Program x16"]
    Psize16 = 1,
    #[doc = "2: Program x32"]
    Psize32 = 2,
    #[doc = "3: Program x64"]
    Psize64 = 3,
}
impl From<PSIZE> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSIZE {
    type Ux = u8;
}
#[doc = "Field `PSIZE` reader - Program size"]
pub type PSIZE_R = crate::FieldReader<PSIZE>;
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSIZE {
        match self.bits {
            0 => PSIZE::Psize8,
            1 => PSIZE::Psize16,
            2 => PSIZE::Psize32,
            3 => PSIZE::Psize64,
            _ => unreachable!(),
        }
    }
    #[doc = "Program x8"]
    #[inline(always)]
    pub fn is_psize8(&self) -> bool {
        *self == PSIZE::Psize8
    }
    #[doc = "Program x16"]
    #[inline(always)]
    pub fn is_psize16(&self) -> bool {
        *self == PSIZE::Psize16
    }
    #[doc = "Program x32"]
    #[inline(always)]
    pub fn is_psize32(&self) -> bool {
        *self == PSIZE::Psize32
    }
    #[doc = "Program x64"]
    #[inline(always)]
    pub fn is_psize64(&self) -> bool {
        *self == PSIZE::Psize64
    }
}
#[doc = "Field `PSIZE` writer - Program size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Program x8"]
    #[inline(always)]
    pub fn psize8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize8)
    }
    #[doc = "Program x16"]
    #[inline(always)]
    pub fn psize16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize16)
    }
    #[doc = "Program x32"]
    #[inline(always)]
    pub fn psize32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize32)
    }
    #[doc = "Program x64"]
    #[inline(always)]
    pub fn psize64(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize64)
    }
}
#[doc = "Mass Erase of sectors 12 to 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER2 {
    #[doc = "1: Erase activated for bank 2 in dual bank mode"]
    MassErase = 1,
}
impl From<MER2> for bool {
    #[inline(always)]
    fn from(variant: MER2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER2` reader - Mass Erase of sectors 12 to 23"]
pub type MER2_R = crate::BitReader<MER2>;
impl MER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER2> {
        match self.bits {
            true => Some(MER2::MassErase),
            _ => None,
        }
    }
    #[doc = "Erase activated for bank 2 in dual bank mode"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2::MassErase
    }
}
#[doc = "Field `MER2` writer - Mass Erase of sectors 12 to 23"]
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG, MER2>;
impl<'a, REG> MER2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for bank 2 in dual bank mode"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER2::MassErase)
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
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Sector number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Mass Erase of sectors 12 to 23"]
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 31 - Lock"]
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
    #[doc = "Bit 1 - Sector Erase"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<CRrs> {
        SER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mass Erase of sectors 0 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<CRrs> {
        MER1_W::new(self, 2)
    }
    #[doc = "Bits 3:7 - Sector number"]
    #[inline(always)]
    #[must_use]
    pub fn snb(&mut self) -> SNB_W<CRrs> {
        SNB_W::new(self, 3)
    }
    #[doc = "Bits 8:9 - Program size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bit 15 - Mass Erase of sectors 12 to 23"]
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<CRrs> {
        MER2_W::new(self, 15)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
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
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x8000_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
