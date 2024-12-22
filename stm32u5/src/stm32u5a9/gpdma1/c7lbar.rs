///Register `C7LBAR` reader
pub type R = crate::R<C7LBARrs>;
///Register `C7LBAR` writer
pub type W = crate::W<C7LBARrs>;
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
        f.debug_struct("C7LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of DMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<C7LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**channel x linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c7lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPDMA1:C7LBAR)*/
pub struct C7LBARrs;
impl crate::RegisterSpec for C7LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c7lbar::R`](R) reader structure
impl crate::Readable for C7LBARrs {}
///`write(|w| ..)` method takes [`c7lbar::W`](W) writer structure
impl crate::Writable for C7LBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C7LBAR to value 0
impl crate::Resettable for C7LBARrs {
    const RESET_VALUE: u32 = 0;
}
