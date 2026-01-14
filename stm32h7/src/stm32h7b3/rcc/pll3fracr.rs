///Register `PLL3FRACR` reader
pub type R = crate::R<PLL3FRACRrs>;
///Register `PLL3FRACR` writer
pub type W = crate::W<PLL3FRACRrs>;
///Field `FRACN3` reader - fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560MHz if PLL3VCOSEL = 0 150 to 420MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x (DIVN3 + (FRACN3 / 213)), with DIVN3 between 8 and 420 FRACN3 can be between 0 and 213 - 1 The input frequency Fref3_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into FRACN1. Set the bit PLL1FRACEN to 1.
pub type FRACN3_R = crate::FieldReader<u16>;
///Field `FRACN3` writer - fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560MHz if PLL3VCOSEL = 0 150 to 420MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x (DIVN3 + (FRACN3 / 213)), with DIVN3 between 8 and 420 FRACN3 can be between 0 and 213 - 1 The input frequency Fref3_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into FRACN1. Set the bit PLL1FRACEN to 1.
pub type FRACN3_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 3:15 - fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560MHz if PLL3VCOSEL = 0 150 to 420MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x (DIVN3 + (FRACN3 / 213)), with DIVN3 between 8 and 420 FRACN3 can be between 0 and 213 - 1 The input frequency Fref3_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into FRACN1. Set the bit PLL1FRACEN to 1.
    #[inline(always)]
    pub fn fracn3(&self) -> FRACN3_R {
        FRACN3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3FRACR")
            .field("fracn3", &self.fracn3())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560MHz if PLL3VCOSEL = 0 150 to 420MHz if PLL3VCOSEL = 1 VCO output frequency = Fref3_ck x (DIVN3 + (FRACN3 / 213)), with DIVN3 between 8 and 420 FRACN3 can be between 0 and 213 - 1 The input frequency Fref3_ck must be between 1 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into FRACN1. Set the bit PLL1FRACEN to 1.
    #[inline(always)]
    pub fn fracn3(&mut self) -> FRACN3_W<'_, PLL3FRACRrs> {
        FRACN3_W::new(self, 3)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:PLL3FRACR)*/
pub struct PLL3FRACRrs;
impl crate::RegisterSpec for PLL3FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3fracr::R`](R) reader structure
impl crate::Readable for PLL3FRACRrs {}
///`write(|w| ..)` method takes [`pll3fracr::W`](W) writer structure
impl crate::Writable for PLL3FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3FRACR to value 0
impl crate::Resettable for PLL3FRACRrs {}
