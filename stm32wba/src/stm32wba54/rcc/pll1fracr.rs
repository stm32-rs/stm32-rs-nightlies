///Register `PLL1FRACR` reader
pub type R = crate::R<PLL1FRACRrs>;
///Register `PLL1FRACR` writer
pub type W = crate::W<PLL1FRACRrs>;
///Field `PLL1FRACN` reader - Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fsubref1_ck/sub x \[multiplication factor for PLL1 VCO + (PLL1FRACN / 2sup13/sup)\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2sup13/sup- 1. The input frequency Fsubref1_ck/sub must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1.
pub type PLL1FRACN_R = crate::FieldReader<u16>;
///Field `PLL1FRACN` writer - Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fsubref1_ck/sub x \[multiplication factor for PLL1 VCO + (PLL1FRACN / 2sup13/sup)\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2sup13/sup- 1. The input frequency Fsubref1_ck/sub must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1.
pub type PLL1FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fsubref1_ck/sub x \[multiplication factor for PLL1 VCO + (PLL1FRACN / 2sup13/sup)\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2sup13/sup- 1. The input frequency Fsubref1_ck/sub must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1.
    #[inline(always)]
    pub fn pll1fracn(&self) -> PLL1FRACN_R {
        PLL1FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1FRACR")
            .field("pll1fracn", &self.pll1fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fsubref1_ck/sub x \[multiplication factor for PLL1 VCO + (PLL1FRACN / 2sup13/sup)\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2sup13/sup- 1. The input frequency Fsubref1_ck/sub must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1.
    #[inline(always)]
    pub fn pll1fracn(&mut self) -> PLL1FRACN_W<'_, PLL1FRACRrs> {
        PLL1FRACN_W::new(self, 3)
    }
}
/**RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:PLL1FRACR)*/
pub struct PLL1FRACRrs;
impl crate::RegisterSpec for PLL1FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1fracr::R`](R) reader structure
impl crate::Readable for PLL1FRACRrs {}
///`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure
impl crate::Writable for PLL1FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1FRACR to value 0
impl crate::Resettable for PLL1FRACRrs {}
