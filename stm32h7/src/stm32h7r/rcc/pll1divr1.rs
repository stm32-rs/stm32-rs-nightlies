///Register `PLL1DIVR1` reader
pub type R = crate::R<PLL1DIVR1rs>;
///Register `PLL1DIVR1` writer
pub type W = crate::W<PLL1DIVR1rs>;
///Field `DIVN1` reader - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 1 and 16 MHz.
pub type DIVN1_R = crate::FieldReader<u16>;
///Field `DIVN1` writer - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 1 and 16 MHz.
pub type DIVN1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVP` reader - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ...
pub type DIVP_R = crate::FieldReader;
///Field `DIVP` writer - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ...
pub type DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ` reader - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ...
pub type DIVQ_R = crate::FieldReader;
///Field `DIVQ` writer - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ...
pub type DIVQ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVR1` reader - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ...
pub type DIVR1_R = crate::FieldReader;
///Field `DIVR1` writer - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ...
pub type DIVR1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 1 and 16 MHz.
    #[inline(always)]
    pub fn divn1(&self) -> DIVN1_R {
        DIVN1_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ...
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ...
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ...
    #[inline(always)]
    pub fn divr1(&self) -> DIVR1_R {
        DIVR1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1DIVR1")
            .field("divn1", &self.divn1())
            .field("divp", &self.divp())
            .field("divq", &self.divq())
            .field("divr1", &self.divr1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544MHz if PLL1VCOSEL = 0 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x DIVN1, when fractional value 0 has been loaded into FRACN, with: DIVN1 between 8 and 420 The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 1 and 16 MHz.
    #[inline(always)]
    #[must_use]
    pub fn divn1(&mut self) -> DIVN1_W<PLL1DIVR1rs> {
        DIVN1_W::new(self, 0)
    }
    ///Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1DIVPEN = 0. ...
    #[inline(always)]
    #[must_use]
    pub fn divp(&mut self) -> DIVP_W<PLL1DIVR1rs> {
        DIVP_W::new(self, 9)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1DIVQEN = 0. ...
    #[inline(always)]
    #[must_use]
    pub fn divq(&mut self) -> DIVQ_W<PLL1DIVR1rs> {
        DIVQ_W::new(self, 16)
    }
    ///Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1DIVREN = 0. ...
    #[inline(always)]
    #[must_use]
    pub fn divr1(&mut self) -> DIVR1_W<PLL1DIVR1rs> {
        DIVR1_W::new(self, 24)
    }
}
/**RCC PLL1 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll1divr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1DIVR1)*/
pub struct PLL1DIVR1rs;
impl crate::RegisterSpec for PLL1DIVR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll1divr1::R`](R) reader structure
impl crate::Readable for PLL1DIVR1rs {}
///`write(|w| ..)` method takes [`pll1divr1::W`](W) writer structure
impl crate::Writable for PLL1DIVR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL1DIVR1 to value 0x0101_0280
impl crate::Resettable for PLL1DIVR1rs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
