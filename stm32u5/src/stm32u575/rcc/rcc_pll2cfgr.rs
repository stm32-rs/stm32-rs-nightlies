///Register `RCC_PLL2CFGR` reader
pub type R = crate::R<RCC_PLL2CFGRrs>;
///Register `RCC_PLL2CFGR` writer
pub type W = crate::W<RCC_PLL2CFGRrs>;
///Field `PLL2SRC` reader - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
pub type PLL2SRC_R = crate::FieldReader;
///Field `PLL2SRC` writer - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
pub type PLL2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL2RGE` reader - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_R = crate::FieldReader;
///Field `PLL2RGE` writer - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
pub type PLL2FRACEN_R = crate::BitReader;
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
pub type PLL2FRACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2M` reader - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_R = crate::FieldReader;
///Field `PLL2M` writer - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL2PEN` reader - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
pub type PLL2PEN_R = crate::BitReader;
///Field `PLL2PEN` writer - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
pub type PLL2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2QEN` reader - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
pub type PLL2QEN_R = crate::BitReader;
///Field `PLL2QEN` writer - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
pub type PLL2QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2REN` reader - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
pub type PLL2REN_R = crate::BitReader;
///Field `PLL2REN` writer - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
pub type PLL2REN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll2src(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2m(&self) -> PLL2M_R {
        PLL2M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
    #[inline(always)]
    pub fn pll2pen(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
    #[inline(always)]
    pub fn pll2qen(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
    #[inline(always)]
    pub fn pll2ren(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR")
            .field("pll2src", &self.pll2src())
            .field("pll2rge", &self.pll2rge())
            .field("pll2fracen", &self.pll2fracen())
            .field("pll2m", &self.pll2m())
            .field("pll2pen", &self.pll2pen())
            .field("pll2qen", &self.pll2qen())
            .field("pll2ren", &self.pll2ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL2 entry clock source This bitfield is set and cleared by software to select PLL2 clock source. It can be written only when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must be�zero.
    #[inline(always)]
    #[must_use]
    pub fn pll2src(&mut self) -> PLL2SRC_W<RCC_PLL2CFGRrs> {
        PLL2SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL2 input frequency range This bitfield is set and reset by software to select the proper reference frequency range used for�PLL2. It must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<RCC_PLL2CFGRrs> {
        PLL2RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL2 fractional latch enable This bit is set and reset by software to latch the content of PLL2FRACN in the ΣΔ modulator. In order to latch the PLL2FRACN value into the ΣΔ modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<RCC_PLL2CFGRrs> {
        PLL2FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL2 This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll2m(&mut self) -> PLL2M_W<RCC_PLL2CFGRrs> {
        PLL2M_W::new(self, 8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<RCC_PLL2CFGRrs> {
        PLL2PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<RCC_PLL2CFGRrs> {
        PLL2QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL2 DIVR divider output enable This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll2ren(&mut self) -> PLL2REN_W<RCC_PLL2CFGRrs> {
        PLL2REN_W::new(self, 18)
    }
}
/**RCC PLL2 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_PLL2CFGR)*/
pub struct RCC_PLL2CFGRrs;
impl crate::RegisterSpec for RCC_PLL2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll2cfgr::R`](R) reader structure
impl crate::Readable for RCC_PLL2CFGRrs {}
///`write(|w| ..)` method takes [`rcc_pll2cfgr::W`](W) writer structure
impl crate::Writable for RCC_PLL2CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL2CFGR to value 0
impl crate::Resettable for RCC_PLL2CFGRrs {
    const RESET_VALUE: u32 = 0;
}
