///Register `DCKCFGR2` reader
pub type R = crate::R<DCKCFGR2rs>;
///Register `DCKCFGR2` writer
pub type W = crate::W<DCKCFGR2rs>;
/**I2CFMP1 kernel clock source selection

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
///Field `FMPI2C1SEL` reader - I2CFMP1 kernel clock source selection
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
///Field `FMPI2C1SEL` writer - I2CFMP1 kernel clock source selection
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
/**SDIO/USBFS clock selection.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL {
    ///0: 48MHz clock from PLL is selected
    Pll = 0,
    ///1: 48MHz clock from PLLSAI is selected
    Pllsai = 1,
}
impl From<CK48MSEL> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CK48MSEL` reader - SDIO/USBFS clock selection.
pub type CK48MSEL_R = crate::BitReader<CK48MSEL>;
impl CK48MSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CK48MSEL {
        match self.bits {
            false => CK48MSEL::Pll,
            true => CK48MSEL::Pllsai,
        }
    }
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL::Pll
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL::Pllsai
    }
}
///Field `CK48MSEL` writer - SDIO/USBFS clock selection.
pub type CK48MSEL_W<'a, REG> = crate::BitWriter<'a, REG, CK48MSEL>;
impl<'a, REG> CK48MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pll)
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut crate::W<REG> {
        self.variant(CK48MSEL::Pllsai)
    }
}
/**SDIO clock selection.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOSEL {
    ///0: 48 MHz clock is selected as SD clock
    Ck48m = 0,
    ///1: System clock is selected as SD clock
    Sysclk = 1,
}
impl From<SDIOSEL> for bool {
    #[inline(always)]
    fn from(variant: SDIOSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIOSEL` reader - SDIO clock selection.
pub type SDIOSEL_R = crate::BitReader<SDIOSEL>;
impl SDIOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDIOSEL {
        match self.bits {
            false => SDIOSEL::Ck48m,
            true => SDIOSEL::Sysclk,
        }
    }
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDIOSEL::Ck48m
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDIOSEL::Sysclk
    }
}
///Field `SDIOSEL` writer - SDIO clock selection.
pub type SDIOSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDIOSEL>;
impl<'a, REG> SDIOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSEL::Ck48m)
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSEL::Sysclk)
    }
}
/**LPTIM1 kernel clock source selection

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
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection
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
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection
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
    ///Bits 22:23 - I2CFMP1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 27 - SDIO/USBFS clock selection.
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDIO clock selection.
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 30:31 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCKCFGR2")
            .field("sdiosel", &self.sdiosel())
            .field("ck48msel", &self.ck48msel())
            .field("fmpi2c1sel", &self.fmpi2c1sel())
            .field("lptim1sel", &self.lptim1sel())
            .finish()
    }
}
impl W {
    ///Bits 22:23 - I2CFMP1 kernel clock source selection
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<'_, DCKCFGR2rs> {
        FMPI2C1SEL_W::new(self, 22)
    }
    ///Bit 27 - SDIO/USBFS clock selection.
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<'_, DCKCFGR2rs> {
        CK48MSEL_W::new(self, 27)
    }
    ///Bit 28 - SDIO clock selection.
    #[inline(always)]
    pub fn sdiosel(&mut self) -> SDIOSEL_W<'_, DCKCFGR2rs> {
        SDIOSEL_W::new(self, 28)
    }
    ///Bits 30:31 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, DCKCFGR2rs> {
        LPTIM1SEL_W::new(self, 30)
    }
}
/**Dedicated Clocks Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dckcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dckcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:DCKCFGR2)*/
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
