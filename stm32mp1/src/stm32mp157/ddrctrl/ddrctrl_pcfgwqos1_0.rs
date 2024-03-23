#[doc = "Register `DDRCTRL_PCFGWQOS1_0` reader"]
pub type R = crate::R<DDRCTRL_PCFGWQOS1_0rs>;
#[doc = "Register `DDRCTRL_PCFGWQOS1_0` writer"]
pub type W = crate::W<DDRCTRL_PCFGWQOS1_0rs>;
#[doc = "Field `WQOS_MAP_TIMEOUT1` reader - WQOS_MAP_TIMEOUT1"]
pub type WQOS_MAP_TIMEOUT1_R = crate::FieldReader<u16>;
#[doc = "Field `WQOS_MAP_TIMEOUT1` writer - WQOS_MAP_TIMEOUT1"]
pub type WQOS_MAP_TIMEOUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WQOS_MAP_TIMEOUT2` reader - WQOS_MAP_TIMEOUT2"]
pub type WQOS_MAP_TIMEOUT2_R = crate::FieldReader<u16>;
#[doc = "Field `WQOS_MAP_TIMEOUT2` writer - WQOS_MAP_TIMEOUT2"]
pub type WQOS_MAP_TIMEOUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    pub fn wqos_map_timeout1(&self) -> WQOS_MAP_TIMEOUT1_R {
        WQOS_MAP_TIMEOUT1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    pub fn wqos_map_timeout2(&self) -> WQOS_MAP_TIMEOUT2_R {
        WQOS_MAP_TIMEOUT2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout1(&mut self) -> WQOS_MAP_TIMEOUT1_W<DDRCTRL_PCFGWQOS1_0rs> {
        WQOS_MAP_TIMEOUT1_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout2(&mut self) -> WQOS_MAP_TIMEOUT2_W<DDRCTRL_PCFGWQOS1_0rs> {
        WQOS_MAP_TIMEOUT2_W::new(self, 16)
    }
}
#[doc = "DDRCTRL port 0 write Q0S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos1_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos1_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCFGWQOS1_0rs;
impl crate::RegisterSpec for DDRCTRL_PCFGWQOS1_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pcfgwqos1_0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS1_0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pcfgwqos1_0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS1_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGWQOS1_0 to value 0"]
impl crate::Resettable for DDRCTRL_PCFGWQOS1_0rs {
    const RESET_VALUE: u32 = 0;
}
