///Register `AFCR` reader
pub type R = crate::R<AFCRrs>;
///Register `AFCR` writer
pub type W = crate::W<AFCRrs>;
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
        f.debug_struct("AFCR")
            .field("frame", &self.frame())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - frame number
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<'_, AFCRrs> {
        FRAME_W::new(self, 0)
    }
}
/**GFXTIM absolute frame counter register

You can [`read`](crate::Reg::read) this register and get [`afcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:AFCR)*/
pub struct AFCRrs;
impl crate::RegisterSpec for AFCRrs {
    type Ux = u32;
}
///`read()` method returns [`afcr::R`](R) reader structure
impl crate::Readable for AFCRrs {}
///`write(|w| ..)` method takes [`afcr::W`](W) writer structure
impl crate::Writable for AFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFCR to value 0
impl crate::Resettable for AFCRrs {}
