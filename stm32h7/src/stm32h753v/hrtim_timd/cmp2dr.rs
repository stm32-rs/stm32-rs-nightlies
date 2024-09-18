///Register `CMP2DR` reader
pub type R = crate::R<CMP2DRrs>;
///Register `CMP2DR` writer
pub type W = crate::W<CMP2DRrs>;
///Field `CMP2x` reader - Timerx Compare 2 value
pub type CMP2X_R = crate::FieldReader<u16>;
///Field `CMP2x` writer - Timerx Compare 2 value
pub type CMP2X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Compare 2 value
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP2DR")
            .field("cmp2x", &self.cmp2x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 2 value
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> CMP2X_W<CMP2DRrs> {
        CMP2X_W::new(self, 0)
    }
}
/**Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_TIMD:CMP2DR)*/
pub struct CMP2DRrs;
impl crate::RegisterSpec for CMP2DRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp2dr::R`](R) reader structure
impl crate::Readable for CMP2DRrs {}
///`write(|w| ..)` method takes [`cmp2dr::W`](W) writer structure
impl crate::Writable for CMP2DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP2DR to value 0
impl crate::Resettable for CMP2DRrs {
    const RESET_VALUE: u32 = 0;
}
