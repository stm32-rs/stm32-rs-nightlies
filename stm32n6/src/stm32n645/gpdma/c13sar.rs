///Register `C13SAR` reader
pub type R = crate::R<C13SARrs>;
///Register `C13SAR` writer
pub type W = crate::W<C13SARrs>;
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
        f.debug_struct("C13SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<C13SARrs> {
        SA_W::new(self, 0)
    }
}
/**GPDMA channel 13 source address register

You can [`read`](crate::Reg::read) this register and get [`c13sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPDMA:C13SAR)*/
pub struct C13SARrs;
impl crate::RegisterSpec for C13SARrs {
    type Ux = u32;
}
///`read()` method returns [`c13sar::R`](R) reader structure
impl crate::Readable for C13SARrs {}
///`write(|w| ..)` method takes [`c13sar::W`](W) writer structure
impl crate::Writable for C13SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C13SAR to value 0
impl crate::Resettable for C13SARrs {}
