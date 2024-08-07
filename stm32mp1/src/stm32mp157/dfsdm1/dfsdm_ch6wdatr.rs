///Register `DFSDM_CH6WDATR` reader
pub type R = crate::R<DFSDM_CH6WDATRrs>;
///Field `WDATA` reader - WDATA
pub type WDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_CH6WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
/**This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6wdatr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:DFSDM_CH6WDATR)*/
pub struct DFSDM_CH6WDATRrs;
impl crate::RegisterSpec for DFSDM_CH6WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ch6wdatr::R`](R) reader structure
impl crate::Readable for DFSDM_CH6WDATRrs {}
///`reset()` method sets DFSDM_CH6WDATR to value 0
impl crate::Resettable for DFSDM_CH6WDATRrs {
    const RESET_VALUE: u32 = 0;
}
