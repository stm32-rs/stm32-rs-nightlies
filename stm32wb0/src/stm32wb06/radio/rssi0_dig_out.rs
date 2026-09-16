///Register `RSSI0_DIG_OUT` reader
pub type R = crate::R<RSSI0_DIG_OUTrs>;
///Field `RSSI_MEAS_OUT_7_0` reader - Measure of the received signal strength.
pub type RSSI_MEAS_OUT_7_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Measure of the received signal strength.
    #[inline(always)]
    pub fn rssi_meas_out_7_0(&self) -> RSSI_MEAS_OUT_7_0_R {
        RSSI_MEAS_OUT_7_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSSI0_DIG_OUT")
            .field("rssi_meas_out_7_0", &self.rssi_meas_out_7_0())
            .finish()
    }
}
/**RSSI0_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`rssi0_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:RSSI0_DIG_OUT)*/
pub struct RSSI0_DIG_OUTrs;
impl crate::RegisterSpec for RSSI0_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`rssi0_dig_out::R`](R) reader structure
impl crate::Readable for RSSI0_DIG_OUTrs {}
///`reset()` method sets RSSI0_DIG_OUT to value 0x08
impl crate::Resettable for RSSI0_DIG_OUTrs {
    const RESET_VALUE: u32 = 0x08;
}
