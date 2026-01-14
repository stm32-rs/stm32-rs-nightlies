///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `BSY` reader - ongoing program flag
pub type BSY_R = crate::BitReader;
///Field `BSY` writer - ongoing program flag
pub type BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WBNE` reader - write buffer not empty flag
pub type WBNE_R = crate::BitReader;
///Field `WBNE` writer - write buffer not empty flag
pub type WBNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QW` reader - wait queue flag
pub type QW_R = crate::BitReader;
///Field `QW` writer - wait queue flag
pub type QW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_BUSY` reader - CRC busy flag
pub type CRC_BUSY_R = crate::BitReader;
///Field `CRC_BUSY` writer - CRC busy flag
pub type CRC_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOP` reader - end-of-program flag
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - end-of-program flag
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERR` reader - write protection error flag
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - write protection error flag
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERR` reader - programming sequence error flag
pub type PGSERR_R = crate::BitReader;
///Field `PGSERR` writer - programming sequence error flag
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRBERR` reader - strobe error flag
pub type STRBERR_R = crate::BitReader;
///Field `STRBERR` writer - strobe error flag
pub type STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCERR` reader - inconsistency error flag
pub type INCERR_R = crate::BitReader;
///Field `INCERR` writer - inconsistency error flag
pub type INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERR` reader - write/erase error flag
pub type OPERR_R = crate::BitReader;
///Field `OPERR` writer - write/erase error flag
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDPERR` reader - read protection error flag
pub type RDPERR_R = crate::BitReader;
///Field `RDPERR` writer - read protection error flag
pub type RDPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDSERR` reader - secure error flag
pub type RDSERR_R = crate::BitReader;
///Field `RDSERR` writer - secure error flag
pub type RDSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNECCERR` reader - single correction error flag
pub type SNECCERR_R = crate::BitReader;
///Field `SNECCERR` writer - single correction error flag
pub type SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBECCERR` reader - ECC double detection error flag
pub type DBECCERR_R = crate::BitReader;
///Field `DBECCERR` writer - ECC double detection error flag
pub type DBECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEND` reader - CRC-complete flag
pub type CRCEND_R = crate::BitReader;
///Field `CRCEND` writer - CRC-complete flag
pub type CRCEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRDERR` reader - CRC read error flag CRCRDERR1 flag is raised when a word is found read protected during a CRC operation on bank 1. An interrupt is generated if CRCRDIE1 and CRCEND1 are set to 1. Writing 1 to CLR_CRCRDERR1 bit in FLASH_CCR1 register clears CRCRDERR1. Note: This flag is valid only when CRCEND1 bit is set to 1
pub type CRCRDERR_R = crate::BitReader;
impl R {
    ///Bit 0 - ongoing program flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - wait queue flag
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CRC busy flag
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - end-of-program flag
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - write protection error flag
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - strobe error flag
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - inconsistency error flag
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write/erase error flag
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - read protection error flag
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - secure error flag
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - single correction error flag
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ECC double detection error flag
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CRC-complete flag
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CRC read error flag CRCRDERR1 flag is raised when a word is found read protected during a CRC operation on bank 1. An interrupt is generated if CRCRDIE1 and CRCEND1 are set to 1. Writing 1 to CLR_CRCRDERR1 bit in FLASH_CCR1 register clears CRCRDERR1. Note: This flag is valid only when CRCEND1 bit is set to 1
    #[inline(always)]
    pub fn crcrderr(&self) -> CRCRDERR_R {
        CRCRDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("bsy", &self.bsy())
            .field("wbne", &self.wbne())
            .field("qw", &self.qw())
            .field("crc_busy", &self.crc_busy())
            .field("eop", &self.eop())
            .field("wrperr", &self.wrperr())
            .field("pgserr", &self.pgserr())
            .field("strberr", &self.strberr())
            .field("incerr", &self.incerr())
            .field("operr", &self.operr())
            .field("rdperr", &self.rdperr())
            .field("rdserr", &self.rdserr())
            .field("sneccerr", &self.sneccerr())
            .field("dbeccerr", &self.dbeccerr())
            .field("crcend", &self.crcend())
            .field("crcrderr", &self.crcrderr())
            .finish()
    }
}
impl W {
    ///Bit 0 - ongoing program flag
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W<'_, SRrs> {
        BSY_W::new(self, 0)
    }
    ///Bit 1 - write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&mut self) -> WBNE_W<'_, SRrs> {
        WBNE_W::new(self, 1)
    }
    ///Bit 2 - wait queue flag
    #[inline(always)]
    pub fn qw(&mut self) -> QW_W<'_, SRrs> {
        QW_W::new(self, 2)
    }
    ///Bit 3 - CRC busy flag
    #[inline(always)]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W<'_, SRrs> {
        CRC_BUSY_W::new(self, 3)
    }
    ///Bit 16 - end-of-program flag
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 16)
    }
    ///Bit 17 - write protection error flag
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 17)
    }
    ///Bit 18 - programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<'_, SRrs> {
        PGSERR_W::new(self, 18)
    }
    ///Bit 19 - strobe error flag
    #[inline(always)]
    pub fn strberr(&mut self) -> STRBERR_W<'_, SRrs> {
        STRBERR_W::new(self, 19)
    }
    ///Bit 21 - inconsistency error flag
    #[inline(always)]
    pub fn incerr(&mut self) -> INCERR_W<'_, SRrs> {
        INCERR_W::new(self, 21)
    }
    ///Bit 22 - write/erase error flag
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, SRrs> {
        OPERR_W::new(self, 22)
    }
    ///Bit 23 - read protection error flag
    #[inline(always)]
    pub fn rdperr(&mut self) -> RDPERR_W<'_, SRrs> {
        RDPERR_W::new(self, 23)
    }
    ///Bit 24 - secure error flag
    #[inline(always)]
    pub fn rdserr(&mut self) -> RDSERR_W<'_, SRrs> {
        RDSERR_W::new(self, 24)
    }
    ///Bit 25 - single correction error flag
    #[inline(always)]
    pub fn sneccerr(&mut self) -> SNECCERR_W<'_, SRrs> {
        SNECCERR_W::new(self, 25)
    }
    ///Bit 26 - ECC double detection error flag
    #[inline(always)]
    pub fn dbeccerr(&mut self) -> DBECCERR_W<'_, SRrs> {
        DBECCERR_W::new(self, 26)
    }
    ///Bit 27 - CRC-complete flag
    #[inline(always)]
    pub fn crcend(&mut self) -> CRCEND_W<'_, SRrs> {
        CRCEND_W::new(self, 27)
    }
}
/**FLASH status register for bank 1

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#FLASH:SR)*/
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
