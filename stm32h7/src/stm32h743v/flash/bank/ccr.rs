///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CLR_EOP` reader - Bank 1 EOP1 flag clear bit
pub type CLR_EOP_R = crate::BitReader;
///Field `CLR_EOP` writer - Bank 1 EOP1 flag clear bit
pub type CLR_EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_WRPERR` reader - Bank 1 WRPERR1 flag clear bit
pub type CLR_WRPERR_R = crate::BitReader;
///Field `CLR_WRPERR` writer - Bank 1 WRPERR1 flag clear bit
pub type CLR_WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_PGSERR` reader - Bank 1 PGSERR1 flag clear bi
pub type CLR_PGSERR_R = crate::BitReader;
///Field `CLR_PGSERR` writer - Bank 1 PGSERR1 flag clear bi
pub type CLR_PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_STRBERR` reader - Bank 1 STRBERR1 flag clear bit
pub type CLR_STRBERR_R = crate::BitReader;
///Field `CLR_STRBERR` writer - Bank 1 STRBERR1 flag clear bit
pub type CLR_STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_INCERR` reader - Bank 1 INCERR1 flag clear bit
pub type CLR_INCERR_R = crate::BitReader;
///Field `CLR_INCERR` writer - Bank 1 INCERR1 flag clear bit
pub type CLR_INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_OPERR` reader - Bank 1 OPERR1 flag clear bit
pub type CLR_OPERR_R = crate::BitReader;
///Field `CLR_OPERR` writer - Bank 1 OPERR1 flag clear bit
pub type CLR_OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_RDPERR` reader - Bank 1 RDPERR1 flag clear bit
pub type CLR_RDPERR_R = crate::BitReader;
///Field `CLR_RDPERR` writer - Bank 1 RDPERR1 flag clear bit
pub type CLR_RDPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_RDSERR` reader - Bank 1 RDSERR1 flag clear bit
pub type CLR_RDSERR_R = crate::BitReader;
///Field `CLR_RDSERR` writer - Bank 1 RDSERR1 flag clear bit
pub type CLR_RDSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_SNECCERR` reader - Bank 1 SNECCERR1 flag clear bit
pub type CLR_SNECCERR_R = crate::BitReader;
///Field `CLR_SNECCERR` writer - Bank 1 SNECCERR1 flag clear bit
pub type CLR_SNECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_DBECCERR` reader - Bank 1 DBECCERR1 flag clear bit
pub type CLR_DBECCERR_R = crate::BitReader;
///Field `CLR_DBECCERR` writer - Bank 1 DBECCERR1 flag clear bit
pub type CLR_DBECCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_CRCEND` reader - Bank 1 CRCEND1 flag clear bit
pub type CLR_CRCEND_R = crate::BitReader;
///Field `CLR_CRCEND` writer - Bank 1 CRCEND1 flag clear bit
pub type CLR_CRCEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Bank 1 EOP1 flag clear bit
    #[inline(always)]
    pub fn clr_eop(&self) -> CLR_EOP_R {
        CLR_EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Bank 1 WRPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_wrperr(&self) -> CLR_WRPERR_R {
        CLR_WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Bank 1 PGSERR1 flag clear bi
    #[inline(always)]
    pub fn clr_pgserr(&self) -> CLR_PGSERR_R {
        CLR_PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bank 1 STRBERR1 flag clear bit
    #[inline(always)]
    pub fn clr_strberr(&self) -> CLR_STRBERR_R {
        CLR_STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Bank 1 INCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_incerr(&self) -> CLR_INCERR_R {
        CLR_INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Bank 1 OPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_operr(&self) -> CLR_OPERR_R {
        CLR_OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Bank 1 RDPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_rdperr(&self) -> CLR_RDPERR_R {
        CLR_RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bank 1 RDSERR1 flag clear bit
    #[inline(always)]
    pub fn clr_rdserr(&self) -> CLR_RDSERR_R {
        CLR_RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bank 1 SNECCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_sneccerr(&self) -> CLR_SNECCERR_R {
        CLR_SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bank 1 DBECCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_dbeccerr(&self) -> CLR_DBECCERR_R {
        CLR_DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Bank 1 CRCEND1 flag clear bit
    #[inline(always)]
    pub fn clr_crcend(&self) -> CLR_CRCEND_R {
        CLR_CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("clr_eop", &self.clr_eop())
            .field("clr_wrperr", &self.clr_wrperr())
            .field("clr_pgserr", &self.clr_pgserr())
            .field("clr_strberr", &self.clr_strberr())
            .field("clr_incerr", &self.clr_incerr())
            .field("clr_operr", &self.clr_operr())
            .field("clr_rdperr", &self.clr_rdperr())
            .field("clr_rdserr", &self.clr_rdserr())
            .field("clr_sneccerr", &self.clr_sneccerr())
            .field("clr_dbeccerr", &self.clr_dbeccerr())
            .field("clr_crcend", &self.clr_crcend())
            .finish()
    }
}
impl W {
    ///Bit 16 - Bank 1 EOP1 flag clear bit
    #[inline(always)]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<CCRrs> {
        CLR_EOP_W::new(self, 16)
    }
    ///Bit 17 - Bank 1 WRPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<CCRrs> {
        CLR_WRPERR_W::new(self, 17)
    }
    ///Bit 18 - Bank 1 PGSERR1 flag clear bi
    #[inline(always)]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<CCRrs> {
        CLR_PGSERR_W::new(self, 18)
    }
    ///Bit 19 - Bank 1 STRBERR1 flag clear bit
    #[inline(always)]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<CCRrs> {
        CLR_STRBERR_W::new(self, 19)
    }
    ///Bit 21 - Bank 1 INCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<CCRrs> {
        CLR_INCERR_W::new(self, 21)
    }
    ///Bit 22 - Bank 1 OPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_operr(&mut self) -> CLR_OPERR_W<CCRrs> {
        CLR_OPERR_W::new(self, 22)
    }
    ///Bit 23 - Bank 1 RDPERR1 flag clear bit
    #[inline(always)]
    pub fn clr_rdperr(&mut self) -> CLR_RDPERR_W<CCRrs> {
        CLR_RDPERR_W::new(self, 23)
    }
    ///Bit 24 - Bank 1 RDSERR1 flag clear bit
    #[inline(always)]
    pub fn clr_rdserr(&mut self) -> CLR_RDSERR_W<CCRrs> {
        CLR_RDSERR_W::new(self, 24)
    }
    ///Bit 25 - Bank 1 SNECCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_sneccerr(&mut self) -> CLR_SNECCERR_W<CCRrs> {
        CLR_SNECCERR_W::new(self, 25)
    }
    ///Bit 26 - Bank 1 DBECCERR1 flag clear bit
    #[inline(always)]
    pub fn clr_dbeccerr(&mut self) -> CLR_DBECCERR_W<CCRrs> {
        CLR_DBECCERR_W::new(self, 26)
    }
    ///Bit 27 - Bank 1 CRCEND1 flag clear bit
    #[inline(always)]
    pub fn clr_crcend(&mut self) -> CLR_CRCEND_W<CCRrs> {
        CLR_CRCEND_W::new(self, 27)
    }
}
/**FLASH clear control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
