///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
///Field `LSI1RDYIE` reader - LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1RDYIE_R = crate::BitReader;
///Field `LSI1RDYIE` writer - LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE32 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE32 oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE32 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE32 oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDYIE` reader - PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1RDYIE_R = crate::BitReader;
///Field `PLL1RDYIE` writer - PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1rdyie(&self) -> LSI1RDYIE_R {
        LSI1RDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE32 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE32 oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsi1rdyie", &self.lsi1rdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pll1rdyie", &self.pll1rdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1rdyie(&mut self) -> LSI1RDYIE_W<'_, CIERrs> {
        LSI1RDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE32 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE32 oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 6 - PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<'_, CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
}
/**RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
