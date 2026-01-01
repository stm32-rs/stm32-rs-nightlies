///Register `SECCFGR3` reader
pub type R = crate::R<SECCFGR3rs>;
///Register `SECCFGR3` writer
pub type W = crate::W<SECCFGR3rs>;
///Field `MODSEC` reader - Defines the secure protection of the MOD system configuration bits.
pub type MODSEC_R = crate::BitReader;
///Field `MODSEC` writer - Defines the secure protection of the MOD system configuration bits.
pub type MODSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSSEC` reader - Defines the secure protection of the SYS system configuration bits.
pub type SYSSEC_R = crate::BitReader;
///Field `SYSSEC` writer - Defines the secure protection of the SYS system configuration bits.
pub type SYSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSSEC` reader - Defines the secure protection of the BUS system configuration bits.
pub type BUSSEC_R = crate::BitReader;
///Field `BUSSEC` writer - Defines the secure protection of the BUS system configuration bits.
pub type BUSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSEC` reader - Defines the secure protection of the PER system configuration bits.
pub type PERSEC_R = crate::BitReader;
///Field `PERSEC` writer - Defines the secure protection of the PER system configuration bits.
pub type PERSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTSEC` reader - Defines the secure protection of the INT system configuration bits.
pub type INTSEC_R = crate::BitReader;
///Field `INTSEC` writer - Defines the secure protection of the INT system configuration bits.
pub type INTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTSEC` reader - Defines the secure protection of the RST system configuration bits.
pub type RSTSEC_R = crate::BitReader;
///Field `RSTSEC` writer - Defines the secure protection of the RST system configuration bits.
pub type RSTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTSEC` reader - Defines the secure protection of the DFT system configuration bits.
pub type DFTSEC_R = crate::BitReader;
///Field `DFTSEC` writer - Defines the secure protection of the DFT system configuration bits.
pub type DFTSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the secure protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modsec(&self) -> MODSEC_R {
        MODSEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the secure protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syssec(&self) -> SYSSEC_R {
        SYSSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the secure protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn bussec(&self) -> BUSSEC_R {
        BUSSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the secure protection of the PER system configuration bits.
    #[inline(always)]
    pub fn persec(&self) -> PERSEC_R {
        PERSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the secure protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intsec(&self) -> INTSEC_R {
        INTSEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the secure protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstsec(&self) -> RSTSEC_R {
        RSTSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the secure protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftsec(&self) -> DFTSEC_R {
        DFTSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR3")
            .field("modsec", &self.modsec())
            .field("syssec", &self.syssec())
            .field("bussec", &self.bussec())
            .field("persec", &self.persec())
            .field("intsec", &self.intsec())
            .field("rstsec", &self.rstsec())
            .field("dftsec", &self.dftsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the MOD system configuration bits.
    #[inline(always)]
    pub fn modsec(&mut self) -> MODSEC_W<'_, SECCFGR3rs> {
        MODSEC_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the SYS system configuration bits.
    #[inline(always)]
    pub fn syssec(&mut self) -> SYSSEC_W<'_, SECCFGR3rs> {
        SYSSEC_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the BUS system configuration bits.
    #[inline(always)]
    pub fn bussec(&mut self) -> BUSSEC_W<'_, SECCFGR3rs> {
        BUSSEC_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the PER system configuration bits.
    #[inline(always)]
    pub fn persec(&mut self) -> PERSEC_W<'_, SECCFGR3rs> {
        PERSEC_W::new(self, 3)
    }
    ///Bit 4 - Defines the secure protection of the INT system configuration bits.
    #[inline(always)]
    pub fn intsec(&mut self) -> INTSEC_W<'_, SECCFGR3rs> {
        INTSEC_W::new(self, 4)
    }
    ///Bit 5 - Defines the secure protection of the RST system configuration bits.
    #[inline(always)]
    pub fn rstsec(&mut self) -> RSTSEC_W<'_, SECCFGR3rs> {
        RSTSEC_W::new(self, 5)
    }
    ///Bit 6 - Defines the secure protection of the DFT system configuration bits.
    #[inline(always)]
    pub fn dftsec(&mut self) -> DFTSEC_W<'_, SECCFGR3rs> {
        DFTSEC_W::new(self, 6)
    }
}
/**RCC system secure configuration register3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR3)*/
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr3::R`](R) reader structure
impl crate::Readable for SECCFGR3rs {}
///`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR3 to value 0
impl crate::Resettable for SECCFGR3rs {}
