///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `RNGSEL` reader - RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RNGSEL_R = crate::FieldReader;
///Field `RNGSEL` writer - RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 12:13 - RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("rngsel", &self.rngsel())
            .finish()
    }
}
impl W {
    ///Bits 12:13 - RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W<'_, CCIPR2rs> {
        RNGSEL_W::new(self, 12)
    }
}
/**RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
