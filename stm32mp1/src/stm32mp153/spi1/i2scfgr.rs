///Register `I2SCFGR` reader
pub type R = crate::R<I2SCFGRrs>;
///Register `I2SCFGR` writer
pub type W = crate::W<I2SCFGRrs>;
///Field `I2SMOD` reader - I2SMOD
pub type I2SMOD_R = crate::BitReader;
///Field `I2SMOD` writer - I2SMOD
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader;
///Field `I2SCFG` writer - I2SCFG
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I2SSTD` reader - I2SSTD
pub type I2SSTD_R = crate::FieldReader;
///Field `I2SSTD` writer - I2SSTD
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PCMSYNC` reader - PCMSYNC
pub type PCMSYNC_R = crate::BitReader;
///Field `PCMSYNC` writer - PCMSYNC
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATLEN` reader - DATLEN
pub type DATLEN_R = crate::FieldReader;
///Field `DATLEN` writer - DATLEN
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHLEN` reader - CHLEN
pub type CHLEN_R = crate::BitReader;
///Field `CHLEN` writer - CHLEN
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::BitReader;
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIXCH` reader - FIXCH
pub type FIXCH_R = crate::BitReader;
///Field `FIXCH` writer - FIXCH
pub type FIXCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WSINV` reader - WSINV
pub type WSINV_R = crate::BitReader;
///Field `WSINV` writer - WSINV
pub type WSINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATFMT` reader - DATFMT
pub type DATFMT_R = crate::BitReader;
///Field `DATFMT` writer - DATFMT
pub type DATFMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SDIV` reader - I2SDIV
pub type I2SDIV_R = crate::FieldReader;
///Field `I2SDIV` writer - I2SDIV
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ODD` reader - ODD
pub type ODD_R = crate::BitReader;
///Field `ODD` writer - ODD
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKOE` reader - MCKOE
pub type MCKOE_R = crate::BitReader;
///Field `MCKOE` writer - MCKOE
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - FIXCH
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WSINV
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DATFMT
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - I2SDIV
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - ODD
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - MCKOE
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCFGR")
            .field("i2smod", &self.i2smod())
            .field("i2scfg", &self.i2scfg())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("datlen", &self.datlen())
            .field("chlen", &self.chlen())
            .field("ckpol", &self.ckpol())
            .field("fixch", &self.fixch())
            .field("wsinv", &self.wsinv())
            .field("datfmt", &self.datfmt())
            .field("i2sdiv", &self.i2sdiv())
            .field("odd", &self.odd())
            .field("mckoe", &self.mckoe())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<'_, I2SCFGRrs> {
        I2SMOD_W::new(self, 0)
    }
    ///Bits 1:3 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<'_, I2SCFGRrs> {
        I2SCFG_W::new(self, 1)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<'_, I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<'_, I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - DATLEN
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<'_, I2SCFGRrs> {
        DATLEN_W::new(self, 8)
    }
    ///Bit 10 - CHLEN
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<'_, I2SCFGRrs> {
        CHLEN_W::new(self, 10)
    }
    ///Bit 11 - CKPOL
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, I2SCFGRrs> {
        CKPOL_W::new(self, 11)
    }
    ///Bit 12 - FIXCH
    #[inline(always)]
    pub fn fixch(&mut self) -> FIXCH_W<'_, I2SCFGRrs> {
        FIXCH_W::new(self, 12)
    }
    ///Bit 13 - WSINV
    #[inline(always)]
    pub fn wsinv(&mut self) -> WSINV_W<'_, I2SCFGRrs> {
        WSINV_W::new(self, 13)
    }
    ///Bit 14 - DATFMT
    #[inline(always)]
    pub fn datfmt(&mut self) -> DATFMT_W<'_, I2SCFGRrs> {
        DATFMT_W::new(self, 14)
    }
    ///Bits 16:23 - I2SDIV
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<'_, I2SCFGRrs> {
        I2SDIV_W::new(self, 16)
    }
    ///Bit 24 - ODD
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<'_, I2SCFGRrs> {
        ODD_W::new(self, 24)
    }
    ///Bit 25 - MCKOE
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<'_, I2SCFGRrs> {
        MCKOE_W::new(self, 25)
    }
}
/**All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:I2SCFGR)*/
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`i2scfgr::R`](R) reader structure
impl crate::Readable for I2SCFGRrs {}
///`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGRrs {}
