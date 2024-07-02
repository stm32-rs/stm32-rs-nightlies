///Register `DSI_LCCR` reader
pub type R = crate::R<DSI_LCCRrs>;
///Register `DSI_LCCR` writer
pub type W = crate::W<DSI_LCCRrs>;
///Field `CMDSIZE` reader - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
pub type CMDSIZE_R = crate::FieldReader<u16>;
///Field `CMDSIZE` writer - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
pub type CMDSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_LCCR")
            .field("cmdsize", &self.cmdsize())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled.
    #[inline(always)]
    #[must_use]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<DSI_LCCRrs> {
        CMDSIZE_W::new(self, 0)
    }
}
/**DSI Host LTDC command configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_lccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_LCCR)*/
pub struct DSI_LCCRrs;
impl crate::RegisterSpec for DSI_LCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_lccr::R`](R) reader structure
impl crate::Readable for DSI_LCCRrs {}
///`write(|w| ..)` method takes [`dsi_lccr::W`](W) writer structure
impl crate::Writable for DSI_LCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_LCCR to value 0
impl crate::Resettable for DSI_LCCRrs {
    const RESET_VALUE: u32 = 0;
}
