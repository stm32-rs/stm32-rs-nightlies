///Register `AFCC1R` reader
pub type R = crate::R<AFCC1Rrs>;
///Register `AFCC1R` writer
pub type W = crate::W<AFCC1Rrs>;
///Field `FRAME` reader - frame number
pub type FRAME_R = crate::FieldReader<u32>;
///Field `FRAME` writer - frame number
pub type FRAME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - frame number
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFCC1R")
            .field("frame", &self.frame())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - frame number
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<'_, AFCC1Rrs> {
        FRAME_W::new(self, 0)
    }
}
/**GFXTIM absolute frame counter compare 1 register

You can [`read`](crate::Reg::read) this register and get [`afcc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afcc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GFXTIM:AFCC1R)*/
pub struct AFCC1Rrs;
impl crate::RegisterSpec for AFCC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`afcc1r::R`](R) reader structure
impl crate::Readable for AFCC1Rrs {}
///`write(|w| ..)` method takes [`afcc1r::W`](W) writer structure
impl crate::Writable for AFCC1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFCC1R to value 0
impl crate::Resettable for AFCC1Rrs {}
