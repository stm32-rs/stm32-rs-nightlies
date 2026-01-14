///Register `SECCFGR0` reader
pub type R = crate::R<SECCFGR0rs>;
///Register `SECCFGR0` writer
pub type W = crate::W<SECCFGR0rs>;
///Field `LSISEC` reader - Defines the secure protection of the LSI oscillator configuration bits.
pub type LSISEC_R = crate::BitReader;
///Field `LSISEC` writer - Defines the secure protection of the LSI oscillator configuration bits.
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESEC` reader - Defines the secure protection of the LSE oscillator configuration bits.
pub type LSESEC_R = crate::BitReader;
///Field `LSESEC` writer - Defines the secure protection of the LSE oscillator configuration bits.
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISEC` reader - Defines the secure protection of the MSI oscillator configuration bits.
pub type MSISEC_R = crate::BitReader;
///Field `MSISEC` writer - Defines the secure protection of the MSI oscillator configuration bits.
pub type MSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSISEC` reader - Defines the secure protection of the HSI oscillator configuration bits.
pub type HSISEC_R = crate::BitReader;
///Field `HSISEC` writer - Defines the secure protection of the HSI oscillator configuration bits.
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEC` reader - Defines the secure protection of the HSE oscillator configuration bits.
pub type HSESEC_R = crate::BitReader;
///Field `HSESEC` writer - Defines the secure protection of the HSE oscillator configuration bits.
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the secure protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the secure protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the secure protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the secure protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the secure protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR0")
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("msisec", &self.msisec())
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the LSI oscillator configuration bits.
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W<'_, SECCFGR0rs> {
        LSISEC_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the LSE oscillator configuration bits.
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W<'_, SECCFGR0rs> {
        LSESEC_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the MSI oscillator configuration bits.
    #[inline(always)]
    pub fn msisec(&mut self) -> MSISEC_W<'_, SECCFGR0rs> {
        MSISEC_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the HSI oscillator configuration bits.
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W<'_, SECCFGR0rs> {
        HSISEC_W::new(self, 3)
    }
    ///Bit 4 - Defines the secure protection of the HSE oscillator configuration bits.
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W<'_, SECCFGR0rs> {
        HSESEC_W::new(self, 4)
    }
}
/**RCC oscillator secure configuration register0

You can [`read`](crate::Reg::read) this register and get [`seccfgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:SECCFGR0)*/
pub struct SECCFGR0rs;
impl crate::RegisterSpec for SECCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr0::R`](R) reader structure
impl crate::Readable for SECCFGR0rs {}
///`write(|w| ..)` method takes [`seccfgr0::W`](W) writer structure
impl crate::Writable for SECCFGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR0 to value 0
impl crate::Resettable for SECCFGR0rs {}
