///Register `PUBCFGCR3` writer
pub type W = crate::W<PUBCFGCR3rs>;
///Field `MODPUBC` writer - Defines the public protection of the MOD configuration bits (enable, ready, divider).
pub type MODPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPUBC` writer - Defines the public protection of the SYS configuration bits (enable, ready, divider).
pub type SYSPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPUBC` writer - Defines the public protection of the BUS configuration bits (enable, ready, divider).
pub type BUSPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPUBC` writer - Defines the public protection of the PER configuration bits (enable, ready, divider).
pub type PERPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPUBC` writer - Defines the public protection of the INT configuration bits (enable, ready, divider).
pub type INTPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPUBC` writer - Defines the public protection of the RST configuration bits (enable, ready, divider).
pub type RSTPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPUBC` writer - Defines the public protection of the DFT configuration bits (enable, ready, divider).
pub type DFTPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modpubc(&mut self) -> MODPUBC_W<'_, PUBCFGCR3rs> {
        MODPUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syspubc(&mut self) -> SYSPUBC_W<'_, PUBCFGCR3rs> {
        SYSPUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn buspubc(&mut self) -> BUSPUBC_W<'_, PUBCFGCR3rs> {
        BUSPUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn perpubc(&mut self) -> PERPUBC_W<'_, PUBCFGCR3rs> {
        PERPUBC_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intpubc(&mut self) -> INTPUBC_W<'_, PUBCFGCR3rs> {
        INTPUBC_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstpubc(&mut self) -> RSTPUBC_W<'_, PUBCFGCR3rs> {
        RSTPUBC_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftpubc(&mut self) -> DFTPUBC_W<'_, PUBCFGCR3rs> {
        DFTPUBC_W::new(self, 6)
    }
}
/**RCC system public configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PUBCFGCR3)*/
pub struct PUBCFGCR3rs;
impl crate::RegisterSpec for PUBCFGCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr3::W`](W) writer structure
impl crate::Writable for PUBCFGCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR3 to value 0
impl crate::Resettable for PUBCFGCR3rs {}
