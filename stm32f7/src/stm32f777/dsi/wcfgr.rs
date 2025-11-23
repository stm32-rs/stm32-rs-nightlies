///Register `WCFGR` reader
pub type R = crate::R<WCFGRrs>;
///Register `WCFGR` writer
pub type W = crate::W<WCFGRrs>;
///Field `DSIM` reader - DSI Mode
pub type DSIM_R = crate::BitReader;
///Field `DSIM` writer - DSI Mode
pub type DSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLMUX` reader - Color Multiplexing
pub type COLMUX_R = crate::FieldReader;
///Field `COLMUX` writer - Color Multiplexing
pub type COLMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TESRC` reader - TE Source
pub type TESRC_R = crate::BitReader;
///Field `TESRC` writer - TE Source
pub type TESRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEPOL` reader - TE Polarity
pub type TEPOL_R = crate::BitReader;
///Field `TEPOL` writer - TE Polarity
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AR` reader - Automatic Refresh
pub type AR_R = crate::BitReader;
///Field `AR` writer - Automatic Refresh
pub type AR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - VSync Polarity
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - VSync Polarity
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DSI Mode
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Color Multiplexing
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - TE Source
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TE Polarity
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic Refresh
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VSync Polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCFGR")
            .field("vspol", &self.vspol())
            .field("ar", &self.ar())
            .field("tepol", &self.tepol())
            .field("tesrc", &self.tesrc())
            .field("colmux", &self.colmux())
            .field("dsim", &self.dsim())
            .finish()
    }
}
impl W {
    ///Bit 0 - DSI Mode
    #[inline(always)]
    pub fn dsim(&mut self) -> DSIM_W<'_, WCFGRrs> {
        DSIM_W::new(self, 0)
    }
    ///Bits 1:3 - Color Multiplexing
    #[inline(always)]
    pub fn colmux(&mut self) -> COLMUX_W<'_, WCFGRrs> {
        COLMUX_W::new(self, 1)
    }
    ///Bit 4 - TE Source
    #[inline(always)]
    pub fn tesrc(&mut self) -> TESRC_W<'_, WCFGRrs> {
        TESRC_W::new(self, 4)
    }
    ///Bit 5 - TE Polarity
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W<'_, WCFGRrs> {
        TEPOL_W::new(self, 5)
    }
    ///Bit 6 - Automatic Refresh
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<'_, WCFGRrs> {
        AR_W::new(self, 6)
    }
    ///Bit 7 - VSync Polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<'_, WCFGRrs> {
        VSPOL_W::new(self, 7)
    }
}
/**DSI Wrapper Configuration Register

You can [`read`](crate::Reg::read) this register and get [`wcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:WCFGR)*/
pub struct WCFGRrs;
impl crate::RegisterSpec for WCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`wcfgr::R`](R) reader structure
impl crate::Readable for WCFGRrs {}
///`write(|w| ..)` method takes [`wcfgr::W`](W) writer structure
impl crate::Writable for WCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WCFGR to value 0
impl crate::Resettable for WCFGRrs {}
