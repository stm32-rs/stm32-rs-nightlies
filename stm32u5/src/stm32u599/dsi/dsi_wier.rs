#[doc = "Register `DSI_WIER` reader"]
pub type R = crate::R<DSI_WIERrs>;
#[doc = "Register `DSI_WIER` writer"]
pub type W = crate::W<DSI_WIERrs>;
#[doc = "Field `TEIE` reader - Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - End of refresh interrupt enable This bit enables the end of refresh interrupt."]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - End of refresh interrupt enable This bit enables the end of refresh interrupt."]
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLIE` reader - PLL lock interrupt enable This bit enables the PLL lock interrupt."]
pub type PLLLIE_R = crate::BitReader;
#[doc = "Field `PLLLIE` writer - PLL lock interrupt enable This bit enables the PLL lock interrupt."]
pub type PLLLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUIE` reader - PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
pub type PLLUIE_R = crate::BitReader;
#[doc = "Field `PLLUIE` writer - PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
pub type PLLUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt enable This bit enables the end of refresh interrupt."]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt enable This bit enables the PLL lock interrupt."]
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<DSI_WIERrs> {
        TEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt enable This bit enables the end of refresh interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<DSI_WIERrs> {
        ERIE_W::new(self, 1)
    }
    #[doc = "Bit 9 - PLL lock interrupt enable This bit enables the PLL lock interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn plllie(&mut self) -> PLLLIE_W<DSI_WIERrs> {
        PLLLIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn plluie(&mut self) -> PLLUIE_W<DSI_WIERrs> {
        PLLUIE_W::new(self, 10)
    }
}
#[doc = "DSI Wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WIERrs;
impl crate::RegisterSpec for DSI_WIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wier::R`](R) reader structure"]
impl crate::Readable for DSI_WIERrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_wier::W`](W) writer structure"]
impl crate::Writable for DSI_WIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_WIER to value 0"]
impl crate::Resettable for DSI_WIERrs {
    const RESET_VALUE: u32 = 0;
}
