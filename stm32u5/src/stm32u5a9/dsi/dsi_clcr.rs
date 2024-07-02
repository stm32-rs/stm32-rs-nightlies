///Register `DSI_CLCR` reader
pub type R = crate::R<DSI_CLCRrs>;
///Register `DSI_CLCR` writer
pub type W = crate::W<DSI_CLCRrs>;
///Field `DPCC` reader - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_R = crate::BitReader;
///Field `DPCC` writer - D-PHY clock control This bit controls the D-PHY clock state:
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACR` reader - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_R = crate::BitReader;
///Field `ACR` writer - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_CLCR")
            .field("dpcc", &self.dpcc())
            .field("acr", &self.acr())
            .finish()
    }
}
impl W {
    ///Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:
    #[inline(always)]
    #[must_use]
    pub fn dpcc(&mut self) -> DPCC_W<DSI_CLCRrs> {
        DPCC_W::new(self, 0)
    }
    ///Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows.
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> ACR_W<DSI_CLCRrs> {
        ACR_W::new(self, 1)
    }
}
/**DSI Host clock lane configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_CLCR)*/
pub struct DSI_CLCRrs;
impl crate::RegisterSpec for DSI_CLCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_clcr::R`](R) reader structure
impl crate::Readable for DSI_CLCRrs {}
///`write(|w| ..)` method takes [`dsi_clcr::W`](W) writer structure
impl crate::Writable for DSI_CLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_CLCR to value 0
impl crate::Resettable for DSI_CLCRrs {
    const RESET_VALUE: u32 = 0;
}
