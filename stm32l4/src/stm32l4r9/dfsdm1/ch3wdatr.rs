///Register `CH3WDATR` reader
pub type R = crate::R<CH3WDATRrs>;
///Register `CH3WDATR` writer
pub type W = crate::W<CH3WDATRrs>;
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
        f.debug_struct("CH3WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<CH3WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
/**CH3WDATR

You can [`read`](crate::Reg::read) this register and get [`ch3wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3WDATR)*/
pub struct CH3WDATRrs;
impl crate::RegisterSpec for CH3WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch3wdatr::R`](R) reader structure
impl crate::Readable for CH3WDATRrs {}
///`write(|w| ..)` method takes [`ch3wdatr::W`](W) writer structure
impl crate::Writable for CH3WDATRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH3WDATR to value 0
impl crate::Resettable for CH3WDATRrs {}
