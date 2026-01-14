///Register `CLK32FREQUENCY_REG` reader
pub type R = crate::R<CLK32FREQUENCY_REGrs>;
///Field `SLOW_FREQUENCY` reader - value equal to (2^39/ SLOW_PERIOD).
pub type SLOW_FREQUENCY_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:26 - value equal to (2^39/ SLOW_PERIOD).
    #[inline(always)]
    pub fn slow_frequency(&self) -> SLOW_FREQUENCY_R {
        SLOW_FREQUENCY_R::new(self.bits & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32FREQUENCY_REG")
            .field("slow_frequency", &self.slow_frequency())
            .finish()
    }
}
/**CLK32FREQUENCY_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32frequency_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO_CONTROL:CLK32FREQUENCY_REG)*/
pub struct CLK32FREQUENCY_REGrs;
impl crate::RegisterSpec for CLK32FREQUENCY_REGrs {
    type Ux = u32;
}
///`read()` method returns [`clk32frequency_reg::R`](R) reader structure
impl crate::Readable for CLK32FREQUENCY_REGrs {}
///`reset()` method sets CLK32FREQUENCY_REG to value 0
impl crate::Resettable for CLK32FREQUENCY_REGrs {}
