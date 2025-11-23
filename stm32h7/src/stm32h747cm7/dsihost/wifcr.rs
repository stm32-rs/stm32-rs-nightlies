///Register `WIFCR` writer
pub type W = crate::W<WIFCRrs>;
///Field `CTEIF` writer - Clear tearing effect interrupt flag
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERIF` writer - Clear end of refresh interrupt flag
pub type CERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPLLLIF` writer - Clear PLL lock interrupt flag
pub type CPLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPLLUIF` writer - Clear PLL unlock interrupt flag
pub type CPLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRRIF` writer - Clear regulator ready interrupt flag
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<WIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear tearing effect interrupt flag
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<'_, WIFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - Clear end of refresh interrupt flag
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W<'_, WIFCRrs> {
        CERIF_W::new(self, 1)
    }
    ///Bit 9 - Clear PLL lock interrupt flag
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W<'_, WIFCRrs> {
        CPLLLIF_W::new(self, 9)
    }
    ///Bit 10 - Clear PLL unlock interrupt flag
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W<'_, WIFCRrs> {
        CPLLUIF_W::new(self, 10)
    }
    ///Bit 13 - Clear regulator ready interrupt flag
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<'_, WIFCRrs> {
        CRRIF_W::new(self, 13)
    }
}
/**DSI wrapper interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DSIHOST:WIFCR)*/
pub struct WIFCRrs;
impl crate::RegisterSpec for WIFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wifcr::W`](W) writer structure
impl crate::Writable for WIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WIFCR to value 0
impl crate::Resettable for WIFCRrs {}
