///Register `GPDMA_C6LBAR` reader
pub type R = crate::R<GPDMA_C6LBARrs>;
///Register `GPDMA_C6LBAR` writer
pub type W = crate::W<GPDMA_C6LBARrs>;
///Field `LBA` reader - linked-list base address of GPDMA channel x
pub type LBA_R = crate::FieldReader<u16>;
///Field `LBA` writer - linked-list base address of GPDMA channel x
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPDMA_C6LBAR")
            .field("lba", &self.lba())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    #[must_use]
    pub fn lba(&mut self) -> LBA_W<GPDMA_C6LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**GPDMA channel 6 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c6lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c6lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GPDMA1:GPDMA_C6LBAR)*/
pub struct GPDMA_C6LBARrs;
impl crate::RegisterSpec for GPDMA_C6LBARrs {
    type Ux = u32;
}
///`read()` method returns [`gpdma_c6lbar::R`](R) reader structure
impl crate::Readable for GPDMA_C6LBARrs {}
///`write(|w| ..)` method takes [`gpdma_c6lbar::W`](W) writer structure
impl crate::Writable for GPDMA_C6LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPDMA_C6LBAR to value 0
impl crate::Resettable for GPDMA_C6LBARrs {
    const RESET_VALUE: u32 = 0;
}
