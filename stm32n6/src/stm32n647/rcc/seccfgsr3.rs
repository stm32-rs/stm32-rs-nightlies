///Register `SECCFGSR3` writer
pub type W = crate::W<SECCFGSR3rs>;
///Field `MODSECS` writer - Defines the secure protection of the MOD configuration bits (enable, ready, divider).
pub type MODSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSSECS` writer - Defines the secure protection of the SYS configuration bits (enable, ready, divider).
pub type SYSSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSSECS` writer - Defines the secure protection of the BUS configuration bits (enable, ready, divider).
pub type BUSSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSECS` writer - Defines the secure protection of the PER configuration bits (enable, ready, divider).
pub type PERSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTSECS` writer - Defines the secure protection of the INT configuration bits (enable, ready, divider).
pub type INTSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTSECS` writer - Defines the secure protection of the RST configuration bits (enable, ready, divider).
pub type RSTSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTSECS` writer - Defines the secure protection of the DFT configuration bits (enable, ready, divider).
pub type DFTSECS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SECCFGSR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modsecs(&mut self) -> MODSECS_W<'_, SECCFGSR3rs> {
        MODSECS_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syssecs(&mut self) -> SYSSECS_W<'_, SECCFGSR3rs> {
        SYSSECS_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn bussecs(&mut self) -> BUSSECS_W<'_, SECCFGSR3rs> {
        BUSSECS_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn persecs(&mut self) -> PERSECS_W<'_, SECCFGSR3rs> {
        PERSECS_W::new(self, 3)
    }
    ///Bit 4 - Defines the secure protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intsecs(&mut self) -> INTSECS_W<'_, SECCFGSR3rs> {
        INTSECS_W::new(self, 4)
    }
    ///Bit 5 - Defines the secure protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstsecs(&mut self) -> RSTSECS_W<'_, SECCFGSR3rs> {
        RSTSECS_W::new(self, 5)
    }
    ///Bit 6 - Defines the secure protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftsecs(&mut self) -> DFTSECS_W<'_, SECCFGSR3rs> {
        DFTSECS_W::new(self, 6)
    }
}
/**RCC system secure configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgsr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGSR3)*/
pub struct SECCFGSR3rs;
impl crate::RegisterSpec for SECCFGSR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`seccfgsr3::W`](W) writer structure
impl crate::Writable for SECCFGSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGSR3 to value 0
impl crate::Resettable for SECCFGSR3rs {}
