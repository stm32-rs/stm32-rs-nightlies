///Register `WD_CONF` reader
pub type R = crate::R<WD_CONFrs>;
///Register `WD_CONF` writer
pub type W = crate::W<WD_CONFrs>;
///Field `AWD_CHX` reader - analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog.
pub type AWD_CHX_R = crate::FieldReader<u16>;
///Field `AWD_CHX` writer - analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog.
pub type AWD_CHX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog.
    #[inline(always)]
    pub fn awd_chx(&self) -> AWD_CHX_R {
        AWD_CHX_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WD_CONF")
            .field("awd_chx", &self.awd_chx())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog.
    #[inline(always)]
    pub fn awd_chx(&mut self) -> AWD_CHX_W<WD_CONFrs> {
        AWD_CHX_W::new(self, 0)
    }
}
/**Channel selection for event monitoring register

You can [`read`](crate::Reg::read) this register and get [`wd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:WD_CONF)*/
pub struct WD_CONFrs;
impl crate::RegisterSpec for WD_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`wd_conf::R`](R) reader structure
impl crate::Readable for WD_CONFrs {}
///`write(|w| ..)` method takes [`wd_conf::W`](W) writer structure
impl crate::Writable for WD_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WD_CONF to value 0
impl crate::Resettable for WD_CONFrs {}
