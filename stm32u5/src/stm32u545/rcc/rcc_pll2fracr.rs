///Register `RCC_PLL2FRACR` reader
pub type R = crate::R<RCC_PLL2FRACRrs>;
///Register `RCC_PLL2FRACR` writer
pub type W = crate::W<RCC_PLL2FRACRrs>;
///Field `PLL2FRACN` reader - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x (PLL2N + (PLL2FRACN / 2&lt;sup>13&lt;/sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref2_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
pub type PLL2FRACN_R = crate::FieldReader<u16>;
///Field `PLL2FRACN` writer - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x (PLL2N + (PLL2FRACN / 2&lt;sup>13&lt;/sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref2_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
pub type PLL2FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x (PLL2N + (PLL2FRACN / 2&lt;sup>13&lt;/sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref2_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
    #[inline(always)]
    pub fn pll2fracn(&self) -> PLL2FRACN_R {
        PLL2FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2FRACR")
            .field("pll2fracn", &self.pll2fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F&lt;sub>ref2_ck&lt;/sub> x (PLL2N + (PLL2FRACN / 2&lt;sup>13&lt;/sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref2_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
    #[inline(always)]
    #[must_use]
    pub fn pll2fracn(&mut self) -> PLL2FRACN_W<RCC_PLL2FRACRrs> {
        PLL2FRACN_W::new(self, 3)
    }
}
/**RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll2fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll2fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:RCC_PLL2FRACR)*/
pub struct RCC_PLL2FRACRrs;
impl crate::RegisterSpec for RCC_PLL2FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll2fracr::R`](R) reader structure
impl crate::Readable for RCC_PLL2FRACRrs {}
///`write(|w| ..)` method takes [`rcc_pll2fracr::W`](W) writer structure
impl crate::Writable for RCC_PLL2FRACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL2FRACR to value 0
impl crate::Resettable for RCC_PLL2FRACRrs {
    const RESET_VALUE: u32 = 0;
}
