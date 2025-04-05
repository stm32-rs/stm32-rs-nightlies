///Register `CH6WDATR` reader
pub type R = crate::R<CH6WDATRrs>;
///Register `CH6WDATR` writer
pub type W = crate::W<CH6WDATRrs>;
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
        f.debug_struct("CH6WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<CH6WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
/**CHWDAT6R

You can [`read`](crate::Reg::read) this register and get [`ch6wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DFSDM1:CH6WDATR)*/
pub struct CH6WDATRrs;
impl crate::RegisterSpec for CH6WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch6wdatr::R`](R) reader structure
impl crate::Readable for CH6WDATRrs {}
///`write(|w| ..)` method takes [`ch6wdatr::W`](W) writer structure
impl crate::Writable for CH6WDATRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH6WDATR to value 0
impl crate::Resettable for CH6WDATRrs {}
