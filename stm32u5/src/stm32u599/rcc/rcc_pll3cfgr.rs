///Register `RCC_PLL3CFGR` reader
pub type R = crate::R<RCC_PLL3CFGRrs>;
///Register `RCC_PLL3CFGR` writer
pub type W = crate::W<RCC_PLL3CFGRrs>;
///Field `PLL3SRC` reader - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
pub type PLL3SRC_R = crate::FieldReader;
///Field `PLL3SRC` writer - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
pub type PLL3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL3RGE` reader - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_R = crate::FieldReader;
///Field `PLL3RGE` writer - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
pub type PLL3FRACEN_R = crate::BitReader;
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
pub type PLL3FRACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3M` reader - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_R = crate::FieldReader;
///Field `PLL3M` writer - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL3PEN` reader - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
pub type PLL3PEN_R = crate::BitReader;
///Field `PLL3PEN` writer - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
pub type PLL3PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3QEN` reader - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
pub type PLL3QEN_R = crate::BitReader;
///Field `PLL3QEN` writer - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
pub type PLL3QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3REN` reader - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
pub type PLL3REN_R = crate::BitReader;
///Field `PLL3REN` writer - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
pub type PLL3REN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3m(&self) -> PLL3M_R {
        PLL3M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR")
            .field("pll3src", &self.pll3src())
            .field("pll3rge", &self.pll3rge())
            .field("pll3fracen", &self.pll3fracen())
            .field("pll3m", &self.pll3m())
            .field("pll3pen", &self.pll3pen())
            .field("pll3qen", &self.pll3qen())
            .field("pll3ren", &self.pll3ren())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL3 entry clock source This bitfield is set and cleared by software to select PLL3 clock source. It can be written only when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must be�zero.
    #[inline(always)]
    #[must_use]
    pub fn pll3src(&mut self) -> PLL3SRC_W<RCC_PLL3CFGRrs> {
        PLL3SRC_W::new(self, 0)
    }
    ///Bits 2:3 - PLL3 input frequency range This bit is set and reset by software to select the proper reference frequency range used for�PLL3. It must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<RCC_PLL3CFGRrs> {
        PLL3RGE_W::new(self, 2)
    }
    ///Bit 4 - PLL3 fractional latch enable This bit is set and reset by software to latch the content of PLL3FRACN in the ΣΔ modulator. In order to latch the PLL3FRACN value into the ΣΔ modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see PLL initialization phase for details).
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<RCC_PLL3CFGRrs> {
        PLL3FRACEN_W::new(self, 4)
    }
    ///Bits 8:11 - Prescaler for PLL3 This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll3m(&mut self) -> PLL3M_W<RCC_PLL3CFGRrs> {
        PLL3M_W::new(self, 8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<RCC_PLL3CFGRrs> {
        PLL3PEN_W::new(self, 16)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<RCC_PLL3CFGRrs> {
        PLL3QEN_W::new(self, 17)
    }
    ///Bit 18 - PLL3 DIVR divider output enable This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
    #[inline(always)]
    #[must_use]
    pub fn pll3ren(&mut self) -> PLL3REN_W<RCC_PLL3CFGRrs> {
        PLL3REN_W::new(self, 18)
    }
}
/**RCC PLL3 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_PLL3CFGR)*/
pub struct RCC_PLL3CFGRrs;
impl crate::RegisterSpec for RCC_PLL3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll3cfgr::R`](R) reader structure
impl crate::Readable for RCC_PLL3CFGRrs {}
///`write(|w| ..)` method takes [`rcc_pll3cfgr::W`](W) writer structure
impl crate::Writable for RCC_PLL3CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL3CFGR to value 0
impl crate::Resettable for RCC_PLL3CFGRrs {
    const RESET_VALUE: u32 = 0;
}
