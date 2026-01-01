///Register `C3LBAR` reader
pub type R = crate::R<C3LBARrs>;
///Register `C3LBAR` writer
pub type W = crate::W<C3LBARrs>;
///Field `LBA` reader - linked-list base address of HPDMA channel x
pub type LBA_R = crate::FieldReader<u16>;
///Field `LBA` writer - linked-list base address of HPDMA channel x
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - linked-list base address of HPDMA channel x
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of HPDMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<'_, C3LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**HPDMA channel 3 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c3lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HPDMA:C3LBAR)*/
pub struct C3LBARrs;
impl crate::RegisterSpec for C3LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c3lbar::R`](R) reader structure
impl crate::Readable for C3LBARrs {}
///`write(|w| ..)` method takes [`c3lbar::W`](W) writer structure
impl crate::Writable for C3LBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3LBAR to value 0
impl crate::Resettable for C3LBARrs {}
