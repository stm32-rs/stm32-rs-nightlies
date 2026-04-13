///Register `SWREG58` reader
pub type R = crate::R<SWREG58rs>;
///Register `SWREG58` writer
pub type W = crate::W<SWREG58rs>;
///Field `SWREG_FIELD` reader - intra slice bitmap for slices 0..31 / Base address for 1st DCT partition (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - intra slice bitmap for slices 0..31 / Base address for 1st DCT partition (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - intra slice bitmap for slices 0..31 / Base address for 1st DCT partition (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG58")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - intra slice bitmap for slices 0..31 / Base address for 1st DCT partition (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG58rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC intra slice bitmap for slices 0..31/base address for 1st DCT partition register

You can [`read`](crate::Reg::read) this register and get [`swreg58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG58)*/
pub struct SWREG58rs;
impl crate::RegisterSpec for SWREG58rs {
    type Ux = u32;
}
///`read()` method returns [`swreg58::R`](R) reader structure
impl crate::Readable for SWREG58rs {}
///`write(|w| ..)` method takes [`swreg58::W`](W) writer structure
impl crate::Writable for SWREG58rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG58 to value 0
impl crate::Resettable for SWREG58rs {}
