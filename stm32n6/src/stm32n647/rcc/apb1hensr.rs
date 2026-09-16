///Register `APB1HENSR` writer
pub type W = crate::W<APB1HENSRrs>;
///Field `MDIOSENS` writer - MDIOS enable
pub type MDIOSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANENS` writer - FDCAN enable
pub type FDCANENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1ENS` writer - UCPD1 enable
pub type UCPD1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS enable
    #[inline(always)]
    pub fn mdiosens(&mut self) -> MDIOSENS_W<'_, APB1HENSRrs> {
        MDIOSENS_W::new(self, 5)
    }
    ///Bit 8 - FDCAN enable
    #[inline(always)]
    pub fn fdcanens(&mut self) -> FDCANENS_W<'_, APB1HENSRrs> {
        FDCANENS_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 enable
    #[inline(always)]
    pub fn ucpd1ens(&mut self) -> UCPD1ENS_W<'_, APB1HENSRrs> {
        UCPD1ENS_W::new(self, 18)
    }
}
/**RCC APB1H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HENSR)*/
pub struct APB1HENSRrs;
impl crate::RegisterSpec for APB1HENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hensr::W`](W) writer structure
impl crate::Writable for APB1HENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HENSR to value 0
impl crate::Resettable for APB1HENSRrs {}
