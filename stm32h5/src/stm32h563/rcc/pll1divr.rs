#[doc = "Register `PLL1DIVR` reader"]
pub type R = crate::R<PLL1DIVRrs>;
#[doc = "Register `PLL1DIVR` writer"]
pub type W = crate::W<PLL1DIVRrs>;
#[doc = "Field `PLL1N` reader - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_R = crate::FieldReader<u16>;
#[doc = "Field `PLL1N` writer - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLL1P` reader - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_R = crate::FieldReader;
#[doc = "Field `PLL1P` writer - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `PLL1Q` reader - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_R = crate::FieldReader;
#[doc = "Field `PLL1Q` writer - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `PLL1R` reader - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_R = crate::FieldReader;
#[doc = "Field `PLL1R` writer - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn pll1n(&self) -> PLL1N_R {
        PLL1N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    pub fn pll1p(&self) -> PLL1P_R {
        PLL1P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn pll1q(&self) -> PLL1Q_R {
        PLL1Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn pll1r(&self) -> PLL1R_R {
        PLL1R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pll1n(&mut self) -> PLL1N_W<PLL1DIVRrs> {
        PLL1N_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1p(&mut self) -> PLL1P_W<PLL1DIVRrs> {
        PLL1P_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1q(&mut self) -> PLL1Q_W<PLL1DIVRrs> {
        PLL1Q_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn pll1r(&mut self) -> PLL1R_W<PLL1DIVRrs> {
        PLL1R_W::new(self, 24)
    }
}
#[doc = "RCC PLL1 dividers register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1DIVRrs;
impl crate::RegisterSpec for PLL1DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1divr::R`](R) reader structure"]
impl crate::Readable for PLL1DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`pll1divr::W`](W) writer structure"]
impl crate::Writable for PLL1DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL1DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
