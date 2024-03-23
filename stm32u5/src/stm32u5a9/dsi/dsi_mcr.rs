#[doc = "Register `DSI_MCR` reader"]
pub type R = crate::R<DSI_MCRrs>;
#[doc = "Register `DSI_MCR` writer"]
pub type W = crate::W<DSI_MCRrs>;
#[doc = "Field `CMDM` reader - Command mode This bit configures the DSI Host in either video or command mode."]
pub type CMDM_R = crate::BitReader;
#[doc = "Field `CMDM` writer - Command mode This bit configures the DSI Host in either video or command mode."]
pub type CMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command mode This bit configures the DSI Host in either video or command mode."]
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command mode This bit configures the DSI Host in either video or command mode."]
    #[inline(always)]
    #[must_use]
    pub fn cmdm(&mut self) -> CMDM_W<DSI_MCRrs> {
        CMDM_W::new(self, 0)
    }
}
#[doc = "DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_MCRrs;
impl crate::RegisterSpec for DSI_MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_mcr::R`](R) reader structure"]
impl crate::Readable for DSI_MCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_mcr::W`](W) writer structure"]
impl crate::Writable for DSI_MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_MCR to value 0x01"]
impl crate::Resettable for DSI_MCRrs {
    const RESET_VALUE: u32 = 0x01;
}
