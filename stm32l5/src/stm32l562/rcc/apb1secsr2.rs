///Register `APB1SECSR2` reader
pub type R = crate::R<APB1SECSR2rs>;
///Field `LPUART1SECF` reader - LPUART1SECF
pub type LPUART1SECF_R = crate::BitReader;
///Field `I2C4SECF` reader - I2C4SECF
pub type I2C4SECF_R = crate::BitReader;
///Field `LPTIM2SECF` reader - LPTIM2SECF
pub type LPTIM2SECF_R = crate::BitReader;
///Field `LPTIM3SECF` reader - LPTIM3SECF
pub type LPTIM3SECF_R = crate::BitReader;
///Field `FDCAN1SECF` reader - FDCAN1SECF
pub type FDCAN1SECF_R = crate::BitReader;
///Field `USBFSSECF` reader - USBFSSECF
pub type USBFSSECF_R = crate::BitReader;
///Field `UCPD1SECF` reader - UCPD1SECF
pub type UCPD1SECF_R = crate::BitReader;
impl R {
    ///Bit 0 - LPUART1SECF
    #[inline(always)]
    pub fn lpuart1secf(&self) -> LPUART1SECF_R {
        LPUART1SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4SECF
    #[inline(always)]
    pub fn i2c4secf(&self) -> I2C4SECF_R {
        I2C4SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2SECF
    #[inline(always)]
    pub fn lptim2secf(&self) -> LPTIM2SECF_R {
        LPTIM2SECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPTIM3SECF
    #[inline(always)]
    pub fn lptim3secf(&self) -> LPTIM3SECF_R {
        LPTIM3SECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - FDCAN1SECF
    #[inline(always)]
    pub fn fdcan1secf(&self) -> FDCAN1SECF_R {
        FDCAN1SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 21 - USBFSSECF
    #[inline(always)]
    pub fn usbfssecf(&self) -> USBFSSECF_R {
        USBFSSECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - UCPD1SECF
    #[inline(always)]
    pub fn ucpd1secf(&self) -> UCPD1SECF_R {
        UCPD1SECF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SECSR2")
            .field("ucpd1secf", &self.ucpd1secf())
            .field("usbfssecf", &self.usbfssecf())
            .field("fdcan1secf", &self.fdcan1secf())
            .field("lptim3secf", &self.lptim3secf())
            .field("lptim2secf", &self.lptim2secf())
            .field("i2c4secf", &self.i2c4secf())
            .field("lpuart1secf", &self.lpuart1secf())
            .finish()
    }
}
/**RCC APB1 security status register 2

You can [`read`](crate::Reg::read) this register and get [`apb1secsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SECSR2)*/
pub struct APB1SECSR2rs;
impl crate::RegisterSpec for APB1SECSR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1secsr2::R`](R) reader structure
impl crate::Readable for APB1SECSR2rs {}
///`reset()` method sets APB1SECSR2 to value 0
impl crate::Resettable for APB1SECSR2rs {}
