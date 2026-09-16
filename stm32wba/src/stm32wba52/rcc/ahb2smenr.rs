///Register `AHB2SMENR` reader
pub type R = crate::R<AHB2SMENRrs>;
///Register `AHB2SMENR` writer
pub type W = crate::W<AHB2SMENRrs>;
///Field `GPIOASMEN` reader - IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOASMEN_R = crate::BitReader;
///Field `GPIOASMEN` writer - IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBSMEN` reader - IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOBSMEN_R = crate::BitReader;
///Field `GPIOBSMEN` writer - IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCSMEN` reader - IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOCSMEN_R = crate::BitReader;
///Field `GPIOCSMEN` writer - IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHSMEN` reader - IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOHSMEN_R = crate::BitReader;
///Field `GPIOHSMEN` writer - IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSMEN` reader - AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type AESSMEN_R = crate::BitReader;
///Field `AESSMEN` writer - AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHSMEN` reader - HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HASHSMEN_R = crate::BitReader;
///Field `HASHSMEN` writer - HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSMEN` reader - Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RNGSMEN_R = crate::BitReader;
///Field `RNGSMEN` writer - Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESSMEN` reader - SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SAESSMEN_R = crate::BitReader;
///Field `SAESSMEN` writer - SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SAESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKASMEN` reader - PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PKASMEN_R = crate::BitReader;
///Field `PKASMEN` writer - PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2SMEN` reader - SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM2SMEN_R = crate::BitReader;
///Field `SRAM2SMEN` writer - SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn saessmen(&self) -> SAESSMEN_R {
        SAESSMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("aessmen", &self.aessmen())
            .field("hashsmen", &self.hashsmen())
            .field("rngsmen", &self.rngsmen())
            .field("saessmen", &self.saessmen())
            .field("pkasmen", &self.pkasmen())
            .field("sram2smen", &self.sram2smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 7 - IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<'_, AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    ///Bit 16 - AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHB2SMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    ///Bit 17 - HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<'_, AHB2SMENRrs> {
        HASHSMEN_W::new(self, 17)
    }
    ///Bit 18 - Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHB2SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 19 - SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn saessmen(&mut self) -> SAESSMEN_W<'_, AHB2SMENRrs> {
        SAESSMEN_W::new(self, 19)
    }
    ///Bit 21 - PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<'_, AHB2SMENRrs> {
        PKASMEN_W::new(self, 21)
    }
    ///Bit 30 - SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 30)
    }
}
/**RCC AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:AHB2SMENR)*/
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2smenr::R`](R) reader structure
impl crate::Readable for AHB2SMENRrs {}
///`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2SMENR to value 0xffff_ffff
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
