///Register `PUBCFGSR3` writer
pub type W = crate::W<PUBCFGSR3rs>;
///Field `MODPUBS` writer - Defines the public protection of the MOD configuration bits (enable, ready, divider).
pub type MODPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPUBS` writer - Defines the public protection of the SYS configuration bits (enable, ready, divider).
pub type SYSPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPUBS` writer - Defines the public protection of the BUS configuration bits (enable, ready, divider).
pub type BUSPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPUBS` writer - Defines the public protection of the PER configuration bits (enable, ready, divider).
pub type PERPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPUBS` writer - Defines the public protection of the INT configuration bits (enable, ready, divider).
pub type INTPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPUBS` writer - Defines the public protection of the RST configuration bits (enable, ready, divider).
pub type RSTPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPUBS` writer - Defines the public protection of the DFT configuration bits (enable, ready, divider).
pub type DFTPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modpubs(&mut self) -> MODPUBS_W<'_, PUBCFGSR3rs> {
        MODPUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syspubs(&mut self) -> SYSPUBS_W<'_, PUBCFGSR3rs> {
        SYSPUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn buspubs(&mut self) -> BUSPUBS_W<'_, PUBCFGSR3rs> {
        BUSPUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn perpubs(&mut self) -> PERPUBS_W<'_, PUBCFGSR3rs> {
        PERPUBS_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intpubs(&mut self) -> INTPUBS_W<'_, PUBCFGSR3rs> {
        INTPUBS_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstpubs(&mut self) -> RSTPUBS_W<'_, PUBCFGSR3rs> {
        RSTPUBS_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftpubs(&mut self) -> DFTPUBS_W<'_, PUBCFGSR3rs> {
        DFTPUBS_W::new(self, 6)
    }
}
/**RCC system public configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGSR3)*/
pub struct PUBCFGSR3rs;
impl crate::RegisterSpec for PUBCFGSR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr3::W`](W) writer structure
impl crate::Writable for PUBCFGSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR3 to value 0
impl crate::Resettable for PUBCFGSR3rs {}
