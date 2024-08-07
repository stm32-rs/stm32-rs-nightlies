///Register `CMP4CR` reader
pub type R = crate::R<CMP4CRrs>;
///Register `CMP4CR` writer
pub type W = crate::W<CMP4CRrs>;
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
        f.debug_struct("CMP4CR")
            .field("cmp4x", &self.cmp4x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 4 value
    #[inline(always)]
    #[must_use]
    pub fn cmp4x(&mut self) -> CMP4X_W<CMP4CRrs> {
        CMP4X_W::new(self, 0)
    }
}
/**Timerx Compare 4 Register

You can [`read`](crate::Reg::read) this register and get [`cmp4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_TIMC:CMP4CR)*/
pub struct CMP4CRrs;
impl crate::RegisterSpec for CMP4CRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp4cr::R`](R) reader structure
impl crate::Readable for CMP4CRrs {}
///`write(|w| ..)` method takes [`cmp4cr::W`](W) writer structure
impl crate::Writable for CMP4CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP4CR to value 0
impl crate::Resettable for CMP4CRrs {
    const RESET_VALUE: u32 = 0;
}
