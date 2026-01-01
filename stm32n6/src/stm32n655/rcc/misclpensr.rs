///Register `MISCLPENSR` writer
pub type W = crate::W<MISCLPENSRrs>;
///Field `DBGLPENS` writer - DBG sleep enable
pub type DBGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPLPENS` writer - XSPIPHYCOMP sleep enable
pub type XSPIPHYCOMPLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERLPENS` writer - PER sleep enable
pub type PERLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG sleep enable
    #[inline(always)]
    pub fn dbglpens(&mut self) -> DBGLPENS_W<'_, MISCLPENSRrs> {
        DBGLPENS_W::new(self, 0)
    }
    ///Bit 3 - XSPIPHYCOMP sleep enable
    #[inline(always)]
    pub fn xspiphycomplpens(&mut self) -> XSPIPHYCOMPLPENS_W<'_, MISCLPENSRrs> {
        XSPIPHYCOMPLPENS_W::new(self, 3)
    }
    ///Bit 6 - PER sleep enable
    #[inline(always)]
    pub fn perlpens(&mut self) -> PERLPENS_W<'_, MISCLPENSRrs> {
        PERLPENS_W::new(self, 6)
    }
}
/**RCC miscellaneous Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:MISCLPENSR)*/
pub struct MISCLPENSRrs;
impl crate::RegisterSpec for MISCLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`misclpensr::W`](W) writer structure
impl crate::Writable for MISCLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCLPENSR to value 0
impl crate::Resettable for MISCLPENSRrs {}
