///Register `SWTRGR` writer
pub type W = crate::W<SWTRGRrs>;
///Field `SWTRIG1` writer - SWTRIG1
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG2` writer - SWTRIG2
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SWTRGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SWTRIG1
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<'_, SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    ///Bit 1 - SWTRIG2
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<'_, SWTRGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
/**DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:SWTRGR)*/
pub struct SWTRGRrs;
impl crate::RegisterSpec for SWTRGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure
impl crate::Writable for SWTRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWTRGR to value 0
impl crate::Resettable for SWTRGRrs {}
