///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `BSY` reader - Bank 1 busy flag
pub type BSY_R = crate::BitReader;
///Field `BSY` writer - Bank 1 busy flag
pub type BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WBNE` reader - Bank 1 write buffer not empty flag
pub type WBNE_R = crate::BitReader;
///Field `WBNE` writer - Bank 1 write buffer not empty flag
pub type WBNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QW` reader - Bank 1 wait queue flag
pub type QW_R = crate::BitReader;
///Field `QW` writer - Bank 1 wait queue flag
pub type QW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_BUSY` reader - Bank 1 CRC busy flag
pub type CRC_BUSY_R = crate::BitReader;
///Field `CRC_BUSY` writer - Bank 1 CRC busy flag
pub type CRC_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOP` reader - Bank 1 end-of-program flag
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - Bank 1 end-of-program flag
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERR` reader - Bank 1 write protection error flag
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - Bank 1 write protection error flag
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERR` reader - Bank 1 programming sequence error flag
pub type PGSERR_R = crate::BitReader;
///Field `PGSERR` writer - Bank 1 programming sequence error flag
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERR` reader - Bank 1 strobe error flag
pub type STRBERR_R = crate::BitReader;
///Field `STRBERR` writer - Bank 1 strobe error flag
pub type STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERR` reader - Bank 1 inconsistency error flag
pub type INCERR_R = crate::BitReader;
///Field `INCERR` writer - Bank 1 inconsistency error flag
pub type INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERR` reader - Bank 1 write
pub type OPERR_R = crate::BitReader;
///Field `OPERR` writer - Bank 1 write
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDPERR` reader - Bank 1 read protection error flag
pub type RDPERR_R = crate::BitReader;
///Field `RDPERR` writer - Bank 1 read protection error flag
pub type RDPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERR` reader - Bank 1 secure error flag
pub type RDSERR_R = crate::BitReader;
///Field `RDSERR` writer - Bank 1 secure error flag
pub type RDSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERR` reader - Bank 1 single correction error flag
pub type SNECCERR_R = crate::BitReader;
///Field `SNECCERR` writer - Bank 1 single correction error flag
pub type SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERR` reader - Bank 1 ECC double detection error flag
pub type DBECCERR_R = crate::BitReader;
///Field `DBECCERR` writer - Bank 1 ECC double detection error flag
pub type DBECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEND` reader - Bank 1 CRC end of calculation flag
pub type CRCEND_R = crate::BitReader;
///Field `CRCEND` writer - Bank 1 CRC end of calculation flag
pub type CRCEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERR` reader - Bank 1 CRC read error flag
pub type CRCRDERR_R = crate::BitReader;
///Field `CRCRDERR` writer - Bank 1 CRC read error flag
pub type CRCRDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Bank 1 busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bank 1 write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bank 1 wait queue flag
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bank 1 CRC busy flag
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Bank 1 end-of-program flag
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Bank 1 write protection error flag
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Bank 1 programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bank 1 strobe error flag
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Bank 1 inconsistency error flag
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Bank 1 write
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Bank 1 read protection error flag
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bank 1 secure error flag
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bank 1 single correction error flag
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bank 1 ECC double detection error flag
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Bank 1 CRC end of calculation flag
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Bank 1 CRC read error flag
    #[inline(always)]
    pub fn crcrderr(&self) -> CRCRDERR_R {
        CRCRDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("crcrderr", &self.crcrderr())
            .field("crcend", &self.crcend())
            .field("dbeccerr", &self.dbeccerr())
            .field("sneccerr", &self.sneccerr())
            .field("rdserr", &self.rdserr())
            .field("rdperr", &self.rdperr())
            .field("operr", &self.operr())
            .field("incerr", &self.incerr())
            .field("strberr", &self.strberr())
            .field("pgserr", &self.pgserr())
            .field("wrperr", &self.wrperr())
            .field("eop", &self.eop())
            .field("crc_busy", &self.crc_busy())
            .field("qw", &self.qw())
            .field("wbne", &self.wbne())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bank 1 busy flag
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W<'_, SRrs> {
        BSY_W::new(self, 0)
    }
    ///Bit 1 - Bank 1 write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&mut self) -> WBNE_W<'_, SRrs> {
        WBNE_W::new(self, 1)
    }
    ///Bit 2 - Bank 1 wait queue flag
    #[inline(always)]
    pub fn qw(&mut self) -> QW_W<'_, SRrs> {
        QW_W::new(self, 2)
    }
    ///Bit 3 - Bank 1 CRC busy flag
    #[inline(always)]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W<'_, SRrs> {
        CRC_BUSY_W::new(self, 3)
    }
    ///Bit 16 - Bank 1 end-of-program flag
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 16)
    }
    ///Bit 17 - Bank 1 write protection error flag
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 17)
    }
    ///Bit 18 - Bank 1 programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<'_, SRrs> {
        PGSERR_W::new(self, 18)
    }
    ///Bit 19 - Bank 1 strobe error flag
    #[inline(always)]
    pub fn strberr(&mut self) -> STRBERR_W<'_, SRrs> {
        STRBERR_W::new(self, 19)
    }
    ///Bit 21 - Bank 1 inconsistency error flag
    #[inline(always)]
    pub fn incerr(&mut self) -> INCERR_W<'_, SRrs> {
        INCERR_W::new(self, 21)
    }
    ///Bit 22 - Bank 1 write
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, SRrs> {
        OPERR_W::new(self, 22)
    }
    ///Bit 23 - Bank 1 read protection error flag
    #[inline(always)]
    pub fn rdperr(&mut self) -> RDPERR_W<'_, SRrs> {
        RDPERR_W::new(self, 23)
    }
    ///Bit 24 - Bank 1 secure error flag
    #[inline(always)]
    pub fn rdserr(&mut self) -> RDSERR_W<'_, SRrs> {
        RDSERR_W::new(self, 24)
    }
    ///Bit 25 - Bank 1 single correction error flag
    #[inline(always)]
    pub fn sneccerr(&mut self) -> SNECCERR_W<'_, SRrs> {
        SNECCERR_W::new(self, 25)
    }
    ///Bit 26 - Bank 1 ECC double detection error flag
    #[inline(always)]
    pub fn dbeccerr(&mut self) -> DBECCERR_W<'_, SRrs> {
        DBECCERR_W::new(self, 26)
    }
    ///Bit 27 - Bank 1 CRC end of calculation flag
    #[inline(always)]
    pub fn crcend(&mut self) -> CRCEND_W<'_, SRrs> {
        CRCEND_W::new(self, 27)
    }
    ///Bit 28 - Bank 1 CRC read error flag
    #[inline(always)]
    pub fn crcrderr(&mut self) -> CRCRDERR_W<'_, SRrs> {
        CRCRDERR_W::new(self, 28)
    }
}
/**FLASH status register for bank 1

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
