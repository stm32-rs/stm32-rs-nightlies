///Register `TS1ALARMB_CFGR` reader
pub type R = crate::R<TS1ALARMB_CFGRrs>;
///Register `TS1ALARMB_CFGR` writer
pub type W = crate::W<TS1ALARMB_CFGRrs>;
///Field `HYSTB_THRESH` reader - Alarm B hysteresis threshold
pub type HYSTB_THRESH_R = crate::FieldReader<u16>;
///Field `HYSTB_THRESH` writer - Alarm B hysteresis threshold
pub type HYSTB_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ALARMB_THRESH` reader - Alarm B threshold
pub type ALARMB_THRESH_R = crate::FieldReader<u16>;
///Field `ALARMB_THRESH` writer - Alarm B threshold
pub type ALARMB_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Alarm B hysteresis threshold
    #[inline(always)]
    pub fn hystb_thresh(&self) -> HYSTB_THRESH_R {
        HYSTB_THRESH_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Alarm B threshold
    #[inline(always)]
    pub fn alarmb_thresh(&self) -> ALARMB_THRESH_R {
        ALARMB_THRESH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS1ALARMB_CFGR")
            .field("hystb_thresh", &self.hystb_thresh())
            .field("alarmb_thresh", &self.alarmb_thresh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Alarm B hysteresis threshold
    #[inline(always)]
    pub fn hystb_thresh(&mut self) -> HYSTB_THRESH_W<'_, TS1ALARMB_CFGRrs> {
        HYSTB_THRESH_W::new(self, 0)
    }
    ///Bits 16:31 - Alarm B threshold
    #[inline(always)]
    pub fn alarmb_thresh(&mut self) -> ALARMB_THRESH_W<'_, TS1ALARMB_CFGRrs> {
        ALARMB_THRESH_W::new(self, 16)
    }
}
/**DTS TS1 alarm B configuration register

You can [`read`](crate::Reg::read) this register and get [`ts1alarmb_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1alarmb_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TS1ALARMB_CFGR)*/
pub struct TS1ALARMB_CFGRrs;
impl crate::RegisterSpec for TS1ALARMB_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ts1alarmb_cfgr::R`](R) reader structure
impl crate::Readable for TS1ALARMB_CFGRrs {}
///`write(|w| ..)` method takes [`ts1alarmb_cfgr::W`](W) writer structure
impl crate::Writable for TS1ALARMB_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS1ALARMB_CFGR to value 0
impl crate::Resettable for TS1ALARMB_CFGRrs {}
