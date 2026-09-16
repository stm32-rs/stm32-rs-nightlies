///Register `DMACCATxBR` reader
pub type R = crate::R<DMACCATX_BRrs>;
///Register `DMACCATxBR` writer
pub type W = crate::W<DMACCATX_BRrs>;
///Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
///Field `CURTBUFAPTR` writer - Application Transmit Buffer Address Pointer
pub type CURTBUFAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCATxBR")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W<'_, DMACCATX_BRrs> {
        CURTBUFAPTR_W::new(self, 0)
    }
}
/**Channel current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccatx_br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccatx_br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_DMA:DMACCATxBR)*/
pub struct DMACCATX_BRrs;
impl crate::RegisterSpec for DMACCATX_BRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccatx_br::R`](R) reader structure
impl crate::Readable for DMACCATX_BRrs {}
///`write(|w| ..)` method takes [`dmaccatx_br::W`](W) writer structure
impl crate::Writable for DMACCATX_BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACCATxBR to value 0
impl crate::Resettable for DMACCATX_BRrs {}
