///Register `APB4HENSR` writer
pub type W = crate::W<APB4HENSRrs>;
///Field `SYSCFGENS` writer - SYSCFG enable
pub type SYSCFGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECENS` writer - BSEC enable
pub type BSECENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSENS` writer - DTS enable
pub type DTSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMENS` writer - BUSPERFM enable
pub type BUSPERFMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG enable
    #[inline(always)]
    pub fn syscfgens(&mut self) -> SYSCFGENS_W<'_, APB4HENSRrs> {
        SYSCFGENS_W::new(self, 0)
    }
    ///Bit 1 - BSEC enable
    #[inline(always)]
    pub fn bsecens(&mut self) -> BSECENS_W<'_, APB4HENSRrs> {
        BSECENS_W::new(self, 1)
    }
    ///Bit 2 - DTS enable
    #[inline(always)]
    pub fn dtsens(&mut self) -> DTSENS_W<'_, APB4HENSRrs> {
        DTSENS_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM enable
    #[inline(always)]
    pub fn busperfmens(&mut self) -> BUSPERFMENS_W<'_, APB4HENSRrs> {
        BUSPERFMENS_W::new(self, 4)
    }
}
/**RCC APB4H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HENSR)*/
pub struct APB4HENSRrs;
impl crate::RegisterSpec for APB4HENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hensr::W`](W) writer structure
impl crate::Writable for APB4HENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HENSR to value 0
impl crate::Resettable for APB4HENSRrs {}
