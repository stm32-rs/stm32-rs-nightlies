///Register `APB4HRSTCR` writer
pub type W = crate::W<APB4HRSTCRrs>;
///Field `SYSCFGRSTC` writer - SYSCFG reset
pub type SYSCFGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSRSTC` writer - DTS reset
pub type DTSRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMRSTC` writer - BUSPERFM reset
pub type BUSPERFMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG reset
    #[inline(always)]
    pub fn syscfgrstc(&mut self) -> SYSCFGRSTC_W<'_, APB4HRSTCRrs> {
        SYSCFGRSTC_W::new(self, 0)
    }
    ///Bit 2 - DTS reset
    #[inline(always)]
    pub fn dtsrstc(&mut self) -> DTSRSTC_W<'_, APB4HRSTCRrs> {
        DTSRSTC_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM reset
    #[inline(always)]
    pub fn busperfmrstc(&mut self) -> BUSPERFMRSTC_W<'_, APB4HRSTCRrs> {
        BUSPERFMRSTC_W::new(self, 4)
    }
}
/**RCC APB4H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB4HRSTCR)*/
pub struct APB4HRSTCRrs;
impl crate::RegisterSpec for APB4HRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hrstcr::W`](W) writer structure
impl crate::Writable for APB4HRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HRSTCR to value 0
impl crate::Resettable for APB4HRSTCRrs {}
