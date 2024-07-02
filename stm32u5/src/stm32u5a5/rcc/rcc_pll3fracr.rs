///Register `RCC_PLL3FRACR` reader
pub type R = crate::R<RCC_PLL3FRACRrs>;
///Register `RCC_PLL3FRACR` writer
pub type W = crate::W<RCC_PLL3FRACRrs>;
///Field `PLL3FRACN` reader - Fractional part of the multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x (PLL3N + (PLL3FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref3_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
pub type PLL3FRACN_R = crate::FieldReader<u16>;
///Field `PLL3FRACN` writer - Fractional part of the multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x (PLL3N + (PLL3FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref3_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
pub type PLL3FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x (PLL3N + (PLL3FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref3_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
    #[inline(always)]
    pub fn pll3fracn(&self) -> PLL3FRACN_R {
        PLL3FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3FRACR")
            .field("pll3fracn", &self.pll3fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = F&lt;sub>ref3_ck&lt;/sub> x (PLL3N + (PLL3FRACN / 2&lt;sup>13&lt;/sup>)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 2&lt;sup>13 &lt;/sup>- 1. The input frequency F&lt;sub>ref3_ck&lt;/sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
    #[inline(always)]
    #[must_use]
    pub fn pll3fracn(&mut self) -> PLL3FRACN_W<RCC_PLL3FRACRrs> {
        PLL3FRACN_W::new(self, 3)
    }
}
/**RCC PLL3 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:RCC_PLL3FRACR)*/
pub struct RCC_PLL3FRACRrs;
impl crate::RegisterSpec for RCC_PLL3FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_pll3fracr::R`](R) reader structure
impl crate::Readable for RCC_PLL3FRACRrs {}
///`write(|w| ..)` method takes [`rcc_pll3fracr::W`](W) writer structure
impl crate::Writable for RCC_PLL3FRACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_PLL3FRACR to value 0
impl crate::Resettable for RCC_PLL3FRACRrs {
    const RESET_VALUE: u32 = 0;
}
