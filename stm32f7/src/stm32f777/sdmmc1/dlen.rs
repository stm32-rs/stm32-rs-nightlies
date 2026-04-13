///Register `DLEN` reader
pub type R = crate::R<DLENrs>;
///Register `DLEN` writer
pub type W = crate::W<DLENrs>;
///Field `DATALENGTH` reader - Data length value
pub type DATALENGTH_R = crate::FieldReader<u32>;
///Field `DATALENGTH` writer - Data length value
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
impl R {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLEN")
            .field("datalength", &self.datalength())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Data length value
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W<'_, DLENrs> {
        DATALENGTH_W::new(self, 0)
    }
}
/**data length register

You can [`read`](crate::Reg::read) this register and get [`dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#SDMMC1:DLEN)*/
pub struct DLENrs;
impl crate::RegisterSpec for DLENrs {
    type Ux = u32;
}
///`read()` method returns [`dlen::R`](R) reader structure
impl crate::Readable for DLENrs {}
///`write(|w| ..)` method takes [`dlen::W`](W) writer structure
impl crate::Writable for DLENrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLEN to value 0
impl crate::Resettable for DLENrs {}
