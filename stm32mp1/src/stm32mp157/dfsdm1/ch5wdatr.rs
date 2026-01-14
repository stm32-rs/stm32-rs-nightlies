///Register `CH5WDATR` reader
pub type R = crate::R<CH5WDATRrs>;
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
        f.debug_struct("CH5WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
/**This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch5wdatr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5WDATR)*/
pub struct CH5WDATRrs;
impl crate::RegisterSpec for CH5WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch5wdatr::R`](R) reader structure
impl crate::Readable for CH5WDATRrs {}
///`reset()` method sets CH5WDATR to value 0
impl crate::Resettable for CH5WDATRrs {}
