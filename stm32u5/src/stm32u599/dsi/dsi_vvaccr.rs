///Register `DSI_VVACCR` reader
pub type R = crate::R<DSI_VVACCRrs>;
///Field `VA` reader - Vertical active duration This field returns the current vertical active period measured in number of horizontal lines.
pub type VA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Vertical active duration This field returns the current vertical active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVACCR")
            .field("va", &self.va())
            .finish()
    }
}
/**DSI Host video VA current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VVACCR)*/
pub struct DSI_VVACCRrs;
impl crate::RegisterSpec for DSI_VVACCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvaccr::R`](R) reader structure
impl crate::Readable for DSI_VVACCRrs {}
///`reset()` method sets DSI_VVACCR to value 0
impl crate::Resettable for DSI_VVACCRrs {
    const RESET_VALUE: u32 = 0;
}
