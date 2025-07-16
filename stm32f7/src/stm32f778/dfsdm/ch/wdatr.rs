///Register `WDATR` reader
pub type R = crate::R<WDATRrs>;
///Field `WDATA` reader - Input channel y watchdog data
pub type WDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input channel y watchdog data
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
/**DFSDM channel watchdog filter data register

You can [`read`](crate::Reg::read) this register and get [`wdatr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDATRrs;
impl crate::RegisterSpec for WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`wdatr::R`](R) reader structure
impl crate::Readable for WDATRrs {}
///`reset()` method sets WDATR to value 0
impl crate::Resettable for WDATRrs {}
