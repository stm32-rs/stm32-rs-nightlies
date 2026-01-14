///Register `DMAC0RXDLAR` reader
pub type R = crate::R<DMAC0RXDLARrs>;
///Register `DMAC0RXDLAR` writer
pub type W = crate::W<DMAC0RXDLARrs>;
///Field `RDESLA` reader - Start of Receive List
pub type RDESLA_R = crate::FieldReader<u32>;
///Field `RDESLA` writer - Start of Receive List
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Start of Receive List
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RXDLAR")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Start of Receive List
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W<'_, DMAC0RXDLARrs> {
        RDESLA_W::new(self, 0)
    }
}
/**Channel 0 Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMAC0RXDLAR)*/
pub struct DMAC0RXDLARrs;
impl crate::RegisterSpec for DMAC0RXDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rxdlar::R`](R) reader structure
impl crate::Readable for DMAC0RXDLARrs {}
///`write(|w| ..)` method takes [`dmac0rxdlar::W`](W) writer structure
impl crate::Writable for DMAC0RXDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RXDLAR to value 0
impl crate::Resettable for DMAC0RXDLARrs {}
