///Register `PRIVCFGR0` reader
pub type R = crate::R<PRIVCFGR0rs>;
///Register `PRIVCFGR0` writer
pub type W = crate::W<PRIVCFGR0rs>;
///Field `LSIPV` reader - Defines the privilege protection of the LSI oscillator configuration bits.
pub type LSIPV_R = crate::BitReader;
///Field `LSIPV` writer - Defines the privilege protection of the LSI oscillator configuration bits.
pub type LSIPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPV` reader - Defines the privilege protection of the LSE oscillator configuration bits.
pub type LSEPV_R = crate::BitReader;
///Field `LSEPV` writer - Defines the privilege protection of the LSE oscillator configuration bits.
pub type LSEPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPV` reader - Defines the privilege protection of the MSI oscillator configuration bits.
pub type MSIPV_R = crate::BitReader;
///Field `MSIPV` writer - Defines the privilege protection of the MSI oscillator configuration bits.
pub type MSIPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPV` reader - Defines the privilege protection of the HSI oscillator configuration bits.
pub type HSIPV_R = crate::BitReader;
///Field `HSIPV` writer - Defines the privilege protection of the HSI oscillator configuration bits.
pub type HSIPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPV` reader - Defines the privilege protection of the HSE oscillator configuration bits.
pub type HSEPV_R = crate::BitReader;
///Field `HSEPV` writer - Defines the privilege protection of the HSE oscillator configuration bits.
pub type HSEPV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the privilege protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsipv(&self) -> LSIPV_R {
        LSIPV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the privilege protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsepv(&self) -> LSEPV_R {
        LSEPV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the privilege protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msipv(&self) -> MSIPV_R {
        MSIPV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the privilege protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsipv(&self) -> HSIPV_R {
        HSIPV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the privilege protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsepv(&self) -> HSEPV_R {
        HSEPV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR0")
            .field("lsipv", &self.lsipv())
            .field("lsepv", &self.lsepv())
            .field("msipv", &self.msipv())
            .field("hsipv", &self.hsipv())
            .field("hsepv", &self.hsepv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsipv(&mut self) -> LSIPV_W<'_, PRIVCFGR0rs> {
        LSIPV_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsepv(&mut self) -> LSEPV_W<'_, PRIVCFGR0rs> {
        LSEPV_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msipv(&mut self) -> MSIPV_W<'_, PRIVCFGR0rs> {
        MSIPV_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsipv(&mut self) -> HSIPV_W<'_, PRIVCFGR0rs> {
        HSIPV_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsepv(&mut self) -> HSEPV_W<'_, PRIVCFGR0rs> {
        HSEPV_W::new(self, 4)
    }
}
/**RCC oscillator privilege configuration register0

You can [`read`](crate::Reg::read) this register and get [`privcfgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PRIVCFGR0)*/
pub struct PRIVCFGR0rs;
impl crate::RegisterSpec for PRIVCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr0::R`](R) reader structure
impl crate::Readable for PRIVCFGR0rs {}
///`write(|w| ..)` method takes [`privcfgr0::W`](W) writer structure
impl crate::Writable for PRIVCFGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR0 to value 0
impl crate::Resettable for PRIVCFGR0rs {}
