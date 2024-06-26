///Register `DSI_DPCBCR` reader
pub type R = crate::R<DSI_DPCBCRrs>;
///Register `DSI_DPCBCR` writer
pub type W = crate::W<DSI_DPCBCRrs>;
///Field `BC` reader - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_R = crate::FieldReader;
///Field `BC` writer - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
pub type BC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 3:7 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_DPCBCR")
            .field("bc", &self.bc())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - Band control This field selects the frequency band used by the D-PHY. Others: Reserved
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<DSI_DPCBCRrs> {
        BC_W::new(self, 3)
    }
}
/**DSI D-PHY clock band control register

You can [`read`](crate::Reg::read) this register and get [`dsi_dpcbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dpcbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_DPCBCR)*/
pub struct DSI_DPCBCRrs;
impl crate::RegisterSpec for DSI_DPCBCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_dpcbcr::R`](R) reader structure
impl crate::Readable for DSI_DPCBCRrs {}
///`write(|w| ..)` method takes [`dsi_dpcbcr::W`](W) writer structure
impl crate::Writable for DSI_DPCBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_DPCBCR to value 0
impl crate::Resettable for DSI_DPCBCRrs {
    const RESET_VALUE: u32 = 0;
}
