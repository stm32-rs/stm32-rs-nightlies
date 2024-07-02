///Register `DSI_VVSACR` reader
pub type R = crate::R<DSI_VVSACRrs>;
///Register `DSI_VVSACR` writer
pub type W = crate::W<DSI_VVSACRrs>;
///Field `VSA` reader - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
pub type VSA_R = crate::FieldReader<u16>;
///Field `VSA` writer - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
pub type VSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVSACR")
            .field("vsa", &self.vsa())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<DSI_VVSACRrs> {
        VSA_W::new(self, 0)
    }
}
/**DSI Host video VSA configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VVSACR)*/
pub struct DSI_VVSACRrs;
impl crate::RegisterSpec for DSI_VVSACRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvsacr::R`](R) reader structure
impl crate::Readable for DSI_VVSACRrs {}
///`write(|w| ..)` method takes [`dsi_vvsacr::W`](W) writer structure
impl crate::Writable for DSI_VVSACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VVSACR to value 0
impl crate::Resettable for DSI_VVSACRrs {
    const RESET_VALUE: u32 = 0;
}
