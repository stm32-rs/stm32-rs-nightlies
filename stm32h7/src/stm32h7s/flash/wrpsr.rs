///Register `WRPSR` reader
pub type R = crate::R<WRPSRrs>;
///Register `WRPSR` writer
pub type W = crate::W<WRPSRrs>;
///Field `WRPS` reader - Write protection for sector n This bit reflects the write protection status of user Flash sector n
pub type WRPS_R = crate::FieldReader;
///Field `WRPS` writer - Write protection for sector n This bit reflects the write protection status of user Flash sector n
pub type WRPS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Write protection for sector n This bit reflects the write protection status of user Flash sector n
    #[inline(always)]
    pub fn wrps(&self) -> WRPS_R {
        WRPS_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSR").field("wrps", &self.wrps()).finish()
    }
}
impl W {
    ///Bits 0:7 - Write protection for sector n This bit reflects the write protection status of user Flash sector n
    #[inline(always)]
    pub fn wrps(&mut self) -> WRPS_W<'_, WRPSRrs> {
        WRPS_W::new(self, 0)
    }
}
/**FLASH write protection status register

You can [`read`](crate::Reg::read) this register and get [`wrpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:WRPSR)*/
pub struct WRPSRrs;
impl crate::RegisterSpec for WRPSRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsr::R`](R) reader structure
impl crate::Readable for WRPSRrs {}
///`write(|w| ..)` method takes [`wrpsr::W`](W) writer structure
impl crate::Writable for WRPSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPSR to value 0
impl crate::Resettable for WRPSRrs {}
