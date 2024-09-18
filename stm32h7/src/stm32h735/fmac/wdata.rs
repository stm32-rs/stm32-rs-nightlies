///Register `WDATA` reader
pub type R = crate::R<WDATArs>;
///Register `WDATA` writer
pub type W = crate::W<WDATArs>;
///Field `WDATA` reader - Write data (write data are transferred to the address indicated by the write pointer)
pub type WDATA_R = crate::FieldReader<u16>;
///Field `WDATA` writer - Write data (write data are transferred to the address indicated by the write pointer)
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Write data (write data are transferred to the address indicated by the write pointer)
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATA")
            .field("wdata", &self.wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Write data (write data are transferred to the address indicated by the write pointer)
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WDATArs> {
        WDATA_W::new(self, 0)
    }
}
/**Write data register

You can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#FMAC:WDATA)*/
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
///`read()` method returns [`wdata::R`](R) reader structure
impl crate::Readable for WDATArs {}
///`write(|w| ..)` method takes [`wdata::W`](W) writer structure
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0;
}
