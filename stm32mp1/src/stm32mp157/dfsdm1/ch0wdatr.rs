///Register `CH0WDATR` reader
pub type R = crate::R<CH0WDATRrs>;
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
        f.debug_struct("CH0WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
/**This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0wdatr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0WDATR)*/
pub struct CH0WDATRrs;
impl crate::RegisterSpec for CH0WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch0wdatr::R`](R) reader structure
impl crate::Readable for CH0WDATRrs {}
///`reset()` method sets CH0WDATR to value 0
impl crate::Resettable for CH0WDATRrs {}
