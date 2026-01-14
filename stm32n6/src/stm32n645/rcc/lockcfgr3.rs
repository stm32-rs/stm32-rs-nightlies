///Register `LOCKCFGR3` writer
pub type W = crate::W<LOCKCFGR3rs>;
///Field `MODLOCK` writer - Defines the lock protection of the MOD system configuration bits.
pub type MODLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSLOCK` writer - Defines the lock protection of the SYS system configuration bits.
pub type SYSLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSLOCK` writer - Defines the lock protection of the BUS system configuration bits.
pub type BUSLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERLOCK` writer - Defines the lock protection of the PER system configuration bits.
pub type PERLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTLOCK` writer - Defines the lock protection of the INT system configuration bits.
pub type INTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTLOCK` writer - Defines the lock protection of the RST system configuration bits.
pub type RSTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTLOCK` writer - Defines the lock protection of the DFT system configuration bits.
pub type DFTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modlock(&mut self) -> MODLOCK_W<'_, LOCKCFGR3rs> {
        MODLOCK_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syslock(&mut self) -> SYSLOCK_W<'_, LOCKCFGR3rs> {
        SYSLOCK_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn buslock(&mut self) -> BUSLOCK_W<'_, LOCKCFGR3rs> {
        BUSLOCK_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the PER system configuration bits.
    #[inline(always)]
    pub fn perlock(&mut self) -> PERLOCK_W<'_, LOCKCFGR3rs> {
        PERLOCK_W::new(self, 3)
    }
    ///Bit 4 - Defines the lock protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intlock(&mut self) -> INTLOCK_W<'_, LOCKCFGR3rs> {
        INTLOCK_W::new(self, 4)
    }
    ///Bit 5 - Defines the lock protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstlock(&mut self) -> RSTLOCK_W<'_, LOCKCFGR3rs> {
        RSTLOCK_W::new(self, 5)
    }
    ///Bit 6 - Defines the lock protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftlock(&mut self) -> DFTLOCK_W<'_, LOCKCFGR3rs> {
        DFTLOCK_W::new(self, 6)
    }
}
/**RCC system lock configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:LOCKCFGR3)*/
pub struct LOCKCFGR3rs;
impl crate::RegisterSpec for LOCKCFGR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgr3::W`](W) writer structure
impl crate::Writable for LOCKCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGR3 to value 0
impl crate::Resettable for LOCKCFGR3rs {}
