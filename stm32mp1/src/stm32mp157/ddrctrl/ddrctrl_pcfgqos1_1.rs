///Register `DDRCTRL_PCFGQOS1_1` reader
pub type R = crate::R<DDRCTRL_PCFGQOS1_1rs>;
///Register `DDRCTRL_PCFGQOS1_1` writer
pub type W = crate::W<DDRCTRL_PCFGQOS1_1rs>;
///Field `RQOS_MAP_TIMEOUTB` reader - RQOS_MAP_TIMEOUTB
pub type RQOS_MAP_TIMEOUTB_R = crate::FieldReader<u16>;
///Field `RQOS_MAP_TIMEOUTB` writer - RQOS_MAP_TIMEOUTB
pub type RQOS_MAP_TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RQOS_MAP_TIMEOUTR` reader - RQOS_MAP_TIMEOUTR
pub type RQOS_MAP_TIMEOUTR_R = crate::FieldReader<u16>;
///Field `RQOS_MAP_TIMEOUTR` writer - RQOS_MAP_TIMEOUTR
pub type RQOS_MAP_TIMEOUTR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - RQOS_MAP_TIMEOUTB
    #[inline(always)]
    pub fn rqos_map_timeoutb(&self) -> RQOS_MAP_TIMEOUTB_R {
        RQOS_MAP_TIMEOUTB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - RQOS_MAP_TIMEOUTR
    #[inline(always)]
    pub fn rqos_map_timeoutr(&self) -> RQOS_MAP_TIMEOUTR_R {
        RQOS_MAP_TIMEOUTR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_PCFGQOS1_1")
            .field("rqos_map_timeoutb", &self.rqos_map_timeoutb())
            .field("rqos_map_timeoutr", &self.rqos_map_timeoutr())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - RQOS_MAP_TIMEOUTB
    #[inline(always)]
    #[must_use]
    pub fn rqos_map_timeoutb(&mut self) -> RQOS_MAP_TIMEOUTB_W<DDRCTRL_PCFGQOS1_1rs> {
        RQOS_MAP_TIMEOUTB_W::new(self, 0)
    }
    ///Bits 16:26 - RQOS_MAP_TIMEOUTR
    #[inline(always)]
    #[must_use]
    pub fn rqos_map_timeoutr(&mut self) -> RQOS_MAP_TIMEOUTR_W<DDRCTRL_PCFGQOS1_1rs> {
        RQOS_MAP_TIMEOUTR_W::new(self, 16)
    }
}
/**DDRCTRL port 1 read Q0S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_pcfgqos1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_pcfgqos1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_PCFGQOS1_1)*/
pub struct DDRCTRL_PCFGQOS1_1rs;
impl crate::RegisterSpec for DDRCTRL_PCFGQOS1_1rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_pcfgqos1_1::R`](R) reader structure
impl crate::Readable for DDRCTRL_PCFGQOS1_1rs {}
///`write(|w| ..)` method takes [`ddrctrl_pcfgqos1_1::W`](W) writer structure
impl crate::Writable for DDRCTRL_PCFGQOS1_1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_PCFGQOS1_1 to value 0
impl crate::Resettable for DDRCTRL_PCFGQOS1_1rs {
    const RESET_VALUE: u32 = 0;
}
