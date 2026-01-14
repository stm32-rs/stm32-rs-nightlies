///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable
pub type TAMP2E_R = crate::BitReader;
///Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3E` reader - Tamper detection on TAMP_IN3 enable
pub type TAMP3E_R = crate::BitReader;
///Field `TAMP3E` writer - Tamper detection on TAMP_IN3 enable
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP1E` reader - Internal tamper 1 enable: RTC power domain supply monitoring
pub type ITAMP1E_R = crate::BitReader;
///Field `ITAMP1E` writer - Internal tamper 1 enable: RTC power domain supply monitoring
pub type ITAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2E` reader - Internal tamper 2 enable: Temperature monitoring
pub type ITAMP2E_R = crate::BitReader;
///Field `ITAMP2E` writer - Internal tamper 2 enable: Temperature monitoring
pub type ITAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3E` reader - Internal tamper 3 enable: LSE monitoring
pub type ITAMP3E_R = crate::BitReader;
///Field `ITAMP3E` writer - Internal tamper 3 enable: LSE monitoring
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP4E` reader - Internal tamper 4 enable: HSE monitoring
pub type ITAMP4E_R = crate::BitReader;
///Field `ITAMP4E` writer - Internal tamper 4 enable: HSE monitoring
pub type ITAMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5E` reader - Internal tamper 5 enable: RTC calendar overflow
pub type ITAMP5E_R = crate::BitReader;
///Field `ITAMP5E` writer - Internal tamper 5 enable: RTC calendar overflow
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP6E` reader - Internal tamper 6 enable: ST manufacturer readout
pub type ITAMP6E_R = crate::BitReader;
///Field `ITAMP6E` writer - Internal tamper 6 enable: ST manufacturer readout
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8E` reader - Internal tamper 8 enable: monotonic counter overflow
pub type ITAMP8E_R = crate::BitReader;
///Field `ITAMP8E` writer - Internal tamper 8 enable: monotonic counter overflow
pub type ITAMP8E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper detection on TAMP_IN3 enable
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 enable: RTC power domain supply monitoring
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 enable: Temperature monitoring
    #[inline(always)]
    pub fn itamp2e(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 enable: monotonic counter overflow
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("tamp1e", &self.tamp1e())
            .field("tamp2e", &self.tamp2e())
            .field("tamp3e", &self.tamp3e())
            .field("itamp1e", &self.itamp1e())
            .field("itamp2e", &self.itamp2e())
            .field("itamp3e", &self.itamp3e())
            .field("itamp4e", &self.itamp4e())
            .field("itamp5e", &self.itamp5e())
            .field("itamp6e", &self.itamp6e())
            .field("itamp8e", &self.itamp8e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, CR1rs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<'_, CR1rs> {
        TAMP2E_W::new(self, 1)
    }
    ///Bit 2 - Tamper detection on TAMP_IN3 enable
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<'_, CR1rs> {
        TAMP3E_W::new(self, 2)
    }
    ///Bit 16 - Internal tamper 1 enable: RTC power domain supply monitoring
    #[inline(always)]
    pub fn itamp1e(&mut self) -> ITAMP1E_W<'_, CR1rs> {
        ITAMP1E_W::new(self, 16)
    }
    ///Bit 17 - Internal tamper 2 enable: Temperature monitoring
    #[inline(always)]
    pub fn itamp2e(&mut self) -> ITAMP2E_W<'_, CR1rs> {
        ITAMP2E_W::new(self, 17)
    }
    ///Bit 18 - Internal tamper 3 enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<'_, CR1rs> {
        ITAMP3E_W::new(self, 18)
    }
    ///Bit 19 - Internal tamper 4 enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4e(&mut self) -> ITAMP4E_W<'_, CR1rs> {
        ITAMP4E_W::new(self, 19)
    }
    ///Bit 20 - Internal tamper 5 enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<'_, CR1rs> {
        ITAMP5E_W::new(self, 20)
    }
    ///Bit 21 - Internal tamper 6 enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<'_, CR1rs> {
        ITAMP6E_W::new(self, 21)
    }
    ///Bit 23 - Internal tamper 8 enable: monotonic counter overflow
    #[inline(always)]
    pub fn itamp8e(&mut self) -> ITAMP8E_W<'_, CR1rs> {
        ITAMP8E_W::new(self, 23)
    }
}
/**TAMP control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#TAMP:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0xffff_0000
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
