///Register `WRPSRP` reader
pub type R = crate::R<WRPSRPrs>;
///Register `WRPSRP` writer
pub type W = crate::W<WRPSRPrs>;
///Field `WRPS` reader - Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR
pub type WRPS_R = crate::FieldReader;
///Field `WRPS` writer - Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR
pub type WRPS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR
    #[inline(always)]
    pub fn wrps(&self) -> WRPS_R {
        WRPS_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSRP")
            .field("wrps", &self.wrps())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Write protection for sector n programming Write to change corresponding bit in FLASH_WRPSR
    #[inline(always)]
    pub fn wrps(&mut self) -> WRPS_W<'_, WRPSRPrs> {
        WRPS_W::new(self, 0)
    }
}
/**FLASH write protection status register programming

You can [`read`](crate::Reg::read) this register and get [`wrpsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:WRPSRP)*/
pub struct WRPSRPrs;
impl crate::RegisterSpec for WRPSRPrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsrp::R`](R) reader structure
impl crate::Readable for WRPSRPrs {}
///`write(|w| ..)` method takes [`wrpsrp::W`](W) writer structure
impl crate::Writable for WRPSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPSRP to value 0
impl crate::Resettable for WRPSRPrs {}
