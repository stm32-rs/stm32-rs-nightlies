///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `CEN` reader - Counter enable
pub type CEN_R = crate::BitReader;
///Field `CEN` writer - Counter enable
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDIS` reader - Update disable
pub type UDIS_R = crate::BitReader;
///Field `UDIS` writer - Update disable
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `URS` reader - Update request source
pub type URS_R = crate::BitReader;
///Field `URS` writer - Update request source
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPM` reader - One-pulse mode
pub type OPM_R = crate::BitReader;
///Field `OPM` writer - One-pulse mode
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARPE` reader - Auto-reload preload enable
pub type ARPE_R = crate::BitReader;
///Field `ARPE` writer - Auto-reload preload enable
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("arpe", &self.arpe())
            .field("opm", &self.opm())
            .field("urs", &self.urs())
            .field("udis", &self.udis())
            .field("cen", &self.cen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<'_, CR1rs> {
        CEN_W::new(self, 0)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W<'_, CR1rs> {
        UDIS_W::new(self, 1)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<'_, CR1rs> {
        URS_W::new(self, 2)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W<'_, CR1rs> {
        OPM_W::new(self, 3)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W<'_, CR1rs> {
        ARPE_W::new(self, 7)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#TIM6:CR1)*/
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
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
