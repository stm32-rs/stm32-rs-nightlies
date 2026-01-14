///Register `LOCKCFGR0` writer
pub type W = crate::W<LOCKCFGR0rs>;
///Field `LSILOCK` writer - Defines the lock protection of the LSI oscillator configuration bits.
pub type LSILOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSELOCK` writer - Defines the lock protection of the LSE oscillator configuration bits.
pub type LSELOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSILOCK` writer - Defines the lock protection of the MSI oscillator configuration bits.
pub type MSILOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSILOCK` writer - Defines the lock protection of the HSI oscillator configuration bits.
pub type HSILOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSELOCK` writer - Defines the lock protection of the HSE oscillator configuration bits.
pub type HSELOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsilock(&mut self) -> LSILOCK_W<'_, LOCKCFGR0rs> {
        LSILOCK_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lselock(&mut self) -> LSELOCK_W<'_, LOCKCFGR0rs> {
        LSELOCK_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msilock(&mut self) -> MSILOCK_W<'_, LOCKCFGR0rs> {
        MSILOCK_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsilock(&mut self) -> HSILOCK_W<'_, LOCKCFGR0rs> {
        HSILOCK_W::new(self, 3)
    }
    ///Bit 4 - Defines the lock protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hselock(&mut self) -> HSELOCK_W<'_, LOCKCFGR0rs> {
        HSELOCK_W::new(self, 4)
    }
}
/**RCC oscillator lock configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:LOCKCFGR0)*/
pub struct LOCKCFGR0rs;
impl crate::RegisterSpec for LOCKCFGR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgr0::W`](W) writer structure
impl crate::Writable for LOCKCFGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGR0 to value 0
impl crate::Resettable for LOCKCFGR0rs {}
