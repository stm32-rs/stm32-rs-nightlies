///Register `PRIVCFGSR3` writer
pub type W = crate::W<PRIVCFGSR3rs>;
///Field `MODPVS` writer - Defines the privilege protection of the MOD configuration bits (enable, ready, divider).
pub type MODPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPVS` writer - Defines the privilege protection of the SYS configuration bits (enable, ready, divider).
pub type SYSPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPVS` writer - Defines the privilege protection of the BUS configuration bits (enable, ready, divider).
pub type BUSPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPVS` writer - Defines the privilege protection of the PER configuration bits (enable, ready, divider).
pub type PERPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPVS` writer - Defines the privilege protection of the INT configuration bits (enable, ready, divider).
pub type INTPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPVS` writer - Defines the privilege protection of the RST configuration bits (enable, ready, divider).
pub type RSTPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPVS` writer - Defines the privilege protection of the DFT configuration bits (enable, ready, divider).
pub type DFTPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGSR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modpvs(&mut self) -> MODPVS_W<'_, PRIVCFGSR3rs> {
        MODPVS_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syspvs(&mut self) -> SYSPVS_W<'_, PRIVCFGSR3rs> {
        SYSPVS_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn buspvs(&mut self) -> BUSPVS_W<'_, PRIVCFGSR3rs> {
        BUSPVS_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn perpvs(&mut self) -> PERPVS_W<'_, PRIVCFGSR3rs> {
        PERPVS_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intpvs(&mut self) -> INTPVS_W<'_, PRIVCFGSR3rs> {
        INTPVS_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstpvs(&mut self) -> RSTPVS_W<'_, PRIVCFGSR3rs> {
        RSTPVS_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftpvs(&mut self) -> DFTPVS_W<'_, PRIVCFGSR3rs> {
        DFTPVS_W::new(self, 6)
    }
}
/**RCC system privilege configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PRIVCFGSR3)*/
pub struct PRIVCFGSR3rs;
impl crate::RegisterSpec for PRIVCFGSR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgsr3::W`](W) writer structure
impl crate::Writable for PRIVCFGSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGSR3 to value 0
impl crate::Resettable for PRIVCFGSR3rs {}
