///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `HSION` reader - HSI16 clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI16 oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI16 oscillator on in case of clock security failure of the HSE32 crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - HSI16 clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI16 oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI16 oscillator on in case of clock security failure of the HSE32 crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIKERON` reader - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 oscillator on even in Stop modes. Keeping the HSI16 oscillator on in Stop modes allows the communication speed not to be reduced by the HSI16 oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSIKERON_R = crate::BitReader;
///Field `HSIKERON` writer - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 oscillator on even in Stop modes. Keeping the HSI16 oscillator on in Stop modes allows the communication speed not to be reduced by the HSI16 oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDY` reader - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
pub type HSIRDY_R = crate::BitReader;
///Field `HSEON` reader - HSE32 clock enable Set and cleared by software. Cleared by hardware to stop the HSE32 clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE32 is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE32 oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSE32 clock enable Set and cleared by software. Cleared by hardware to stop the HSE32 clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE32 is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE32 oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDY` reader - HSE32 clock ready flag Set by hardware to indicate that the HSE32 oscillator is stable. This bit is set both when HSE32 is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSERDY_R = crate::BitReader;
///Field `HSECSSON` reader - HSE32 clock security system enable Set by software to enable the HSE32 clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE32 oscillator is ready and disabled by hardware if a HSE32 clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSECSSON_R = crate::BitReader;
///Field `HSECSSON` writer - HSE32 clock security system enable Set by software to enable the HSE32 clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE32 oscillator is ready and disabled by hardware if a HSE32 clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPRE` reader - HSE32 clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE32 clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSEPRE_R = crate::BitReader;
///Field `HSEPRE` writer - HSE32 clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE32 clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type HSEPRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1ON` reader - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE32 is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1ON_R = crate::BitReader;
///Field `PLL1ON` writer - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE32 is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RDY` reader - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type PLL1RDY_R = crate::BitReader;
impl R {
    ///Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI16 oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI16 oscillator on in case of clock security failure of the HSE32 crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 oscillator on even in Stop modes. Keeping the HSI16 oscillator on in Stop modes allows the communication speed not to be reduced by the HSI16 oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - HSE32 clock enable Set and cleared by software. Cleared by hardware to stop the HSE32 clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE32 is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE32 oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE32 clock ready flag Set by hardware to indicate that the HSE32 oscillator is stable. This bit is set both when HSE32 is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - HSE32 clock security system enable Set by software to enable the HSE32 clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE32 oscillator is ready and disabled by hardware if a HSE32 clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HSE32 clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE32 clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE32 is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1on(&self) -> PLL1ON_R {
        PLL1ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsecsson", &self.hsecsson())
            .field("hsepre", &self.hsepre())
            .field("pll1on", &self.pll1on())
            .field("pll1rdy", &self.pll1rdy())
            .finish()
    }
}
impl W {
    ///Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI16 oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI16 oscillator on in case of clock security failure of the HSE32 crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI16 enable for some peripheral kernels Set and cleared by software to force HSI16 oscillator on even in Stop modes. Keeping the HSI16 oscillator on in Stop modes allows the communication speed not to be reduced by the HSI16 oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 16 - HSE32 clock enable Set and cleared by software. Cleared by hardware to stop the HSE32 clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE32 is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE32 oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 19 - HSE32 clock security system enable Set by software to enable the HSE32 clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE32 oscillator is ready and disabled by hardware if a HSE32 clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W<'_, CRrs> {
        HSECSSON_W::new(self, 19)
    }
    ///Bit 20 - HSE32 clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE32 clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn hsepre(&mut self) -> HSEPRE_W<'_, CRrs> {
        HSEPRE_W::new(self, 20)
    }
    ///Bit 24 - PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE32 is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn pll1on(&mut self) -> PLL1ON_W<'_, CRrs> {
        PLL1ON_W::new(self, 24)
    }
}
/**RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x0500
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0500;
}
