///Register `OPTKEYR_` reader
pub type R = crate::R<OPTKEYR_rs>;
///Register `OPTKEYR_` writer
pub type W = crate::W<OPTKEYR_rs>;
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
        f.debug_struct("OPTKEYR_")
            .field("optkeyr", &self.optkeyr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Unlock key option bytes
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<'_, OPTKEYR_rs> {
        OPTKEYR_W::new(self, 0)
    }
}
/**FLASH option key register

You can [`read`](crate::Reg::read) this register and get [`optkeyr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#FLASH:OPTKEYR_)*/
pub struct OPTKEYR_rs;
impl crate::RegisterSpec for OPTKEYR_rs {
    type Ux = u32;
}
///`read()` method returns [`optkeyr_::R`](R) reader structure
impl crate::Readable for OPTKEYR_rs {}
///`write(|w| ..)` method takes [`optkeyr_::W`](W) writer structure
impl crate::Writable for OPTKEYR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTKEYR_ to value 0
impl crate::Resettable for OPTKEYR_rs {}
