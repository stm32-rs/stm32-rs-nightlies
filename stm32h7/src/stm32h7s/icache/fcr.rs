///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `CBSYENDF` writer - clear busy end flag Set by software.
pub type CBSYENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERRF` writer - clear cache error flag Set by software.
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - clear busy end flag Set by software.
    #[inline(always)]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W<FCRrs> {
        CBSYENDF_W::new(self, 1)
    }
    ///Bit 2 - clear cache error flag Set by software.
    #[inline(always)]
    pub fn cerrf(&mut self) -> CERRF_W<FCRrs> {
        CERRF_W::new(self, 2)
    }
}
/**ICACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ICACHE:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
