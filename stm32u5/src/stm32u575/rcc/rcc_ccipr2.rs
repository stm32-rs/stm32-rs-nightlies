///Register `RCC_CCIPR2` reader
pub type R = crate::R<RCC_CCIPR2rs>;
///Register `RCC_CCIPR2` writer
pub type W = crate::W<RCC_CCIPR2rs>;
///Field `MDF1SEL` reader - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_R = crate::FieldReader;
///Field `MDF1SEL` writer - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAI1SEL` reader - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_R = crate::FieldReader;
///Field `SAI1SEL` writer - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAI2SEL` reader - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type SAI2SEL_R = crate::FieldReader;
///Field `SAI2SEL` writer - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type SAI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAESSEL` reader - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAESSEL_R = crate::BitReader;
///Field `SAESSEL` writer - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SAESSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEL` reader - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_R = crate::FieldReader;
///Field `RNGSEL` writer - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDMMCSEL` reader - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
pub type SDMMCSEL_R = crate::BitReader;
///Field `SDMMCSEL` writer - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSISEL` reader - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub type DSISEL_R = crate::BitReader;
///Field `DSISEL` writer - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6SEL` reader - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART6SEL_R = crate::FieldReader;
///Field `USART6SEL` writer - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LTDCSEL` reader - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCSEL_R = crate::BitReader;
///Field `LTDCSEL` writer - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type LTDCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPISEL` reader - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_R = crate::FieldReader;
///Field `OCTOSPISEL` writer - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSPI1SEL` reader - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type HSPI1SEL_R = crate::FieldReader;
///Field `HSPI1SEL` writer - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type HSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C5SEL` reader - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type I2C5SEL_R = crate::FieldReader;
///Field `I2C5SEL` writer - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type I2C5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C6SEL` reader - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type I2C6SEL_R = crate::FieldReader;
///Field `I2C6SEL` writer - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type I2C6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OTGHSSEL` reader - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type OTGHSSEL_R = crate::FieldReader;
///Field `OTGHSSEL` writer - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type OTGHSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    pub fn mdf1sel(&self) -> MDF1SEL_R {
        MDF1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn saessel(&self) -> SAESSEL_R {
        SAESSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn ltdcsel(&self) -> LTDCSEL_R {
        LTDCSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1sel(&self) -> HSPI1SEL_R {
        HSPI1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c5sel(&self) -> I2C5SEL_R {
        I2C5SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn i2c6sel(&self) -> I2C6SEL_R {
        I2C6SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn otghssel(&self) -> OTGHSSEL_R {
        OTGHSSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR2")
            .field("mdf1sel", &self.mdf1sel())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .field("saessel", &self.saessel())
            .field("rngsel", &self.rngsel())
            .field("sdmmcsel", &self.sdmmcsel())
            .field("dsisel", &self.dsisel())
            .field("usart6sel", &self.usart6sel())
            .field("ltdcsel", &self.ltdcsel())
            .field("octospisel", &self.octospisel())
            .field("hspi1sel", &self.hspi1sel())
            .field("i2c5sel", &self.i2c5sel())
            .field("i2c6sel", &self.i2c6sel())
            .field("otghssel", &self.otghssel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    #[must_use]
    pub fn mdf1sel(&mut self) -> MDF1SEL_W<RCC_CCIPR2rs> {
        MDF1SEL_W::new(self, 0)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<RCC_CCIPR2rs> {
        SAI1SEL_W::new(self, 5)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<RCC_CCIPR2rs> {
        SAI2SEL_W::new(self, 8)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn saessel(&mut self) -> SAESSEL_W<RCC_CCIPR2rs> {
        SAESSEL_W::new(self, 11)
    }
    ///Bits 12:13 - RNG kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<RCC_CCIPR2rs> {
        RNGSEL_W::new(self, 12)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change it only after reset and before enabling the SDMMC.
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<RCC_CCIPR2rs> {
        SDMMCSEL_W::new(self, 14)
    }
    ///Bit 15 - DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn dsisel(&mut self) -> DSISEL_W<RCC_CCIPR2rs> {
        DSISEL_W::new(self, 15)
    }
    ///Bits 16:17 - USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart6sel(&mut self) -> USART6SEL_W<RCC_CCIPR2rs> {
        USART6SEL_W::new(self, 16)
    }
    ///Bit 18 - LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn ltdcsel(&mut self) -> LTDCSEL_W<RCC_CCIPR2rs> {
        LTDCSEL_W::new(self, 18)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W<RCC_CCIPR2rs> {
        OCTOSPISEL_W::new(self, 20)
    }
    ///Bits 22:23 - HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn hspi1sel(&mut self) -> HSPI1SEL_W<RCC_CCIPR2rs> {
        HSPI1SEL_W::new(self, 22)
    }
    ///Bits 24:25 - I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c5sel(&mut self) -> I2C5SEL_W<RCC_CCIPR2rs> {
        I2C5SEL_W::new(self, 24)
    }
    ///Bits 26:27 - I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn i2c6sel(&mut self) -> I2C6SEL_W<RCC_CCIPR2rs> {
        I2C6SEL_W::new(self, 26)
    }
    ///Bits 30:31 - OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn otghssel(&mut self) -> OTGHSSEL_W<RCC_CCIPR2rs> {
        OTGHSSEL_W::new(self, 30)
    }
}
/**RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_CCIPR2)*/
pub struct RCC_CCIPR2rs;
impl crate::RegisterSpec for RCC_CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ccipr2::R`](R) reader structure
impl crate::Readable for RCC_CCIPR2rs {}
///`write(|w| ..)` method takes [`rcc_ccipr2::W`](W) writer structure
impl crate::Writable for RCC_CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CCIPR2 to value 0
impl crate::Resettable for RCC_CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
