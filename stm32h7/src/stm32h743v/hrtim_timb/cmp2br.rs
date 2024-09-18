///Register `CMP2BR` reader
pub type R = crate::R<CMP2BRrs>;
///Register `CMP2BR` writer
pub type W = crate::W<CMP2BRrs>;
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
        f.debug_struct("CMP2BR")
            .field("cmp2x", &self.cmp2x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 2 value
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> CMP2X_W<CMP2BRrs> {
        CMP2X_W::new(self, 0)
    }
}
/**Timerx Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`cmp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMB:CMP2BR)*/
pub struct CMP2BRrs;
impl crate::RegisterSpec for CMP2BRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp2br::R`](R) reader structure
impl crate::Readable for CMP2BRrs {}
///`write(|w| ..)` method takes [`cmp2br::W`](W) writer structure
impl crate::Writable for CMP2BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP2BR to value 0
impl crate::Resettable for CMP2BRrs {
    const RESET_VALUE: u32 = 0;
}
