///Register `DSI_LPCR` reader
pub type R = crate::R<DSI_LPCRrs>;
///Register `DSI_LPCR` writer
pub type W = crate::W<DSI_LPCRrs>;
///Field `DEP` reader - Data enable polarity This bit configures the polarity of data enable pin.
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - Data enable polarity This bit configures the polarity of data enable pin.
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSP` reader - VSYNC polarity This bit configures the polarity of VSYNC pin.
pub type VSP_R = crate::BitReader;
///Field `VSP` writer - VSYNC polarity This bit configures the polarity of VSYNC pin.
pub type VSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSP` reader - HSYNC polarity This bit configures the polarity of HSYNC pin.
pub type HSP_R = crate::BitReader;
///Field `HSP` writer - HSYNC polarity This bit configures the polarity of HSYNC pin.
pub type HSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data enable polarity This bit configures the polarity of data enable pin.
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSYNC polarity This bit configures the polarity of VSYNC pin.
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSYNC polarity This bit configures the polarity of HSYNC pin.
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_LPCR")
            .field("dep", &self.dep())
            .field("vsp", &self.vsp())
            .field("hsp", &self.hsp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data enable polarity This bit configures the polarity of data enable pin.
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<DSI_LPCRrs> {
        DEP_W::new(self, 0)
    }
    ///Bit 1 - VSYNC polarity This bit configures the polarity of VSYNC pin.
    #[inline(always)]
    #[must_use]
    pub fn vsp(&mut self) -> VSP_W<DSI_LPCRrs> {
        VSP_W::new(self, 1)
    }
    ///Bit 2 - HSYNC polarity This bit configures the polarity of HSYNC pin.
    #[inline(always)]
    #[must_use]
    pub fn hsp(&mut self) -> HSP_W<DSI_LPCRrs> {
        HSP_W::new(self, 2)
    }
}
/**DSI Host LTDC polarity configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_lpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_LPCR)*/
pub struct DSI_LPCRrs;
impl crate::RegisterSpec for DSI_LPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_lpcr::R`](R) reader structure
impl crate::Readable for DSI_LPCRrs {}
///`write(|w| ..)` method takes [`dsi_lpcr::W`](W) writer structure
impl crate::Writable for DSI_LPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_LPCR to value 0
impl crate::Resettable for DSI_LPCRrs {
    const RESET_VALUE: u32 = 0;
}
