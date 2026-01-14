///Register `JTAGOUTR` writer
pub type W = crate::W<JTAGOUTRrs>;
///Field `JDATAOUT` writer - JTAG output data
pub type JDATAOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<JTAGOUTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - JTAG output data
    #[inline(always)]
    pub fn jdataout(&mut self) -> JDATAOUT_W<'_, JTAGOUTRrs> {
        JDATAOUT_W::new(self, 0)
    }
}
/**BSEC JTAG output register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtagoutr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:JTAGOUTR)*/
pub struct JTAGOUTRrs;
impl crate::RegisterSpec for JTAGOUTRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`jtagoutr::W`](W) writer structure
impl crate::Writable for JTAGOUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JTAGOUTR to value 0
impl crate::Resettable for JTAGOUTRrs {}
