///Register `WDATR` reader
pub type R = crate::R<WDATRrs>;
///Register `WDATR` writer
pub type W = crate::W<WDATRrs>;
///Field `WDATA` reader - WDATA
pub type WDATA_R = crate::FieldReader<u16>;
///Field `WDATA` writer - WDATA
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATR")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<'_, WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
/**channel watchdog filter data register

You can [`read`](crate::Reg::read) this register and get [`wdatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDATRrs;
impl crate::RegisterSpec for WDATRrs {
    type Ux = u32;
}
///`read()` method returns [`wdatr::R`](R) reader structure
impl crate::Readable for WDATRrs {}
///`write(|w| ..)` method takes [`wdatr::W`](W) writer structure
impl crate::Writable for WDATRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDATR to value 0
impl crate::Resettable for WDATRrs {}
