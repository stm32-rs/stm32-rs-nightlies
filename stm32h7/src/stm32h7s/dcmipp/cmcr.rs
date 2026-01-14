///Register `CMCR` writer
pub type W = crate::W<CMCRrs>;
///Field `CFC` writer - Clear frame counter When this bit is set, the frame counter associated to a pipe is cleared. It resets DCMIPP_CMFRCR register. This bit is always read at 0.
pub type CFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - Clear frame counter When this bit is set, the frame counter associated to a pipe is cleared. It resets DCMIPP_CMFRCR register. This bit is always read at 0.
    #[inline(always)]
    pub fn cfc(&mut self) -> CFC_W<'_, CMCRrs> {
        CFC_W::new(self, 4)
    }
}
/**DCMIPP common configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:CMCR)*/
pub struct CMCRrs;
impl crate::RegisterSpec for CMCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmcr::W`](W) writer structure
impl crate::Writable for CMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMCR to value 0
impl crate::Resettable for CMCRrs {}
