///Register `C1LBAR` reader
pub type R = crate::R<C1LBARrs>;
///Register `C1LBAR` writer
pub type W = crate::W<C1LBARrs>;
///Field `LBA` reader - linked-list base address of DMA channel x
pub type LBA_R = crate::FieldReader<u16>;
///Field `LBA` writer - linked-list base address of DMA channel x
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - linked-list base address of DMA channel x
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of DMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<C1LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c1lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GPDMA1:C1LBAR)*/
pub struct C1LBARrs;
impl crate::RegisterSpec for C1LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c1lbar::R`](R) reader structure
impl crate::Readable for C1LBARrs {}
///`write(|w| ..)` method takes [`c1lbar::W`](W) writer structure
impl crate::Writable for C1LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1LBAR to value 0
impl crate::Resettable for C1LBARrs {
    const RESET_VALUE: u32 = 0;
}
