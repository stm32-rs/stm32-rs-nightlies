///Register `DFLT4RSFR` reader
pub type R = crate::R<DFLT4RSFRrs>;
///Register `DFLT4RSFR` writer
pub type W = crate::W<DFLT4RSFRrs>;
///Field `RSFLTBYP` reader - Reshaper filter bypass
pub type RSFLTBYP_R = crate::BitReader;
///Field `RSFLTBYP` writer - Reshaper filter bypass
pub type RSFLTBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSFLTD` reader - Reshaper filter decimation ratio
pub type RSFLTD_R = crate::BitReader;
///Field `RSFLTD` writer - Reshaper filter decimation ratio
pub type RSFLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFBYP` reader - High-pass filter bypass
pub type HPFBYP_R = crate::BitReader;
///Field `HPFBYP` writer - High-pass filter bypass
pub type HPFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPFC` reader - High-pass filter cut-off frequency
pub type HPFC_R = crate::FieldReader;
///Field `HPFC` writer - High-pass filter cut-off frequency
pub type HPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    pub fn rsfltbyp(&self) -> RSFLTBYP_R {
        RSFLTBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    pub fn rsfltd(&self) -> RSFLTD_R {
        RSFLTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    pub fn hpfc(&self) -> HPFC_R {
        HPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT4RSFR")
            .field("rsfltbyp", &self.rsfltbyp())
            .field("rsfltd", &self.rsfltd())
            .field("hpfbyp", &self.hpfbyp())
            .field("hpfc", &self.hpfc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reshaper filter bypass
    #[inline(always)]
    pub fn rsfltbyp(&mut self) -> RSFLTBYP_W<'_, DFLT4RSFRrs> {
        RSFLTBYP_W::new(self, 0)
    }
    ///Bit 4 - Reshaper filter decimation ratio
    #[inline(always)]
    pub fn rsfltd(&mut self) -> RSFLTD_W<'_, DFLT4RSFRrs> {
        RSFLTD_W::new(self, 4)
    }
    ///Bit 7 - High-pass filter bypass
    #[inline(always)]
    pub fn hpfbyp(&mut self) -> HPFBYP_W<'_, DFLT4RSFRrs> {
        HPFBYP_W::new(self, 7)
    }
    ///Bits 8:9 - High-pass filter cut-off frequency
    #[inline(always)]
    pub fn hpfc(&mut self) -> HPFC_W<'_, DFLT4RSFRrs> {
        HPFC_W::new(self, 8)
    }
}
/**MDF reshape filter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4rsfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4rsfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:DFLT4RSFR)*/
pub struct DFLT4RSFRrs;
impl crate::RegisterSpec for DFLT4RSFRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt4rsfr::R`](R) reader structure
impl crate::Readable for DFLT4RSFRrs {}
///`write(|w| ..)` method takes [`dflt4rsfr::W`](W) writer structure
impl crate::Writable for DFLT4RSFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT4RSFR to value 0
impl crate::Resettable for DFLT4RSFRrs {}
