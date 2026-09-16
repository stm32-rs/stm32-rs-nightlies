///Register `APB1ENR2` reader
pub type R = crate::R<APB1ENR2rs>;
///Register `APB1ENR2` writer
pub type W = crate::W<APB1ENR2rs>;
///Field `LPTIM2EN` reader - LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM2EN_R = crate::BitReader;
///Field `LPTIM2EN` writer - LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR2")
            .field("lptim2en", &self.lptim2en())
            .finish()
    }
}
impl W {
    ///Bit 5 - LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
}
/**RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:APB1ENR2)*/
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr2::R`](R) reader structure
impl crate::Readable for APB1ENR2rs {}
///`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR2 to value 0
impl crate::Resettable for APB1ENR2rs {}
