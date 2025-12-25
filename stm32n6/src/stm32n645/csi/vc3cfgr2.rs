///Register `VC3CFGR2` reader
pub type R = crate::R<VC3CFGR2rs>;
///Register `VC3CFGR2` writer
pub type W = crate::W<VC3CFGR2rs>;
///Field `DT1` reader - Data type 1 class selection for virtual channel x
pub type DT1_R = crate::FieldReader;
///Field `DT1` writer - Data type 1 class selection for virtual channel x
pub type DT1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT1FT` reader - Data type 1 format
pub type DT1FT_R = crate::FieldReader;
///Field `DT1FT` writer - Data type 1 format
pub type DT1FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DT2` reader - Data type 2 class selection for virtual channel x
pub type DT2_R = crate::FieldReader;
///Field `DT2` writer - Data type 2 class selection for virtual channel x
pub type DT2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT2FT` reader - Data type 2 format
pub type DT2FT_R = crate::FieldReader;
///Field `DT2FT` writer - Data type 2 format
pub type DT2FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - Data type 1 class selection for virtual channel x
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Data type 1 format
    #[inline(always)]
    pub fn dt1ft(&self) -> DT1FT_R {
        DT1FT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:21 - Data type 2 class selection for virtual channel x
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:28 - Data type 2 format
    #[inline(always)]
    pub fn dt2ft(&self) -> DT2FT_R {
        DT2FT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VC3CFGR2")
            .field("dt1", &self.dt1())
            .field("dt1ft", &self.dt1ft())
            .field("dt2", &self.dt2())
            .field("dt2ft", &self.dt2ft())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Data type 1 class selection for virtual channel x
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W<'_, VC3CFGR2rs> {
        DT1_W::new(self, 0)
    }
    ///Bits 8:12 - Data type 1 format
    #[inline(always)]
    pub fn dt1ft(&mut self) -> DT1FT_W<'_, VC3CFGR2rs> {
        DT1FT_W::new(self, 8)
    }
    ///Bits 16:21 - Data type 2 class selection for virtual channel x
    #[inline(always)]
    pub fn dt2(&mut self) -> DT2_W<'_, VC3CFGR2rs> {
        DT2_W::new(self, 16)
    }
    ///Bits 24:28 - Data type 2 format
    #[inline(always)]
    pub fn dt2ft(&mut self) -> DT2FT_W<'_, VC3CFGR2rs> {
        DT2FT_W::new(self, 24)
    }
}
/**CSI-2 Host virtual channel 3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`vc3cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc3cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:VC3CFGR2)*/
pub struct VC3CFGR2rs;
impl crate::RegisterSpec for VC3CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`vc3cfgr2::R`](R) reader structure
impl crate::Readable for VC3CFGR2rs {}
///`write(|w| ..)` method takes [`vc3cfgr2::W`](W) writer structure
impl crate::Writable for VC3CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VC3CFGR2 to value 0
impl crate::Resettable for VC3CFGR2rs {}
