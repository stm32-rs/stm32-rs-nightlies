///Register `DMACCARxBR` reader
pub type R = crate::R<DMACCARX_BRrs>;
///Register `DMACCARxBR` writer
pub type W = crate::W<DMACCARX_BRrs>;
///Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
///Field `CURRBUFAPTR` writer - Application Receive Buffer Address Pointer
pub type CURRBUFAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Application Receive Buffer Address Pointer
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCARxBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Application Receive Buffer Address Pointer
    #[inline(always)]
    pub fn currbufaptr(&mut self) -> CURRBUFAPTR_W<'_, DMACCARX_BRrs> {
        CURRBUFAPTR_W::new(self, 0)
    }
}
/**Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccarx_br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_DMA:DMACCARxBR)*/
pub struct DMACCARX_BRrs;
impl crate::RegisterSpec for DMACCARX_BRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarx_br::R`](R) reader structure
impl crate::Readable for DMACCARX_BRrs {}
///`write(|w| ..)` method takes [`dmaccarx_br::W`](W) writer structure
impl crate::Writable for DMACCARX_BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACCARxBR to value 0
impl crate::Resettable for DMACCARX_BRrs {}
