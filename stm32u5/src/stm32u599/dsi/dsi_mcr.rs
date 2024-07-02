///Register `DSI_MCR` reader
pub type R = crate::R<DSI_MCRrs>;
///Register `DSI_MCR` writer
pub type W = crate::W<DSI_MCRrs>;
///Field `CMDM` reader - Command mode This bit configures the DSI Host in either video or command mode.
pub type CMDM_R = crate::BitReader;
///Field `CMDM` writer - Command mode This bit configures the DSI Host in either video or command mode.
pub type CMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Command mode This bit configures the DSI Host in either video or command mode.
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_MCR")
            .field("cmdm", &self.cmdm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command mode This bit configures the DSI Host in either video or command mode.
    #[inline(always)]
    #[must_use]
    pub fn cmdm(&mut self) -> CMDM_W<DSI_MCRrs> {
        CMDM_W::new(self, 0)
    }
}
/**DSI Host mode configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_MCR)*/
pub struct DSI_MCRrs;
impl crate::RegisterSpec for DSI_MCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_mcr::R`](R) reader structure
impl crate::Readable for DSI_MCRrs {}
///`write(|w| ..)` method takes [`dsi_mcr::W`](W) writer structure
impl crate::Writable for DSI_MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_MCR to value 0x01
impl crate::Resettable for DSI_MCRrs {
    const RESET_VALUE: u32 = 0x01;
}
