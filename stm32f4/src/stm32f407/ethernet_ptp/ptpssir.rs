///Register `PTPSSIR` reader
pub type R = crate::R<PTPSSIRrs>;
///Register `PTPSSIR` writer
pub type W = crate::W<PTPSSIRrs>;
///Field `STSSI` reader - STSSI
pub type STSSI_R = crate::FieldReader;
///Field `STSSI` writer - STSSI
pub type STSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - STSSI
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPSSIR")
            .field("stssi", &self.stssi())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - STSSI
    #[inline(always)]
    pub fn stssi(&mut self) -> STSSI_W<'_, PTPSSIRrs> {
        STSSI_W::new(self, 0)
    }
}
/**Ethernet PTP subsecond increment register

You can [`read`](crate::Reg::read) this register and get [`ptpssir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpssir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_PTP:PTPSSIR)*/
pub struct PTPSSIRrs;
impl crate::RegisterSpec for PTPSSIRrs {
    type Ux = u32;
}
///`read()` method returns [`ptpssir::R`](R) reader structure
impl crate::Readable for PTPSSIRrs {}
///`write(|w| ..)` method takes [`ptpssir::W`](W) writer structure
impl crate::Writable for PTPSSIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPSSIR to value 0
impl crate::Resettable for PTPSSIRrs {}
