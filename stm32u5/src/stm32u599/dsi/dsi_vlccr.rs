///Register `DSI_VLCCR` reader
pub type R = crate::R<DSI_VLCCRrs>;
///Field `HLINE` reader - Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
pub type HLINE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles.
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VLCCR")
            .field("hline", &self.hline())
            .finish()
    }
}
/**DSI Host video line current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vlccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VLCCR)*/
pub struct DSI_VLCCRrs;
impl crate::RegisterSpec for DSI_VLCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vlccr::R`](R) reader structure
impl crate::Readable for DSI_VLCCRrs {}
///`reset()` method sets DSI_VLCCR to value 0
impl crate::Resettable for DSI_VLCCRrs {
    const RESET_VALUE: u32 = 0;
}
