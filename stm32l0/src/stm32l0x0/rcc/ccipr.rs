#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPRrs>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPRrs>;
#[doc = "USART1 clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    #[doc = "0: APB clock selected as peripheral clock"]
    Apb = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    System = 1,
    #[doc = "2: HSI16 clock selected as peripheral clock"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected as peripheral clock"]
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
#[doc = "Field `USART1SEL` reader - USART1 clock source selection bits"]
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == USART1SEL::Apb
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == USART1SEL::System
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL::Hsi16
    }
    #[doc = "LSE clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection bits"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Apb)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::System)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Hsi16)
    }
    #[doc = "LSE clock selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection bits"]
pub use USART1SEL_R as USART2SEL_R;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection bits"]
pub use USART1SEL_R as LPUART1SEL_R;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection bits"]
pub use USART1SEL_W as USART2SEL_W;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection bits"]
pub use USART1SEL_W as LPUART1SEL_W;
#[doc = "I2C1 clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL {
    #[doc = "0: APB clock selected as peripheral clock"]
    Apb = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    System = 1,
    #[doc = "2: HSI16 clock selected as peripheral clock"]
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
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection bits"]
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL>;
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1SEL> {
        match self.bits {
            0 => Some(I2C1SEL::Apb),
            1 => Some(I2C1SEL::System),
            2 => Some(I2C1SEL::Hsi16),
            _ => None,
        }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL::Apb
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C1SEL::System
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL::Hsi16
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection bits"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Apb)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::System)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL::Hsi16)
    }
}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection bits"]
pub use I2C1SEL_R as I2C3SEL_R;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection bits"]
pub use I2C1SEL_W as I2C3SEL_W;
#[doc = "Low Power Timer clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    #[doc = "0: APB clock selected as Timer clock"]
    Apb = 0,
    #[doc = "1: LSI clock selected as Timer clock"]
    Lsi = 1,
    #[doc = "2: HSI16 clock selected as Timer clock"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected as Timer clock"]
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
#[doc = "Field `LPTIM1SEL` reader - Low Power Timer clock source selection bits"]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "APB clock selected as Timer clock"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == LPTIM1SEL::Apb
    }
    #[doc = "LSI clock selected as Timer clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    #[doc = "HSI16 clock selected as Timer clock"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL::Hsi16
    }
    #[doc = "LSE clock selected as Timer clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low Power Timer clock source selection bits"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LPTIM1SEL>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APB clock selected as Timer clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb)
    }
    #[doc = "LSI clock selected as Timer clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    #[doc = "HSI16 clock selected as Timer clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi16)
    }
    #[doc = "LSE clock selected as Timer clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
