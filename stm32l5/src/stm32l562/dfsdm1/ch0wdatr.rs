///Register `CH0WDATR` reader
pub type R = crate::R<CH0WDATRrs>;
///Register `CH0WDATR` writer
pub type W = crate::W<CH0WDATRrs>;
///Field `WDATA` reader - WDATA
pub type WDATA_R = crate::FieldReader<u16>;
///Field `WDATA` writer - WDATA
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH0WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<CH0WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
/**channel watchdog filter data register

You can [`read`](crate::Reg::read) this register and get [`ch0wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DFSDM1:CH0WDATR)*/
pub struct CH0WDATRrs;
impl crate::RegisterSpec for CH0WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch0wdatr::R`](R) reader structure
impl crate::Readable for CH0WDATRrs {}
///`write(|w| ..)` method takes [`ch0wdatr::W`](W) writer structure
impl crate::Writable for CH0WDATRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH0WDATR to value 0
impl crate::Resettable for CH0WDATRrs {}
