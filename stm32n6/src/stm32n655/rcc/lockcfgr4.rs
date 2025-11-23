///Register `LOCKCFGR4` writer
pub type W = crate::W<LOCKCFGR4rs>;
///Field `ACLKNLOCK` writer - Defines the lock protection of the ACLKN bus configuration bits.
pub type ACLKNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCLOCK` writer - Defines the lock protection of the ACLKNC bus configuration bits.
pub type ACLKNCLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMLOCK` writer - Defines the lock protection of the AHBM bus configuration bits.
pub type AHBMLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1LOCK` writer - Defines the lock protection of the AHB1 bus configuration bits.
pub type AHB1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2LOCK` writer - Defines the lock protection of the AHB2 bus configuration bits.
pub type AHB2LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3LOCK` writer - Defines the lock protection of the AHB3 bus configuration bits.
pub type AHB3LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4LOCK` writer - Defines the lock protection of the AHB4 bus configuration bits.
pub type AHB4LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5LOCK` writer - Defines the lock protection of the AHB5 bus configuration bits.
pub type AHB5LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1LOCK` writer - Defines the lock protection of the APB1 bus configuration bits.
pub type APB1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2LOCK` writer - Defines the lock protection of the APB2 bus configuration bits.
pub type APB2LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3LOCK` writer - Defines the lock protection of the APB3 bus configuration bits.
pub type APB3LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4LOCK` writer - Defines the lock protection of the APB4 bus configuration bits.
pub type APB4LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5LOCK` writer - Defines the lock protection of the APB5 bus configuration bits.
pub type APB5LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCLOCK` writer - Defines the lock protection of the NOC bus configuration bits.
pub type NOCLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknlock(&mut self) -> ACLKNLOCK_W<'_, LOCKCFGR4rs> {
        ACLKNLOCK_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclknclock(&mut self) -> ACLKNCLOCK_W<'_, LOCKCFGR4rs> {
        ACLKNCLOCK_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmlock(&mut self) -> AHBMLOCK_W<'_, LOCKCFGR4rs> {
        AHBMLOCK_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1lock(&mut self) -> AHB1LOCK_W<'_, LOCKCFGR4rs> {
        AHB1LOCK_W::new(self, 3)
    }
    ///Bit 4 - Defines the lock protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2lock(&mut self) -> AHB2LOCK_W<'_, LOCKCFGR4rs> {
        AHB2LOCK_W::new(self, 4)
    }
    ///Bit 5 - Defines the lock protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3lock(&mut self) -> AHB3LOCK_W<'_, LOCKCFGR4rs> {
        AHB3LOCK_W::new(self, 5)
    }
    ///Bit 6 - Defines the lock protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4lock(&mut self) -> AHB4LOCK_W<'_, LOCKCFGR4rs> {
        AHB4LOCK_W::new(self, 6)
    }
    ///Bit 7 - Defines the lock protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5lock(&mut self) -> AHB5LOCK_W<'_, LOCKCFGR4rs> {
        AHB5LOCK_W::new(self, 7)
    }
    ///Bit 8 - Defines the lock protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1lock(&mut self) -> APB1LOCK_W<'_, LOCKCFGR4rs> {
        APB1LOCK_W::new(self, 8)
    }
    ///Bit 9 - Defines the lock protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2lock(&mut self) -> APB2LOCK_W<'_, LOCKCFGR4rs> {
        APB2LOCK_W::new(self, 9)
    }
    ///Bit 10 - Defines the lock protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3lock(&mut self) -> APB3LOCK_W<'_, LOCKCFGR4rs> {
        APB3LOCK_W::new(self, 10)
    }
    ///Bit 11 - Defines the lock protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4lock(&mut self) -> APB4LOCK_W<'_, LOCKCFGR4rs> {
        APB4LOCK_W::new(self, 11)
    }
    ///Bit 12 - Defines the lock protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5lock(&mut self) -> APB5LOCK_W<'_, LOCKCFGR4rs> {
        APB5LOCK_W::new(self, 12)
    }
    ///Bit 13 - Defines the lock protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn noclock(&mut self) -> NOCLOCK_W<'_, LOCKCFGR4rs> {
        NOCLOCK_W::new(self, 13)
    }
}
/**RCC bus lock configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:LOCKCFGR4)*/
pub struct LOCKCFGR4rs;
impl crate::RegisterSpec for LOCKCFGR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgr4::W`](W) writer structure
impl crate::Writable for LOCKCFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGR4 to value 0
impl crate::Resettable for LOCKCFGR4rs {}
