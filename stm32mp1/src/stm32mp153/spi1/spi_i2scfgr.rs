#[doc = "Register `SPI_I2SCFGR` reader"]
pub type R = crate::R<SPI_I2SCFGRrs>;
#[doc = "Register `SPI_I2SCFGR` writer"]
pub type W = crate::W<SPI_I2SCFGRrs>;
#[doc = "Field `I2SMOD` reader - I2SMOD"]
pub type I2SMOD_R = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2SMOD"]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub type I2SCFG_R = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2SCFG"]
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I2SSTD` reader - I2SSTD"]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2SSTD"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCMSYNC"]
pub type PCMSYNC_R = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCMSYNC"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - DATLEN"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - DATLEN"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHLEN` reader - CHLEN"]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - CHLEN"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPOL` reader - CKPOL"]
pub type CKPOL_R = crate::BitReader;
#[doc = "Field `CKPOL` writer - CKPOL"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXCH` reader - FIXCH"]
pub type FIXCH_R = crate::BitReader;
#[doc = "Field `FIXCH` writer - FIXCH"]
pub type FIXCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WSINV` reader - WSINV"]
pub type WSINV_R = crate::BitReader;
#[doc = "Field `WSINV` writer - WSINV"]
pub type WSINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATFMT` reader - DATFMT"]
pub type DATFMT_R = crate::BitReader;
#[doc = "Field `DATFMT` writer - DATFMT"]
pub type DATFMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SDIV` reader - I2SDIV"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2SDIV"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - ODD"]
pub type ODD_R = crate::BitReader;
#[doc = "Field `ODD` writer - ODD"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - MCKOE"]
pub type MCKOE_R = crate::BitReader;
#[doc = "Field `MCKOE` writer - MCKOE"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2SMOD"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIXCH"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WSINV"]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DATFMT"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - ODD"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MCKOE"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2SMOD"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<SPI_I2SCFGRrs> {
        I2SMOD_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - I2SCFG"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<SPI_I2SCFGRrs> {
        I2SCFG_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<SPI_I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<SPI_I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - DATLEN"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<SPI_I2SCFGRrs> {
        DATLEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - CHLEN"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<SPI_I2SCFGRrs> {
        CHLEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - CKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<SPI_I2SCFGRrs> {
        CKPOL_W::new(self, 11)
    }
    #[doc = "Bit 12 - FIXCH"]
    #[inline(always)]
    #[must_use]
    pub fn fixch(&mut self) -> FIXCH_W<SPI_I2SCFGRrs> {
        FIXCH_W::new(self, 12)
    }
    #[doc = "Bit 13 - WSINV"]
    #[inline(always)]
    #[must_use]
    pub fn wsinv(&mut self) -> WSINV_W<SPI_I2SCFGRrs> {
        WSINV_W::new(self, 13)
    }
    #[doc = "Bit 14 - DATFMT"]
    #[inline(always)]
    #[must_use]
    pub fn datfmt(&mut self) -> DATFMT_W<SPI_I2SCFGRrs> {
        DATFMT_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - I2SDIV"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<SPI_I2SCFGRrs> {
        I2SDIV_W::new(self, 16)
    }
    #[doc = "Bit 24 - ODD"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<SPI_I2SCFGRrs> {
        ODD_W::new(self, 24)
    }
    #[doc = "Bit 25 - MCKOE"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<SPI_I2SCFGRrs> {
        MCKOE_W::new(self, 25)
    }
}
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_I2SCFGRrs;
impl crate::RegisterSpec for SPI_I2SCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_i2scfgr::R`](R) reader structure"]
impl crate::Readable for SPI_I2SCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`spi_i2scfgr::W`](W) writer structure"]
impl crate::Writable for SPI_I2SCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_I2SCFGR to value 0"]
impl crate::Resettable for SPI_I2SCFGRrs {
    const RESET_VALUE: u32 = 0;
}
