#[doc = "Register `DDRCTRL_PCFGWQOS0_0` reader"]
pub type R = crate::R<DDRCTRL_PCFGWQOS0_0rs>;
#[doc = "Register `DDRCTRL_PCFGWQOS0_0` writer"]
pub type W = crate::W<DDRCTRL_PCFGWQOS0_0rs>;
#[doc = "Field `WQOS_MAP_LEVEL1` reader - WQOS_MAP_LEVEL1"]
pub type WQOS_MAP_LEVEL1_R = crate::FieldReader;
#[doc = "Field `WQOS_MAP_LEVEL1` writer - WQOS_MAP_LEVEL1"]
pub type WQOS_MAP_LEVEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WQOS_MAP_LEVEL2` reader - WQOS_MAP_LEVEL2"]
pub type WQOS_MAP_LEVEL2_R = crate::FieldReader;
#[doc = "Field `WQOS_MAP_LEVEL2` writer - WQOS_MAP_LEVEL2"]
pub type WQOS_MAP_LEVEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WQOS_MAP_REGION0` reader - WQOS_MAP_REGION0"]
pub type WQOS_MAP_REGION0_R = crate::FieldReader;
#[doc = "Field `WQOS_MAP_REGION0` writer - WQOS_MAP_REGION0"]
pub type WQOS_MAP_REGION0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WQOS_MAP_REGION1` reader - WQOS_MAP_REGION1"]
pub type WQOS_MAP_REGION1_R = crate::FieldReader;
#[doc = "Field `WQOS_MAP_REGION1` writer - WQOS_MAP_REGION1"]
pub type WQOS_MAP_REGION1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WQOS_MAP_REGION2` reader - WQOS_MAP_REGION2"]
pub type WQOS_MAP_REGION2_R = crate::FieldReader;
#[doc = "Field `WQOS_MAP_REGION2` writer - WQOS_MAP_REGION2"]
pub type WQOS_MAP_REGION2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn wqos_map_level1(&self) -> WQOS_MAP_LEVEL1_R {
        WQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn wqos_map_level2(&self) -> WQOS_MAP_LEVEL2_R {
        WQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn wqos_map_region0(&self) -> WQOS_MAP_REGION0_R {
        WQOS_MAP_REGION0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn wqos_map_region1(&self) -> WQOS_MAP_REGION1_R {
        WQOS_MAP_REGION1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn wqos_map_region2(&self) -> WQOS_MAP_REGION2_R {
        WQOS_MAP_REGION2_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_level1(&mut self) -> WQOS_MAP_LEVEL1_W<DDRCTRL_PCFGWQOS0_0rs> {
        WQOS_MAP_LEVEL1_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_level2(&mut self) -> WQOS_MAP_LEVEL2_W<DDRCTRL_PCFGWQOS0_0rs> {
        WQOS_MAP_LEVEL2_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_region0(&mut self) -> WQOS_MAP_REGION0_W<DDRCTRL_PCFGWQOS0_0rs> {
        WQOS_MAP_REGION0_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_region1(&mut self) -> WQOS_MAP_REGION1_W<DDRCTRL_PCFGWQOS0_0rs> {
        WQOS_MAP_REGION1_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_region2(&mut self) -> WQOS_MAP_REGION2_W<DDRCTRL_PCFGWQOS0_0rs> {
        WQOS_MAP_REGION2_W::new(self, 24)
    }
}
#[doc = "DDRCTRL port 0 write Q0S configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos0_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos0_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCFGWQOS0_0rs;
impl crate::RegisterSpec for DDRCTRL_PCFGWQOS0_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pcfgwqos0_0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS0_0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pcfgwqos0_0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS0_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGWQOS0_0 to value 0x0e00"]
impl crate::Resettable for DDRCTRL_PCFGWQOS0_0rs {
    const RESET_VALUE: u32 = 0x0e00;
}
