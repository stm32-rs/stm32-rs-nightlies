///Register `DSI_VCCCR` reader
pub type R = crate::R<DSI_VCCCRrs>;
///Field `NUMC` reader - Number of chunks This field returns the number of chunks being transmitted during a line period.
pub type NUMC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - Number of chunks This field returns the number of chunks being transmitted during a line period.
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VCCCR")
            .field("numc", &self.numc())
            .finish()
    }
}
/**DSI Host video chunks current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vcccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VCCCR)*/
pub struct DSI_VCCCRrs;
impl crate::RegisterSpec for DSI_VCCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vcccr::R`](R) reader structure
impl crate::Readable for DSI_VCCCRrs {}
///`reset()` method sets DSI_VCCCR to value 0
impl crate::Resettable for DSI_VCCCRrs {
    const RESET_VALUE: u32 = 0;
}
