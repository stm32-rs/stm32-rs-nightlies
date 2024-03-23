#[doc = "Register `DSI_CLCR` reader"]
pub type R = crate::R<DSI_CLCRrs>;
#[doc = "Register `DSI_CLCR` writer"]
pub type W = crate::W<DSI_CLCRrs>;
#[doc = "Field `DPCC` reader - D-PHY clock control This bit controls the D-PHY clock state:"]
pub type DPCC_R = crate::BitReader;
#[doc = "Field `DPCC` writer - D-PHY clock control This bit controls the D-PHY clock state:"]
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACR` reader - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
pub type ACR_R = crate::BitReader;
#[doc = "Field `ACR` writer - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:"]
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D-PHY clock control This bit controls the D-PHY clock state:"]
    #[inline(always)]
    #[must_use]
    pub fn dpcc(&mut self) -> DPCC_W<DSI_CLCRrs> {
        DPCC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> ACR_W<DSI_CLCRrs> {
        ACR_W::new(self, 1)
    }
}
#[doc = "DSI Host clock lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_clcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_clcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CLCRrs;
impl crate::RegisterSpec for DSI_CLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_clcr::R`](R) reader structure"]
impl crate::Readable for DSI_CLCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_clcr::W`](W) writer structure"]
impl crate::Writable for DSI_CLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_CLCR to value 0"]
impl crate::Resettable for DSI_CLCRrs {
    const RESET_VALUE: u32 = 0;
}
