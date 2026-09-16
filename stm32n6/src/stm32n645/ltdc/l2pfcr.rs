///Register `L2PFCR` reader
pub type R = crate::R<L2PFCRrs>;
///Register `L2PFCR` writer
pub type W = crate::W<L2PFCRrs>;
///Field `PF` reader - pixel format
pub type PF_R = crate::FieldReader;
///Field `PF` writer - pixel format
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - pixel format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2PFCR").field("pf", &self.pf()).finish()
    }
}
impl W {
    ///Bits 0:2 - pixel format
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<'_, L2PFCRrs> {
        PF_W::new(self, 0)
    }
}
/**LTDC layerx pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`l2pfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2pfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2PFCR)*/
pub struct L2PFCRrs;
impl crate::RegisterSpec for L2PFCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2pfcr::R`](R) reader structure
impl crate::Readable for L2PFCRrs {}
///`write(|w| ..)` method takes [`l2pfcr::W`](W) writer structure
impl crate::Writable for L2PFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2PFCR to value 0
impl crate::Resettable for L2PFCRrs {}
