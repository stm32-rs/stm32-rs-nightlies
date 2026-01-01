///Register `PCFGQOS0_0` reader
pub type R = crate::R<PCFGQOS0_0rs>;
///Register `PCFGQOS0_0` writer
pub type W = crate::W<PCFGQOS0_0rs>;
///Field `RQOS_MAP_LEVEL1` reader - RQOS_MAP_LEVEL1
pub type RQOS_MAP_LEVEL1_R = crate::FieldReader;
///Field `RQOS_MAP_LEVEL1` writer - RQOS_MAP_LEVEL1
pub type RQOS_MAP_LEVEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RQOS_MAP_LEVEL2` reader - RQOS_MAP_LEVEL2
pub type RQOS_MAP_LEVEL2_R = crate::FieldReader;
///Field `RQOS_MAP_LEVEL2` writer - RQOS_MAP_LEVEL2
pub type RQOS_MAP_LEVEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RQOS_MAP_REGION0` reader - RQOS_MAP_REGION0
pub type RQOS_MAP_REGION0_R = crate::FieldReader;
///Field `RQOS_MAP_REGION0` writer - RQOS_MAP_REGION0
pub type RQOS_MAP_REGION0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RQOS_MAP_REGION1` reader - RQOS_MAP_REGION1
pub type RQOS_MAP_REGION1_R = crate::FieldReader;
///Field `RQOS_MAP_REGION1` writer - RQOS_MAP_REGION1
pub type RQOS_MAP_REGION1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RQOS_MAP_REGION2` reader - RQOS_MAP_REGION2
pub type RQOS_MAP_REGION2_R = crate::FieldReader;
///Field `RQOS_MAP_REGION2` writer - RQOS_MAP_REGION2
pub type RQOS_MAP_REGION2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - RQOS_MAP_LEVEL1
    #[inline(always)]
    pub fn rqos_map_level1(&self) -> RQOS_MAP_LEVEL1_R {
        RQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - RQOS_MAP_LEVEL2
    #[inline(always)]
    pub fn rqos_map_level2(&self) -> RQOS_MAP_LEVEL2_R {
        RQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:17 - RQOS_MAP_REGION0
    #[inline(always)]
    pub fn rqos_map_region0(&self) -> RQOS_MAP_REGION0_R {
        RQOS_MAP_REGION0_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - RQOS_MAP_REGION1
    #[inline(always)]
    pub fn rqos_map_region1(&self) -> RQOS_MAP_REGION1_R {
        RQOS_MAP_REGION1_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - RQOS_MAP_REGION2
    #[inline(always)]
    pub fn rqos_map_region2(&self) -> RQOS_MAP_REGION2_R {
        RQOS_MAP_REGION2_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFGQOS0_0")
            .field("rqos_map_level1", &self.rqos_map_level1())
            .field("rqos_map_level2", &self.rqos_map_level2())
            .field("rqos_map_region0", &self.rqos_map_region0())
            .field("rqos_map_region1", &self.rqos_map_region1())
            .field("rqos_map_region2", &self.rqos_map_region2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - RQOS_MAP_LEVEL1
    #[inline(always)]
    pub fn rqos_map_level1(&mut self) -> RQOS_MAP_LEVEL1_W<'_, PCFGQOS0_0rs> {
        RQOS_MAP_LEVEL1_W::new(self, 0)
    }
    ///Bits 8:11 - RQOS_MAP_LEVEL2
    #[inline(always)]
    pub fn rqos_map_level2(&mut self) -> RQOS_MAP_LEVEL2_W<'_, PCFGQOS0_0rs> {
        RQOS_MAP_LEVEL2_W::new(self, 8)
    }
    ///Bits 16:17 - RQOS_MAP_REGION0
    #[inline(always)]
    pub fn rqos_map_region0(&mut self) -> RQOS_MAP_REGION0_W<'_, PCFGQOS0_0rs> {
        RQOS_MAP_REGION0_W::new(self, 16)
    }
    ///Bits 20:21 - RQOS_MAP_REGION1
    #[inline(always)]
    pub fn rqos_map_region1(&mut self) -> RQOS_MAP_REGION1_W<'_, PCFGQOS0_0rs> {
        RQOS_MAP_REGION1_W::new(self, 20)
    }
    ///Bits 24:25 - RQOS_MAP_REGION2
    #[inline(always)]
    pub fn rqos_map_region2(&mut self) -> RQOS_MAP_REGION2_W<'_, PCFGQOS0_0rs> {
        RQOS_MAP_REGION2_W::new(self, 24)
    }
}
/**DDRCTRL port 0 read Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgqos0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgqos0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PCFGQOS0_0)*/
pub struct PCFGQOS0_0rs;
impl crate::RegisterSpec for PCFGQOS0_0rs {
    type Ux = u32;
}
///`read()` method returns [`pcfgqos0_0::R`](R) reader structure
impl crate::Readable for PCFGQOS0_0rs {}
///`write(|w| ..)` method takes [`pcfgqos0_0::W`](W) writer structure
impl crate::Writable for PCFGQOS0_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCFGQOS0_0 to value 0x0200_0e00
impl crate::Resettable for PCFGQOS0_0rs {
    const RESET_VALUE: u32 = 0x0200_0e00;
}
