///Register `CONFR0` writer
pub type W = crate::W<CONFR0rs>;
///Field `START` writer - Start This bit start or stop the encoding or decoding process. Note: Reads always return 0.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CONFR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Start This bit start or stop the encoding or decoding process. Note: Reads always return 0.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CONFR0rs> {
        START_W::new(self, 0)
    }
}
/**JPEG codec control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#JPEG:CONFR0)*/
pub struct CONFR0rs;
impl crate::RegisterSpec for CONFR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`confr0::W`](W) writer structure
impl crate::Writable for CONFR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR0 to value 0
impl crate::Resettable for CONFR0rs {}
