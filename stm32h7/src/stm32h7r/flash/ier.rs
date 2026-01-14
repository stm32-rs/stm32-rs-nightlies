///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `EOPIE` reader - End-of-program interrupt control bit
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - End-of-program interrupt control bit
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERRIE` reader - Write protection error interrupt enable bit
pub type WRPERRIE_R = crate::BitReader;
///Field `WRPERRIE` writer - Write protection error interrupt enable bit
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERRIE` reader - Programming sequence error interrupt enable bit
pub type PGSERRIE_R = crate::BitReader;
///Field `PGSERRIE` writer - Programming sequence error interrupt enable bit
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERRIE` reader - Strobe error interrupt enable bit
pub type STRBERRIE_R = crate::BitReader;
///Field `STRBERRIE` writer - Strobe error interrupt enable bit
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBLERRIE` reader - Option byte loading error interrupt enable bit
pub type OBLERRIE_R = crate::BitReader;
///Field `OBLERRIE` writer - Option byte loading error interrupt enable bit
pub type OBLERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERRIE` reader - Inconsistency error interrupt enable bit
pub type INCERRIE_R = crate::BitReader;
///Field `INCERRIE` writer - Inconsistency error interrupt enable bit
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERRIE` reader - Read security error interrupt enable bit
pub type RDSERRIE_R = crate::BitReader;
///Field `RDSERRIE` writer - Read security error interrupt enable bit
pub type RDSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERRIE` reader - ECC single correction error interrupt enable bit
pub type SNECCERRIE_R = crate::BitReader;
///Field `SNECCERRIE` writer - ECC single correction error interrupt enable bit
pub type SNECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERRIE` reader - ECC double detection error interrupt enable bit
pub type DBECCERRIE_R = crate::BitReader;
///Field `DBECCERRIE` writer - ECC double detection error interrupt enable bit
pub type DBECCERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENDIE` reader - CRC end of calculation interrupt enable bit
pub type CRCENDIE_R = crate::BitReader;
///Field `CRCENDIE` writer - CRC end of calculation interrupt enable bit
pub type CRCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERRIE` reader - CRC read error interrupt enable bit
pub type CRCRDERRIE_R = crate::BitReader;
///Field `CRCRDERRIE` writer - CRC read error interrupt enable bit
pub type CRCRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - End-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Option byte loading error interrupt enable bit
    #[inline(always)]
    pub fn oblerrie(&self) -> OBLERRIE_R {
        OBLERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Read security error interrupt enable bit
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
    ///Bit 27 - CRC end of calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CRC read error interrupt enable bit
    #[inline(always)]
    pub fn crcrderrie(&self) -> CRCRDERRIE_R {
        CRCRDERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("eopie", &self.eopie())
            .field("wrperrie", &self.wrperrie())
            .field("pgserrie", &self.pgserrie())
            .field("strberrie", &self.strberrie())
            .field("oblerrie", &self.oblerrie())
            .field("incerrie", &self.incerrie())
            .field("rdserrie", &self.rdserrie())
            .field("sneccerrie", &self.sneccerrie())
            .field("dbeccerrie", &self.dbeccerrie())
            .field("crcendie", &self.crcendie())
            .field("crcrderrie", &self.crcrderrie())
            .finish()
    }
}
impl W {
    ///Bit 16 - End-of-program interrupt control bit
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, IERrs> {
        EOPIE_W::new(self, 16)
    }
    ///Bit 17 - Write protection error interrupt enable bit
    #[inline(always)]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<'_, IERrs> {
        WRPERRIE_W::new(self, 17)
    }
    ///Bit 18 - Programming sequence error interrupt enable bit
    #[inline(always)]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<'_, IERrs> {
        PGSERRIE_W::new(self, 18)
    }
    ///Bit 19 - Strobe error interrupt enable bit
    #[inline(always)]
    pub fn strberrie(&mut self) -> STRBERRIE_W<'_, IERrs> {
        STRBERRIE_W::new(self, 19)
    }
    ///Bit 20 - Option byte loading error interrupt enable bit
    #[inline(always)]
    pub fn oblerrie(&mut self) -> OBLERRIE_W<'_, IERrs> {
        OBLERRIE_W::new(self, 20)
    }
    ///Bit 21 - Inconsistency error interrupt enable bit
    #[inline(always)]
    pub fn incerrie(&mut self) -> INCERRIE_W<'_, IERrs> {
        INCERRIE_W::new(self, 21)
    }
    ///Bit 24 - Read security error interrupt enable bit
    #[inline(always)]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<'_, IERrs> {
        RDSERRIE_W::new(self, 24)
    }
    ///Bit 25 - ECC single correction error interrupt enable bit
    #[inline(always)]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W<'_, IERrs> {
        SNECCERRIE_W::new(self, 25)
    }
    ///Bit 26 - ECC double detection error interrupt enable bit
    #[inline(always)]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<'_, IERrs> {
        DBECCERRIE_W::new(self, 26)
    }
    ///Bit 27 - CRC end of calculation interrupt enable bit
    #[inline(always)]
    pub fn crcendie(&mut self) -> CRCENDIE_W<'_, IERrs> {
        CRCENDIE_W::new(self, 27)
    }
    ///Bit 28 - CRC read error interrupt enable bit
    #[inline(always)]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<'_, IERrs> {
        CRCRDERRIE_W::new(self, 28)
    }
}
/**FLASH interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
