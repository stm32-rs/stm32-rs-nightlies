///Register `AHB4ENCR` writer
pub type W = crate::W<AHB4ENCRrs>;
///Field `GPIOAENC` writer - GPIOA enable
pub type GPIOAENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBENC` writer - GPIOB enable
pub type GPIOBENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCENC` writer - GPIOC enable
pub type GPIOCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODENC` writer - GPIOD enable
pub type GPIODENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEENC` writer - GPIOE enable
pub type GPIOEENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFENC` writer - GPIOF enable
pub type GPIOFENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGENC` writer - GPIOG enable
pub type GPIOGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHENC` writer - GPIOH enable
pub type GPIOHENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONENC` writer - GPION enable
pub type GPIONENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOENC` writer - GPIOO enable
pub type GPIOOENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPENC` writer - GPIOP enable
pub type GPIOPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQENC` writer - GPIOQ enable
pub type GPIOQENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRENC` writer - PWR enable
pub type PWRENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENC` writer - CRC enable
pub type CRCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA enable
    #[inline(always)]
    pub fn gpioaenc(&mut self) -> GPIOAENC_W<'_, AHB4ENCRrs> {
        GPIOAENC_W::new(self, 0)
    }
    ///Bit 1 - GPIOB enable
    #[inline(always)]
    pub fn gpiobenc(&mut self) -> GPIOBENC_W<'_, AHB4ENCRrs> {
        GPIOBENC_W::new(self, 1)
    }
    ///Bit 2 - GPIOC enable
    #[inline(always)]
    pub fn gpiocenc(&mut self) -> GPIOCENC_W<'_, AHB4ENCRrs> {
        GPIOCENC_W::new(self, 2)
    }
    ///Bit 3 - GPIOD enable
    #[inline(always)]
    pub fn gpiodenc(&mut self) -> GPIODENC_W<'_, AHB4ENCRrs> {
        GPIODENC_W::new(self, 3)
    }
    ///Bit 4 - GPIOE enable
    #[inline(always)]
    pub fn gpioeenc(&mut self) -> GPIOEENC_W<'_, AHB4ENCRrs> {
        GPIOEENC_W::new(self, 4)
    }
    ///Bit 5 - GPIOF enable
    #[inline(always)]
    pub fn gpiofenc(&mut self) -> GPIOFENC_W<'_, AHB4ENCRrs> {
        GPIOFENC_W::new(self, 5)
    }
    ///Bit 6 - GPIOG enable
    #[inline(always)]
    pub fn gpiogenc(&mut self) -> GPIOGENC_W<'_, AHB4ENCRrs> {
        GPIOGENC_W::new(self, 6)
    }
    ///Bit 7 - GPIOH enable
    #[inline(always)]
    pub fn gpiohenc(&mut self) -> GPIOHENC_W<'_, AHB4ENCRrs> {
        GPIOHENC_W::new(self, 7)
    }
    ///Bit 13 - GPION enable
    #[inline(always)]
    pub fn gpionenc(&mut self) -> GPIONENC_W<'_, AHB4ENCRrs> {
        GPIONENC_W::new(self, 13)
    }
    ///Bit 14 - GPIOO enable
    #[inline(always)]
    pub fn gpiooenc(&mut self) -> GPIOOENC_W<'_, AHB4ENCRrs> {
        GPIOOENC_W::new(self, 14)
    }
    ///Bit 15 - GPIOP enable
    #[inline(always)]
    pub fn gpiopenc(&mut self) -> GPIOPENC_W<'_, AHB4ENCRrs> {
        GPIOPENC_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ enable
    #[inline(always)]
    pub fn gpioqenc(&mut self) -> GPIOQENC_W<'_, AHB4ENCRrs> {
        GPIOQENC_W::new(self, 16)
    }
    ///Bit 18 - PWR enable
    #[inline(always)]
    pub fn pwrenc(&mut self) -> PWRENC_W<'_, AHB4ENCRrs> {
        PWRENC_W::new(self, 18)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcenc(&mut self) -> CRCENC_W<'_, AHB4ENCRrs> {
        CRCENC_W::new(self, 19)
    }
}
/**RCC AHB4 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB4ENCR)*/
pub struct AHB4ENCRrs;
impl crate::RegisterSpec for AHB4ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4encr::W`](W) writer structure
impl crate::Writable for AHB4ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENCR to value 0
impl crate::Resettable for AHB4ENCRrs {}
