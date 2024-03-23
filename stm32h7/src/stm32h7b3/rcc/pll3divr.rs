#[doc = "Register `PLL3DIVR` reader"]
pub type R = crate::R<PLL3DIVRrs>;
#[doc = "Register `PLL3DIVR` writer"]
pub type W = crate::W<PLL3DIVRrs>;
#[doc = "Field `DIVN3` reader - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
pub type DIVN3_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN3` writer - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
pub type DIVN3_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP3` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVP3_R = crate::FieldReader;
#[doc = "Field `DIVP3` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVP3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVQ3` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVQ3_R = crate::FieldReader;
#[doc = "Field `DIVQ3` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVQ3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVR3` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVR3_R = crate::FieldReader;
#[doc = "Field `DIVR3` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
pub type DIVR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL3VCOSEL = 0 150 to 420Â MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x DIVN3, when fractional value 0 has been loaded into FRACN3, with: DIVN3 between 8 and 420 The input frequency Fref3_ck must be between 1 and 16MHz"]
    #[inline(always)]
    #[must_use]
    pub fn divn3(&mut self) -> DIVN3_W<PLL3DIVRrs> {
        DIVN3_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divp3(&mut self) -> DIVP3_W<PLL3DIVRrs> {
        DIVP3_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divq3(&mut self) -> DIVQ3_W<PLL3DIVRrs> {
        DIVQ3_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = PLL3RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divr3(&mut self) -> DIVR3_W<PLL3DIVRrs> {
        DIVR3_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
