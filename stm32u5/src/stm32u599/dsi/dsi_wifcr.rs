#[doc = "Register `DSI_WIFCR` writer"]
pub type W = crate::W<DSI_WIFCRrs>;
#[doc = "Field `CTEIF` writer - Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the DSI_WSR register."]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERIF` writer - Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the DSI_WSR register."]
pub type CERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLLIF` writer - Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the DSI_WSR register."]
pub type CPLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLUIF` writer - Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the DSI_WSR register."]
pub type CPLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the DSI_WSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<DSI_WIFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the DSI_WSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<DSI_WIFCRrs> {
        CERIF_W::new(self, 1)
    }
    #[doc = "Bit 9 - Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the DSI_WSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<DSI_WIFCRrs> {
        CPLLLIF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the DSI_WSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<DSI_WIFCRrs> {
        CPLLUIF_W::new(self, 10)
    }
}
#[doc = "DSI Wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WIFCRrs;
impl crate::RegisterSpec for DSI_WIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_wifcr::W`](W) writer structure"]
impl crate::Writable for DSI_WIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_WIFCR to value 0"]
impl crate::Resettable for DSI_WIFCRrs {
    const RESET_VALUE: u32 = 0;
}
