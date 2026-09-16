///Register `L1BLCR` reader
pub type R = crate::R<L1BLCRrs>;
///Register `L1BLCR` writer
pub type W = crate::W<L1BLCRrs>;
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
        f.debug_struct("L1BLCR").field("bl", &self.bl()).finish()
    }
}
impl W {
    ///Bits 0:7 - burst length
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, L1BLCRrs> {
        BL_W::new(self, 0)
    }
}
/**LTDC layerx burst length configuration register

You can [`read`](crate::Reg::read) this register and get [`l1blcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1blcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L1BLCR)*/
pub struct L1BLCRrs;
impl crate::RegisterSpec for L1BLCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1blcr::R`](R) reader structure
impl crate::Readable for L1BLCRrs {}
///`write(|w| ..)` method takes [`l1blcr::W`](W) writer structure
impl crate::Writable for L1BLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1BLCR to value 0
impl crate::Resettable for L1BLCRrs {}
