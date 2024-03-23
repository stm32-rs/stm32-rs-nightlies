#[doc = "Register `WDATR` reader"]
pub type R = crate::R<WDATRrs>;
#[doc = "Field `WDATA` reader - Input channel y watchdog data"]
pub type WDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input channel y watchdog data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdatr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATRrs;
impl crate::RegisterSpec for WDATRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdatr::R`](R) reader structure"]
impl crate::Readable for WDATRrs {}
#[doc = "`reset()` method sets WDATR to value 0"]
impl crate::Resettable for WDATRrs {
    const RESET_VALUE: u32 = 0;
}
