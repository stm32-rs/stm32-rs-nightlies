///Register `RCC_PLL3DIVR` reader
pub type R = crate::R<RCC_PLL3DIVRrs>;
///Register `RCC_PLL3DIVR` writer
pub type W = crate::W<RCC_PLL3DIVRrs>;
///Field `PLL3N` reader - Multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x PLL3N, when fractional value 0 has been loaded in PLL3FRACN, with: PLL3N between 4 and 512 input frequency F&lt;sub>ref3_ck&lt;/sub> between 4 and 16MHz
pub type PLL3N_R = crate::FieldReader<u16>;
///Field `PLL3N` writer - Multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x PLL3N, when fractional value 0 has been loaded in PLL3FRACN, with: PLL3N between 4 and 512 input frequency F&lt;sub>ref3_ck&lt;/sub> between 4 and 16MHz
pub type PLL3N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PLL3P` reader - PLL3 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll3_p_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3P_R = crate::FieldReader;
///Field `PLL3P` writer - PLL3 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll3_p_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3P_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLL3Q` reader - PLL3 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll3_q_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3Q_R = crate::FieldReader;
///Field `PLL3Q` writer - PLL3 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll3_q_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLL3R` reader - PLL3 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll3_r_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3R_R = crate::FieldReader;
///Field `PLL3R` writer - PLL3 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll3_r_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3R_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x PLL3N, when fractional value 0 has been loaded in PLL3FRACN, with: PLL3N between 4 and 512 input frequency F&lt;sub>ref3_ck&lt;/sub> between 4 and 16MHz
    #[inline(always)]
    pub fn pll3n(&self) -> PLL3N_R {
        PLL3N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL3 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll3_p_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3p(&self) -> PLL3P_R {
        PLL3P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll3_q_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3q(&self) -> PLL3Q_R {
        PLL3Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL3 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll3_r_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3r(&self) -> PLL3R_R {
        PLL3R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3DIVR")
            .field("pll3n", &self.pll3n())
            .field("pll3p", &self.pll3p())
            .field("pll3q", &self.pll3q())
            .field("pll3r", &self.pll3r())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x PLL3N, when fractional value 0 has been loaded in PLL3FRACN, with: PLL3N between 4 and 512 input frequency F&lt;sub>ref3_ck&lt;/sub> between 4 and 16MHz
    #[inline(always)]
    #[must_use]
    pub fn pll3n(&mut self) -> PLL3N_W<RCC_PLL3DIVRrs> {
        PLL3N_W::new(self, 0)
    }
    ///Bits 9:15 - PLL3 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll3_p_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll3p(&mut self) -> PLL3P_W<RCC_PLL3DIVRrs> {
        PLL3P_W::new(self, 9)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll3_q_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll3q(&mut self) -> PLL3Q_W<RCC_PLL3DIVRrs> {
        PLL3Q_W::new(self, 16)
    }
    ///Bits 24:30 - PLL3 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll3_r_ck clock. It can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll3r(&mut self) -> PLL3R_W<RCC_PLL3DIVRrs> {
        PLL3R_W::new(self, 24)
    }
}
/**RCC PLL3 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_PLL3DIVR)*/
pub struct RCC_PLL3DIVRrs;
impl crate::RegisterSpec for RCC_PLL3DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll3divr::R`](R) reader structure
impl crate::Readable for RCC_PLL3DIVRrs {}
///`write(|w| ..)` method takes [`rcc_pll3divr::W`](W) writer structure
impl crate::Writable for RCC_PLL3DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL3DIVR to value 0x0101_0280
impl crate::Resettable for RCC_PLL3DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
