///Register `CMP1CR` reader
pub type R = crate::R<CMP1CRrs>;
///Register `CMP1CR` writer
pub type W = crate::W<CMP1CRrs>;
///Field `CMP1` reader - Timerx Compare 1 value
pub type CMP1_R = crate::FieldReader<u16>;
///Field `CMP1` writer - Timerx Compare 1 value
pub type CMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `REP` reader - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REP_R = crate::FieldReader;
///Field `REP` writer - Timerx Repetition value (aliased from HRTIM_REPx register)
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1CR")
            .field("rep", &self.rep())
            .field("cmp1", &self.cmp1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W<'_, CMP1CRrs> {
        CMP1_W::new(self, 0)
    }
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<'_, CMP1CRrs> {
        REP_W::new(self, 16)
    }
}
/**Timerx Compare 1 Compound Register

You can [`read`](crate::Reg::read) this register and get [`cmp1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#HRTIM_TIMA:CMP1CR)*/
pub struct CMP1CRrs;
impl crate::RegisterSpec for CMP1CRrs {
    type Ux = u32;
}
///`read()` method returns [`cmp1cr::R`](R) reader structure
impl crate::Readable for CMP1CRrs {}
///`write(|w| ..)` method takes [`cmp1cr::W`](W) writer structure
impl crate::Writable for CMP1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMP1CR to value 0
impl crate::Resettable for CMP1CRrs {}
