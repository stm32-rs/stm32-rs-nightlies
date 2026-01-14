///Register `DCKCFGR2` reader
pub type R = crate::R<DCKCFGR2rs>;
///Register `DCKCFGR2` writer
pub type W = crate::W<DCKCFGR2rs>;
/**FMPI2C1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMPI2C1SEL {
    ///0: APB clock selected as I2C clock
    Apb = 0,
    ///1: System clock selected as I2C clock
    Sysclk = 1,
    ///2: HSI clock selected as I2C clock
    Hsi = 2,
}
impl From<FMPI2C1SEL> for u8 {
    #[inline(always)]
    fn from(variant: FMPI2C1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMPI2C1SEL {
    type Ux = u8;
}
impl crate::IsEnum for FMPI2C1SEL {}
///Field `FMPI2C1SEL` reader - FMPI2C1 kernel clock source selection
pub type FMPI2C1SEL_R = crate::FieldReader<FMPI2C1SEL>;
impl FMPI2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FMPI2C1SEL> {
        match self.bits {
            0 => Some(FMPI2C1SEL::Apb),
            1 => Some(FMPI2C1SEL::Sysclk),
            2 => Some(FMPI2C1SEL::Hsi),
            _ => None,
        }
    }
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == FMPI2C1SEL::Apb
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == FMPI2C1SEL::Sysclk
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == FMPI2C1SEL::Hsi
    }
}
///Field `FMPI2C1SEL` writer - FMPI2C1 kernel clock source selection
pub type FMPI2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMPI2C1SEL>;
impl<'a, REG> FMPI2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Apb)
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Sysclk)
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1SEL::Hsi)
    }
}
/**LPTIM1SEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL {
    ///0: APB1 clock (PCLK1) selected as LPTILM1 clock
    Apb1 = 0,
    ///1: LSI clock is selected as LPTILM1 clock
    Lsi = 1,
    ///2: HSI clock is selected as LPTILM1 clock
    Hsi = 2,
    ///3: LSE clock is selected as LPTILM1 clock
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
///Field `LPTIM1SEL` reader - LPTIM1SEL
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL>;
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SEL {
        match self.bits {
            0 => LPTIM1SEL::Apb1,
            1 => LPTIM1SEL::Lsi,
            2 => LPTIM1SEL::Hsi,
            3 => LPTIM1SEL::Lse,
            _ => unreachable!(),
        }
    }
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL::Apb1
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL::Lsi
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL::Hsi
    }
    ///LSE clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL::Lse
    }
}
///Field `LPTIM1SEL` writer - LPTIM1SEL
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPTIM1SEL, crate::Safe>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Apb1)
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lsi)
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Hsi)
    }
    ///LSE clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL::Lse)
    }
}
impl R {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 30:31 - LPTIM1SEL
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCKCFGR2")
            .field("fmpi2c1sel", &self.fmpi2c1sel())
            .field("lptim1sel", &self.lptim1sel())
            .finish()
    }
}
impl W {
    ///Bits 22:23 - FMPI2C1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<'_, DCKCFGR2rs> {
        FMPI2C1SEL_W::new(self, 22)
    }
    ///Bits 30:31 - LPTIM1SEL
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, DCKCFGR2rs> {
        LPTIM1SEL_W::new(self, 30)
    }
}
/**DCKCFGR2 register

You can [`read`](crate::Reg::read) this register and get [`dckcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:DCKCFGR2)*/
pub struct DCKCFGR2rs;
impl crate::RegisterSpec for DCKCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`dckcfgr2::R`](R) reader structure
impl crate::Readable for DCKCFGR2rs {}
///`write(|w| ..)` method takes [`dckcfgr2::W`](W) writer structure
impl crate::Writable for DCKCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCKCFGR2 to value 0
impl crate::Resettable for DCKCFGR2rs {}
