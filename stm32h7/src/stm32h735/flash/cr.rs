///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LOCK` reader - configuration lock bit
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - configuration lock bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG` reader - program enable bit
pub type PG_R = crate::BitReader;
///Field `PG` writer - program enable bit
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SER` reader - sector erase request
pub type SER_R = crate::BitReader;
///Field `SER` writer - sector erase request
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BER` reader - erase request
pub type BER_R = crate::BitReader;
///Field `BER` writer - erase request
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - program size
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - program size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FW` reader - write forcing control bit
pub type FW_R = crate::BitReader;
///Field `FW` writer - write forcing control bit
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - bank or sector erase start control bit
pub type START_R = crate::BitReader;
///Field `START` writer - bank or sector erase start control bit
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNB` reader - sector erase selection number
pub type SNB_R = crate::FieldReader;
///Field `SNB` writer - sector erase selection number
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CRC_EN` reader - CRC control bit
pub type CRC_EN_R = crate::BitReader;
///Field `CRC_EN` writer - CRC control bit
pub type CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - end-of-program interrupt control bit
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - end-of-program interrupt control bit
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRIE` reader - write protection error interrupt enable bit
pub type WRPERRIE_R = crate::BitReader;
///Field `WRPERRIE` writer - write protection error interrupt enable bit
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRIE` reader - programming sequence error interrupt enable bit
pub type PGSERRIE_R = crate::BitReader;
///Field `PGSERRIE` writer - programming sequence error interrupt enable bit
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRIE` reader - strobe error interrupt enable bit
pub type STRBERRIE_R = crate::BitReader;
///Field `STRBERRIE` writer - strobe error interrupt enable bit
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRIE` reader - inconsistency error interrupt enable bit
pub type INCERRIE_R = crate::BitReader;
///Field `INCERRIE` writer - inconsistency error interrupt enable bit
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERRIE` reader - write/erase error interrupt enable bit
pub type OPERRIE_R = crate::BitReader;
///Field `OPERRIE` writer - write/erase error interrupt enable bit
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDPERRIE` reader - read protection error interrupt enable bit
pub type RDPERRIE_R = crate::BitReader;
///Field `RDPERRIE` writer - read protection error interrupt enable bit
pub type RDPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERRIE` reader - secure error interrupt enable bit
pub type RDSERRIE_R = crate::BitReader;
///Field `RDSERRIE` writer - secure error interrupt enable bit
pub type RDSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERRIE` reader - ECC single correction error interrupt enable bit
pub type SNECCERRIE_R = crate::BitReader;
///Field `SNECCERRIE` writer - ECC single correction error interrupt enable bit
pub type SNECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERRIE` reader - ECC double detection error interrupt enable bit
pub type DBECCERRIE_R = crate::BitReader;
///Field `DBECCERRIE` writer - ECC double detection error interrupt enable bit
pub type DBECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENDIE` reader - end of CRC calculation interrupt enable bit
pub type CRCENDIE_R = crate::BitReader;
///Field `CRCENDIE` writer - end of CRC calculation interrupt enable bit
pub type CRCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERRIE` reader - CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_R = crate::BitReader;
///Field `CRCRDERRIE` writer - CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - configuration lock bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - program enable bit
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - sector erase request
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - erase request
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - program size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - write forcing control bit
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - bank or sector erase start control bit
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - sector erase selection number
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 15 - CRC control bit
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - end-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write/erase error interrupt enable bit
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - read protection error interrupt enable bit
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - secure error interrupt enable bit
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn sneccerrie(&self) -> SNECCERRIE_R {
        SNECCERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ECC double detection error interrupt enable bit
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - end of CRC calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcrderrie(&self) -> CRCRDERRIE_R {
        CRCRDERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lock", &self.lock())
            .field("pg", &self.pg())
            .field("ser", &self.ser())
            .field("ber", &self.ber())
            .field("psize", &self.psize())
            .field("fw", &self.fw())
            .field("start", &self.start())
            .field("snb", &self.snb())
            .field("crc_en", &self.crc_en())
            .field("eopie", &self.eopie())
            .field("wrperrie", &self.wrperrie())
            .field("pgserrie", &self.pgserrie())
            .field("strberrie", &self.strberrie())
            .field("incerrie", &self.incerrie())
            .field("operrie", &self.operrie())
            .field("rdperrie", &self.rdperrie())
            .field("rdserrie", &self.rdserrie())
            .field("sneccerrie", &self.sneccerrie())
            .field("dbeccerrie", &self.dbeccerrie())
            .field("crcendie", &self.crcendie())
            .field("crcrderrie", &self.crcrderrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - configuration lock bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - program enable bit
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CRrs> {
        PG_W::new(self, 1)
    }
    ///Bit 2 - sector erase request
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<'_, CRrs> {
        SER_W::new(self, 2)
    }
    ///Bit 3 - erase request
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<'_, CRrs> {
        BER_W::new(self, 3)
    }
    ///Bits 4:5 - program size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CRrs> {
        PSIZE_W::new(self, 4)
    }
    ///Bit 6 - write forcing control bit
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W<'_, CRrs> {
        FW_W::new(self, 6)
    }
    ///Bit 7 - bank or sector erase start control bit
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 7)
    }
    ///Bits 8:10 - sector erase selection number
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<'_, CRrs> {
        SNB_W::new(self, 8)
    }
    ///Bit 15 - CRC control bit
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W<'_, CRrs> {
        CRC_EN_W::new(self, 15)
    }
    ///Bit 16 - end-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, CRrs> {
        EOPIE_W::new(self, 16)
    }
    ///Bit 17 - write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<'_, CRrs> {
        WRPERRIE_W::new(self, 17)
    }
    ///Bit 18 - programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<'_, CRrs> {
        PGSERRIE_W::new(self, 18)
    }
    ///Bit 19 - strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W<'_, CRrs> {
        STRBERRIE_W::new(self, 19)
    }
    ///Bit 21 - inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W<'_, CRrs> {
        INCERRIE_W::new(self, 21)
    }
    ///Bit 22 - write/erase error interrupt enable bit
    #[inline(always)]
    pub fn operrie(&mut self) -> OPERRIE_W<'_, CRrs> {
        OPERRIE_W::new(self, 22)
    }
    ///Bit 23 - read protection error interrupt enable bit
    #[inline(always)]
    pub fn rdperrie(&mut self) -> RDPERRIE_W<'_, CRrs> {
        RDPERRIE_W::new(self, 23)
    }
    ///Bit 24 - secure error interrupt enable bit
    #[inline(always)]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<'_, CRrs> {
        RDSERRIE_W::new(self, 24)
    }
    ///Bit 25 - ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W<'_, CRrs> {
        SNECCERRIE_W::new(self, 25)
    }
    ///Bit 26 - ECC double detection error interrupt enable bit
    #[inline(always)]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<'_, CRrs> {
        DBECCERRIE_W::new(self, 26)
    }
    ///Bit 27 - end of CRC calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&mut self) -> CRCENDIE_W<'_, CRrs> {
        CRCENDIE_W::new(self, 27)
    }
    ///Bit 28 - CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<'_, CRrs> {
        CRCRDERRIE_W::new(self, 28)
    }
}
/**FLASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:CR)*/
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
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
