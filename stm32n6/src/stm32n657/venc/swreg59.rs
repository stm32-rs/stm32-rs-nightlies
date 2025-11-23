///Register `SWREG59` reader
pub type R = crate::R<SWREG59rs>;
///Register `SWREG59` writer
pub type W = crate::W<SWREG59rs>;
///Field `SWREG_FIELD` reader - intra slice bitmap for slices 32..63 / Base address for 2nd DCT partition (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - intra slice bitmap for slices 32..63 / Base address for 2nd DCT partition (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - intra slice bitmap for slices 32..63 / Base address for 2nd DCT partition (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG59")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - intra slice bitmap for slices 32..63 / Base address for 2nd DCT partition (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG59rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC intra slice bitmap for slices 32..63/base address for 2nd DCT partition register

You can [`read`](crate::Reg::read) this register and get [`swreg59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG59)*/
pub struct SWREG59rs;
impl crate::RegisterSpec for SWREG59rs {
    type Ux = u32;
}
///`read()` method returns [`swreg59::R`](R) reader structure
impl crate::Readable for SWREG59rs {}
///`write(|w| ..)` method takes [`swreg59::W`](W) writer structure
impl crate::Writable for SWREG59rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG59 to value 0
impl crate::Resettable for SWREG59rs {}
