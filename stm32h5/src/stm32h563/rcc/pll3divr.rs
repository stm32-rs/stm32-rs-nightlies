#[doc = "Register `PLL3DIVR` reader"]
pub type R = crate::R<PLL3DIVRrs>;
#[doc = "Register `PLL3DIVR` writer"]
pub type W = crate::W<PLL3DIVRrs>;
#[doc = "Field `PLL3N` reader - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
pub type PLL3N_R = crate::FieldReader<u16>;
#[doc = "Field `PLL3N` writer - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
pub type PLL3N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLL3P` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3P_R = crate::FieldReader;
#[doc = "Field `PLL3P` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3P_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `PLL3Q` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3Q_R = crate::FieldReader;
#[doc = "Field `PLL3Q` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3Q_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `PLL3R` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3R_R = crate::FieldReader;
#[doc = "Field `PLL3R` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
pub type PLL3R_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn pll3n(&self) -> PLL3N_R {
        PLL3N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3p(&self) -> PLL3P_R {
        PLL3P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3q(&self) -> PLL3Q_R {
        PLL3Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn pll3r(&self) -> PLL3R_R {
        PLL3R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pll3n(&mut self) -> PLL3N_W<PLL3DIVRrs> {
        PLL3N_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3p(&mut self) -> PLL3P_W<PLL3DIVRrs> {
        PLL3P_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3q(&mut self) -> PLL3Q_W<PLL3DIVRrs> {
        PLL3Q_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll3r(&mut self) -> PLL3R_W<PLL3DIVRrs> {
        PLL3R_W::new(self, 24)
    }
}
#[doc = "RCC PLL3 dividers register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL3DIVRrs;
impl crate::RegisterSpec for PLL3DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3divr::R`](R) reader structure"]
impl crate::Readable for PLL3DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`pll3divr::W`](W) writer structure"]
impl crate::Writable for PLL3DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL3DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
