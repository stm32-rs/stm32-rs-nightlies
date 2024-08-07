///Register `CMP3CR` reader
pub type R = crate::R<CMP3CRrs>;
///Register `CMP3CR` writer
pub type W = crate::W<CMP3CRrs>;
///Field `CMP3x` reader - Timerx Compare 3 value
pub type CMP3X_R = crate::FieldReader<u16>;
///Field `CMP3x` writer - Timerx Compare 3 value
pub type CMP3X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP3CR")
            .field("cmp3x", &self.cmp3x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    #[must_use]
    pub fn cmp3x(&mut self) -> CMP3X_W<CMP3CRrs> {
        CMP3X_W::new(self, 0)
    }
}
/**Timerx Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`cmp3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_TIMC:CMP3CR)*/
pub struct CMP3CRrs;
impl crate::RegisterSpec for CMP3CRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp3cr::R`](R) reader structure
impl crate::Readable for CMP3CRrs {}
///`write(|w| ..)` method takes [`cmp3cr::W`](W) writer structure
impl crate::Writable for CMP3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP3CR to value 0
impl crate::Resettable for CMP3CRrs {
    const RESET_VALUE: u32 = 0;
}
