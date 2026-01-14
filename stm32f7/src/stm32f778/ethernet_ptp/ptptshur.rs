///Register `PTPTSHUR` reader
pub type R = crate::R<PTPTSHURrs>;
///Register `PTPTSHUR` writer
pub type W = crate::W<PTPTSHURrs>;
///Field `TSUS` reader - TSUS
pub type TSUS_R = crate::FieldReader<u32>;
///Field `TSUS` writer - TSUS
pub type TSUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSUS
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSHUR")
            .field("tsus", &self.tsus())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSUS
    #[inline(always)]
    pub fn tsus(&mut self) -> TSUS_W<'_, PTPTSHURrs> {
        TSUS_W::new(self, 0)
    }
}
/**Ethernet PTP time stamp high update register

You can [`read`](crate::Reg::read) this register and get [`ptptshur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#Ethernet_PTP:PTPTSHUR)*/
pub struct PTPTSHURrs;
impl crate::RegisterSpec for PTPTSHURrs {
    type Ux = u32;
}
///`read()` method returns [`ptptshur::R`](R) reader structure
impl crate::Readable for PTPTSHURrs {}
///`write(|w| ..)` method takes [`ptptshur::W`](W) writer structure
impl crate::Writable for PTPTSHURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTSHUR to value 0
impl crate::Resettable for PTPTSHURrs {}
