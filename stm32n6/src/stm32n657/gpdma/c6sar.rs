///Register `C6SAR` reader
pub type R = crate::R<C6SARrs>;
///Register `C6SAR` writer
pub type W = crate::W<C6SARrs>;
///Field `SA` reader - source address
pub type SA_R = crate::FieldReader<u32>;
///Field `SA` writer - source address
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C6SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, C6SARrs> {
        SA_W::new(self, 0)
    }
}
/**GPDMA channel 6 source address register

You can [`read`](crate::Reg::read) this register and get [`c6sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPDMA:C6SAR)*/
pub struct C6SARrs;
impl crate::RegisterSpec for C6SARrs {
    type Ux = u32;
}
///`read()` method returns [`c6sar::R`](R) reader structure
impl crate::Readable for C6SARrs {}
///`write(|w| ..)` method takes [`c6sar::W`](W) writer structure
impl crate::Writable for C6SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C6SAR to value 0
impl crate::Resettable for C6SARrs {}
