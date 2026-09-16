///Register `DMACCATxDR` reader
pub type R = crate::R<DMACCATX_DRrs>;
///Register `DMACCATxDR` writer
pub type W = crate::W<DMACCATX_DRrs>;
///Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
///Field `CURTDESAPTR` writer - Application Transmit Descriptor Address Pointer
pub type CURTDESAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCATxDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn curtdesaptr(&mut self) -> CURTDESAPTR_W<'_, DMACCATX_DRrs> {
        CURTDESAPTR_W::new(self, 0)
    }
}
/**Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccatx_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccatx_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_DMA:DMACCATxDR)*/
pub struct DMACCATX_DRrs;
impl crate::RegisterSpec for DMACCATX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccatx_dr::R`](R) reader structure
impl crate::Readable for DMACCATX_DRrs {}
///`write(|w| ..)` method takes [`dmaccatx_dr::W`](W) writer structure
impl crate::Writable for DMACCATX_DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACCATxDR to value 0
impl crate::Resettable for DMACCATX_DRrs {}
