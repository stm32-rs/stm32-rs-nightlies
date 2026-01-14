///Register `DMACCARxDR` reader
pub type R = crate::R<DMACCARX_DRrs>;
///Register `DMACCARxDR` writer
pub type W = crate::W<DMACCARX_DRrs>;
///Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
///Field `CURRDESAPTR` writer - Application Receive Descriptor Address Pointer
pub type CURRDESAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Application Receive Descriptor Address Pointer
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCARxDR")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Application Receive Descriptor Address Pointer
    #[inline(always)]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W<'_, DMACCARX_DRrs> {
        CURRDESAPTR_W::new(self, 0)
    }
}
/**Channel current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccarx_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#Ethernet_DMA:DMACCARxDR)*/
pub struct DMACCARX_DRrs;
impl crate::RegisterSpec for DMACCARX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarx_dr::R`](R) reader structure
impl crate::Readable for DMACCARX_DRrs {}
///`write(|w| ..)` method takes [`dmaccarx_dr::W`](W) writer structure
impl crate::Writable for DMACCARX_DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACCARxDR to value 0
impl crate::Resettable for DMACCARX_DRrs {}
