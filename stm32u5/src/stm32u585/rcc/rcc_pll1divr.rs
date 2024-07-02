///Register `RCC_PLL1DIVR` reader
pub type R = crate::R<RCC_PLL1DIVRrs>;
///Register `RCC_PLL1DIVR` writer
pub type W = crate::W<RCC_PLL1DIVRrs>;
///Field `PLL1N` reader - Multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x PLL1N, when fractional value 0 has been loaded in PLL1FRACN, with: PLL1N between 4 and 512 input frequency F&lt;sub>ref1_ck&lt;/sub> between 4 and 16�MHz
pub type PLL1N_R = crate::FieldReader<u16>;
///Field `PLL1N` writer - Multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x PLL1N, when fractional value 0 has been loaded in PLL1FRACN, with: PLL1N between 4 and 512 input frequency F&lt;sub>ref1_ck&lt;/sub> between 4 and 16�MHz
pub type PLL1N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PLL1P` reader - PLL1 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll1_p_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1P_R = crate::FieldReader;
///Field `PLL1P` writer - PLL1 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll1_p_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1P_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLL1Q` reader - PLL1 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll1_q_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1Q_R = crate::FieldReader;
///Field `PLL1Q` writer - PLL1 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll1_q_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLL1R` reader - PLL1 DIVR division factor This bitfield is set and reset by software to control frequency of the pll1_r_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Only division by one and even division factors are allowed. ...
pub type PLL1R_R = crate::FieldReader;
///Field `PLL1R` writer - PLL1 DIVR division factor This bitfield is set and reset by software to control frequency of the pll1_r_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Only division by one and even division factors are allowed. ...
pub type PLL1R_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x PLL1N, when fractional value 0 has been loaded in PLL1FRACN, with: PLL1N between 4 and 512 input frequency F&lt;sub>ref1_ck&lt;/sub> between 4 and 16�MHz
    #[inline(always)]
    pub fn pll1n(&self) -> PLL1N_R {
        PLL1N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll1_p_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1p(&self) -> PLL1P_R {
        PLL1P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll1_q_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1q(&self) -> PLL1Q_R {
        PLL1Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL1 DIVR division factor This bitfield is set and reset by software to control frequency of the pll1_r_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Only division by one and even division factors are allowed. ...
    #[inline(always)]
    pub fn pll1r(&self) -> PLL1R_R {
        PLL1R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL1DIVR")
            .field("pll1n", &self.pll1n())
            .field("pll1p", &self.pll1p())
            .field("pll1q", &self.pll1q())
            .field("pll1r", &self.pll1r())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x PLL1N, when fractional value 0 has been loaded in PLL1FRACN, with: PLL1N between 4 and 512 input frequency F&lt;sub>ref1_ck&lt;/sub> between 4 and 16�MHz
    #[inline(always)]
    #[must_use]
    pub fn pll1n(&mut self) -> PLL1N_W<RCC_PLL1DIVRrs> {
        PLL1N_W::new(self, 0)
    }
    ///Bits 9:15 - PLL1 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll1_p_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll1p(&mut self) -> PLL1P_W<RCC_PLL1DIVRrs> {
        PLL1P_W::new(self, 9)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll1_q_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll1q(&mut self) -> PLL1Q_W<RCC_PLL1DIVRrs> {
        PLL1Q_W::new(self, 16)
    }
    ///Bits 24:30 - PLL1 DIVR division factor This bitfield is set and reset by software to control frequency of the pll1_r_ck clock. It can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Only division by one and even division factors are allowed. ...
    #[inline(always)]
    #[must_use]
    pub fn pll1r(&mut self) -> PLL1R_W<RCC_PLL1DIVRrs> {
        PLL1R_W::new(self, 24)
    }
}
/**RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:RCC_PLL1DIVR)*/
pub struct RCC_PLL1DIVRrs;
impl crate::RegisterSpec for RCC_PLL1DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll1divr::R`](R) reader structure
impl crate::Readable for RCC_PLL1DIVRrs {}
///`write(|w| ..)` method takes [`rcc_pll1divr::W`](W) writer structure
impl crate::Writable for RCC_PLL1DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL1DIVR to value 0x0101_0280
impl crate::Resettable for RCC_PLL1DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
