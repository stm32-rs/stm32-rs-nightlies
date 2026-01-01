///Register `PTPTTLR` reader
pub type R = crate::R<PTPTTLRrs>;
///Register `PTPTTLR` writer
pub type W = crate::W<PTPTTLRrs>;
///Field `TTSL` reader - Target time stamp low
pub type TTSL_R = crate::FieldReader<u32>;
///Field `TTSL` writer - Target time stamp low
pub type TTSL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Target time stamp low
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTTLR")
            .field("ttsl", &self.ttsl())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Target time stamp low
    #[inline(always)]
    pub fn ttsl(&mut self) -> TTSL_W<'_, PTPTTLRrs> {
        TTSL_W::new(self, 0)
    }
}
/**Ethernet PTP target time low register

You can [`read`](crate::Reg::read) this register and get [`ptpttlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpttlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#Ethernet_PTP:PTPTTLR)*/
pub struct PTPTTLRrs;
impl crate::RegisterSpec for PTPTTLRrs {
    type Ux = u32;
}
///`read()` method returns [`ptpttlr::R`](R) reader structure
impl crate::Readable for PTPTTLRrs {}
///`write(|w| ..)` method takes [`ptpttlr::W`](W) writer structure
impl crate::Writable for PTPTTLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTTLR to value 0
impl crate::Resettable for PTPTTLRrs {}
