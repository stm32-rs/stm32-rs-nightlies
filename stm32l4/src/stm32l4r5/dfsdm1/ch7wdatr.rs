///Register `CH7WDATR` reader
pub type R = crate::R<CH7WDATRrs>;
///Register `CH7WDATR` writer
pub type W = crate::W<CH7WDATRrs>;
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
        f.debug_struct("CH7WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<CH7WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
/**CH7WDATR

You can [`read`](crate::Reg::read) this register and get [`ch7wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:CH7WDATR)*/
pub struct CH7WDATRrs;
impl crate::RegisterSpec for CH7WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`ch7wdatr::R`](R) reader structure
impl crate::Readable for CH7WDATRrs {}
///`write(|w| ..)` method takes [`ch7wdatr::W`](W) writer structure
impl crate::Writable for CH7WDATRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH7WDATR to value 0
impl crate::Resettable for CH7WDATRrs {
    const RESET_VALUE: u32 = 0;
}
