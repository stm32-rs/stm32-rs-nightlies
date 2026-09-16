///Register `BDCR1` reader
pub type R = crate::R<BDCR1rs>;
///Register `BDCR1` writer
pub type W = crate::W<BDCR1rs>;
///Field `MONEN` reader - V less than sub>BAT less than /sub> and temperature monitoring enable
pub type MONEN_R = crate::BitReader;
///Field `MONEN` writer - V less than sub>BAT less than /sub> and temperature monitoring enable
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBATL` reader - V less than sub>BAT less than /sub> level monitoring versus low threshold
pub type VBATL_R = crate::BitReader;
///Field `VBATH` reader - V less than sub>BAT less than /sub> level monitoring versus high threshold
pub type VBATH_R = crate::BitReader;
///Field `TEMPL` reader - Temperature level monitoring versus low threshold
pub type TEMPL_R = crate::BitReader;
///Field `TEMPH` reader - Temperature level monitoring versus high threshold
pub type TEMPH_R = crate::BitReader;
impl R {
    ///Bit 0 - V less than sub>BAT less than /sub> and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - V less than sub>BAT less than /sub> level monitoring versus low threshold
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - V less than sub>BAT less than /sub> level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR1")
            .field("monen", &self.monen())
            .field("vbatl", &self.vbatl())
            .field("vbath", &self.vbath())
            .field("templ", &self.templ())
            .field("temph", &self.temph())
            .finish()
    }
}
impl W {
    ///Bit 0 - V less than sub>BAT less than /sub> and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W<'_, BDCR1rs> {
        MONEN_W::new(self, 0)
    }
}
/**PWR backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:BDCR1)*/
pub struct BDCR1rs;
impl crate::RegisterSpec for BDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bdcr1::R`](R) reader structure
impl crate::Readable for BDCR1rs {}
///`write(|w| ..)` method takes [`bdcr1::W`](W) writer structure
impl crate::Writable for BDCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR1 to value 0
impl crate::Resettable for BDCR1rs {}
