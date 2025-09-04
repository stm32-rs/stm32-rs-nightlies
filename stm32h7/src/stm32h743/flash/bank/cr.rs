///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LOCK` reader - Bank 1 configuration lock bit
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Bank 1 configuration lock bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG` reader - Bank 1 program enable bit
pub type PG_R = crate::BitReader;
///Field `PG` writer - Bank 1 program enable bit
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SER` reader - Bank 1 sector erase request
pub type SER_R = crate::BitReader;
///Field `SER` writer - Bank 1 sector erase request
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BER` reader - Bank 1 erase request
pub type BER_R = crate::BitReader;
///Field `BER` writer - Bank 1 erase request
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - Bank 1 program size
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - Bank 1 program size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FW` reader - Bank 1 write forcing control bit
pub type FW_R = crate::BitReader;
///Field `FW` writer - Bank 1 write forcing control bit
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Bank 1 bank or sector erase start control bit
pub type START_R = crate::BitReader;
///Field `START` writer - Bank 1 bank or sector erase start control bit
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNB` reader - Bank 1 sector erase selection number
pub type SNB_R = crate::FieldReader;
///Field `SNB` writer - Bank 1 sector erase selection number
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPSS` reader - Bank 2 special sector selection bit
pub type SPSS_R = crate::BitReader;
///Field `SPSS` writer - Bank 2 special sector selection bit
pub type SPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_EN` reader - Bank 1 CRC control bit
pub type CRC_EN_R = crate::BitReader;
///Field `CRC_EN` writer - Bank 1 CRC control bit
pub type CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Bank 1 end-of-program interrupt control bit
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Bank 1 end-of-program interrupt control bit
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRIE` reader - Bank 1 write protection error interrupt enable bit
pub type WRPERRIE_R = crate::BitReader;
///Field `WRPERRIE` writer - Bank 1 write protection error interrupt enable bit
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRIE` reader - Bank 1 programming sequence error interrupt enable bit
pub type PGSERRIE_R = crate::BitReader;
///Field `PGSERRIE` writer - Bank 1 programming sequence error interrupt enable bit
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRIE` reader - Bank 1 strobe error interrupt enable bit
pub type STRBERRIE_R = crate::BitReader;
///Field `STRBERRIE` writer - Bank 1 strobe error interrupt enable bit
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRIE` reader - Bank 1 inconsistency error interrupt enable bit
pub type INCERRIE_R = crate::BitReader;
///Field `INCERRIE` writer - Bank 1 inconsistency error interrupt enable bit
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERRIE` reader - Bank 1 write/erase error interrupt enable bit
pub type OPERRIE_R = crate::BitReader;
///Field `OPERRIE` writer - Bank 1 write/erase error interrupt enable bit
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDPERRIE` reader - Bank 1 read protection error interrupt enable bit
pub type RDPERRIE_R = crate::BitReader;
///Field `RDPERRIE` writer - Bank 1 read protection error interrupt enable bit
pub type RDPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERRIE` reader - Bank 1 secure error interrupt enable bit
pub type RDSERRIE_R = crate::BitReader;
///Field `RDSERRIE` writer - Bank 1 secure error interrupt enable bit
pub type RDSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERR` reader - Bank 1 ECC single correction error interrupt enable bit
pub type SNECCERR_R = crate::BitReader;
///Field `SNECCERR` writer - Bank 1 ECC single correction error interrupt enable bit
pub type SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERRIE` reader - Bank 1 ECC double detection error interrupt enable bit
pub type DBECCERRIE_R = crate::BitReader;
///Field `DBECCERRIE` writer - Bank 1 ECC double detection error interrupt enable bit
pub type DBECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENDIE` reader - Bank 1 end of CRC calculation interrupt enable bit
pub type CRCENDIE_R = crate::BitReader;
///Field `CRCENDIE` writer - Bank 1 end of CRC calculation interrupt enable bit
pub type CRCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERRIE` reader - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_R = crate::BitReader;
///Field `CRCRDERRIE` writer - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
pub type CRCRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Bank 1 configuration lock bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bank 1 program enable bit
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bank 1 sector erase request
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bank 1 erase request
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Bank 1 program size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Bank 1 write forcing control bit
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bank 1 bank or sector erase start control bit
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Bank 1 sector erase selection number
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 14 - Bank 2 special sector selection bit
    #[inline(always)]
    pub fn spss(&self) -> SPSS_R {
        SPSS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bank 1 CRC control bit
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Bank 1 end-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Bank 1 write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Bank 1 programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bank 1 strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Bank 1 inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Bank 1 write/erase error interrupt enable bit
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Bank 1 read protection error interrupt enable bit
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bank 1 secure error interrupt enable bit
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bank 1 ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bank 1 ECC double detection error interrupt enable bit
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Bank 1 end of CRC calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
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
            .field("sneccerr", &self.sneccerr())
            .field("dbeccerrie", &self.dbeccerrie())
            .field("crcendie", &self.crcendie())
            .field("crcrderrie", &self.crcrderrie())
            .field("spss", &self.spss())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bank 1 configuration lock bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - Bank 1 program enable bit
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 1)
    }
    ///Bit 2 - Bank 1 sector erase request
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W<CRrs> {
        SER_W::new(self, 2)
    }
    ///Bit 3 - Bank 1 erase request
    #[inline(always)]
    pub fn ber(&mut self) -> BER_W<CRrs> {
        BER_W::new(self, 3)
    }
    ///Bits 4:5 - Bank 1 program size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 4)
    }
    ///Bit 6 - Bank 1 write forcing control bit
    #[inline(always)]
    pub fn fw(&mut self) -> FW_W<CRrs> {
        FW_W::new(self, 6)
    }
    ///Bit 7 - Bank 1 bank or sector erase start control bit
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 7)
    }
    ///Bits 8:10 - Bank 1 sector erase selection number
    #[inline(always)]
    pub fn snb(&mut self) -> SNB_W<CRrs> {
        SNB_W::new(self, 8)
    }
    ///Bit 14 - Bank 2 special sector selection bit
    #[inline(always)]
    pub fn spss(&mut self) -> SPSS_W<CRrs> {
        SPSS_W::new(self, 14)
    }
    ///Bit 15 - Bank 1 CRC control bit
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W<CRrs> {
        CRC_EN_W::new(self, 15)
    }
    ///Bit 16 - Bank 1 end-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 16)
    }
    ///Bit 17 - Bank 1 write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<CRrs> {
        WRPERRIE_W::new(self, 17)
    }
    ///Bit 18 - Bank 1 programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<CRrs> {
        PGSERRIE_W::new(self, 18)
    }
    ///Bit 19 - Bank 1 strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W<CRrs> {
        STRBERRIE_W::new(self, 19)
    }
    ///Bit 21 - Bank 1 inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W<CRrs> {
        INCERRIE_W::new(self, 21)
    }
    ///Bit 22 - Bank 1 write/erase error interrupt enable bit
    #[inline(always)]
    pub fn operrie(&mut self) -> OPERRIE_W<CRrs> {
        OPERRIE_W::new(self, 22)
    }
    ///Bit 23 - Bank 1 read protection error interrupt enable bit
    #[inline(always)]
    pub fn rdperrie(&mut self) -> RDPERRIE_W<CRrs> {
        RDPERRIE_W::new(self, 23)
    }
    ///Bit 24 - Bank 1 secure error interrupt enable bit
    #[inline(always)]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<CRrs> {
        RDSERRIE_W::new(self, 24)
    }
    ///Bit 25 - Bank 1 ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn sneccerr(&mut self) -> SNECCERR_W<CRrs> {
        SNECCERR_W::new(self, 25)
    }
    ///Bit 26 - Bank 1 ECC double detection error interrupt enable bit
    #[inline(always)]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<CRrs> {
        DBECCERRIE_W::new(self, 26)
    }
    ///Bit 27 - Bank 1 end of CRC calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&mut self) -> CRCENDIE_W<CRrs> {
        CRCENDIE_W::new(self, 27)
    }
    ///Bit 28 - Bank 1 CRC read error interrupt enable bit When CRCRDERRIE1 bit is set to 1, an interrupt is generated when a protected area (PCROP or secure-only) has been detected during the last CRC computation on bank 1. CRCRDERRIE1 can be programmed only when LOCK1 is cleared to 0.
    #[inline(always)]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<CRrs> {
        CRCRDERRIE_W::new(self, 28)
    }
}
/**FLASH control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
