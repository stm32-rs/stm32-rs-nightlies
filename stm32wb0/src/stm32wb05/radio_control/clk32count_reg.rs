///Register `CLK32COUNT_REG` reader
pub type R = crate::R<CLK32COUNT_REGrs>;
///Register `CLK32COUNT_REG` writer
pub type W = crate::W<CLK32COUNT_REGrs>;
///Field `SLOW_COUNT` reader - program the window length (in slow clock period) for slow clock measurement.
pub type SLOW_COUNT_R = crate::FieldReader<u16>;
///Field `SLOW_COUNT` writer - program the window length (in slow clock period) for slow clock measurement.
pub type SLOW_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - program the window length (in slow clock period) for slow clock measurement.
    #[inline(always)]
    pub fn slow_count(&self) -> SLOW_COUNT_R {
        SLOW_COUNT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32COUNT_REG")
            .field("slow_count", &self.slow_count())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - program the window length (in slow clock period) for slow clock measurement.
    #[inline(always)]
    pub fn slow_count(&mut self) -> SLOW_COUNT_W<'_, CLK32COUNT_REGrs> {
        SLOW_COUNT_W::new(self, 0)
    }
}
/**CLK32COUNT_REG register

You can [`read`](crate::Reg::read) this register and get [`clk32count_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk32count_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO_CONTROL:CLK32COUNT_REG)*/
pub struct CLK32COUNT_REGrs;
impl crate::RegisterSpec for CLK32COUNT_REGrs {
    type Ux = u32;
}
///`read()` method returns [`clk32count_reg::R`](R) reader structure
impl crate::Readable for CLK32COUNT_REGrs {}
///`write(|w| ..)` method takes [`clk32count_reg::W`](W) writer structure
impl crate::Writable for CLK32COUNT_REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLK32COUNT_REG to value 0x17
impl crate::Resettable for CLK32COUNT_REGrs {
    const RESET_VALUE: u32 = 0x17;
}
