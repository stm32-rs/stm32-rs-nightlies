///Register `ACTION` reader
pub type R = crate::R<ACTIONrs>;
///Register `ACTION` writer
pub type W = crate::W<ACTIONrs>;
///Field `REACTION_VALUE` reader - REACTION_VALUE
pub type REACTION_VALUE_R = crate::FieldReader;
///Field `REACTION_VALUE` writer - REACTION_VALUE
pub type REACTION_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - REACTION_VALUE
    #[inline(always)]
    pub fn reaction_value(&self) -> REACTION_VALUE_R {
        REACTION_VALUE_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTION")
            .field("reaction_value", &self.reaction_value())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - REACTION_VALUE
    #[inline(always)]
    pub fn reaction_value(&mut self) -> REACTION_VALUE_W<'_, ACTIONrs> {
        REACTION_VALUE_W::new(self, 0)
    }
}
/**Controls interrupt and bus error response behavior when regions permission failures occur.

You can [`read`](crate::Reg::read) this register and get [`action::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`action::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:ACTION)*/
pub struct ACTIONrs;
impl crate::RegisterSpec for ACTIONrs {
    type Ux = u32;
}
///`read()` method returns [`action::R`](R) reader structure
impl crate::Readable for ACTIONrs {}
///`write(|w| ..)` method takes [`action::W`](W) writer structure
impl crate::Writable for ACTIONrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACTION to value 0
impl crate::Resettable for ACTIONrs {}
