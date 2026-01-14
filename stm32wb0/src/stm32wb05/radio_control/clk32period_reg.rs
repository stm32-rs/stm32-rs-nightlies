///Register `CLK32PERIOD_REG` reader
pub type R = crate::R<CLK32PERIOD_REGrs>;
///Field `SLOW_PERIOD` reader - indicates slow clock period information.
pub type SLOW_PERIOD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:18 - indicates slow clock period information.
    #[inline(always)]
    pub fn slow_period(&self) -> SLOW_PERIOD_R {
        SLOW_PERIOD_R::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32PERIOD_REG")
            .field("slow_period", &self.slow_period())
            .finish()
    }
}
/**CLK32PERIOD_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32period_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO_CONTROL:CLK32PERIOD_REG)*/
pub struct CLK32PERIOD_REGrs;
impl crate::RegisterSpec for CLK32PERIOD_REGrs {
    type Ux = u32;
}
///`read()` method returns [`clk32period_reg::R`](R) reader structure
impl crate::Readable for CLK32PERIOD_REGrs {}
///`reset()` method sets CLK32PERIOD_REG to value 0
impl crate::Resettable for CLK32PERIOD_REGrs {}
