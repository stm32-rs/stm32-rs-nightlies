///Register `PRIVCFGR3` reader
pub type R = crate::R<PRIVCFGR3rs>;
///Register `PRIVCFGR3` writer
pub type W = crate::W<PRIVCFGR3rs>;
///Field `MODPV` reader - Defines the privilege protection of the MOD system configuration bits.
pub type MODPV_R = crate::BitReader;
///Field `MODPV` writer - Defines the privilege protection of the MOD system configuration bits.
pub type MODPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPV` reader - Defines the privilege protection of the SYS system configuration bits.
pub type SYSPV_R = crate::BitReader;
///Field `SYSPV` writer - Defines the privilege protection of the SYS system configuration bits.
pub type SYSPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPV` reader - Defines the privilege protection of the BUS system configuration bits.
pub type BUSPV_R = crate::BitReader;
///Field `BUSPV` writer - Defines the privilege protection of the BUS system configuration bits.
pub type BUSPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPV` reader - Defines the privilege protection of the PER system configuration bits.
pub type PERPV_R = crate::BitReader;
///Field `PERPV` writer - Defines the privilege protection of the PER system configuration bits.
pub type PERPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPV` reader - Defines the privilege protection of the INT system configuration bits.
pub type INTPV_R = crate::BitReader;
///Field `INTPV` writer - Defines the privilege protection of the INT system configuration bits.
pub type INTPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPV` reader - Defines the privilege protection of the RST system configuration bits.
pub type RSTPV_R = crate::BitReader;
///Field `RSTPV` writer - Defines the privilege protection of the RST system configuration bits.
pub type RSTPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPV` reader - Defines the privilege protection of the DFT system configuration bits.
pub type DFTPV_R = crate::BitReader;
///Field `DFTPV` writer - Defines the privilege protection of the DFT system configuration bits.
pub type DFTPV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the privilege protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modpv(&self) -> MODPV_R {
        MODPV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the privilege protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syspv(&self) -> SYSPV_R {
        SYSPV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the privilege protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn buspv(&self) -> BUSPV_R {
        BUSPV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the privilege protection of the PER system configuration bits.
    #[inline(always)]
    pub fn perpv(&self) -> PERPV_R {
        PERPV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the privilege protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intpv(&self) -> INTPV_R {
        INTPV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the privilege protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstpv(&self) -> RSTPV_R {
        RSTPV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the privilege protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftpv(&self) -> DFTPV_R {
        DFTPV_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR3")
            .field("modpv", &self.modpv())
            .field("syspv", &self.syspv())
            .field("buspv", &self.buspv())
            .field("perpv", &self.perpv())
            .field("intpv", &self.intpv())
            .field("rstpv", &self.rstpv())
            .field("dftpv", &self.dftpv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modpv(&mut self) -> MODPV_W<'_, PRIVCFGR3rs> {
        MODPV_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syspv(&mut self) -> SYSPV_W<'_, PRIVCFGR3rs> {
        SYSPV_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn buspv(&mut self) -> BUSPV_W<'_, PRIVCFGR3rs> {
        BUSPV_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PER system configuration bits.
    #[inline(always)]
    pub fn perpv(&mut self) -> PERPV_W<'_, PRIVCFGR3rs> {
        PERPV_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intpv(&mut self) -> INTPV_W<'_, PRIVCFGR3rs> {
        INTPV_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstpv(&mut self) -> RSTPV_W<'_, PRIVCFGR3rs> {
        RSTPV_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftpv(&mut self) -> DFTPV_W<'_, PRIVCFGR3rs> {
        DFTPV_W::new(self, 6)
    }
}
/**RCC system privilege configuration register3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PRIVCFGR3)*/
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr3::R`](R) reader structure
impl crate::Readable for PRIVCFGR3rs {}
///`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR3 to value 0
impl crate::Resettable for PRIVCFGR3rs {}
