///Register `VC2CFGR4` reader
pub type R = crate::R<VC2CFGR4rs>;
///Register `VC2CFGR4` writer
pub type W = crate::W<VC2CFGR4rs>;
///Field `DT5` reader - Data type 5 class selection for virtual channel x
pub type DT5_R = crate::FieldReader;
///Field `DT5` writer - Data type 5 class selection for virtual channel x
pub type DT5_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT5FT` reader - Data type 5 format
pub type DT5FT_R = crate::FieldReader;
///Field `DT5FT` writer - Data type 5 format
pub type DT5FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DT6` reader - Data type 6 class selection for virtual channel x
pub type DT6_R = crate::FieldReader;
///Field `DT6` writer - Data type 6 class selection for virtual channel x
pub type DT6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT6FT` reader - Data type 6 format
pub type DT6FT_R = crate::FieldReader;
///Field `DT6FT` writer - Data type 6 format
pub type DT6FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - Data type 5 class selection for virtual channel x
    #[inline(always)]
    pub fn dt5(&self) -> DT5_R {
        DT5_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Data type 5 format
    #[inline(always)]
    pub fn dt5ft(&self) -> DT5FT_R {
        DT5FT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:21 - Data type 6 class selection for virtual channel x
    #[inline(always)]
    pub fn dt6(&self) -> DT6_R {
        DT6_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:28 - Data type 6 format
    #[inline(always)]
    pub fn dt6ft(&self) -> DT6FT_R {
        DT6FT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VC2CFGR4")
            .field("dt5", &self.dt5())
            .field("dt5ft", &self.dt5ft())
            .field("dt6", &self.dt6())
            .field("dt6ft", &self.dt6ft())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Data type 5 class selection for virtual channel x
    #[inline(always)]
    pub fn dt5(&mut self) -> DT5_W<'_, VC2CFGR4rs> {
        DT5_W::new(self, 0)
    }
    ///Bits 8:12 - Data type 5 format
    #[inline(always)]
    pub fn dt5ft(&mut self) -> DT5FT_W<'_, VC2CFGR4rs> {
        DT5FT_W::new(self, 8)
    }
    ///Bits 16:21 - Data type 6 class selection for virtual channel x
    #[inline(always)]
    pub fn dt6(&mut self) -> DT6_W<'_, VC2CFGR4rs> {
        DT6_W::new(self, 16)
    }
    ///Bits 24:28 - Data type 6 format
    #[inline(always)]
    pub fn dt6ft(&mut self) -> DT6FT_W<'_, VC2CFGR4rs> {
        DT6FT_W::new(self, 24)
    }
}
/**CSI-2 Host virtual channel 2 configuration register 4

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:VC2CFGR4)*/
pub struct VC2CFGR4rs;
impl crate::RegisterSpec for VC2CFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`vc2cfgr4::R`](R) reader structure
impl crate::Readable for VC2CFGR4rs {}
///`write(|w| ..)` method takes [`vc2cfgr4::W`](W) writer structure
impl crate::Writable for VC2CFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VC2CFGR4 to value 0
impl crate::Resettable for VC2CFGR4rs {}
