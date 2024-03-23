#[doc = "Register `RCC_PLL2DIVR` reader"]
pub type R = crate::R<RCC_PLL2DIVRrs>;
#[doc = "Register `RCC_PLL2DIVR` writer"]
pub type W = crate::W<RCC_PLL2DIVRrs>;
#[doc = "Field `PLL2N` reader - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F&lt;sub>ref2_ck&lt;/sub> between 1MHz and 16MHz"]
pub type PLL2N_R = crate::FieldReader<u16>;
#[doc = "Field `PLL2N` writer - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F&lt;sub>ref2_ck&lt;/sub> between 1MHz and 16MHz"]
pub type PLL2N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLL2P` reader - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2P_R = crate::FieldReader;
#[doc = "Field `PLL2P` writer - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2P_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLL2Q` reader - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2Q_R = crate::FieldReader;
#[doc = "Field `PLL2Q` writer - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLL2R` reader - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2R_R = crate::FieldReader;
#[doc = "Field `PLL2R` writer - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
pub type PLL2R_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F&lt;sub>ref2_ck&lt;/sub> between 1MHz and 16MHz"]
    #[inline(always)]
    pub fn pll2n(&self) -> PLL2N_R {
        PLL2N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn pll2p(&self) -> PLL2P_R {
        PLL2P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn pll2q(&self) -> PLL2Q_R {
        PLL2Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn pll2r(&self) -> PLL2R_R {
        PLL2R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F&lt;sub>ref2_ck&lt;/sub> between 1MHz and 16MHz"]
    #[inline(always)]
    #[must_use]
    pub fn pll2n(&mut self) -> PLL2N_W<RCC_PLL2DIVRrs> {
        PLL2N_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll2p(&mut self) -> PLL2P_W<RCC_PLL2DIVRrs> {
        PLL2P_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll2q(&mut self) -> PLL2Q_W<RCC_PLL2DIVRrs> {
        PLL2Q_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll2r(&mut self) -> PLL2R_W<RCC_PLL2DIVRrs> {
        PLL2R_W::new(self, 24)
    }
}
#[doc = "RCC PLL2 dividers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL2DIVRrs;
impl crate::RegisterSpec for RCC_PLL2DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll2divr::R`](R) reader structure"]
impl crate::Readable for RCC_PLL2DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll2divr::W`](W) writer structure"]
impl crate::Writable for RCC_PLL2DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for RCC_PLL2DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
