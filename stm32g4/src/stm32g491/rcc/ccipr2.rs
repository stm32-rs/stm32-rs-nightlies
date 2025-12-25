///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
/**I2C4 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL {
    ///0: PCLK clock selected as I2C clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as I2C clock
    System = 1,
    ///2: HSI16 clock selected as I2C clock
    Hsi16 = 2,
}
impl From<I2C4SEL> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C4SEL {
    type Ux = u8;
}
impl crate::IsEnum for I2C4SEL {}
///Field `I2C4SEL` reader - I2C4 clock source selection
pub type I2C4SEL_R = crate::FieldReader<I2C4SEL>;
impl I2C4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C4SEL> {
        match self.bits {
            0 => Some(I2C4SEL::Pclk),
            1 => Some(I2C4SEL::System),
            2 => Some(I2C4SEL::Hsi16),
            _ => None,
        }
    }
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C4SEL::Pclk
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C4SEL::System
    }
    ///HSI16 clock selected as I2C clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C4SEL::Hsi16
    }
}
///Field `I2C4SEL` writer - I2C4 clock source selection
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C4SEL>;
impl<'a, REG> I2C4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Pclk)
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::System)
    }
    ///HSI16 clock selected as I2C clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SEL::Hsi16)
    }
}
/**QUADSPI clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSPISEL {
    ///0: System clock selected as QUADSPI kernel clock
    System = 0,
    ///1: HSI16 clock selected as QUADSPI kernel clock
    Hsi16 = 1,
    ///2: PLL 'Q' clock selected as QUADSPI kernel clock
    Pllq = 2,
}
impl From<QSPISEL> for u8 {
    #[inline(always)]
    fn from(variant: QSPISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QSPISEL {
    type Ux = u8;
}
impl crate::IsEnum for QSPISEL {}
///Field `QSPISEL` reader - QUADSPI clock source selection
pub type QSPISEL_R = crate::FieldReader<QSPISEL>;
impl QSPISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<QSPISEL> {
        match self.bits {
            0 => Some(QSPISEL::System),
            1 => Some(QSPISEL::Hsi16),
            2 => Some(QSPISEL::Pllq),
            _ => None,
        }
    }
    ///System clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == QSPISEL::System
    }
    ///HSI16 clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == QSPISEL::Hsi16
    }
    ///PLL 'Q' clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == QSPISEL::Pllq
    }
}
///Field `QSPISEL` writer - QUADSPI clock source selection
pub type QSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QSPISEL>;
impl<'a, REG> QSPISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn system(self) -> &'a mut crate::W<REG> {
        self.variant(QSPISEL::System)
    }
    ///HSI16 clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(QSPISEL::Hsi16)
    }
    ///PLL 'Q' clock selected as QUADSPI kernel clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(QSPISEL::Pllq)
    }
}
impl R {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 20:21 - QUADSPI clock source selection
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("i2c4sel", &self.i2c4sel())
            .field("qspisel", &self.qspisel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<'_, CCIPR2rs> {
        I2C4SEL_W::new(self, 0)
    }
    ///Bits 20:21 - QUADSPI clock source selection
    #[inline(always)]
    pub fn qspisel(&mut self) -> QSPISEL_W<'_, CCIPR2rs> {
        QSPISEL_W::new(self, 20)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
