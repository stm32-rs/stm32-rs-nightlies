#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `BSY` reader - Bank 1 busy flag"]
pub type BSY_R = crate::BitReader;
#[doc = "Field `BSY` writer - Bank 1 busy flag"]
pub type BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBNE` reader - Bank 1 write buffer not empty flag"]
pub type WBNE_R = crate::BitReader;
#[doc = "Field `WBNE` writer - Bank 1 write buffer not empty flag"]
pub type WBNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QW` reader - Bank 1 wait queue flag"]
pub type QW_R = crate::BitReader;
#[doc = "Field `QW` writer - Bank 1 wait queue flag"]
pub type QW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BUSY` reader - Bank 1 CRC busy flag"]
pub type CRC_BUSY_R = crate::BitReader;
#[doc = "Field `CRC_BUSY` writer - Bank 1 CRC busy flag"]
pub type CRC_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` reader - Bank 1 end-of-program flag"]
pub type EOP_R = crate::BitReader;
#[doc = "Field `EOP` writer - Bank 1 end-of-program flag"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Bank 1 write protection error flag"]
pub type WRPERR_R = crate::BitReader;
#[doc = "Field `WRPERR` writer - Bank 1 write protection error flag"]
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Bank 1 programming sequence error flag"]
pub type PGSERR_R = crate::BitReader;
#[doc = "Field `PGSERR` writer - Bank 1 programming sequence error flag"]
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERR` reader - Bank 1 strobe error flag"]
pub type STRBERR_R = crate::BitReader;
#[doc = "Field `STRBERR` writer - Bank 1 strobe error flag"]
pub type STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERR` reader - Bank 1 inconsistency error flag"]
pub type INCERR_R = crate::BitReader;
#[doc = "Field `INCERR` writer - Bank 1 inconsistency error flag"]
pub type INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Bank 1 write"]
pub type OPERR_R = crate::BitReader;
#[doc = "Field `OPERR` writer - Bank 1 write"]
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERR` reader - Bank 1 read protection error flag"]
pub type RDPERR_R = crate::BitReader;
#[doc = "Field `RDPERR` writer - Bank 1 read protection error flag"]
pub type RDPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERR` reader - Bank 1 secure error flag"]
pub type RDSERR_R = crate::BitReader;
#[doc = "Field `RDSERR` writer - Bank 1 secure error flag"]
pub type RDSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERR` reader - Bank 1 single correction error flag"]
pub type SNECCERR_R = crate::BitReader;
#[doc = "Field `SNECCERR` writer - Bank 1 single correction error flag"]
pub type SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERR` reader - Bank 1 ECC double detection error flag"]
pub type DBECCERR_R = crate::BitReader;
#[doc = "Field `DBECCERR` writer - Bank 1 ECC double detection error flag"]
pub type DBECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEND` reader - Bank 1 CRC end of calculation flag"]
pub type CRCEND_R = crate::BitReader;
#[doc = "Field `CRCEND` writer - Bank 1 CRC end of calculation flag"]
pub type CRCEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRDERR` reader - Bank 1 CRC read error flag"]
pub type CRCRDERR_R = crate::BitReader;
#[doc = "Field `CRCRDERR` writer - Bank 1 CRC read error flag"]
pub type CRCRDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 1 busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation flag"]
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error flag"]
    #[inline(always)]
    pub fn crcrderr(&self) -> CRCRDERR_R {
        CRCRDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn bsy(&mut self) -> BSY_W<SRrs> {
        BSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn wbne(&mut self) -> WBNE_W<SRrs> {
        WBNE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    #[must_use]
    pub fn qw(&mut self) -> QW_W<SRrs> {
        QW_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W<SRrs> {
        CRC_BUSY_W::new(self, 3)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SRrs> {
        WRPERR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<SRrs> {
        PGSERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    #[must_use]
    pub fn strberr(&mut self) -> STRBERR_W<SRrs> {
        STRBERR_W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    #[must_use]
    pub fn incerr(&mut self) -> INCERR_W<SRrs> {
        INCERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<SRrs> {
        OPERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdperr(&mut self) -> RDPERR_W<SRrs> {
        RDPERR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdserr(&mut self) -> RDSERR_W<SRrs> {
        RDSERR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerr(&mut self) -> SNECCERR_W<SRrs> {
        SNECCERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerr(&mut self) -> DBECCERR_W<SRrs> {
        DBECCERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcend(&mut self) -> CRCEND_W<SRrs> {
        CRCEND_W::new(self, 27)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcrderr(&mut self) -> CRCRDERR_W<SRrs> {
        CRCRDERR_W::new(self, 28)
    }
}
#[doc = "FLASH status register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
