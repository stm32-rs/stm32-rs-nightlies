///Register `CMP3AR` reader
pub type R = crate::R<CMP3ARrs>;
///Register `CMP3AR` writer
pub type W = crate::W<CMP3ARrs>;
///Field `CMP3x` reader - Timerx Compare 3 value
pub type CMP3X_R = crate::FieldReader<u16>;
///Field `CMP3x` writer - Timerx Compare 3 value
pub type CMP3X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP3AR")
            .field("cmp3x", &self.cmp3x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    #[must_use]
    pub fn cmp3x(&mut self) -> CMP3X_W<CMP3ARrs> {
        CMP3X_W::new(self, 0)
    }
}
/**Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMA:CMP3AR)*/
pub struct CMP3ARrs;
impl crate::RegisterSpec for CMP3ARrs {
    type Ux = u32;
}
///`read()` method returns [`cmp3ar::R`](R) reader structure
impl crate::Readable for CMP3ARrs {}
///`write(|w| ..)` method takes [`cmp3ar::W`](W) writer structure
impl crate::Writable for CMP3ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP3AR to value 0
impl crate::Resettable for CMP3ARrs {
    const RESET_VALUE: u32 = 0;
}
