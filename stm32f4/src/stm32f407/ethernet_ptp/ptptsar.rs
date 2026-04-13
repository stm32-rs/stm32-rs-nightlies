///Register `PTPTSAR` reader
pub type R = crate::R<PTPTSARrs>;
///Register `PTPTSAR` writer
pub type W = crate::W<PTPTSARrs>;
///Field `TSA` reader - TSA
pub type TSA_R = crate::FieldReader<u32>;
///Field `TSA` writer - TSA
pub type TSA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSA
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSAR").field("tsa", &self.tsa()).finish()
    }
}
impl W {
    ///Bits 0:31 - TSA
    #[inline(always)]
    pub fn tsa(&mut self) -> TSA_W<'_, PTPTSARrs> {
        TSA_W::new(self, 0)
    }
}
/**Ethernet PTP time stamp addend register

You can [`read`](crate::Reg::read) this register and get [`ptptsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_PTP:PTPTSAR)*/
pub struct PTPTSARrs;
impl crate::RegisterSpec for PTPTSARrs {
    type Ux = u32;
}
///`read()` method returns [`ptptsar::R`](R) reader structure
impl crate::Readable for PTPTSARrs {}
///`write(|w| ..)` method takes [`ptptsar::W`](W) writer structure
impl crate::Writable for PTPTSARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTSAR to value 0
impl crate::Resettable for PTPTSARrs {}
