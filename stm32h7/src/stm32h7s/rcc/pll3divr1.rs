///Register `PLL3DIVR1` reader
pub type R = crate::R<PLL3DIVR1rs>;
///Register `PLL3DIVR1` writer
pub type W = crate::W<PLL3DIVR1rs>;
///Field `DIVN3` reader - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL3VCOSEL = 0 150 to 420 MHz if PLL3VCOSEL = 1 VCO output frequency = F<sub>ref3_ck</sub> x DIVN3, when fractional value 0 has been loaded into FRACN, with: DIVN3 between 8 and 420 The input frequency F<sub>ref3_ck</sub> must be between 1 and 16MHz
pub type DIVN3_R = crate::FieldReader<u16>;
///Field `DIVN3` writer - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL3VCOSEL = 0 150 to 420 MHz if PLL3VCOSEL = 1 VCO output frequency = F<sub>ref3_ck</sub> x DIVN3, when fractional value 0 has been loaded into FRACN, with: DIVN3 between 8 and 420 The input frequency F<sub>ref3_ck</sub> must be between 1 and 16MHz
pub type DIVN3_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVP` reader - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3DIVPEN = 0. ...
pub type DIVP_R = crate::FieldReader;
///Field `DIVP` writer - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3DIVPEN = 0. ...
pub type DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ` reader - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3DIVQEN = 0. ...
pub type DIVQ_R = crate::FieldReader;
///Field `DIVQ` writer - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3DIVQEN = 0. ...
pub type DIVQ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVR3` reader - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3DIVREN = 0. ...
pub type DIVR3_R = crate::FieldReader;
///Field `DIVR3` writer - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3DIVREN = 0. ...
pub type DIVR3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL3VCOSEL = 0 150 to 420 MHz if PLL3VCOSEL = 1 VCO output frequency = F<sub>ref3_ck</sub> x DIVN3, when fractional value 0 has been loaded into FRACN, with: DIVN3 between 8 and 420 The input frequency F<sub>ref3_ck</sub> must be between 1 and 16MHz
    #[inline(always)]
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3DIVPEN = 0. ...
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3DIVQEN = 0. ...
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3DIVREN = 0. ...
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3DIVR1")
            .field("divn3", &self.divn3())
            .field("divp", &self.divp())
            .field("divq", &self.divq())
            .field("divr3", &self.divr3())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL3 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL3ON = PLL3RDY = 0). ...........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 544 MHz if PLL3VCOSEL = 0 150 to 420 MHz if PLL3VCOSEL = 1 VCO output frequency = F<sub>ref3_ck</sub> x DIVN3, when fractional value 0 has been loaded into FRACN, with: DIVN3 between 8 and 420 The input frequency F<sub>ref3_ck</sub> must be between 1 and 16MHz
    #[inline(always)]
    pub fn divn3(&mut self) -> DIVN3_W<PLL3DIVR1rs> {
        DIVN3_W::new(self, 0)
    }
    ///Bits 9:15 - PLL3 DIVP division factor Set and reset by software to control the frequency of the pll3_p_ck clock. These bits can be written only when the PLL3DIVPEN = 0. ...
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W<PLL3DIVR1rs> {
        DIVP_W::new(self, 9)
    }
    ///Bits 16:22 - PLL3 DIVQ division factor Set and reset by software to control the frequency of the pll3_q_ck clock. These bits can be written only when the PLL3DIVQEN = 0. ...
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W<PLL3DIVR1rs> {
        DIVQ_W::new(self, 16)
    }
    ///Bits 24:30 - PLL3 DIVR division factor Set and reset by software to control the frequency of the pll3_r_ck clock. These bits can be written only when the PLL3DIVREN = 0. ...
    #[inline(always)]
    pub fn divr3(&mut self) -> DIVR3_W<PLL3DIVR1rs> {
        DIVR3_W::new(self, 24)
    }
}
/**RCC PLL3 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll3divr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:PLL3DIVR1)*/
pub struct PLL3DIVR1rs;
impl crate::RegisterSpec for PLL3DIVR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll3divr1::R`](R) reader structure
impl crate::Readable for PLL3DIVR1rs {}
///`write(|w| ..)` method takes [`pll3divr1::W`](W) writer structure
impl crate::Writable for PLL3DIVR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL3DIVR1 to value 0x0101_0280
impl crate::Resettable for PLL3DIVR1rs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
