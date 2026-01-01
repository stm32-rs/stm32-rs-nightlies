///Register `AHB5ENR` reader
pub type R = crate::R<AHB5ENRrs>;
///Register `AHB5ENR` writer
pub type W = crate::W<AHB5ENRrs>;
///Field `RADIOEN` reader - 2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).
pub type RADIOEN_R = crate::BitReader;
///Field `RADIOEN` writer - 2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).
pub type RADIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).
    #[inline(always)]
    pub fn radioen(&self) -> RADIOEN_R {
        RADIOEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5ENR")
            .field("radioen", &self.radioen())
            .finish()
    }
}
impl W {
    ///Bit 0 - 2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).
    #[inline(always)]
    pub fn radioen(&mut self) -> RADIOEN_W<'_, AHB5ENRrs> {
        RADIOEN_W::new(self, 0)
    }
}
/**RCC AHB5 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB5ENR)*/
pub struct AHB5ENRrs;
impl crate::RegisterSpec for AHB5ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5enr::R`](R) reader structure
impl crate::Readable for AHB5ENRrs {}
///`write(|w| ..)` method takes [`ahb5enr::W`](W) writer structure
impl crate::Writable for AHB5ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5ENR to value 0
impl crate::Resettable for AHB5ENRrs {}
