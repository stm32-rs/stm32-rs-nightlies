///Register `TIMEOUTREG` reader
pub type R = crate::R<TIMEOUTREGrs>;
///Register `TIMEOUTREG` writer
pub type W = crate::W<TIMEOUTREGrs>;
///Field `TIMEOUT` reader - Timer1 or Timer2 Timeout value (depending on Destination register)
pub type TIMEOUT_R = crate::FieldReader<u32>;
///Field `TIMEOUT` writer - Timer1 or Timer2 Timeout value (depending on Destination register)
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Timer1 or Timer2 Timeout value (depending on Destination register)
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUTREG")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Timer1 or Timer2 Timeout value (depending on Destination register)
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<'_, TIMEOUTREGrs> {
        TIMEOUT_W::new(self, 0)
    }
}
/**TIMEOUTREG register

You can [`read`](crate::Reg::read) this register and get [`timeoutreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:TIMEOUTREG)*/
pub struct TIMEOUTREGrs;
impl crate::RegisterSpec for TIMEOUTREGrs {
    type Ux = u32;
}
///`read()` method returns [`timeoutreg::R`](R) reader structure
impl crate::Readable for TIMEOUTREGrs {}
///`write(|w| ..)` method takes [`timeoutreg::W`](W) writer structure
impl crate::Writable for TIMEOUTREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMEOUTREG to value 0
impl crate::Resettable for TIMEOUTREGrs {}
