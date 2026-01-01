///Register `PRIVCFGCR3` writer
pub type W = crate::W<PRIVCFGCR3rs>;
///Field `MODPVC` writer - Defines the privilege protection of the MOD configuration bits (enable, ready, divider).
pub type MODPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPVC` writer - Defines the privilege protection of the SYS configuration bits (enable, ready, divider).
pub type SYSPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPVC` writer - Defines the privilege protection of the BUS configuration bits (enable, ready, divider).
pub type BUSPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPVC` writer - Defines the privilege protection of the PER configuration bits (enable, ready, divider).
pub type PERPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPVC` writer - Defines the privilege protection of the INT configuration bits (enable, ready, divider).
pub type INTPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPVC` writer - Defines the privilege protection of the RST configuration bits (enable, ready, divider).
pub type RSTPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPVC` writer - Defines the privilege protection of the DFT configuration bits (enable, ready, divider).
pub type DFTPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the MOD configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn modpvc(&mut self) -> MODPVC_W<'_, PRIVCFGCR3rs> {
        MODPVC_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the SYS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn syspvc(&mut self) -> SYSPVC_W<'_, PRIVCFGCR3rs> {
        SYSPVC_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the BUS configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn buspvc(&mut self) -> BUSPVC_W<'_, PRIVCFGCR3rs> {
        BUSPVC_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PER configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn perpvc(&mut self) -> PERPVC_W<'_, PRIVCFGCR3rs> {
        PERPVC_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the INT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn intpvc(&mut self) -> INTPVC_W<'_, PRIVCFGCR3rs> {
        INTPVC_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the RST configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn rstpvc(&mut self) -> RSTPVC_W<'_, PRIVCFGCR3rs> {
        RSTPVC_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the DFT configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn dftpvc(&mut self) -> DFTPVC_W<'_, PRIVCFGCR3rs> {
        DFTPVC_W::new(self, 6)
    }
}
/**RCC system privilege configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PRIVCFGCR3)*/
pub struct PRIVCFGCR3rs;
impl crate::RegisterSpec for PRIVCFGCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgcr3::W`](W) writer structure
impl crate::Writable for PRIVCFGCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGCR3 to value 0
impl crate::Resettable for PRIVCFGCR3rs {}
