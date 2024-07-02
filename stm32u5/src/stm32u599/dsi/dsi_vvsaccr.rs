///Register `DSI_VVSACCR` reader
pub type R = crate::R<DSI_VVSACCRrs>;
///Field `VSA` reader - Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines.
pub type VSA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVSACCR")
            .field("vsa", &self.vsa())
            .finish()
    }
}
/**DSI Host video VSA current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvsaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VVSACCR)*/
pub struct DSI_VVSACCRrs;
impl crate::RegisterSpec for DSI_VVSACCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvsaccr::R`](R) reader structure
impl crate::Readable for DSI_VVSACCRrs {}
///`reset()` method sets DSI_VVSACCR to value 0
impl crate::Resettable for DSI_VVSACCRrs {
    const RESET_VALUE: u32 = 0;
}
