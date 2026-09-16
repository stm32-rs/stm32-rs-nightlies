///Register `LOCKCFGSR3` writer
pub type W = crate::W<LOCKCFGSR3rs>;
///Field `MODLOCKS` writer - Defines the lock protection of the MOD configuration bits (enable, ready, divider).
pub type MODLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSLOCKS` writer - Defines the lock protection of the SYS configuration bits (enable, ready, divider).
pub type SYSLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSLOCKS` writer - Defines the lock protection of the BUS configuration bits (enable, ready, divider).
pub type BUSLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERLOCKS` writer - Defines the lock protection of the PER configuration bits (enable, ready, divider).
pub type PERLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTLOCKS` writer - Defines the lock protection of the INT configuration bits (enable, ready, divider).
pub type INTLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTLOCKS` writer - Defines the lock protection of the RST configuration bits (enable, ready, divider).
pub type RSTLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTLOCKS` writer - Defines the lock protection of the DFT configuration bits (enable, ready, divider).
pub type DFTLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGSR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modlocks(&mut self) -> MODLOCKS_W<'_, LOCKCFGSR3rs> {
        MODLOCKS_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syslocks(&mut self) -> SYSLOCKS_W<'_, LOCKCFGSR3rs> {
        SYSLOCKS_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn buslocks(&mut self) -> BUSLOCKS_W<'_, LOCKCFGSR3rs> {
        BUSLOCKS_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn perlocks(&mut self) -> PERLOCKS_W<'_, LOCKCFGSR3rs> {
        PERLOCKS_W::new(self, 3)
    }
    ///Bit 4 - Defines the lock protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intlocks(&mut self) -> INTLOCKS_W<'_, LOCKCFGSR3rs> {
        INTLOCKS_W::new(self, 4)
    }
    ///Bit 5 - Defines the lock protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstlocks(&mut self) -> RSTLOCKS_W<'_, LOCKCFGSR3rs> {
        RSTLOCKS_W::new(self, 5)
    }
    ///Bit 6 - Defines the lock protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftlocks(&mut self) -> DFTLOCKS_W<'_, LOCKCFGSR3rs> {
        DFTLOCKS_W::new(self, 6)
    }
}
/**RCC system lock configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgsr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:LOCKCFGSR3)*/
pub struct LOCKCFGSR3rs;
impl crate::RegisterSpec for LOCKCFGSR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgsr3::W`](W) writer structure
impl crate::Writable for LOCKCFGSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGSR3 to value 0
impl crate::Resettable for LOCKCFGSR3rs {}
