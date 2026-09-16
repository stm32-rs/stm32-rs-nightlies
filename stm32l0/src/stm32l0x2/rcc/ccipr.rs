///Register `CCIPR` reader
pub type R = crate::R<CCIPRrs>;
///Register `CCIPR` writer
pub type W = crate::W<CCIPRrs>;
/**USART1 clock source selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: System clock selected as peripheral clock
    System = 1,
    ///2: HSI16 clock selected as peripheral clock
    Hsi16 = 2,
    ///3: LSE clock selected as peripheral clock
    Lse = 3,
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
///Field `USART1SEL` reader - USART1 clock source selection bits
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1SEL {
        match self.bits {
            0 => USART1SEL::Apb,
            1 => USART1SEL::System,
            2 => USART1SEL::Hsi16,
            3 => USART1SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == USART1SEL::Apb
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == USART1SEL::System
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL::Hsi16
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
///Field `USART1SEL` writer - USART1 clock source selection bits
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SEL, crate::Safe>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Apb)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::System)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi16)
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
///Field `USART2SEL` reader - USART2 clock source selection bits
pub use USART1SEL_R as USART2SEL_R;
///Field `LPUART1SEL` reader - LPUART1 clock source selection bits
pub use USART1SEL_R as LPUART1SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection bits
pub use USART1SEL_W as USART2SEL_W;
///Field `LPUART1SEL` writer - LPUART1 clock source selection bits
pub use USART1SEL_W as LPUART1SEL_W;
/**I2C1 clock source selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    ///0: APB clock selected as peripheral clock
    Apb = 0,
    ///1: System clock selected as peripheral clock
    System = 1,
    ///2: HSI16 clock selected as peripheral clock
    Hsi16 = 2,
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
///Field `I2C1SEL` reader - I2C1 clock source selection bits
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::Apb),
            1 => Some(I2C1SEL::System),
            2 => Some(I2C1SEL::Hsi16),
            _ => None,
        }
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL::Apb
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C1SEL::System
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL::Hsi16
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection bits
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Apb)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::System)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi16)
    }
}
///Field `I2C3SEL` reader - I2C3 clock source selection bits
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C3SEL` writer - I2C3 clock source selection bits
pub use I2C1SEL_W as I2C3SEL_W;
/**Low Power Timer clock source selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: APB clock selected as Timer clock
    Apb = 0,
    ///1: LSI clock selected as Timer clock
    Lsi = 1,
    ///2: HSI16 clock selected as Timer clock
    Hsi16 = 2,
    ///3: LSE clock selected as Timer clock
    Lse = 3,
}
impl From<LPTIM1SEL> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM1SEL {}
///Field `LPTIM1SEL` reader - Low Power Timer clock source selection bits
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SEL {
        match self.bits {
            0 => LPTIM1SEL::Apb,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi16,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///APB clock selected as Timer clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == LPTIM1SEL::Apb
    }
    ///LSI clock selected as Timer clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI16 clock selected as Timer clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    ///LSE clock selected as Timer clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
///Field `LPTIM1SEL` writer - Low Power Timer clock source selection bits
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM1SEL, crate::Safe>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as Timer clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb)
    }
    ///LSI clock selected as Timer clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI16 clock selected as Timer clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    ///LSE clock selected as Timer clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
///Field `HSI48MSEL` reader - 48 MHz HSI48 clock source selection bit
pub type HSI48MSEL_R = crate::BitReader;
///Field `HSI48MSEL` writer - 48 MHz HSI48 clock source selection bit
pub type HSI48MSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 26 - 48 MHz HSI48 clock source selection bit
    #[inline(always)]
    pub fn hsi48msel(&self) -> HSI48MSEL_R {
        HSI48MSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("hsi48msel", &self.hsi48msel())
            .field("lptim1sel", &self.lptim1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("usart1sel", &self.usart1sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("i2c3sel", &self.i2c3sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    ///Bit 26 - 48 MHz HSI48 clock source selection bit
    #[inline(always)]
    pub fn hsi48msel(&mut self) -> HSI48MSEL_W<'_, CCIPRrs> {
        HSI48MSEL_W::new(self, 26)
    }
}
/**Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#RCC:CCIPR)*/
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
///`read()` method returns [`ccipr::R`](R) reader structure
impl crate::Readable for CCIPRrs {}
///`write(|w| ..)` method takes [`ccipr::W`](W) writer structure
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPRrs {}
