///Register `OPTKEYR` reader
pub type R = crate::R<OPTKEYRrs>;
///Register `OPTKEYR` writer
pub type W = crate::W<OPTKEYRrs>;
///Field `OPTKEYR` reader - Unlock key option bytes
pub type OPTKEYR_R = crate::FieldReader<u32>;
///Field `OPTKEYR` writer - Unlock key option bytes
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Unlock key option bytes
    #[inline(always)]
    pub fn optkeyr(&self) -> OPTKEYR_R {
        OPTKEYR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTKEYR")
            .field("optkeyr", &self.optkeyr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Unlock key option bytes
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<OPTKEYRrs> {
        OPTKEYR_W::new(self, 0)
    }
}
/**FLASH option key register

You can [`read`](crate::Reg::read) this register and get [`optkeyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#FLASH:OPTKEYR)*/
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
///`read()` method returns [`optkeyr::R`](R) reader structure
impl crate::Readable for OPTKEYRrs {}
///`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYRrs {
    const RESET_VALUE: u32 = 0;
}
