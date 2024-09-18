///Register `CMP1BR` reader
pub type R = crate::R<CMP1BRrs>;
///Register `CMP1BR` writer
pub type W = crate::W<CMP1BRrs>;
///Field `CMP1x` reader - Timerx Compare 1 value
pub type CMP1X_R = crate::FieldReader<u16>;
///Field `CMP1x` writer - Timerx Compare 1 value
pub type CMP1X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1BR")
            .field("cmp1x", &self.cmp1x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<CMP1BRrs> {
        CMP1X_W::new(self, 0)
    }
}
/**Timerx Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`cmp1br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMB:CMP1BR)*/
pub struct CMP1BRrs;
impl crate::RegisterSpec for CMP1BRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp1br::R`](R) reader structure
impl crate::Readable for CMP1BRrs {}
///`write(|w| ..)` method takes [`cmp1br::W`](W) writer structure
impl crate::Writable for CMP1BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP1BR to value 0
impl crate::Resettable for CMP1BRrs {
    const RESET_VALUE: u32 = 0;
}
