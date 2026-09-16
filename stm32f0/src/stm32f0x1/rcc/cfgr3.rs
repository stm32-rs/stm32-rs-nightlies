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
/**HDMI CEC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSW {
    ///0: HSI clock divided by 244 selected as CEC clock source
    HsiDiv244 = 0,
    ///1: LSE clock selected as CEC clock source
    Lse = 1,
}
impl From<CECSW> for bool {
    #[inline(always)]
    fn from(variant: CECSW) -> Self {
        variant as u8 != 0
    }
}
///Field `CECSW` reader - HDMI CEC clock source selection
pub type CECSW_R = crate::BitReader<CECSW>;
impl CECSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CECSW {
        match self.bits {
            false => CECSW::HsiDiv244,
            true => CECSW::Lse,
        }
    }
    ///HSI clock divided by 244 selected as CEC clock source
    #[inline(always)]
    pub fn is_hsi_div244(&self) -> bool {
        *self == CECSW::HsiDiv244
    }
    ///LSE clock selected as CEC clock source
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSW::Lse
    }
}
///Field `CECSW` writer - HDMI CEC clock source selection
pub type CECSW_W<'a, REG> = crate::BitWriter<'a, REG, CECSW>;
impl<'a, REG> CECSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI clock divided by 244 selected as CEC clock source
    #[inline(always)]
    pub fn hsi_div244(self) -> &'a mut crate::W<REG> {
        self.variant(CECSW::HsiDiv244)
    }
    ///LSE clock selected as CEC clock source
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSW::Lse)
    }
}
/**USB clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSW {
    ///0: HSI48 selected as USB clock source
    Hsi48 = 0,
    ///1: PLL clock selected as USB clock source
    Pllclk = 1,
}
impl From<USBSW> for bool {
    #[inline(always)]
    fn from(variant: USBSW) -> Self {
        variant as u8 != 0
    }
}
///Field `USBSW` reader - USB clock source selection
pub type USBSW_R = crate::BitReader<USBSW>;
impl USBSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBSW {
        match self.bits {
            false => USBSW::Hsi48,
            true => USBSW::Pllclk,
        }
    }
    ///HSI48 selected as USB clock source
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSW::Hsi48
    }
    ///PLL clock selected as USB clock source
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == USBSW::Pllclk
    }
}
///Field `USBSW` writer - USB clock source selection
pub type USBSW_W<'a, REG> = crate::BitWriter<'a, REG, USBSW>;
impl<'a, REG> USBSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI48 selected as USB clock source
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSW::Hsi48)
    }
    ///PLL clock selected as USB clock source
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut crate::W<REG> {
        self.variant(USBSW::Pllclk)
    }
}
///Field `ADCSW` reader - ADCSW is deprecated. See ADC field in CFGR2 register.
pub type ADCSW_R = crate::BitReader;
///Field `ADCSW` writer - ADCSW is deprecated. See ADC field in CFGR2 register.
pub type ADCSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SW` reader - USART2 clock source selection
pub use USART1SW_R as USART2SW_R;
///Field `USART3SW` reader - USART3 clock source
pub use USART1SW_R as USART3SW_R;
///Field `USART2SW` writer - USART2 clock source selection
pub use USART1SW_W as USART2SW_W;
///Field `USART3SW` writer - USART3 clock source
pub use USART1SW_W as USART3SW_W;
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
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsw(&self) -> CECSW_R {
        CECSW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB clock source selection
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register.
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - USART3 clock source
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("usart1sw", &self.usart1sw())
            .field("i2c1sw", &self.i2c1sw())
            .field("cecsw", &self.cecsw())
            .field("usbsw", &self.usbsw())
            .field("adcsw", &self.adcsw())
            .field("usart2sw", &self.usart2sw())
            .field("usart3sw", &self.usart3sw())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sw(&mut self) -> USART1SW_W<'_, CFGR3rs> {
        USART1SW_W::new(self, 0)
    }
    ///Bit 4 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<'_, CFGR3rs> {
        I2C1SW_W::new(self, 4)
    }
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsw(&mut self) -> CECSW_W<'_, CFGR3rs> {
        CECSW_W::new(self, 6)
    }
    ///Bit 7 - USB clock source selection
    #[inline(always)]
    pub fn usbsw(&mut self) -> USBSW_W<'_, CFGR3rs> {
        USBSW_W::new(self, 7)
    }
    ///Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register.
    #[inline(always)]
    pub fn adcsw(&mut self) -> ADCSW_W<'_, CFGR3rs> {
        ADCSW_W::new(self, 8)
    }
    ///Bits 16:17 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sw(&mut self) -> USART2SW_W<'_, CFGR3rs> {
        USART2SW_W::new(self, 16)
    }
    ///Bits 18:19 - USART3 clock source
    #[inline(always)]
    pub fn usart3sw(&mut self) -> USART3SW_W<'_, CFGR3rs> {
        USART3SW_W::new(self, 18)
    }
}
/**Clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#RCC:CFGR3)*/
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
