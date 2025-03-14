///Register `I2SCFGR` reader
pub type R = crate::R<I2SCFGRrs>;
///Register `I2SCFGR` writer
pub type W = crate::W<I2SCFGRrs>;
///Field `CHLEN` reader - CHLEN
pub type CHLEN_R = crate::BitReader;
///Field `CHLEN` writer - CHLEN
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATLEN` reader - DATLEN
pub type DATLEN_R = crate::FieldReader;
///Field `DATLEN` writer - DATLEN
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::BitReader;
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SSTD` reader - I2SSTD
pub type I2SSTD_R = crate::FieldReader;
///Field `I2SSTD` writer - I2SSTD
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PCMSYNC` reader - PCMSYNC
pub type PCMSYNC_R = crate::BitReader;
///Field `PCMSYNC` writer - PCMSYNC
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader;
///Field `I2SCFG` writer - I2SCFG
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2SE` reader - I2SE
pub type I2SE_R = crate::BitReader;
///Field `I2SE` writer - I2SE
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SMOD` reader - I2SMOD
pub type I2SMOD_R = crate::BitReader;
///Field `I2SMOD` writer - I2SMOD
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCFGR")
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("ckpol", &self.ckpol())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("i2scfg", &self.i2scfg())
            .field("i2se", &self.i2se())
            .field("i2smod", &self.i2smod())
            .finish()
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W<I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#SPI1:I2SCFGR)*/
pub struct I2SCFGRrs;
impl crate::RegisterSpec for I2SCFGRrs {
    type Ux = u16;
}
///`read()` method returns [`i2scfgr::R`](R) reader structure
impl crate::Readable for I2SCFGRrs {}
///`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure
impl crate::Writable for I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGRrs {}
