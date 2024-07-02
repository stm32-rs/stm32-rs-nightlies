///Register `DSI_DPDL0SRCR` reader
pub type R = crate::R<DSI_DPDL0SRCRrs>;
///Register `DSI_DPDL0SRCR` writer
pub type W = crate::W<DSI_DPDL0SRCRrs>;
///Field `SRC` reader - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
pub type SRC_R = crate::FieldReader;
///Field `SRC` writer - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_DPDL0SRCR")
            .field("src", &self.src())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<DSI_DPDL0SRCRrs> {
        SRC_W::new(self, 0)
    }
}
/**DSI D-PHY data lane 0 skew rate control register

You can [`read`](crate::Reg::read) this register and get [`dsi_dpdl0srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dpdl0srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_DPDL0SRCR)*/
pub struct DSI_DPDL0SRCRrs;
impl crate::RegisterSpec for DSI_DPDL0SRCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_dpdl0srcr::R`](R) reader structure
impl crate::Readable for DSI_DPDL0SRCRrs {}
///`write(|w| ..)` method takes [`dsi_dpdl0srcr::W`](W) writer structure
impl crate::Writable for DSI_DPDL0SRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_DPDL0SRCR to value 0
impl crate::Resettable for DSI_DPDL0SRCRrs {
    const RESET_VALUE: u32 = 0;
}
