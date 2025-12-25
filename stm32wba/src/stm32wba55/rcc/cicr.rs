///Register `CICR` writer
pub type W = crate::W<CICRrs>;
///Field `LSI1RDYC` writer - LSI1 ready interrupt clear Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE32 ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDYC` writer - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSC` writer - High speed external clock security system interrupt clear Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI1 ready interrupt clear Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1rdyc(&mut self) -> LSI1RDYC_W<'_, CICRrs> {
        LSI1RDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 3 - HSI16 ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE32 ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 6 - PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<'_, CICRrs> {
        PLL1RDYC_W::new(self, 6)
    }
    ///Bit 10 - High speed external clock security system interrupt clear Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HSECSSC_W<'_, CICRrs> {
        HSECSSC_W::new(self, 10)
    }
}
/**RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {}
