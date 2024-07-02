///Register `RCC_CCIPR3` reader
pub type R = crate::R<RCC_CCIPR3rs>;
///Register `RCC_CCIPR3` writer
pub type W = crate::W<RCC_CCIPR3rs>;
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
pub type LPUART1SEL_R = crate::FieldReader;
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI3SEL` reader - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_R = crate::FieldReader;
///Field `SPI3SEL` writer - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C3SEL` reader - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type I2C3SEL_R = crate::FieldReader;
///Field `I2C3SEL` writer - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
pub type I2C3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM34SEL` reader - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
pub type LPTIM34SEL_R = crate::FieldReader;
///Field `LPTIM34SEL` writer - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
pub type LPTIM34SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_R = crate::FieldReader;
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADCDACSEL` reader - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
pub type ADCDACSEL_R = crate::FieldReader;
///Field `ADCDACSEL` writer - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
pub type ADCDACSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DAC1SEL` reader - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
pub type DAC1SEL_R = crate::BitReader;
///Field `DAC1SEL` writer - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
pub type DAC1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1SEL` reader - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_R = crate::FieldReader;
///Field `ADF1SEL` writer - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
pub type ADF1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
    #[inline(always)]
    pub fn lptim34sel(&self) -> LPTIM34SEL_R {
        LPTIM34SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
    #[inline(always)]
    pub fn dac1sel(&self) -> DAC1SEL_R {
        DAC1SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    pub fn adf1sel(&self) -> ADF1SEL_R {
        ADF1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR3")
            .field("lpuart1sel", &self.lpuart1sel())
            .field("spi3sel", &self.spi3sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("lptim34sel", &self.lptim34sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("adcdacsel", &self.adcdacsel())
            .field("dac1sel", &self.dac1sel())
            .field("adf1sel", &self.adf1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16, LSE, or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<RCC_CCIPR3rs> {
        LPUART1SEL_W::new(self, 0)
    }
    ///Bits 3:4 - SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<RCC_CCIPR3rs> {
        SPI3SEL_W::new(self, 3)
    }
    ///Bits 6:7 - I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<RCC_CCIPR3rs> {
        I2C3SEL_W::new(self, 6)
    }
    ///Bits 8:9 - LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON�=�1.
    #[inline(always)]
    #[must_use]
    pub fn lptim34sel(&mut self) -> LPTIM34SEL_W<RCC_CCIPR3rs> {
        LPTIM34SEL_W::new(self, 8)
    }
    ///Bits 10:11 - LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<RCC_CCIPR3rs> {
        LPTIM1SEL_W::new(self, 10)
    }
    ///Bits 12:14 - ADC1, ADC2, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in�Stop 2 mode).
    #[inline(always)]
    #[must_use]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<RCC_CCIPR3rs> {
        ADCDACSEL_W::new(self, 12)
    }
    ///Bit 15 - DAC1 sample-and-hold clock source selection This bit is used to select the DAC1 sample-and-hold clock source.
    #[inline(always)]
    #[must_use]
    pub fn dac1sel(&mut self) -> DAC1SEL_W<RCC_CCIPR3rs> {
        DAC1SEL_W::new(self, 15)
    }
    ///Bits 16:18 - ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn adf1sel(&mut self) -> ADF1SEL_W<RCC_CCIPR3rs> {
        ADF1SEL_W::new(self, 16)
    }
}
/**RCC peripherals independent clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RCC:RCC_CCIPR3)*/
pub struct RCC_CCIPR3rs;
impl crate::RegisterSpec for RCC_CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ccipr3::R`](R) reader structure
impl crate::Readable for RCC_CCIPR3rs {}
///`write(|w| ..)` method takes [`rcc_ccipr3::W`](W) writer structure
impl crate::Writable for RCC_CCIPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CCIPR3 to value 0
impl crate::Resettable for RCC_CCIPR3rs {
    const RESET_VALUE: u32 = 0;
}
