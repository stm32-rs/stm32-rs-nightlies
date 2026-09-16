///Register `PCFGWQOS0_0` reader
pub type R = crate::R<PCFGWQOS0_0rs>;
///Register `PCFGWQOS0_0` writer
pub type W = crate::W<PCFGWQOS0_0rs>;
///Field `WQOS_MAP_LEVEL1` reader - WQOS_MAP_LEVEL1
pub type WQOS_MAP_LEVEL1_R = crate::FieldReader;
///Field `WQOS_MAP_LEVEL1` writer - WQOS_MAP_LEVEL1
pub type WQOS_MAP_LEVEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WQOS_MAP_LEVEL2` reader - WQOS_MAP_LEVEL2
pub type WQOS_MAP_LEVEL2_R = crate::FieldReader;
///Field `WQOS_MAP_LEVEL2` writer - WQOS_MAP_LEVEL2
pub type WQOS_MAP_LEVEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WQOS_MAP_REGION0` reader - WQOS_MAP_REGION0
pub type WQOS_MAP_REGION0_R = crate::FieldReader;
///Field `WQOS_MAP_REGION0` writer - WQOS_MAP_REGION0
pub type WQOS_MAP_REGION0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WQOS_MAP_REGION1` reader - WQOS_MAP_REGION1
pub type WQOS_MAP_REGION1_R = crate::FieldReader;
///Field `WQOS_MAP_REGION1` writer - WQOS_MAP_REGION1
pub type WQOS_MAP_REGION1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WQOS_MAP_REGION2` reader - WQOS_MAP_REGION2
pub type WQOS_MAP_REGION2_R = crate::FieldReader;
///Field `WQOS_MAP_REGION2` writer - WQOS_MAP_REGION2
pub type WQOS_MAP_REGION2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - WQOS_MAP_LEVEL1
    #[inline(always)]
    pub fn wqos_map_level1(&self) -> WQOS_MAP_LEVEL1_R {
        WQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - WQOS_MAP_LEVEL2
    #[inline(always)]
    pub fn wqos_map_level2(&self) -> WQOS_MAP_LEVEL2_R {
        WQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:17 - WQOS_MAP_REGION0
    #[inline(always)]
    pub fn wqos_map_region0(&self) -> WQOS_MAP_REGION0_R {
        WQOS_MAP_REGION0_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - WQOS_MAP_REGION1
    #[inline(always)]
    pub fn wqos_map_region1(&self) -> WQOS_MAP_REGION1_R {
        WQOS_MAP_REGION1_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - WQOS_MAP_REGION2
    #[inline(always)]
    pub fn wqos_map_region2(&self) -> WQOS_MAP_REGION2_R {
        WQOS_MAP_REGION2_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFGWQOS0_0")
            .field("wqos_map_level1", &self.wqos_map_level1())
            .field("wqos_map_level2", &self.wqos_map_level2())
            .field("wqos_map_region0", &self.wqos_map_region0())
            .field("wqos_map_region1", &self.wqos_map_region1())
            .field("wqos_map_region2", &self.wqos_map_region2())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - WQOS_MAP_LEVEL1
    #[inline(always)]
    pub fn wqos_map_level1(&mut self) -> WQOS_MAP_LEVEL1_W<'_, PCFGWQOS0_0rs> {
        WQOS_MAP_LEVEL1_W::new(self, 0)
    }
    ///Bits 8:11 - WQOS_MAP_LEVEL2
    #[inline(always)]
    pub fn wqos_map_level2(&mut self) -> WQOS_MAP_LEVEL2_W<'_, PCFGWQOS0_0rs> {
        WQOS_MAP_LEVEL2_W::new(self, 8)
    }
    ///Bits 16:17 - WQOS_MAP_REGION0
    #[inline(always)]
    pub fn wqos_map_region0(&mut self) -> WQOS_MAP_REGION0_W<'_, PCFGWQOS0_0rs> {
        WQOS_MAP_REGION0_W::new(self, 16)
    }
    ///Bits 20:21 - WQOS_MAP_REGION1
    #[inline(always)]
    pub fn wqos_map_region1(&mut self) -> WQOS_MAP_REGION1_W<'_, PCFGWQOS0_0rs> {
        WQOS_MAP_REGION1_W::new(self, 20)
    }
    ///Bits 24:25 - WQOS_MAP_REGION2
    #[inline(always)]
    pub fn wqos_map_region2(&mut self) -> WQOS_MAP_REGION2_W<'_, PCFGWQOS0_0rs> {
        WQOS_MAP_REGION2_W::new(self, 24)
    }
}
/**DDRCTRL port 0 write Q0S configuration register 0

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PCFGWQOS0_0)*/
pub struct PCFGWQOS0_0rs;
impl crate::RegisterSpec for PCFGWQOS0_0rs {
    type Ux = u32;
}
///`read()` method returns [`pcfgwqos0_0::R`](R) reader structure
impl crate::Readable for PCFGWQOS0_0rs {}
///`write(|w| ..)` method takes [`pcfgwqos0_0::W`](W) writer structure
impl crate::Writable for PCFGWQOS0_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCFGWQOS0_0 to value 0x0e00
impl crate::Resettable for PCFGWQOS0_0rs {
    const RESET_VALUE: u32 = 0x0e00;
}
