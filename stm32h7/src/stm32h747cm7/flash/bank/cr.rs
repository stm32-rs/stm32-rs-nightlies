#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `LOCK` reader - Bank 1 configuration lock bit"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Bank 1 configuration lock bit"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG` reader - Bank 1 internal buffer control bit"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Bank 1 internal buffer control bit"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER` reader - Bank 1 sector erase request"]
pub type SER_R = crate::BitReader;
#[doc = "Field `SER` writer - Bank 1 sector erase request"]
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER` reader - Bank 1 erase request"]
pub type BER_R = crate::BitReader;
#[doc = "Field `BER` writer - Bank 1 erase request"]
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - Bank 1 program size"]
pub type PSIZE_R = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Bank 1 program size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FW` reader - Bank 1 write forcing control bit"]
pub type FW_R = crate::BitReader;
#[doc = "Field `FW` writer - Bank 1 write forcing control bit"]
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Bank 1 erase start control bit"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Bank 1 erase start control bit"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNB` reader - Bank 1 sector erase selection number"]
pub type SNB_R = crate::FieldReader;
#[doc = "Field `SNB` writer - Bank 1 sector erase selection number"]
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRC_EN` reader - Bank 1 CRC control bit"]
pub type CRC_EN_R = crate::BitReader;
#[doc = "Field `CRC_EN` writer - Bank 1 CRC control bit"]
pub type CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERRIE` reader - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE_R = crate::BitReader;
#[doc = "Field `WRPERRIE` writer - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERRIE` reader - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE_R = crate::BitReader;
#[doc = "Field `PGSERRIE` writer - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERRIE` reader - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE_R = crate::BitReader;
#[doc = "Field `STRBERRIE` writer - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERRIE` reader - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE_R = crate::BitReader;
#[doc = "Field `INCERRIE` writer - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERRIE` reader - Bank 1 write"]
pub type OPERRIE_R = crate::BitReader;
#[doc = "Field `OPERRIE` writer - Bank 1 write"]
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERRIE` reader - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE_R = crate::BitReader;
#[doc = "Field `RDPERRIE` writer - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERRIE` reader - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE_R = crate::BitReader;
#[doc = "Field `RDSERRIE` writer - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERRIE` reader - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE_R = crate::BitReader;
#[doc = "Field `SNECCERRIE` writer - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERRIE` reader - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE_R = crate::BitReader;
#[doc = "Field `DBECCERRIE` writer - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCENDIE` reader - Bank 1 CRC end of calculation interrupt enable bit"]
pub type CRCENDIE_R = crate::BitReader;
#[doc = "Field `CRCENDIE` writer - Bank 1 CRC end of calculation interrupt enable bit"]
pub type CRCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRDERRIE` reader - Bank 1 CRC read error interrupt enable bit"]
pub type CRCRDERRIE_R = crate::BitReader;
#[doc = "Field `CRCRDERRIE` writer - Bank 1 CRC read error interrupt enable bit"]
pub type CRCRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 internal buffer control bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 erase start control bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie(&self) -> SNECCERRIE_R {
        SNECCERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error interrupt enable bit"]
    #[inline(always)]
    pub fn crcrderrie(&self) -> CRCRDERRIE_R {
        CRCRDERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 1 internal buffer control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<CRrs> {
        SER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ber(&mut self) -> BER_W<CRrs> {
        BER_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw(&mut self) -> FW_W<CRrs> {
        FW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 1 erase start control bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    #[must_use]
    pub fn snb(&mut self) -> SNB_W<CRrs> {
        SNB_W::new(self, 8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc_en(&mut self) -> CRC_EN_W<CRrs> {
        CRC_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<CRrs> {
        WRPERRIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<CRrs> {
        PGSERRIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn strberrie(&mut self) -> STRBERRIE_W<CRrs> {
        STRBERRIE_W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn incerrie(&mut self) -> INCERRIE_W<CRrs> {
        INCERRIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    #[must_use]
    pub fn operrie(&mut self) -> OPERRIE_W<CRrs> {
        OPERRIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdperrie(&mut self) -> RDPERRIE_W<CRrs> {
        RDPERRIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<CRrs> {
        RDSERRIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W<CRrs> {
        SNECCERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<CRrs> {
        DBECCERRIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcendie(&mut self) -> CRCENDIE_W<CRrs> {
        CRCENDIE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<CRrs> {
        CRCRDERRIE_W::new(self, 28)
    }
}
#[doc = "FLASH control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x31"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x31;
}
