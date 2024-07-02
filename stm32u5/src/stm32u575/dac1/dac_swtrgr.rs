///Register `DAC_SWTRGR` writer
pub type W = crate::W<DAC_SWTRGRrs>;
///Field `SWTRIG1` writer - DAC channel1 software trigger
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG2` writer - DAC channel2 software trigger
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DAC_SWTRGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<DAC_SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<DAC_SWTRGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
/**DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DAC1:DAC_SWTRGR)*/
pub struct DAC_SWTRGRrs;
impl crate::RegisterSpec for DAC_SWTRGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dac_swtrgr::W`](W) writer structure
impl crate::Writable for DAC_SWTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_SWTRGR to value 0
impl crate::Resettable for DAC_SWTRGRrs {
    const RESET_VALUE: u32 = 0;
}
