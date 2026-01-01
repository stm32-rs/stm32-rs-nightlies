///Register `PLL3DIVR` reader
pub type R = crate::R<PLL3DIVRrs>;
///Register `PLL3DIVR` writer
pub type W = crate::W<PLL3DIVRrs>;
///Field `PLL3N` reader - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved
pub type PLL3N_R = crate::FieldReader<u16>;
///Field `PLL3N` writer - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved
pub type PLL3N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PLL3P` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3P_R = crate::FieldReader;
///Field `PLL3P` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3P_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `PLL3Q` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3Q_R = crate::FieldReader;
///Field `PLL3Q` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `PLL3R` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3R_R = crate::FieldReader;
///Field `PLL3R` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3R_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved
    #[inline(always)]
    pub fn pll3n(&self) -> PLL3N_R {
        PLL3N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3p(&self) -> PLL3P_R {
        PLL3P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3q(&self) -> PLL3Q_R {
        PLL3Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3r(&self) -> PLL3R_R {
        PLL3R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3DIVR")
            .field("pll3n", &self.pll3n())
            .field("pll3p", &self.pll3p())
            .field("pll3q", &self.pll3q())
            .field("pll3r", &self.pll3r())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL3VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0). ... ... Others: reserved
    #[inline(always)]
    pub fn pll3n(&mut self) -> PLL3N_W<'_, PLL3DIVRrs> {
        PLL3N_W::new(self, 0)
    }
    ///Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3p(&mut self) -> PLL3P_W<'_, PLL3DIVRrs> {
        PLL3P_W::new(self, 9)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3q(&mut self) -> PLL3Q_W<'_, PLL3DIVRrs> {
        PLL3Q_W::new(self, 16)
    }
    ///Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3r(&mut self) -> PLL3R_W<'_, PLL3DIVRrs> {
        PLL3R_W::new(self, 24)
    }
}
/**RCC PLL3 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll3divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL3DIVR)*/
pub struct PLL3DIVRrs;
impl crate::RegisterSpec for PLL3DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3divr::R`](R) reader structure
impl crate::Readable for PLL3DIVRrs {}
///`write(|w| ..)` method takes [`pll3divr::W`](W) writer structure
impl crate::Writable for PLL3DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3DIVR to value 0x0101_0280
impl crate::Resettable for PLL3DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
