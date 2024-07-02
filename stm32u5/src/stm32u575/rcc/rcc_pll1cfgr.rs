///Register `RCC_PLL1CFGR` reader
pub type R = crate::R<RCC_PLL1CFGRrs>;
///Register `RCC_PLL1CFGR` writer
pub type W = crate::W<RCC_PLL1CFGRrs>;
///Field `PLL1SRC` reader - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
pub type PLL1SRC_R = crate::FieldReader;
///Field `PLL1SRC` writer - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
pub type PLL1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL1RGE` reader - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_R = crate::FieldReader;
///Field `PLL1RGE` writer - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
pub type PLL1FRACEN_R = crate::BitReader;
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1M` reader - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_R = crate::FieldReader;
///Field `PLL1M` writer - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL1MBOOST` reader - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
pub type PLL1MBOOST_R = crate::FieldReader;
///Field `PLL1MBOOST` writer - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
pub type PLL1MBOOST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL1PEN` reader - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
pub type PLL1PEN_R = crate::BitReader;
///Field `PLL1PEN` writer - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1QEN` reader - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
pub type PLL1QEN_R = crate::BitReader;
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
pub type PLL1QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1REN` reader - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
pub type PLL1REN_R = crate::BitReader;
///Field `PLL1REN` writer - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
pub type PLL1REN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
    #[inline(always)]
    pub fn pll1mboost(&self) -> PLL1MBOOST_R {
        PLL1MBOOST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL1CFGR")
            .field("pll1src", &self.pll1src())
            .field("pll1rge", &self.pll1rge())
            .field("pll1fracen", &self.pll1fracen())
            .field("pll1m", &self.pll1m())
            .field("pll1mboost", &self.pll1mboost())
            .field("pll1pen", &self.pll1pen())
            .field("pll1qen", &self.pll1qen())
            .field("pll1ren", &self.pll1ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL1 entry clock source This bitfield is set and cleared by software to select PLL1 clock source. It can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value must be zero.
    #[inline(always)]
    #[must_use]
    pub fn pll1src(&mut self) -> PLL1SRC_W<RCC_PLL1CFGRrs> {
        PLL1SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL1 input frequency range This bit is set and reset by software to select the proper reference frequency range used for PLL1. It must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<RCC_PLL1CFGRrs> {
        PLL1RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL1 fractional latch enable This bit is set and reset by software to latch the content of PLL1FRACN in the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<RCC_PLL1CFGRrs> {
        PLL1FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL1 This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll1m(&mut self) -> PLL1M_W<RCC_PLL1CFGRrs> {
        PLL1M_W::new(self, 8)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1�input�clock�frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPODboost mode is disabled (see Section�10: Power control (PWR)). others: reserved
    #[inline(always)]
    #[must_use]
    pub fn pll1mboost(&mut self) -> PLL1MBOOST_W<RCC_PLL1CFGRrs> {
        PLL1MBOOST_W::new(self, 12)
    }
    ///Bit 16 - PLL1 DIVP divider output enable This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<RCC_PLL1CFGRrs> {
        PLL1PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<RCC_PLL1CFGRrs> {
        PLL1QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL1 DIVR divider output enable This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used. This bit can be cleared only when the PLL1 is not used as SYSCLK.
    #[inline(always)]
    #[must_use]
    pub fn pll1ren(&mut self) -> PLL1REN_W<RCC_PLL1CFGRrs> {
        PLL1REN_W::new(self, 18)
    }
}
/**RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_PLL1CFGR)*/
pub struct RCC_PLL1CFGRrs;
impl crate::RegisterSpec for RCC_PLL1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll1cfgr::R`](R) reader structure
impl crate::Readable for RCC_PLL1CFGRrs {}
///`write(|w| ..)` method takes [`rcc_pll1cfgr::W`](W) writer structure
impl crate::Writable for RCC_PLL1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL1CFGR to value 0
impl crate::Resettable for RCC_PLL1CFGRrs {
    const RESET_VALUE: u32 = 0;
}
