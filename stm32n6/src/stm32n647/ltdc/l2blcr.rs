///Register `L2BLCR` reader
pub type R = crate::R<L2BLCRrs>;
///Register `L2BLCR` writer
pub type W = crate::W<L2BLCRrs>;
///Field `BL` reader - burst length
pub type BL_R = crate::FieldReader;
///Field `BL` writer - burst length
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - burst length
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2BLCR").field("bl", &self.bl()).finish()
    }
}
impl W {
    ///Bits 0:7 - burst length
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, L2BLCRrs> {
        BL_W::new(self, 0)
    }
}
/**LTDC layerx burst length configuration register

You can [`read`](crate::Reg::read) this register and get [`l2blcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2blcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:L2BLCR)*/
pub struct L2BLCRrs;
impl crate::RegisterSpec for L2BLCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2blcr::R`](R) reader structure
impl crate::Readable for L2BLCRrs {}
///`write(|w| ..)` method takes [`l2blcr::W`](W) writer structure
impl crate::Writable for L2BLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2BLCR to value 0
impl crate::Resettable for L2BLCRrs {}
