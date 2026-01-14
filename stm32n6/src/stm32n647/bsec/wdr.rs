///Register `WDR` writer
pub type W = crate::W<WDRrs>;
///Field `WRDATA` writer - OTP write data
pub type WRDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<WDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OTP write data
    #[inline(always)]
    pub fn wrdata(&mut self) -> WRDATA_W<'_, WDRrs> {
        WRDATA_W::new(self, 0)
    }
}
/**BSEC write data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:WDR)*/
pub struct WDRrs;
impl crate::RegisterSpec for WDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdr::W`](W) writer structure
impl crate::Writable for WDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDR to value 0
impl crate::Resettable for WDRrs {}
