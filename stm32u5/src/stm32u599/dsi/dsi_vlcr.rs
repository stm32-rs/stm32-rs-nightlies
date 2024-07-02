///Register `DSI_VLCR` reader
pub type R = crate::R<DSI_VLCRrs>;
///Register `DSI_VLCR` writer
pub type W = crate::W<DSI_VLCRrs>;
///Field `HLINE` reader - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
pub type HLINE_R = crate::FieldReader<u16>;
///Field `HLINE` writer - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
pub type HLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VLCR")
            .field("hline", &self.hline())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn hline(&mut self) -> HLINE_W<DSI_VLCRrs> {
        HLINE_W::new(self, 0)
    }
}
/**DSI Host video line configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VLCR)*/
pub struct DSI_VLCRrs;
impl crate::RegisterSpec for DSI_VLCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vlcr::R`](R) reader structure
impl crate::Readable for DSI_VLCRrs {}
///`write(|w| ..)` method takes [`dsi_vlcr::W`](W) writer structure
impl crate::Writable for DSI_VLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VLCR to value 0
impl crate::Resettable for DSI_VLCRrs {
    const RESET_VALUE: u32 = 0;
}
