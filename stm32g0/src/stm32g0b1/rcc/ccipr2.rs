///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
/**2S1SEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SEL {
    ///0: SYSCLK clock selected
    Sysclk = 0,
    ///1: PLLPCLK clock selected
    Pllp = 1,
    ///2: HSI16 clock selected
    Hsi16 = 2,
    ///3: I2S_CKIN clock selected
    Ckin = 3,
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
///Field `I2S1SEL` reader - 2S1SEL
pub type I2S1SEL_R = crate::FieldReader<I2S1SEL>;
impl I2S1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2S1SEL {
        match self.bits {
            0 => I2S1SEL::Sysclk,
            1 => I2S1SEL::Pllp,
            2 => I2S1SEL::Hsi16,
            3 => I2S1SEL::Ckin,
            _ => unreachable!(),
        }
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2S1SEL::Sysclk
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == I2S1SEL::Pllp
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2S1SEL::Hsi16
    }
    ///I2S_CKIN clock selected
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2S1SEL::Ckin
    }
}
///Field `I2S1SEL` writer - 2S1SEL
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2S1SEL, crate::Safe>;
impl<'a, REG> I2S1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Sysclk)
    }
    ///PLLPCLK clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Pllp)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Hsi16)
    }
    ///I2S_CKIN clock selected
    #[inline(always)]
    pub fn ckin(self) -> &'a mut crate::W<REG> {
        self.variant(I2S1SEL::Ckin)
    }
}
///Field `I2S2SEL` reader - I2S2SEL
pub use I2S1SEL_R as I2S2SEL_R;
///Field `I2S2SEL` writer - I2S2SEL
pub use I2S1SEL_W as I2S2SEL_W;
/**FDCANSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL {
    ///0: PCLK clock selected
    Pclk = 0,
    ///1: PLLQCLK clock selected
    Pllq = 1,
    ///2: HSE clock selected
    Hse = 2,
}
impl From<FDCANSEL> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FDCANSEL {
    type Ux = u8;
}
impl crate::IsEnum for FDCANSEL {}
///Field `FDCANSEL` reader - FDCANSEL
pub type FDCANSEL_R = crate::FieldReader<FDCANSEL>;
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FDCANSEL> {
        match self.bits {
            0 => Some(FDCANSEL::Pclk),
            1 => Some(FDCANSEL::Pllq),
            2 => Some(FDCANSEL::Hse),
            _ => None,
        }
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == FDCANSEL::Pclk
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == FDCANSEL::Pllq
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL::Hse
    }
}
///Field `FDCANSEL` writer - FDCANSEL
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCANSEL>;
impl<'a, REG> FDCANSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pclk)
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Pllq)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(FDCANSEL::Hse)
    }
}
/**USBSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL {
    ///1: PLLQCLK clock selected
    Pllq = 1,
    ///2: HSE clock selected
    Hse = 2,
}
impl From<USBSEL> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBSEL {
    type Ux = u8;
}
impl crate::IsEnum for USBSEL {}
///Field `USBSEL` reader - USBSEL
pub type USBSEL_R = crate::FieldReader<USBSEL>;
impl USBSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USBSEL> {
        match self.bits {
            1 => Some(USBSEL::Pllq),
            2 => Some(USBSEL::Hse),
            _ => None,
        }
    }
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == USBSEL::Pllq
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == USBSEL::Hse
    }
}
///Field `USBSEL` writer - USBSEL
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBSEL>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLLQCLK clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Pllq)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::Hse)
    }
}
impl R {
    ///Bits 0:1 - 2S1SEL
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - I2S2SEL
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 8:9 - FDCANSEL
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - USBSEL
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("i2s1sel", &self.i2s1sel())
            .field("i2s2sel", &self.i2s2sel())
            .field("fdcansel", &self.fdcansel())
            .field("usbsel", &self.usbsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - 2S1SEL
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<'_, CCIPR2rs> {
        I2S1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - I2S2SEL
    #[inline(always)]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<'_, CCIPR2rs> {
        I2S2SEL_W::new(self, 2)
    }
    ///Bits 8:9 - FDCANSEL
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, CCIPR2rs> {
        FDCANSEL_W::new(self, 8)
    }
    ///Bits 12:13 - USBSEL
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<'_, CCIPR2rs> {
        USBSEL_W::new(self, 12)
    }
}
/**Peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#RCC:CCIPR2)*/
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
