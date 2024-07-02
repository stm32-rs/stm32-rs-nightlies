///Register `DSI_VCCR` reader
pub type R = crate::R<DSI_VCCRrs>;
///Register `DSI_VCCR` writer
pub type W = crate::W<DSI_VCCRrs>;
///Field `NUMC` reader - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
pub type NUMC_R = crate::FieldReader<u16>;
///Field `NUMC` writer - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
pub type NUMC_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VCCR")
            .field("numc", &self.numc())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line.
    #[inline(always)]
    #[must_use]
    pub fn numc(&mut self) -> NUMC_W<DSI_VCCRrs> {
        NUMC_W::new(self, 0)
    }
}
/**DSI Host video chunks configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VCCR)*/
pub struct DSI_VCCRrs;
impl crate::RegisterSpec for DSI_VCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vccr::R`](R) reader structure
impl crate::Readable for DSI_VCCRrs {}
///`write(|w| ..)` method takes [`dsi_vccr::W`](W) writer structure
impl crate::Writable for DSI_VCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VCCR to value 0
impl crate::Resettable for DSI_VCCRrs {
    const RESET_VALUE: u32 = 0;
}
