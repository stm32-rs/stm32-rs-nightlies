///Register `DMAC0TXDLAR` reader
pub type R = crate::R<DMAC0TXDLARrs>;
///Register `DMAC0TXDLAR` writer
pub type W = crate::W<DMAC0TXDLARrs>;
///Field `TDESLA` reader - Start of Transmit List
pub type TDESLA_R = crate::FieldReader<u32>;
///Field `TDESLA` writer - Start of Transmit List
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0TXDLAR")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W<'_, DMAC0TXDLARrs> {
        TDESLA_W::new(self, 0)
    }
}
/**Channel 0 T0 descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0txdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAC0TXDLAR)*/
pub struct DMAC0TXDLARrs;
impl crate::RegisterSpec for DMAC0TXDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0txdlar::R`](R) reader structure
impl crate::Readable for DMAC0TXDLARrs {}
///`write(|w| ..)` method takes [`dmac0txdlar::W`](W) writer structure
impl crate::Writable for DMAC0TXDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0TXDLAR to value 0
impl crate::Resettable for DMAC0TXDLARrs {}
