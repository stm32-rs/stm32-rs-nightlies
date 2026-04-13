///Register `PCFGWQOS1_1` reader
pub type R = crate::R<PCFGWQOS1_1rs>;
///Register `PCFGWQOS1_1` writer
pub type W = crate::W<PCFGWQOS1_1rs>;
///Field `WQOS_MAP_TIMEOUT1` reader - WQOS_MAP_TIMEOUT1
pub type WQOS_MAP_TIMEOUT1_R = crate::FieldReader<u16>;
///Field `WQOS_MAP_TIMEOUT1` writer - WQOS_MAP_TIMEOUT1
pub type WQOS_MAP_TIMEOUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `WQOS_MAP_TIMEOUT2` reader - WQOS_MAP_TIMEOUT2
pub type WQOS_MAP_TIMEOUT2_R = crate::FieldReader<u16>;
///Field `WQOS_MAP_TIMEOUT2` writer - WQOS_MAP_TIMEOUT2
pub type WQOS_MAP_TIMEOUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - WQOS_MAP_TIMEOUT1
    #[inline(always)]
    pub fn wqos_map_timeout1(&self) -> WQOS_MAP_TIMEOUT1_R {
        WQOS_MAP_TIMEOUT1_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - WQOS_MAP_TIMEOUT2
    #[inline(always)]
    pub fn wqos_map_timeout2(&self) -> WQOS_MAP_TIMEOUT2_R {
        WQOS_MAP_TIMEOUT2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCFGWQOS1_1")
            .field("wqos_map_timeout1", &self.wqos_map_timeout1())
            .field("wqos_map_timeout2", &self.wqos_map_timeout2())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - WQOS_MAP_TIMEOUT1
    #[inline(always)]
    pub fn wqos_map_timeout1(&mut self) -> WQOS_MAP_TIMEOUT1_W<'_, PCFGWQOS1_1rs> {
        WQOS_MAP_TIMEOUT1_W::new(self, 0)
    }
    ///Bits 16:26 - WQOS_MAP_TIMEOUT2
    #[inline(always)]
    pub fn wqos_map_timeout2(&mut self) -> WQOS_MAP_TIMEOUT2_W<'_, PCFGWQOS1_1rs> {
        WQOS_MAP_TIMEOUT2_W::new(self, 16)
    }
}
/**DDRCTRL port 1 write Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pcfgwqos1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfgwqos1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCFGWQOS1_1)*/
pub struct PCFGWQOS1_1rs;
impl crate::RegisterSpec for PCFGWQOS1_1rs {
    type Ux = u32;
}
///`read()` method returns [`pcfgwqos1_1::R`](R) reader structure
impl crate::Readable for PCFGWQOS1_1rs {}
///`write(|w| ..)` method takes [`pcfgwqos1_1::W`](W) writer structure
impl crate::Writable for PCFGWQOS1_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCFGWQOS1_1 to value 0
impl crate::Resettable for PCFGWQOS1_1rs {}
