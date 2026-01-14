///Register `AHB5RSTR` reader
pub type R = crate::R<AHB5RSTRrs>;
///Register `AHB5RSTR` writer
pub type W = crate::W<AHB5RSTRrs>;
///Field `RADIORST` reader - 2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIORST_R = crate::BitReader;
///Field `RADIORST` writer - 2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiorst(&self) -> RADIORST_R {
        RADIORST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5RSTR")
            .field("radiorst", &self.radiorst())
            .finish()
    }
}
impl W {
    ///Bit 0 - 2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiorst(&mut self) -> RADIORST_W<'_, AHB5RSTRrs> {
        RADIORST_W::new(self, 0)
    }
}
/**RCC AHB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RCC:AHB5RSTR)*/
pub struct AHB5RSTRrs;
impl crate::RegisterSpec for AHB5RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5rstr::R`](R) reader structure
impl crate::Readable for AHB5RSTRrs {}
///`write(|w| ..)` method takes [`ahb5rstr::W`](W) writer structure
impl crate::Writable for AHB5RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTR to value 0
impl crate::Resettable for AHB5RSTRrs {}
