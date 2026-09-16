///Register `DMAR` reader
pub type R = crate::R<DMARrs>;
///Register `DMAR` writer
pub type W = crate::W<DMARrs>;
///Field `DMAR` reader - DMA register for burst accesses
pub type DMAR_R = crate::FieldReader<u16>;
///Field `DMAR` writer - DMA register for burst accesses
pub type DMAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAR").field("dmar", &self.dmar()).finish()
    }
}
impl W {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, DMARrs> {
        DMAR_W::new(self, 0)
    }
}
/**DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#TIM3:DMAR)*/
pub struct DMARrs;
impl crate::RegisterSpec for DMARrs {
    type Ux = u16;
}
///`read()` method returns [`dmar::R`](R) reader structure
impl crate::Readable for DMARrs {}
///`write(|w| ..)` method takes [`dmar::W`](W) writer structure
impl crate::Writable for DMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAR to value 0
impl crate::Resettable for DMARrs {}
