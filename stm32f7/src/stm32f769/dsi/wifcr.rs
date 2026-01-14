///Register `WIFCR` reader
pub type R = crate::R<WIFCRrs>;
///Register `WIFCR` writer
pub type W = crate::W<WIFCRrs>;
///Field `CTEIF` reader - Clear Tearing Effect Interrupt Flag
pub type CTEIF_R = crate::BitReader;
///Field `CTEIF` writer - Clear Tearing Effect Interrupt Flag
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERIF` reader - Clear End of Refresh Interrupt Flag
pub type CERIF_R = crate::BitReader;
///Field `CERIF` writer - Clear End of Refresh Interrupt Flag
pub type CERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPLLLIF` reader - Clear PLL Lock Interrupt Flag
pub type CPLLLIF_R = crate::BitReader;
///Field `CPLLLIF` writer - Clear PLL Lock Interrupt Flag
pub type CPLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPLLUIF` reader - Clear PLL Unlock Interrupt Flag
pub type CPLLUIF_R = crate::BitReader;
///Field `CPLLUIF` writer - Clear PLL Unlock Interrupt Flag
pub type CPLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRRIF` reader - Clear Regulator Ready Interrupt Flag
pub type CRRIF_R = crate::BitReader;
///Field `CRRIF` writer - Clear Regulator Ready Interrupt Flag
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear Tearing Effect Interrupt Flag
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear End of Refresh Interrupt Flag
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Clear PLL Lock Interrupt Flag
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clear PLL Unlock Interrupt Flag
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Clear Regulator Ready Interrupt Flag
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFCR")
            .field("crrif", &self.crrif())
            .field("cplluif", &self.cplluif())
            .field("cplllif", &self.cplllif())
            .field("cerif", &self.cerif())
            .field("cteif", &self.cteif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Tearing Effect Interrupt Flag
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<'_, WIFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - Clear End of Refresh Interrupt Flag
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W<'_, WIFCRrs> {
        CERIF_W::new(self, 1)
    }
    ///Bit 9 - Clear PLL Lock Interrupt Flag
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W<'_, WIFCRrs> {
        CPLLLIF_W::new(self, 9)
    }
    ///Bit 10 - Clear PLL Unlock Interrupt Flag
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W<'_, WIFCRrs> {
        CPLLUIF_W::new(self, 10)
    }
    ///Bit 13 - Clear Regulator Ready Interrupt Flag
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<'_, WIFCRrs> {
        CRRIF_W::new(self, 13)
    }
}
/**DSI Wrapper Interrupt Flag Clear Register

You can [`read`](crate::Reg::read) this register and get [`wifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WIFCR)*/
pub struct WIFCRrs;
impl crate::RegisterSpec for WIFCRrs {
    type Ux = u32;
}
///`read()` method returns [`wifcr::R`](R) reader structure
impl crate::Readable for WIFCRrs {}
///`write(|w| ..)` method takes [`wifcr::W`](W) writer structure
impl crate::Writable for WIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WIFCR to value 0
impl crate::Resettable for WIFCRrs {}
