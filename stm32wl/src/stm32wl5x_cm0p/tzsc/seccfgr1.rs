///Register `SECCFGR1` reader
pub type R = crate::R<SECCFGR1rs>;
///Register `SECCFGR1` writer
pub type W = crate::W<SECCFGR1rs>;
///Field `AESSEC` reader - AESSEC
pub type AESSEC_R = crate::BitReader;
///Field `AESSEC` writer - AESSEC
pub type AESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEC` reader - RNGSEC
pub type RNGSEC_R = crate::BitReader;
///Field `RNGSEC` writer - RNGSEC
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKASEC` reader - PKASEC
pub type PKASEC_R = crate::BitReader;
///Field `PKASEC` writer - PKASEC
pub type PKASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - AESSEC
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGSEC
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - PKASEC
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR1")
            .field("aessec", &self.aessec())
            .field("rngsec", &self.rngsec())
            .field("pkasec", &self.pkasec())
            .finish()
    }
}
impl W {
    ///Bit 2 - AESSEC
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W<'_, SECCFGR1rs> {
        AESSEC_W::new(self, 2)
    }
    ///Bit 3 - RNGSEC
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W<'_, SECCFGR1rs> {
        RNGSEC_W::new(self, 3)
    }
    ///Bit 13 - PKASEC
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W<'_, SECCFGR1rs> {
        PKASEC_W::new(self, 13)
    }
}
/**TZSC security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TZSC:SECCFGR1)*/
pub struct SECCFGR1rs;
impl crate::RegisterSpec for SECCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr1::R`](R) reader structure
impl crate::Readable for SECCFGR1rs {}
///`write(|w| ..)` method takes [`seccfgr1::W`](W) writer structure
impl crate::Writable for SECCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR1 to value 0
impl crate::Resettable for SECCFGR1rs {}
