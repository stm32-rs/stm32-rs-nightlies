///Register `AHB4SMENR` reader
pub type R = crate::R<AHB4SMENRrs>;
///Register `AHB4SMENR` writer
pub type W = crate::W<AHB4SMENRrs>;
///Field `PWRSMEN` reader - PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4SMEN` reader - ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_R = crate::BitReader;
///Field `ADC4SMEN` writer - ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&self) -> ADC4SMEN_R {
        ADC4SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4SMENR")
            .field("pwrsmen", &self.pwrsmen())
            .field("adc4smen", &self.adc4smen())
            .finish()
    }
}
impl W {
    ///Bit 2 - PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, AHB4SMENRrs> {
        PWRSMEN_W::new(self, 2)
    }
    ///Bit 5 - ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&mut self) -> ADC4SMEN_W<'_, AHB4SMENRrs> {
        ADC4SMEN_W::new(self, 5)
    }
}
/**RCC AHB4 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb4smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB4SMENR)*/
pub struct AHB4SMENRrs;
impl crate::RegisterSpec for AHB4SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4smenr::R`](R) reader structure
impl crate::Readable for AHB4SMENRrs {}
///`write(|w| ..)` method takes [`ahb4smenr::W`](W) writer structure
impl crate::Writable for AHB4SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4SMENR to value 0xffff_ffff
impl crate::Resettable for AHB4SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
