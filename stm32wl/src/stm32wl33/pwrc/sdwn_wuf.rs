///Register `SDWN_WUF` reader
pub type R = crate::R<SDWN_WUFrs>;
///Register `SDWN_WUF` writer
pub type W = crate::W<SDWN_WUFrs>;
///Field `WUF` reader - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred
pub type WUF_R = crate::BitReader;
///Field `WUF` writer - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred
pub type WUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDWN_WUF")
            .field("wuf", &self.wuf())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUF PB0 I/O WakeUp from shutdown Flag This bit is set when a wakeup from shutdown is detected on PB0 pin. It is cleared by a PORESETn or by writing 0 in this bit field. - 0: Shutdown wakeup from PB0 not occurred - 1: Shutdown wakeup from PB0 occurred
    #[inline(always)]
    pub fn wuf(&mut self) -> WUF_W<'_, SDWN_WUFrs> {
        WUF_W::new(self, 0)
    }
}
/**SDWN_WUF register

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SDWN_WUF)*/
pub struct SDWN_WUFrs;
impl crate::RegisterSpec for SDWN_WUFrs {
    type Ux = u32;
}
///`read()` method returns [`sdwn_wuf::R`](R) reader structure
impl crate::Readable for SDWN_WUFrs {}
///`write(|w| ..)` method takes [`sdwn_wuf::W`](W) writer structure
impl crate::Writable for SDWN_WUFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDWN_WUF to value 0
impl crate::Resettable for SDWN_WUFrs {}
