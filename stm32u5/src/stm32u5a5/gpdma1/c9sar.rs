///Register `C9SAR` reader
pub type R = crate::R<C9SARrs>;
///Register `C9SAR` writer
pub type W = crate::W<C9SARrs>;
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
        f.debug_struct("C9SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<C9SARrs> {
        SA_W::new(self, 0)
    }
}
/**GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`c9sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GPDMA1:C9SAR)*/
pub struct C9SARrs;
impl crate::RegisterSpec for C9SARrs {
    type Ux = u32;
}
///`read()` method returns [`c9sar::R`](R) reader structure
impl crate::Readable for C9SARrs {}
///`write(|w| ..)` method takes [`c9sar::W`](W) writer structure
impl crate::Writable for C9SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C9SAR to value 0
impl crate::Resettable for C9SARrs {
    const RESET_VALUE: u32 = 0;
}
