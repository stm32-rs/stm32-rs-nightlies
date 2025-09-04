///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
/**USART1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SW {
    ///0: PCLK selected as USART clock source
    Pclk = 0,
    ///1: SYSCLK selected as USART clock source
    Sysclk = 1,
    ///2: LSE selected as USART clock source
    Lse = 2,
    ///3: HSI selected as USART clock source
    Hsi = 3,
}
impl From<USART1SW> for u8 {
    #[inline(always)]
    fn from(variant: USART1SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SW {
    type Ux = u8;
}
impl crate::IsEnum for USART1SW {}
///Field `USART1SW` reader - USART1 clock source selection
pub type USART1SW_R = crate::FieldReader<USART1SW>;
impl USART1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1SW {
        match self.bits {
            0 => USART1SW::Pclk,
            1 => USART1SW::Sysclk,
            2 => USART1SW::Lse,
            3 => USART1SW::Hsi,
            _ => unreachable!(),
        }
    }
    ///PCLK selected as USART clock source
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SW::Pclk
    }
    ///SYSCLK selected as USART clock source
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW::Sysclk
    }
    ///LSE selected as USART clock source
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW::Lse
    }
    ///HSI selected as USART clock source
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW::Hsi
    }
}
///Field `USART1SW` writer - USART1 clock source selection
pub type USART1SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1SW, crate::Safe>;
impl<'a, REG> USART1SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK selected as USART clock source
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Pclk)
    }
    ///SYSCLK selected as USART clock source
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Sysclk)
    }
    ///LSE selected as USART clock source
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Lse)
    }
    ///HSI selected as USART clock source
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Hsi)
    }
}
/**I2C1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SW {
    ///0: HSI clock selected as I2C clock source
    Hsi = 0,
    ///1: SYSCLK clock selected as I2C clock source
    Sysclk = 1,
}
impl From<I2C1SW> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1SW` reader - I2C1 clock source selection
pub type I2C1SW_R = crate::BitReader<I2C1SW>;
impl I2C1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SW {
        match self.bits {
            false => I2C1SW::Hsi,
            true => I2C1SW::Sysclk,
        }
    }
    ///HSI clock selected as I2C clock source
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SW::Hsi
    }
    ///SYSCLK clock selected as I2C clock source
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW::Sysclk
    }
}
///Field `I2C1SW` writer - I2C1 clock source selection
pub type I2C1SW_W<'a, REG> = crate::BitWriter<'a, REG, I2C1SW>;
impl<'a, REG> I2C1SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI clock selected as I2C clock source
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SW::Hsi)
    }
    ///SYSCLK clock selected as I2C clock source
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SW::Sysclk)
    }
}
///Field `I2C2SW` reader - I2C2 clock source selection
pub use I2C1SW_R as I2C2SW_R;
///Field `I2C3SW` reader - I2C3 clock source selection
pub use I2C1SW_R as I2C3SW_R;
///Field `I2C2SW` writer - I2C2 clock source selection
pub use I2C1SW_W as I2C2SW_W;
///Field `I2C3SW` writer - I2C3 clock source selection
pub use I2C1SW_W as I2C3SW_W;
/**Timer1 clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SW {
    ///0: PCLK2 clock (doubled frequency when prescaled)
    Pclk2 = 0,
    ///1: PLL vco output (running up to 144 MHz)
    Pll = 1,
}
impl From<TIM1SW> for bool {
    #[inline(always)]
    fn from(variant: TIM1SW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1SW` reader - Timer1 clock source selection
pub type TIM1SW_R = crate::BitReader<TIM1SW>;
impl TIM1SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1SW {
        match self.bits {
            false => TIM1SW::Pclk2,
            true => TIM1SW::Pll,
        }
    }
    ///PCLK2 clock (doubled frequency when prescaled)
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == TIM1SW::Pclk2
    }
    ///PLL vco output (running up to 144 MHz)
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TIM1SW::Pll
    }
}
///Field `TIM1SW` writer - Timer1 clock source selection
pub type TIM1SW_W<'a, REG> = crate::BitWriter<'a, REG, TIM1SW>;
impl<'a, REG> TIM1SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCLK2 clock (doubled frequency when prescaled)
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SW::Pclk2)
    }
    ///PLL vco output (running up to 144 MHz)
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SW::Pll)
    }
}
///Field `TIM8SW` reader - Timer8 clock source selection
pub use TIM1SW_R as TIM8SW_R;
///Field `TIM15SW` reader - Timer15 clock source selection
pub use TIM1SW_R as TIM15SW_R;
///Field `TIM16SW` reader - Timer16 clock source selection
pub use TIM1SW_R as TIM16SW_R;
///Field `TIM17SW` reader - Timer17 clock source selection
pub use TIM1SW_R as TIM17SW_R;
///Field `TIM20SW` reader - Timer20 clock source selection
pub use TIM1SW_R as TIM20SW_R;
///Field `TIM2SW` reader - Timer2 clock source selection
pub use TIM1SW_R as TIM2SW_R;
///Field `TIM34SW` reader - Timer34 clock source selection
pub use TIM1SW_R as TIM34SW_R;
///Field `TIM8SW` writer - Timer8 clock source selection
pub use TIM1SW_W as TIM8SW_W;
///Field `TIM15SW` writer - Timer15 clock source selection
pub use TIM1SW_W as TIM15SW_W;
///Field `TIM16SW` writer - Timer16 clock source selection
pub use TIM1SW_W as TIM16SW_W;
///Field `TIM17SW` writer - Timer17 clock source selection
pub use TIM1SW_W as TIM17SW_W;
///Field `TIM20SW` writer - Timer20 clock source selection
pub use TIM1SW_W as TIM20SW_W;
///Field `TIM2SW` writer - Timer2 clock source selection
pub use TIM1SW_W as TIM2SW_W;
///Field `TIM34SW` writer - Timer34 clock source selection
pub use TIM1SW_W as TIM34SW_W;
///Field `USART2SW` reader - USART2 clock source selection
pub use USART1SW_R as USART2SW_R;
///Field `USART3SW` reader - USART3 clock source selection
pub use USART1SW_R as USART3SW_R;
///Field `UART4SW` reader - UART4 clock source selection
pub use USART1SW_R as UART4SW_R;
///Field `UART5SW` reader - UART5 clock source selection
pub use USART1SW_R as UART5SW_R;
///Field `USART2SW` writer - USART2 clock source selection
pub use USART1SW_W as USART2SW_W;
///Field `USART3SW` writer - USART3 clock source selection
pub use USART1SW_W as USART3SW_W;
///Field `UART4SW` writer - UART4 clock source selection
pub use USART1SW_W as UART4SW_W;
///Field `UART5SW` writer - UART5 clock source selection
pub use USART1SW_W as UART5SW_W;
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sw(&self) -> I2C2SW_R {
        I2C2SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sw(&self) -> I2C3SW_R {
        I2C3SW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Timer1 clock source selection
    #[inline(always)]
    pub fn tim1sw(&self) -> TIM1SW_R {
        TIM1SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer8 clock source selection
    #[inline(always)]
    pub fn tim8sw(&self) -> TIM8SW_R {
        TIM8SW_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Timer15 clock source selection
    #[inline(always)]
    pub fn tim15sw(&self) -> TIM15SW_R {
        TIM15SW_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Timer16 clock source selection
    #[inline(always)]
    pub fn tim16sw(&self) -> TIM16SW_R {
        TIM16SW_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Timer17 clock source selection
    #[inline(always)]
    pub fn tim17sw(&self) -> TIM17SW_R {
        TIM17SW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Timer20 clock source selection
    #[inline(always)]
    pub fn tim20sw(&self) -> TIM20SW_R {
        TIM20SW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sw(&self) -> UART4SW_R {
        UART4SW_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sw(&self) -> UART5SW_R {
        UART5SW_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Timer2 clock source selection
    #[inline(always)]
    pub fn tim2sw(&self) -> TIM2SW_R {
        TIM2SW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer34 clock source selection
    #[inline(always)]
    pub fn tim34sw(&self) -> TIM34SW_R {
        TIM34SW_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("usart1sw", &self.usart1sw())
            .field("i2c1sw", &self.i2c1sw())
            .field("i2c2sw", &self.i2c2sw())
            .field("i2c3sw", &self.i2c3sw())
            .field("usart2sw", &self.usart2sw())
            .field("usart3sw", &self.usart3sw())
            .field("tim1sw", &self.tim1sw())
            .field("tim8sw", &self.tim8sw())
            .field("uart4sw", &self.uart4sw())
            .field("uart5sw", &self.uart5sw())
            .field("tim20sw", &self.tim20sw())
            .field("tim15sw", &self.tim15sw())
            .field("tim16sw", &self.tim16sw())
            .field("tim17sw", &self.tim17sw())
            .field("tim2sw", &self.tim2sw())
            .field("tim34sw", &self.tim34sw())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sw(&mut self) -> USART1SW_W<CFGR3rs> {
        USART1SW_W::new(self, 0)
    }
    ///Bit 4 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<CFGR3rs> {
        I2C1SW_W::new(self, 4)
    }
    ///Bit 5 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sw(&mut self) -> I2C2SW_W<CFGR3rs> {
        I2C2SW_W::new(self, 5)
    }
    ///Bit 6 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sw(&mut self) -> I2C3SW_W<CFGR3rs> {
        I2C3SW_W::new(self, 6)
    }
    ///Bit 8 - Timer1 clock source selection
    #[inline(always)]
    pub fn tim1sw(&mut self) -> TIM1SW_W<CFGR3rs> {
        TIM1SW_W::new(self, 8)
    }
    ///Bit 9 - Timer8 clock source selection
    #[inline(always)]
    pub fn tim8sw(&mut self) -> TIM8SW_W<CFGR3rs> {
        TIM8SW_W::new(self, 9)
    }
    ///Bit 10 - Timer15 clock source selection
    #[inline(always)]
    pub fn tim15sw(&mut self) -> TIM15SW_W<CFGR3rs> {
        TIM15SW_W::new(self, 10)
    }
    ///Bit 11 - Timer16 clock source selection
    #[inline(always)]
    pub fn tim16sw(&mut self) -> TIM16SW_W<CFGR3rs> {
        TIM16SW_W::new(self, 11)
    }
    ///Bit 13 - Timer17 clock source selection
    #[inline(always)]
    pub fn tim17sw(&mut self) -> TIM17SW_W<CFGR3rs> {
        TIM17SW_W::new(self, 13)
    }
    ///Bit 15 - Timer20 clock source selection
    #[inline(always)]
    pub fn tim20sw(&mut self) -> TIM20SW_W<CFGR3rs> {
        TIM20SW_W::new(self, 15)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sw(&mut self) -> USART2SW_W<CFGR3rs> {
        USART2SW_W::new(self, 16)
    }
    ///Bits 18:19 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sw(&mut self) -> USART3SW_W<CFGR3rs> {
        USART3SW_W::new(self, 18)
    }
    ///Bits 20:21 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sw(&mut self) -> UART4SW_W<CFGR3rs> {
        UART4SW_W::new(self, 20)
    }
    ///Bits 22:23 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sw(&mut self) -> UART5SW_W<CFGR3rs> {
        UART5SW_W::new(self, 22)
    }
    ///Bit 24 - Timer2 clock source selection
    #[inline(always)]
    pub fn tim2sw(&mut self) -> TIM2SW_W<CFGR3rs> {
        TIM2SW_W::new(self, 24)
    }
    ///Bit 25 - Timer34 clock source selection
    #[inline(always)]
    pub fn tim34sw(&mut self) -> TIM34SW_W<CFGR3rs> {
        TIM34SW_W::new(self, 25)
    }
}
/**Clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#RCC:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
