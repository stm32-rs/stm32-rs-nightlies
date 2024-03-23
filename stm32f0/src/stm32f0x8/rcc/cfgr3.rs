#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<CFGR3rs>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<CFGR3rs>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SW {
    #[doc = "0: PCLK selected as USART clock source"]
    Pclk = 0,
    #[doc = "1: SYSCLK selected as USART clock source"]
    Sysclk = 1,
    #[doc = "2: LSE selected as USART clock source"]
    Lse = 2,
    #[doc = "3: HSI selected as USART clock source"]
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
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub type USART1SW_R = crate::FieldReader<USART1SW>;
impl USART1SW_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SW::Pclk
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW::Sysclk
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW::Lse
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW::Hsi
    }
}
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub type USART1SW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USART1SW>;
impl<'a, REG> USART1SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Pclk)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Sysclk)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Lse)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SW::Hsi)
    }
}
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SW {
    #[doc = "0: HSI clock selected as I2C clock source"]
    Hsi = 0,
    #[doc = "1: SYSCLK clock selected as I2C clock source"]
    Sysclk = 1,
}
impl From<I2C1SW> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub type I2C1SW_R = crate::BitReader<I2C1SW>;
impl I2C1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SW {
        match self.bits {
            false => I2C1SW::Hsi,
            true => I2C1SW::Sysclk,
        }
    }
    #[doc = "HSI clock selected as I2C clock source"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SW::Hsi
    }
    #[doc = "SYSCLK clock selected as I2C clock source"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW::Sysclk
    }
}
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub type I2C1SW_W<'a, REG> = crate::BitWriter<'a, REG, I2C1SW>;
impl<'a, REG> I2C1SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI clock selected as I2C clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SW::Hsi)
    }
    #[doc = "SYSCLK clock selected as I2C clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SW::Sysclk)
    }
}
#[doc = "HDMI CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSW {
    #[doc = "0: HSI clock divided by 244 selected as CEC clock source"]
    HsiDiv244 = 0,
    #[doc = "1: LSE clock selected as CEC clock source"]
    Lse = 1,
}
impl From<CECSW> for bool {
    #[inline(always)]
    fn from(variant: CECSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECSW` reader - HDMI CEC clock source selection"]
pub type CECSW_R = crate::BitReader<CECSW>;
impl CECSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CECSW {
        match self.bits {
            false => CECSW::HsiDiv244,
            true => CECSW::Lse,
        }
    }
    #[doc = "HSI clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn is_hsi_div244(&self) -> bool {
        *self == CECSW::HsiDiv244
    }
    #[doc = "LSE clock selected as CEC clock source"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSW::Lse
    }
}
#[doc = "Field `CECSW` writer - HDMI CEC clock source selection"]
pub type CECSW_W<'a, REG> = crate::BitWriter<'a, REG, CECSW>;
impl<'a, REG> CECSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn hsi_div244(self) -> &'a mut crate::W<REG> {
        self.variant(CECSW::HsiDiv244)
    }
    #[doc = "LSE clock selected as CEC clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(CECSW::Lse)
    }
}
#[doc = "USB clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSW {
    #[doc = "0: HSI48 selected as USB clock source"]
    Hsi48 = 0,
    #[doc = "1: PLL clock selected as USB clock source"]
    Pllclk = 1,
}
impl From<USBSW> for bool {
    #[inline(always)]
    fn from(variant: USBSW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSW` reader - USB clock source selection"]
pub type USBSW_R = crate::BitReader<USBSW>;
impl USBSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBSW {
        match self.bits {
            false => USBSW::Hsi48,
            true => USBSW::Pllclk,
        }
    }
    #[doc = "HSI48 selected as USB clock source"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSW::Hsi48
    }
    #[doc = "PLL clock selected as USB clock source"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == USBSW::Pllclk
    }
}
#[doc = "Field `USBSW` writer - USB clock source selection"]
pub type USBSW_W<'a, REG> = crate::BitWriter<'a, REG, USBSW>;
impl<'a, REG> USBSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 selected as USB clock source"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut crate::W<REG> {
        self.variant(USBSW::Hsi48)
    }
    #[doc = "PLL clock selected as USB clock source"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut crate::W<REG> {
        self.variant(USBSW::Pllclk)
    }
}
#[doc = "Field `ADCSW` reader - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_R = crate::BitReader;
#[doc = "Field `ADCSW` writer - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub use USART1SW_R as USART2SW_R;
#[doc = "Field `USART3SW` reader - USART3 clock source"]
pub use USART1SW_R as USART3SW_R;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub use USART1SW_W as USART2SW_W;
#[doc = "Field `USART3SW` writer - USART3 clock source"]
pub use USART1SW_W as USART3SW_W;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&self) -> CECSW_R {
        CECSW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sw(&mut self) -> USART1SW_W<CFGR3rs> {
        USART1SW_W::new(self, 0)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<CFGR3rs> {
        I2C1SW_W::new(self, 4)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn cecsw(&mut self) -> CECSW_W<CFGR3rs> {
        CECSW_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbsw(&mut self) -> USBSW_W<CFGR3rs> {
        USBSW_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    #[must_use]
    pub fn adcsw(&mut self) -> ADCSW_W<CFGR3rs> {
        ADCSW_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sw(&mut self) -> USART2SW_W<CFGR3rs> {
        USART2SW_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sw(&mut self) -> USART3SW_W<CFGR3rs> {
        USART3SW_W::new(self, 18)
    }
}
#[doc = "Clock configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for CFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
