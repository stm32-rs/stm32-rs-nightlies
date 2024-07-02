///Register `CMP1CER` reader
pub type R = crate::R<CMP1CERrs>;
///Register `CMP1CER` writer
pub type W = crate::W<CMP1CERrs>;
///Field `CMP1x` reader - Timerx Compare 1 value
pub type CMP1X_R = crate::FieldReader<u16>;
///Field `CMP1x` writer - Timerx Compare 1 value
pub type CMP1X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `REPx` reader - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REPX_R = crate::FieldReader;
///Field `REPx` writer - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REPX_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1CER")
            .field("repx", &self.repx())
            .field("cmp1x", &self.cmp1x())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> CMP1X_W<CMP1CERrs> {
        CMP1X_W::new(self, 0)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<CMP1CERrs> {
        REPX_W::new(self, 16)
    }
}
/**Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIME:CMP1CER)*/
pub struct CMP1CERrs;
impl crate::RegisterSpec for CMP1CERrs {
    type Ux = u32;
}
///`read()` method returns [`cmp1cer::R`](R) reader structure
impl crate::Readable for CMP1CERrs {}
///`write(|w| ..)` method takes [`cmp1cer::W`](W) writer structure
impl crate::Writable for CMP1CERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP1CER to value 0
impl crate::Resettable for CMP1CERrs {
    const RESET_VALUE: u32 = 0;
}
