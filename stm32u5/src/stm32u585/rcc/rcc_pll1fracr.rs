///Register `RCC_PLL1FRACR` reader
pub type R = crate::R<RCC_PLL1FRACRrs>;
///Register `RCC_PLL1FRACR` writer
pub type W = crate::W<RCC_PLL1FRACRrs>;
///Field `PLL1FRACN` reader - Fractional part of the multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x (PLL1N + (PLL1FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 2&lt;sup>13&lt;/sup>- 1. The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as�follows: Set PLL1FRACEN = 0. Write the new fractional value into PLL1FRACN. Set PLL1FRACEN = 1.
pub type PLL1FRACN_R = crate::FieldReader<u16>;
///Field `PLL1FRACN` writer - Fractional part of the multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x (PLL1N + (PLL1FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 2&lt;sup>13&lt;/sup>- 1. The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as�follows: Set PLL1FRACEN = 0. Write the new fractional value into PLL1FRACN. Set PLL1FRACEN = 1.
pub type PLL1FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x (PLL1N + (PLL1FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 2&lt;sup>13&lt;/sup>- 1. The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as�follows: Set PLL1FRACEN = 0. Write the new fractional value into PLL1FRACN. Set PLL1FRACEN = 1.
    #[inline(always)]
    pub fn pll1fracn(&self) -> PLL1FRACN_R {
        PLL1FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL1FRACR")
            .field("pll1fracn", &self.pll1fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F&lt;sub>ref1_ck&lt;/sub> x (PLL1N + (PLL1FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 2&lt;sup>13&lt;/sup>- 1. The input frequency F&lt;sub>ref1_ck&lt;/sub> must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as�follows: Set PLL1FRACEN = 0. Write the new fractional value into PLL1FRACN. Set PLL1FRACEN = 1.
    #[inline(always)]
    #[must_use]
    pub fn pll1fracn(&mut self) -> PLL1FRACN_W<RCC_PLL1FRACRrs> {
        PLL1FRACN_W::new(self, 3)
    }
}
/**RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RCC:RCC_PLL1FRACR)*/
pub struct RCC_PLL1FRACRrs;
impl crate::RegisterSpec for RCC_PLL1FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll1fracr::R`](R) reader structure
impl crate::Readable for RCC_PLL1FRACRrs {}
///`write(|w| ..)` method takes [`rcc_pll1fracr::W`](W) writer structure
impl crate::Writable for RCC_PLL1FRACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL1FRACR to value 0
impl crate::Resettable for RCC_PLL1FRACRrs {
    const RESET_VALUE: u32 = 0;
}
