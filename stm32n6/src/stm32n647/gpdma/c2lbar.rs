///Register `C2LBAR` reader
pub type R = crate::R<C2LBARrs>;
///Register `C2LBAR` writer
pub type W = crate::W<C2LBARrs>;
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
        f.debug_struct("C2LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of GPDMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<'_, C2LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**GPDMA channel 2 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c2lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPDMA:C2LBAR)*/
pub struct C2LBARrs;
impl crate::RegisterSpec for C2LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c2lbar::R`](R) reader structure
impl crate::Readable for C2LBARrs {}
///`write(|w| ..)` method takes [`c2lbar::W`](W) writer structure
impl crate::Writable for C2LBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2LBAR to value 0
impl crate::Resettable for C2LBARrs {}
