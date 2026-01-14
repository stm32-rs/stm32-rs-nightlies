///Register `C14LBAR` reader
pub type R = crate::R<C14LBARrs>;
///Register `C14LBAR` writer
pub type W = crate::W<C14LBARrs>;
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
        f.debug_struct("C14LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of HPDMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<'_, C14LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**HPDMA channel 14 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`c14lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HPDMA:C14LBAR)*/
pub struct C14LBARrs;
impl crate::RegisterSpec for C14LBARrs {
    type Ux = u32;
}
///`read()` method returns [`c14lbar::R`](R) reader structure
impl crate::Readable for C14LBARrs {}
///`write(|w| ..)` method takes [`c14lbar::W`](W) writer structure
impl crate::Writable for C14LBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C14LBAR to value 0
impl crate::Resettable for C14LBARrs {}
