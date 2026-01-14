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
/**Sector Erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SER {
    ///1: Erase activated for selected sector
    SectorErase = 1,
}
impl From<SER> for bool {
    #[inline(always)]
    fn from(variant: SER) -> Self {
        variant as u8 != 0
    }
}
///Field `SER` reader - Sector Erase
pub type SER_R = crate::BitReader<SER>;
impl SER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SER> {
        match self.bits {
            true => Some(SER::SectorErase),
            _ => None,
        }
    }
    ///Erase activated for selected sector
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SER::SectorErase
    }
}
///Field `SER` writer - Sector Erase
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG, SER>;
impl<'a, REG> SER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Erase activated for selected sector
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut crate::W<REG> {
        self.variant(SER::SectorErase)
    }
}
/**Mass Erase of sectors 0 to 11

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER1 {
    ///1: Erase activated for all user sectors or bank 1 in dual bank mode
    MassErase = 1,
}
impl From<MER1> for bool {
    #[inline(always)]
    fn from(variant: MER1) -> Self {
        variant as u8 != 0
    }
}
///Field `MER1` reader - Mass Erase of sectors 0 to 11
pub type MER1_R = crate::BitReader<MER1>;
impl MER1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER1> {
        match self.bits {
            true => Some(MER1::MassErase),
            _ => None,
        }
    }
    ///Erase activated for all user sectors or bank 1 in dual bank mode
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1::MassErase
    }
}
///Field `MER1` writer - Mass Erase of sectors 0 to 11
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG, MER1>;
impl<'a, REG> MER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Erase activated for all user sectors or bank 1 in dual bank mode
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER1::MassErase)
    }
}
///Field `SNB` reader - Sector number
pub type SNB_R = crate::FieldReader;
///Field `SNB` writer - Sector number
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Program size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    ///0: Program x8
    Psize8 = 0,
    ///1: Program x16
    Psize16 = 1,
    ///2: Program x32
    Psize32 = 2,
    ///3: Program x64
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
impl crate::IsEnum for PSIZE {}
///Field `PSIZE` reader - Program size
pub type PSIZE_R = crate::FieldReader<PSIZE>;
impl PSIZE_R {
    ///Get enumerated values variant
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
    ///Program x8
    #[inline(always)]
    pub fn is_psize8(&self) -> bool {
        *self == PSIZE::Psize8
    }
    ///Program x16
    #[inline(always)]
    pub fn is_psize16(&self) -> bool {
        *self == PSIZE::Psize16
    }
    ///Program x32
    #[inline(always)]
    pub fn is_psize32(&self) -> bool {
        *self == PSIZE::Psize32
    }
    ///Program x64
    #[inline(always)]
    pub fn is_psize64(&self) -> bool {
        *self == PSIZE::Psize64
    }
}
///Field `PSIZE` writer - Program size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE, crate::Safe>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Program x8
    #[inline(always)]
    pub fn psize8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize8)
    }
    ///Program x16
    #[inline(always)]
    pub fn psize16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize16)
    }
    ///Program x32
    #[inline(always)]
    pub fn psize32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize32)
    }
    ///Program x64
    #[inline(always)]
    pub fn psize64(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Psize64)
    }
}
/**Mass Erase of sectors 12 to 23

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER2 {
    ///1: Erase activated for bank 2 in dual bank mode
    MassErase = 1,
}
impl From<MER2> for bool {
    #[inline(always)]
    fn from(variant: MER2) -> Self {
        variant as u8 != 0
    }
}
///Field `MER2` reader - Mass Erase of sectors 12 to 23
pub type MER2_R = crate::BitReader<MER2>;
impl MER2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MER2> {
        match self.bits {
            true => Some(MER2::MassErase),
            _ => None,
        }
    }
    ///Erase activated for bank 2 in dual bank mode
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER2::MassErase
    }
}
///Field `MER2` writer - Mass Erase of sectors 12 to 23
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG, MER2>;
impl<'a, REG> MER2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Erase activated for bank 2 in dual bank mode
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER2::MassErase)
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
/**Lock

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: FLASH_CR register is unlocked
    Unlocked = 0,
    ///1: FLASH_CR register is locked
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - Lock
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
///Field `LOCK` writer - Lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sector Erase
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:7 - Sector number
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - Mass Erase of sectors 12 to 23
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 31 - Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("ser", &self.ser())
            .field("mer1", &self.mer1())
            .field("snb", &self.snb())
            .field("psize", &self.psize())
            .field("mer2", &self.mer2())
            .field("strt", &self.strt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
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
    ///Bit 1 - Sector Erase
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<'_, CRrs> {
        SER_W::new(self, 1)
    }
    ///Bit 2 - Mass Erase of sectors 0 to 11
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<'_, CRrs> {
        MER1_W::new(self, 2)
    }
    ///Bits 3:7 - Sector number
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<'_, CRrs> {
        SNB_W::new(self, 3)
    }
    ///Bits 8:9 - Program size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CRrs> {
        PSIZE_W::new(self, 8)
    }
    ///Bit 15 - Mass Erase of sectors 12 to 23
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W<'_, CRrs> {
        MER2_W::new(self, 15)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, CRrs> {
        STRT_W::new(self, 16)
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
    ///Bit 31 - Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#FLASH:CR)*/
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
///`reset()` method sets CR to value 0x8000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
