///Register `AHB5SMENR` reader
pub type R = crate::R<AHB5SMENRrs>;
///Register `AHB5SMENR` writer
pub type W = crate::W<AHB5SMENRrs>;
///Field `RADIOSMEN` reader - 2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIOSMEN_R = crate::BitReader;
///Field `RADIOSMEN` writer - 2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIOSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiosmen(&self) -> RADIOSMEN_R {
        RADIOSMEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5SMENR")
            .field("radiosmen", &self.radiosmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - 2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiosmen(&mut self) -> RADIOSMEN_W<'_, AHB5SMENRrs> {
        RADIOSMEN_W::new(self, 0)
    }
}
/**RCC AHB5 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb5smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:AHB5SMENR)*/
pub struct AHB5SMENRrs;
impl crate::RegisterSpec for AHB5SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5smenr::R`](R) reader structure
impl crate::Readable for AHB5SMENRrs {}
///`write(|w| ..)` method takes [`ahb5smenr::W`](W) writer structure
impl crate::Writable for AHB5SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5SMENR to value 0xffff_ffff
impl crate::Resettable for AHB5SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
