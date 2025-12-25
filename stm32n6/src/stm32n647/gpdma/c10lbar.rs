///Register `C10LBAR` reader
pub type R = crate::R<C10LBARrs>;
///Register `C10LBAR` writer
pub type W = crate::W<C10LBARrs>;
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
        f.debug_struct("C10LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<'_, C10LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**GPDMA channel 10 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c10lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPDMA:C10LBAR)*/
pub struct C10LBARrs;
impl crate::RegisterSpec for C10LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c10lbar::R`](R) reader structure
impl crate::Readable for C10LBARrs {}
///`write(|w| ..)` method takes [`c10lbar::W`](W) writer structure
impl crate::Writable for C10LBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C10LBAR to value 0
impl crate::Resettable for C10LBARrs {}
