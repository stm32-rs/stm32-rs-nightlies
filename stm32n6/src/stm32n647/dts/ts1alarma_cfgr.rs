///Register `TS1ALARMA_CFGR` reader
pub type R = crate::R<TS1ALARMA_CFGRrs>;
///Register `TS1ALARMA_CFGR` writer
pub type W = crate::W<TS1ALARMA_CFGRrs>;
///Field `HYSTA_THRESH` reader - Alarm A hysteresis threshold
pub type HYSTA_THRESH_R = crate::FieldReader<u16>;
///Field `HYSTA_THRESH` writer - Alarm A hysteresis threshold
pub type HYSTA_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ALARMA_THRESH` reader - Alarm A threshold
pub type ALARMA_THRESH_R = crate::FieldReader<u16>;
///Field `ALARMA_THRESH` writer - Alarm A threshold
pub type ALARMA_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Alarm A hysteresis threshold
    #[inline(always)]
    pub fn hysta_thresh(&self) -> HYSTA_THRESH_R {
        HYSTA_THRESH_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Alarm A threshold
    #[inline(always)]
    pub fn alarma_thresh(&self) -> ALARMA_THRESH_R {
        ALARMA_THRESH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS1ALARMA_CFGR")
            .field("hysta_thresh", &self.hysta_thresh())
            .field("alarma_thresh", &self.alarma_thresh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Alarm A hysteresis threshold
    #[inline(always)]
    pub fn hysta_thresh(&mut self) -> HYSTA_THRESH_W<'_, TS1ALARMA_CFGRrs> {
        HYSTA_THRESH_W::new(self, 0)
    }
    ///Bits 16:31 - Alarm A threshold
    #[inline(always)]
    pub fn alarma_thresh(&mut self) -> ALARMA_THRESH_W<'_, TS1ALARMA_CFGRrs> {
        ALARMA_THRESH_W::new(self, 16)
    }
}
/**DTS TS1 alarm A configuration register

You can [`read`](crate::Reg::read) this register and get [`ts1alarma_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1alarma_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1ALARMA_CFGR)*/
pub struct TS1ALARMA_CFGRrs;
impl crate::RegisterSpec for TS1ALARMA_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ts1alarma_cfgr::R`](R) reader structure
impl crate::Readable for TS1ALARMA_CFGRrs {}
///`write(|w| ..)` method takes [`ts1alarma_cfgr::W`](W) writer structure
impl crate::Writable for TS1ALARMA_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS1ALARMA_CFGR to value 0
impl crate::Resettable for TS1ALARMA_CFGRrs {}
