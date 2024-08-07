///Register `CMP4FR` reader
pub type R = crate::R<CMP4FRrs>;
///Register `CMP4FR` writer
pub type W = crate::W<CMP4FRrs>;
///Field `CMP4x` reader - Timerx Compare 4 value
pub type CMP4X_R = crate::FieldReader<u16>;
///Field `CMP4x` writer - Timerx Compare 4 value
pub type CMP4X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Compare 4 value
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP4FR")
            .field("cmp4x", &self.cmp4x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 4 value
    #[inline(always)]
    #[must_use]
    pub fn cmp4x(&mut self) -> CMP4X_W<CMP4FRrs> {
        CMP4X_W::new(self, 0)
    }
}
/**Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_TIMF:CMP4FR)*/
pub struct CMP4FRrs;
impl crate::RegisterSpec for CMP4FRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp4fr::R`](R) reader structure
impl crate::Readable for CMP4FRrs {}
///`write(|w| ..)` method takes [`cmp4fr::W`](W) writer structure
impl crate::Writable for CMP4FRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP4FR to value 0
impl crate::Resettable for CMP4FRrs {
    const RESET_VALUE: u32 = 0;
}
