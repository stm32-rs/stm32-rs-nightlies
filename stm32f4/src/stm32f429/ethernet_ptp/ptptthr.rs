///Register `PTPTTHR` reader
pub type R = crate::R<PTPTTHRrs>;
///Register `PTPTTHR` writer
pub type W = crate::W<PTPTTHRrs>;
///Field `TTSH` reader - 0
pub type TTSH_R = crate::FieldReader<u32>;
///Field `TTSH` writer - 0
pub type TTSH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 0
    #[inline(always)]
    pub fn ttsh(&self) -> TTSH_R {
        TTSH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTTHR")
            .field("ttsh", &self.ttsh())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 0
    #[inline(always)]
    pub fn ttsh(&mut self) -> TTSH_W<'_, PTPTTHRrs> {
        TTSH_W::new(self, 0)
    }
}
/**Ethernet PTP target time high register

You can [`read`](crate::Reg::read) this register and get [`ptptthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#Ethernet_PTP:PTPTTHR)*/
pub struct PTPTTHRrs;
impl crate::RegisterSpec for PTPTTHRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptthr::R`](R) reader structure
impl crate::Readable for PTPTTHRrs {}
///`write(|w| ..)` method takes [`ptptthr::W`](W) writer structure
impl crate::Writable for PTPTTHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTTHR to value 0
impl crate::Resettable for PTPTTHRrs {}
