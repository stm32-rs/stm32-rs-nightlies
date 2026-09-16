///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
///Field `PWREN` reader - PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4EN` reader - ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type ADC4EN_R = crate::BitReader;
///Field `ADC4EN` writer - ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type ADC4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn adc4en(&self) -> ADC4EN_R {
        ADC4EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4ENR")
            .field("pwren", &self.pwren())
            .field("adc4en", &self.adc4en())
            .finish()
    }
}
impl W {
    ///Bit 2 - PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, AHB4ENRrs> {
        PWREN_W::new(self, 2)
    }
    ///Bit 5 - ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn adc4en(&mut self) -> ADC4EN_W<'_, AHB4ENRrs> {
        ADC4EN_W::new(self, 5)
    }
}
/**RCC AHB4 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:AHB4ENR)*/
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4enr::R`](R) reader structure
impl crate::Readable for AHB4ENRrs {}
///`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENR to value 0
impl crate::Resettable for AHB4ENRrs {}
