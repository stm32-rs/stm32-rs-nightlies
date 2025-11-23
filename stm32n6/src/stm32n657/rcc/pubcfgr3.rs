///Register `PUBCFGR3` reader
pub type R = crate::R<PUBCFGR3rs>;
///Register `PUBCFGR3` writer
pub type W = crate::W<PUBCFGR3rs>;
///Field `MODPUB` reader - Defines the public protection of the MOD system configuration bits.
pub type MODPUB_R = crate::BitReader;
///Field `MODPUB` writer - Defines the public protection of the MOD system configuration bits.
pub type MODPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPUB` reader - Defines the public protection of the SYS system configuration bits.
pub type SYSPUB_R = crate::BitReader;
///Field `SYSPUB` writer - Defines the public protection of the SYS system configuration bits.
pub type SYSPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPUB` reader - Defines the public protection of the BUS system configuration bits.
pub type BUSPUB_R = crate::BitReader;
///Field `BUSPUB` writer - Defines the public protection of the BUS system configuration bits.
pub type BUSPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERPUB` reader - Defines the public protection of the PER system configuration bits.
pub type PERPUB_R = crate::BitReader;
///Field `PERPUB` writer - Defines the public protection of the PER system configuration bits.
pub type PERPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTPUB` reader - Defines the public protection of the INT system configuration bits.
pub type INTPUB_R = crate::BitReader;
///Field `INTPUB` writer - Defines the public protection of the INT system configuration bits.
pub type INTPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPUB` reader - Defines the public protection of the RST system configuration bits.
pub type RSTPUB_R = crate::BitReader;
///Field `RSTPUB` writer - Defines the public protection of the RST system configuration bits.
pub type RSTPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTPUB` reader - Defines the public protection of the DFT system configuration bits.
pub type DFTPUB_R = crate::BitReader;
///Field `DFTPUB` writer - Defines the public protection of the DFT system configuration bits.
pub type DFTPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the public protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modpub(&self) -> MODPUB_R {
        MODPUB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the public protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syspub(&self) -> SYSPUB_R {
        SYSPUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the public protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn buspub(&self) -> BUSPUB_R {
        BUSPUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the public protection of the PER system configuration bits.
    #[inline(always)]
    pub fn perpub(&self) -> PERPUB_R {
        PERPUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the public protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intpub(&self) -> INTPUB_R {
        INTPUB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the public protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstpub(&self) -> RSTPUB_R {
        RSTPUB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the public protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftpub(&self) -> DFTPUB_R {
        DFTPUB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUBCFGR3")
            .field("modpub", &self.modpub())
            .field("syspub", &self.syspub())
            .field("buspub", &self.buspub())
            .field("perpub", &self.perpub())
            .field("intpub", &self.intpub())
            .field("rstpub", &self.rstpub())
            .field("dftpub", &self.dftpub())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modpub(&mut self) -> MODPUB_W<'_, PUBCFGR3rs> {
        MODPUB_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syspub(&mut self) -> SYSPUB_W<'_, PUBCFGR3rs> {
        SYSPUB_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn buspub(&mut self) -> BUSPUB_W<'_, PUBCFGR3rs> {
        BUSPUB_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PER system configuration bits.
    #[inline(always)]
    pub fn perpub(&mut self) -> PERPUB_W<'_, PUBCFGR3rs> {
        PERPUB_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intpub(&mut self) -> INTPUB_W<'_, PUBCFGR3rs> {
        INTPUB_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstpub(&mut self) -> RSTPUB_W<'_, PUBCFGR3rs> {
        RSTPUB_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftpub(&mut self) -> DFTPUB_W<'_, PUBCFGR3rs> {
        DFTPUB_W::new(self, 6)
    }
}
/**RCC system public configuration register3

You can [`read`](crate::Reg::read) this register and get [`pubcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PUBCFGR3)*/
pub struct PUBCFGR3rs;
impl crate::RegisterSpec for PUBCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`pubcfgr3::R`](R) reader structure
impl crate::Readable for PUBCFGR3rs {}
///`write(|w| ..)` method takes [`pubcfgr3::W`](W) writer structure
impl crate::Writable for PUBCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGR3 to value 0
impl crate::Resettable for PUBCFGR3rs {}
