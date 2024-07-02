///Register `GPDMA_C11DAR` reader
pub type R = crate::R<GPDMA_C11DARrs>;
///Register `GPDMA_C11DAR` writer
pub type W = crate::W<GPDMA_C11DARrs>;
///Field `DA` reader - destination address
pub type DA_R = crate::FieldReader<u32>;
///Field `DA` writer - destination address
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPDMA_C11DAR")
            .field("da", &self.da())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - destination address
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<GPDMA_C11DARrs> {
        DA_W::new(self, 0)
    }
}
/**GPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c11dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c11dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPDMA1:GPDMA_C11DAR)*/
pub struct GPDMA_C11DARrs;
impl crate::RegisterSpec for GPDMA_C11DARrs {
    type Ux = u32;
}
///`read()` method returns [`gpdma_c11dar::R`](R) reader structure
impl crate::Readable for GPDMA_C11DARrs {}
///`write(|w| ..)` method takes [`gpdma_c11dar::W`](W) writer structure
impl crate::Writable for GPDMA_C11DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPDMA_C11DAR to value 0
impl crate::Resettable for GPDMA_C11DARrs {
    const RESET_VALUE: u32 = 0;
}
