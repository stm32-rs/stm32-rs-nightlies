///Register `RCC_CCIPR2` reader
pub type R = crate::R<RCC_CCIPR2rs>;
///Register `RCC_CCIPR2` writer
pub type W = crate::W<RCC_CCIPR2rs>;
/**USB clock source selection Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSEL {
    ///0: HSIUSB48
    B0x0 = 0,
    ///1: HSE
    B0x1 = 1,
}
impl From<USBSEL> for bool {
    #[inline(always)]
    fn from(variant: USBSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `USBSEL` reader - USB clock source selection Set and cleared by software.
pub type USBSEL_R = crate::BitReader<USBSEL>;
impl USBSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBSEL {
        match self.bits {
            false => USBSEL::B0x0,
            true => USBSEL::B0x1,
        }
    }
    ///HSIUSB48
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBSEL::B0x0
    }
    ///HSE
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBSEL::B0x1
    }
}
///Field `USBSEL` writer - USB clock source selection Set and cleared by software.
pub type USBSEL_W<'a, REG> = crate::BitWriter<'a, REG, USBSEL>;
impl<'a, REG> USBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSIUSB48
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::B0x0)
    }
    ///HSE
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBSEL::B0x1)
    }
}
impl R {
    ///Bit 12 - USB clock source selection Set and cleared by software.
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR2")
            .field("usbsel", &self.usbsel())
            .finish()
    }
}
impl W {
    ///Bit 12 - USB clock source selection Set and cleared by software.
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<'_, RCC_CCIPR2rs> {
        USBSEL_W::new(self, 12)
    }
}
/**RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_CCIPR2)*/
pub struct RCC_CCIPR2rs;
impl crate::RegisterSpec for RCC_CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ccipr2::R`](R) reader structure
impl crate::Readable for RCC_CCIPR2rs {}
///`write(|w| ..)` method takes [`rcc_ccipr2::W`](W) writer structure
impl crate::Writable for RCC_CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CCIPR2 to value 0
impl crate::Resettable for RCC_CCIPR2rs {}
