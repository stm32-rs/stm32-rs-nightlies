///Register `PUBCFGR0` reader
pub type R = crate::R<PUBCFGR0rs>;
///Register `PUBCFGR0` writer
pub type W = crate::W<PUBCFGR0rs>;
///Field `LSIPUB` reader - Defines the public protection of the LSI oscillator configuration bits.
pub type LSIPUB_R = crate::BitReader;
///Field `LSIPUB` writer - Defines the public protection of the LSI oscillator configuration bits.
pub type LSIPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPUB` reader - Defines the public protection of the LSE oscillator configuration bits.
pub type LSEPUB_R = crate::BitReader;
///Field `LSEPUB` writer - Defines the public protection of the LSE oscillator configuration bits.
pub type LSEPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPUB` reader - Defines the public protection of the MSI oscillator configuration bits.
pub type MSIPUB_R = crate::BitReader;
///Field `MSIPUB` writer - Defines the public protection of the MSI oscillator configuration bits.
pub type MSIPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPUB` reader - Defines the public protection of the HSI oscillator configuration bits.
pub type HSIPUB_R = crate::BitReader;
///Field `HSIPUB` writer - Defines the public protection of the HSI oscillator configuration bits.
pub type HSIPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPUB` reader - Defines the public protection of the HSE oscillator configuration bits.
pub type HSEPUB_R = crate::BitReader;
///Field `HSEPUB` writer - Defines the public protection of the HSE oscillator configuration bits.
pub type HSEPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the public protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsipub(&self) -> LSIPUB_R {
        LSIPUB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the public protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsepub(&self) -> LSEPUB_R {
        LSEPUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the public protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msipub(&self) -> MSIPUB_R {
        MSIPUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the public protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsipub(&self) -> HSIPUB_R {
        HSIPUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the public protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsepub(&self) -> HSEPUB_R {
        HSEPUB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUBCFGR0")
            .field("lsipub", &self.lsipub())
            .field("lsepub", &self.lsepub())
            .field("msipub", &self.msipub())
            .field("hsipub", &self.hsipub())
            .field("hsepub", &self.hsepub())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsipub(&mut self) -> LSIPUB_W<'_, PUBCFGR0rs> {
        LSIPUB_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsepub(&mut self) -> LSEPUB_W<'_, PUBCFGR0rs> {
        LSEPUB_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msipub(&mut self) -> MSIPUB_W<'_, PUBCFGR0rs> {
        MSIPUB_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsipub(&mut self) -> HSIPUB_W<'_, PUBCFGR0rs> {
        HSIPUB_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsepub(&mut self) -> HSEPUB_W<'_, PUBCFGR0rs> {
        HSEPUB_W::new(self, 4)
    }
}
/**RCC oscillator public configuration register0

You can [`read`](crate::Reg::read) this register and get [`pubcfgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGR0)*/
pub struct PUBCFGR0rs;
impl crate::RegisterSpec for PUBCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`pubcfgr0::R`](R) reader structure
impl crate::Readable for PUBCFGR0rs {}
///`write(|w| ..)` method takes [`pubcfgr0::W`](W) writer structure
impl crate::Writable for PUBCFGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGR0 to value 0
impl crate::Resettable for PUBCFGR0rs {}
