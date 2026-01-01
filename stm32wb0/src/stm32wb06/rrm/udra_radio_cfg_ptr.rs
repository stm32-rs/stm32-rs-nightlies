///Register `UDRA_RADIO_CFG_PTR` reader
pub type R = crate::R<UDRA_RADIO_CFG_PTRrs>;
///Field `RADIO_CONFIG_ADDRESS` reader - UDRA radio configuration address.
pub type RADIO_CONFIG_ADDRESS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - UDRA radio configuration address.
    #[inline(always)]
    pub fn radio_config_address(&self) -> RADIO_CONFIG_ADDRESS_R {
        RADIO_CONFIG_ADDRESS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDRA_RADIO_CFG_PTR")
            .field("radio_config_address", &self.radio_config_address())
            .finish()
    }
}
/**UDRA_RADIO_CFG_PTR register

You can [`read`](crate::Reg::read) this register and get [`udra_radio_cfg_ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RRM:UDRA_RADIO_CFG_PTR)*/
pub struct UDRA_RADIO_CFG_PTRrs;
impl crate::RegisterSpec for UDRA_RADIO_CFG_PTRrs {
    type Ux = u32;
}
///`read()` method returns [`udra_radio_cfg_ptr::R`](R) reader structure
impl crate::Readable for UDRA_RADIO_CFG_PTRrs {}
///`reset()` method sets UDRA_RADIO_CFG_PTR to value 0
impl crate::Resettable for UDRA_RADIO_CFG_PTRrs {}
