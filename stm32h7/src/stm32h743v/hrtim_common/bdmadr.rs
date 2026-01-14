///Register `BDMADR` reader
pub type R = crate::R<BDMADRrs>;
///Register `BDMADR` writer
pub type W = crate::W<BDMADRrs>;
///Field `BDMADR` reader - Burst DMA Data register
pub type BDMADR_R = crate::FieldReader<u32>;
///Field `BDMADR` writer - Burst DMA Data register
pub type BDMADR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    pub fn bdmadr(&self) -> BDMADR_R {
        BDMADR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMADR")
            .field("bdmadr", &self.bdmadr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BDMADR_W<'_, BDMADRrs> {
        BDMADR_W::new(self, 0)
    }
}
/**Burst DMA Data Register

You can [`read`](crate::Reg::read) this register and get [`bdmadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Common:BDMADR)*/
pub struct BDMADRrs;
impl crate::RegisterSpec for BDMADRrs {
    type Ux = u32;
}
///`read()` method returns [`bdmadr::R`](R) reader structure
impl crate::Readable for BDMADRrs {}
///`write(|w| ..)` method takes [`bdmadr::W`](W) writer structure
impl crate::Writable for BDMADRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets BDMADR to value 0
impl crate::Resettable for BDMADRrs {}
