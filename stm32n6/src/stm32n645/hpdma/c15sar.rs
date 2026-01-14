///Register `C15SAR` reader
pub type R = crate::R<C15SARrs>;
///Register `C15SAR` writer
pub type W = crate::W<C15SARrs>;
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
        f.debug_struct("C15SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, C15SARrs> {
        SA_W::new(self, 0)
    }
}
/**HPDMA channel 15 source address register

You can [`read`](crate::Reg::read) this register and get [`c15sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HPDMA:C15SAR)*/
pub struct C15SARrs;
impl crate::RegisterSpec for C15SARrs {
    type Ux = u32;
}
///`read()` method returns [`c15sar::R`](R) reader structure
impl crate::Readable for C15SARrs {}
///`write(|w| ..)` method takes [`c15sar::W`](W) writer structure
impl crate::Writable for C15SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C15SAR to value 0
impl crate::Resettable for C15SARrs {}
