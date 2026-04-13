///Register `RCC_CCIPR1` reader
pub type R = crate::R<RCC_CCIPR1rs>;
///Register `RCC_CCIPR1` writer
pub type W = crate::W<RCC_CCIPR1rs>;
/**USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: PCLK
    B0x0 = 0,
    ///1: SYSCLK
    B0x1 = 1,
    ///2: HSIKER
    B0x2 = 2,
    ///3: LSE
    B0x3 = 3,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART1SEL {}
///Field `USART1SEL` reader - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1SEL {
        match self.bits {
            0 => USART1SEL::B0x0,
            1 => USART1SEL::B0x1,
            2 => USART1SEL::B0x2,
            3 => USART1SEL::B0x3,
            _ => unreachable!(),
        }
    }
    ///PCLK
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1SEL::B0x0
    }
    ///SYSCLK
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1SEL::B0x1
    }
    ///HSIKER
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == USART1SEL::B0x2
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == USART1SEL::B0x3
    }
}
///Field `USART1SEL` writer - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SEL, crate::Safe>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::B0x0)
    }
    ///SYSCLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::B0x1)
    }
    ///HSIKER
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::B0x2)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::B0x3)
    }
}
/**I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    ///0: PCLK
    B0x0 = 0,
    ///1: SYSCLK
    B0x1 = 1,
    ///2: HSIKER
    B0x2 = 2,
}
impl From<I2C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2C1SEL {}
///Field `I2C1SEL` reader - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::B0x0),
            1 => Some(I2C1SEL::B0x1),
            2 => Some(I2C1SEL::B0x2),
            _ => None,
        }
    }
    ///PCLK
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1SEL::B0x0
    }
    ///SYSCLK
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1SEL::B0x1
    }
    ///HSIKER
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C1SEL::B0x2
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::B0x0)
    }
    ///SYSCLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::B0x1)
    }
    ///HSIKER
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::B0x2)
    }
}
/**I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SEL {
    ///0: SYSCLK
    B0x0 = 0,
    ///2: HSIKER
    B0x2 = 2,
    ///3: I2S_CKIN
    B0x3 = 3,
}
impl From<I2S1SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S1SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2S1SEL {}
///Field `I2S1SEL` reader - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
pub type I2S1SEL_R = crate::FieldReader<I2S1SEL>;
impl I2S1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2S1SEL> {
        match self.bits {
            0 => Some(I2S1SEL::B0x0),
            2 => Some(I2S1SEL::B0x2),
            3 => Some(I2S1SEL::B0x3),
            _ => None,
        }
    }
    ///SYSCLK
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2S1SEL::B0x0
    }
    ///HSIKER
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2S1SEL::B0x2
    }
    ///I2S_CKIN
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2S1SEL::B0x3
    }
}
///Field `I2S1SEL` writer - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2S1SEL>;
impl<'a, REG> I2S1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::B0x0)
    }
    ///HSIKER
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::B0x2)
    }
    ///I2S_CKIN
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::B0x3)
    }
}
/**ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL {
    ///0: System clock
    B0x0 = 0,
    ///2: HSIKER
    B0x2 = 2,
}
impl From<ADCSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADCSEL {}
///Field `ADCSEL` reader - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
pub type ADCSEL_R = crate::FieldReader<ADCSEL>;
impl ADCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCSEL> {
        match self.bits {
            0 => Some(ADCSEL::B0x0),
            2 => Some(ADCSEL::B0x2),
            _ => None,
        }
    }
    ///System clock
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCSEL::B0x0
    }
    ///HSIKER
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ADCSEL::B0x2
    }
}
///Field `ADCSEL` writer - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCSEL>;
impl<'a, REG> ADCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::B0x0)
    }
    ///HSIKER
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSEL::B0x2)
    }
}
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
        f.debug_struct("RCC_CCIPR1")
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
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, RCC_CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, RCC_CCIPR1rs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2S1 clock source selection This bitfield is controlled by software to select I2S1 clock source as follows:
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<'_, RCC_CCIPR1rs> {
        I2S1SEL_W::new(self, 14)
    }
    ///Bits 30:31 - ADCs clock source selection This bitfield is controlled by software to select the asynchronous clock source for ADC:
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, RCC_CCIPR1rs> {
        ADCSEL_W::new(self, 30)
    }
}
/**RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CCIPR1)*/
pub struct RCC_CCIPR1rs;
impl crate::RegisterSpec for RCC_CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ccipr1::R`](R) reader structure
impl crate::Readable for RCC_CCIPR1rs {}
///`write(|w| ..)` method takes [`rcc_ccipr1::W`](W) writer structure
impl crate::Writable for RCC_CCIPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CCIPR1 to value 0
impl crate::Resettable for RCC_CCIPR1rs {}
