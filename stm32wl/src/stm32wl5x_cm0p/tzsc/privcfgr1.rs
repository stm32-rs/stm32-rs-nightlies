///Register `PRIVCFGR1` reader
pub type R = crate::R<PRIVCFGR1rs>;
///Register `PRIVCFGR1` writer
pub type W = crate::W<PRIVCFGR1rs>;
///Field `AESPRIV` reader - AESPRIV
pub type AESPRIV_R = crate::BitReader;
///Field `AESPRIV` writer - AESPRIV
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGPRIV` reader - RNGPRIV
pub type RNGPRIV_R = crate::BitReader;
///Field `RNGPRIV` writer - RNGPRIV
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUBGHZSPIPRIV` reader - SUBGHZSPIPRIV
pub type SUBGHZSPIPRIV_R = crate::BitReader;
///Field `SUBGHZSPIPRIV` writer - SUBGHZSPIPRIV
pub type SUBGHZSPIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAPRIV` reader - PKAPRIV
pub type PKAPRIV_R = crate::BitReader;
///Field `PKAPRIV` writer - PKAPRIV
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    pub fn subghzspipriv(&self) -> SUBGHZSPIPRIV_R {
        SUBGHZSPIPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR1")
            .field("aespriv", &self.aespriv())
            .field("rngpriv", &self.rngpriv())
            .field("subghzspipriv", &self.subghzspipriv())
            .field("pkapriv", &self.pkapriv())
            .finish()
    }
}
impl W {
    ///Bit 2 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W<'_, PRIVCFGR1rs> {
        AESPRIV_W::new(self, 2)
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<'_, PRIVCFGR1rs> {
        RNGPRIV_W::new(self, 3)
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    pub fn subghzspipriv(&mut self) -> SUBGHZSPIPRIV_W<'_, PRIVCFGR1rs> {
        SUBGHZSPIPRIV_W::new(self, 4)
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<'_, PRIVCFGR1rs> {
        PKAPRIV_W::new(self, 13)
    }
}
/**TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TZSC:PRIVCFGR1)*/
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr1::R`](R) reader structure
impl crate::Readable for PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1rs {}
