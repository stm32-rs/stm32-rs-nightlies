#[doc = "Register `DSI_PCTLR` reader"]
pub type R = crate::R<DSI_PCTLRrs>;
#[doc = "Register `DSI_PCTLR` writer"]
pub type W = crate::W<DSI_PCTLRrs>;
#[doc = "Field `DEN` reader - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE` reader - Clock enable This bit enables the D-PHY clock lane module:"]
pub type CKE_R = crate::BitReader;
#[doc = "Field `CKE` writer - Clock enable This bit enables the D-PHY clock lane module:"]
pub type CKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock enable This bit enables the D-PHY clock lane module:"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<DSI_PCTLRrs> {
        DEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock enable This bit enables the D-PHY clock lane module:"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<DSI_PCTLRrs> {
        CKE_W::new(self, 2)
    }
}
#[doc = "DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCTLRrs;
impl crate::RegisterSpec for DSI_PCTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pctlr::R`](R) reader structure"]
impl crate::Readable for DSI_PCTLRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_pctlr::W`](W) writer structure"]
impl crate::Writable for DSI_PCTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_PCTLR to value 0"]
impl crate::Resettable for DSI_PCTLRrs {
    const RESET_VALUE: u32 = 0;
}
