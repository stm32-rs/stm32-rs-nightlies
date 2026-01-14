///Register `PLL2SSCGR` reader
pub type R = crate::R<PLL2SSCGRrs>;
///Register `PLL2SSCGR` writer
pub type W = crate::W<PLL2SSCGRrs>;
///Field `MODPER` reader - Modulation Period Adjustment for PLL2
pub type MODPER_R = crate::FieldReader<u16>;
///Field `MODPER` writer - Modulation Period Adjustment for PLL2
pub type MODPER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `TPDFNDIS` reader - Dithering TPDF noise control for PLL2
pub type TPDFNDIS_R = crate::BitReader;
///Field `TPDFNDIS` writer - Dithering TPDF noise control for PLL2
pub type TPDFNDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPDFNDIS` reader - Dithering RPDF noise control for PLL2
pub type RPDFNDIS_R = crate::BitReader;
///Field `RPDFNDIS` writer - Dithering RPDF noise control for PLL2
pub type RPDFNDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPREADSEL` reader - Spread spectrum clock generator mode for PLL2
pub type SPREADSEL_R = crate::BitReader;
///Field `SPREADSEL` writer - Spread spectrum clock generator mode for PLL2
pub type SPREADSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCSTEP` reader - Modulation Depth Adjustment for PLL2
pub type INCSTEP_R = crate::FieldReader<u16>;
///Field `INCSTEP` writer - Modulation Depth Adjustment for PLL2
pub type INCSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:12 - Modulation Period Adjustment for PLL2
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bit 13 - Dithering TPDF noise control for PLL2
    #[inline(always)]
    pub fn tpdfndis(&self) -> TPDFNDIS_R {
        TPDFNDIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Dithering RPDF noise control for PLL2
    #[inline(always)]
    pub fn rpdfndis(&self) -> RPDFNDIS_R {
        RPDFNDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Spread spectrum clock generator mode for PLL2
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:30 - Modulation Depth Adjustment for PLL2
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2SSCGR")
            .field("modper", &self.modper())
            .field("tpdfndis", &self.tpdfndis())
            .field("rpdfndis", &self.rpdfndis())
            .field("spreadsel", &self.spreadsel())
            .field("incstep", &self.incstep())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Modulation Period Adjustment for PLL2
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W<'_, PLL2SSCGRrs> {
        MODPER_W::new(self, 0)
    }
    ///Bit 13 - Dithering TPDF noise control for PLL2
    #[inline(always)]
    pub fn tpdfndis(&mut self) -> TPDFNDIS_W<'_, PLL2SSCGRrs> {
        TPDFNDIS_W::new(self, 13)
    }
    ///Bit 14 - Dithering RPDF noise control for PLL2
    #[inline(always)]
    pub fn rpdfndis(&mut self) -> RPDFNDIS_W<'_, PLL2SSCGRrs> {
        RPDFNDIS_W::new(self, 14)
    }
    ///Bit 15 - Spread spectrum clock generator mode for PLL2
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<'_, PLL2SSCGRrs> {
        SPREADSEL_W::new(self, 15)
    }
    ///Bits 16:30 - Modulation Depth Adjustment for PLL2
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W<'_, PLL2SSCGRrs> {
        INCSTEP_W::new(self, 16)
    }
}
/**RCC PLL2 Spread Spectrum Clock Generator register

You can [`read`](crate::Reg::read) this register and get [`pll2sscgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2sscgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2SSCGR)*/
pub struct PLL2SSCGRrs;
impl crate::RegisterSpec for PLL2SSCGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2sscgr::R`](R) reader structure
impl crate::Readable for PLL2SSCGRrs {}
///`write(|w| ..)` method takes [`pll2sscgr::W`](W) writer structure
impl crate::Writable for PLL2SSCGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2SSCGR to value 0
impl crate::Resettable for PLL2SSCGRrs {}
