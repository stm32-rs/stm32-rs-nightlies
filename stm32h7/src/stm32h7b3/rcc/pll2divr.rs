#[doc = "Register `PLL2DIVR` reader"]
pub type R = crate::R<PLL2DIVRrs>;
#[doc = "Register `PLL2DIVR` writer"]
pub type W = crate::W<PLL2DIVRrs>;
#[doc = "Field `DIVN2` reader - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz."]
pub type DIVN2_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN2` writer - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz."]
pub type DIVN2_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP2` reader - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVP2_R = crate::FieldReader;
#[doc = "Field `DIVP2` writer - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVP2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVQ2` reader - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVQ2_R = crate::FieldReader;
#[doc = "Field `DIVQ2` writer - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVQ2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVR2` reader - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVR2_R = crate::FieldReader;
#[doc = "Field `DIVR2` writer - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
pub type DIVR2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz."]
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz."]
    #[inline(always)]
    #[must_use]
    pub fn divn2(&mut self) -> DIVN2_W<PLL2DIVRrs> {
        DIVN2_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divp2(&mut self) -> DIVP2_W<PLL2DIVRrs> {
        DIVP2_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divq2(&mut self) -> DIVQ2_W<PLL2DIVRrs> {
        DIVQ2_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ..."]
    #[inline(always)]
    #[must_use]
    pub fn divr2(&mut self) -> DIVR2_W<PLL2DIVRrs> {
        DIVR2_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2DIVRrs;
impl crate::RegisterSpec for PLL2DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2divr::R`](R) reader structure"]
impl crate::Readable for PLL2DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure"]
impl crate::Writable for PLL2DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL2DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
