///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
///Field `USART1SEL` reader - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C1SEL` reader - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2S1SEL` reader - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
pub type I2S1SEL_R = crate::FieldReader;
///Field `I2S1SEL` writer - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADCSEL` reader - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
pub type ADCSEL_R = crate::FieldReader;
///Field `ADCSEL` writer - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 30:31 - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("usart1sel", &self.usart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2s1sel", &self.i2s1sel())
            .field("adcsel", &self.adcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPR1rs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<CCIPR1rs> {
        I2S1SEL_W::new(self, 14)
    }
    ///Bits 30:31 - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPR1rs> {
        ADCSEL_W::new(self, 30)
    }
}
/**RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}
