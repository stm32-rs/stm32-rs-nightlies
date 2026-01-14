///Register `PLL1CFGR` reader
pub type R = crate::R<PLL1CFGRrs>;
///Register `PLL1CFGR` writer
pub type W = crate::W<PLL1CFGRrs>;
///Field `PLL1SRC` reader - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.
pub type PLL1SRC_R = crate::FieldReader;
///Field `PLL1SRC` writer - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.
pub type PLL1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_R = crate::FieldReader;
///Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the modulator. In order to latch the PLL1FRACN value into the modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).
pub type PLL1FRACEN_R = crate::BitReader;
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the modulator. In order to latch the PLL1FRACN value into the modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1M` reader - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_R = crate::FieldReader;
///Field `PLL1M` writer - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL1PEN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1PEN_R = crate::BitReader;
///Field `PLL1PEN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1QEN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1QEN_R = crate::BitReader;
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1REN` reader - PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1REN_R = crate::BitReader;
///Field `PLL1REN` writer - PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1REN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RCLKPRE` reader - pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.
pub type PLL1RCLKPRE_R = crate::BitReader;
///Field `PLL1RCLKPRE` writer - pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.
pub type PLL1RCLKPRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RCLKPRESTEP` reader - pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.
pub type PLL1RCLKPRESTEP_R = crate::BitReader;
///Field `PLL1RCLKPRESTEP` writer - pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.
pub type PLL1RCLKPRESTEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1RCLKPRERDY` reader - pll1rclkpre not divided ready. Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre.
pub type PLL1RCLKPRERDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the modulator. In order to latch the PLL1FRACN value into the modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:10 - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.
    #[inline(always)]
    pub fn pll1rclkpre(&self) -> PLL1RCLKPRE_R {
        PLL1RCLKPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.
    #[inline(always)]
    pub fn pll1rclkprestep(&self) -> PLL1RCLKPRESTEP_R {
        PLL1RCLKPRESTEP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - pll1rclkpre not divided ready. Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre.
    #[inline(always)]
    pub fn pll1rclkprerdy(&self) -> PLL1RCLKPRERDY_R {
        PLL1RCLKPRERDY_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR")
            .field("pll1src", &self.pll1src())
            .field("pll1rge", &self.pll1rge())
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1m", &self.pll1m())
            .field("pll1pen", &self.pll1pen())
            .field("pll1qen", &self.pll1qen())
            .field("pll1ren", &self.pll1ren())
            .field("pll1rclkpre", &self.pll1rclkpre())
            .field("pll1rclkprestep", &self.pll1rclkprestep())
            .field("pll1rclkprerdy", &self.pll1rclkprerdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.
    #[inline(always)]
    pub fn pll1src(&mut self) -> PLL1SRC_W<'_, PLL1CFGRrs> {
        PLL1SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<'_, PLL1CFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the modulator. In order to latch the PLL1FRACN value into the modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<'_, PLL1CFGRrs> {
        PLL1FRACEN_W::new(self, 4)
    }
    ///Bits 8:10 - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&mut self) -> PLL1M_W<'_, PLL1CFGRrs> {
        PLL1M_W::new(self, 8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<'_, PLL1CFGRrs> {
        PLL1PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<'_, PLL1CFGRrs> {
        PLL1QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1ren(&mut self) -> PLL1REN_W<'_, PLL1CFGRrs> {
        PLL1REN_W::new(self, 18)
    }
    ///Bit 20 - pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.
    #[inline(always)]
    pub fn pll1rclkpre(&mut self) -> PLL1RCLKPRE_W<'_, PLL1CFGRrs> {
        PLL1RCLKPRE_W::new(self, 20)
    }
    ///Bit 21 - pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.
    #[inline(always)]
    pub fn pll1rclkprestep(&mut self) -> PLL1RCLKPRESTEP_W<'_, PLL1CFGRrs> {
        PLL1RCLKPRESTEP_W::new(self, 21)
    }
}
/**RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:PLL1CFGR)*/
pub struct PLL1CFGRrs;
impl crate::RegisterSpec for PLL1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr::R`](R) reader structure
impl crate::Readable for PLL1CFGRrs {}
///`write(|w| ..)` method takes [`pll1cfgr::W`](W) writer structure
impl crate::Writable for PLL1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR to value 0
impl crate::Resettable for PLL1CFGRrs {}
