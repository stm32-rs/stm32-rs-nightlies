///Register `VC2CFGR3` reader
pub type R = crate::R<VC2CFGR3rs>;
///Register `VC2CFGR3` writer
pub type W = crate::W<VC2CFGR3rs>;
///Field `DT3` reader - Data type 3 class selection for virtual channel x
pub type DT3_R = crate::FieldReader;
///Field `DT3` writer - Data type 3 class selection for virtual channel x
pub type DT3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT3FT` reader - Data type 3 format
pub type DT3FT_R = crate::FieldReader;
///Field `DT3FT` writer - Data type 3 format
pub type DT3FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DT4` reader - Data type 4 class selection for virtual channel x
pub type DT4_R = crate::FieldReader;
///Field `DT4` writer - Data type 4 class selection for virtual channel x
pub type DT4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT4FT` reader - Data type 4 format
pub type DT4FT_R = crate::FieldReader;
///Field `DT4FT` writer - Data type 4 format
pub type DT4FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - Data type 3 class selection for virtual channel x
    #[inline(always)]
    pub fn dt3(&self) -> DT3_R {
        DT3_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Data type 3 format
    #[inline(always)]
    pub fn dt3ft(&self) -> DT3FT_R {
        DT3FT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:21 - Data type 4 class selection for virtual channel x
    #[inline(always)]
    pub fn dt4(&self) -> DT4_R {
        DT4_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:28 - Data type 4 format
    #[inline(always)]
    pub fn dt4ft(&self) -> DT4FT_R {
        DT4FT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VC2CFGR3")
            .field("dt3", &self.dt3())
            .field("dt3ft", &self.dt3ft())
            .field("dt4", &self.dt4())
            .field("dt4ft", &self.dt4ft())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Data type 3 class selection for virtual channel x
    #[inline(always)]
    pub fn dt3(&mut self) -> DT3_W<'_, VC2CFGR3rs> {
        DT3_W::new(self, 0)
    }
    ///Bits 8:12 - Data type 3 format
    #[inline(always)]
    pub fn dt3ft(&mut self) -> DT3FT_W<'_, VC2CFGR3rs> {
        DT3FT_W::new(self, 8)
    }
    ///Bits 16:21 - Data type 4 class selection for virtual channel x
    #[inline(always)]
    pub fn dt4(&mut self) -> DT4_W<'_, VC2CFGR3rs> {
        DT4_W::new(self, 16)
    }
    ///Bits 24:28 - Data type 4 format
    #[inline(always)]
    pub fn dt4ft(&mut self) -> DT4FT_W<'_, VC2CFGR3rs> {
        DT4FT_W::new(self, 24)
    }
}
/**CSI-2 Host virtual channel 2 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:VC2CFGR3)*/
pub struct VC2CFGR3rs;
impl crate::RegisterSpec for VC2CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`vc2cfgr3::R`](R) reader structure
impl crate::Readable for VC2CFGR3rs {}
///`write(|w| ..)` method takes [`vc2cfgr3::W`](W) writer structure
impl crate::Writable for VC2CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VC2CFGR3 to value 0
impl crate::Resettable for VC2CFGR3rs {}
