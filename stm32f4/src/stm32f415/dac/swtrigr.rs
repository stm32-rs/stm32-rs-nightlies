///Register `SWTRIGR` writer
pub type W = crate::W<SWTRIGRrs>;
///Field `SWTRIG1` writer - DAC channel1 software trigger
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG2` writer - DAC channel2 software trigger
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SWTRIGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<'_, SWTRIGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<'_, SWTRIGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
/**software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#DAC:SWTRIGR)*/
pub struct SWTRIGRrs;
impl crate::RegisterSpec for SWTRIGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure
impl crate::Writable for SWTRIGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWTRIGR to value 0
impl crate::Resettable for SWTRIGRrs {}
