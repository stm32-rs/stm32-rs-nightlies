///Register `DCACHE_FCR` writer
pub type W = crate::W<DCACHE_FCRrs>;
///Field `CBSYENDF` writer - CBSYENDF
pub type CBSYENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERRF` writer - CERRF
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMDENDF` writer - CCMDENDF
pub type CCMDENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DCACHE_FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - CBSYENDF
    #[inline(always)]
    #[must_use]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W<DCACHE_FCRrs> {
        CBSYENDF_W::new(self, 1)
    }
    ///Bit 2 - CERRF
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<DCACHE_FCRrs> {
        CERRF_W::new(self, 2)
    }
    ///Bit 4 - CCMDENDF
    #[inline(always)]
    #[must_use]
    pub fn ccmdendf(&mut self) -> CCMDENDF_W<DCACHE_FCRrs> {
        CCMDENDF_W::new(self, 4)
    }
}
/**DCACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DCACHE:DCACHE_FCR)*/
pub struct DCACHE_FCRrs;
impl crate::RegisterSpec for DCACHE_FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dcache_fcr::W`](W) writer structure
impl crate::Writable for DCACHE_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_FCR to value 0
impl crate::Resettable for DCACHE_FCRrs {
    const RESET_VALUE: u32 = 0;
}
