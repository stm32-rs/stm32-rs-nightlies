///Register `L1PFCR` reader
pub type R = crate::R<L1PFCRrs>;
///Register `L1PFCR` writer
pub type W = crate::W<L1PFCRrs>;
///Field `PF` reader - PF
pub type PF_R = crate::FieldReader;
///Field `PF` writer - PF
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - PF
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1PFCR").field("pf", &self.pf()).finish()
    }
}
impl W {
    ///Bits 0:2 - PF
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<'_, L1PFCRrs> {
        PF_W::new(self, 0)
    }
}
/**This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).

You can [`read`](crate::Reg::read) this register and get [`l1pfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1PFCR)*/
pub struct L1PFCRrs;
impl crate::RegisterSpec for L1PFCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1pfcr::R`](R) reader structure
impl crate::Readable for L1PFCRrs {}
///`write(|w| ..)` method takes [`l1pfcr::W`](W) writer structure
impl crate::Writable for L1PFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1PFCR to value 0
impl crate::Resettable for L1PFCRrs {}
