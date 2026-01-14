///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `OR` reader - OR
pub type OR_R = crate::FieldReader;
///Field `OR` writer - OR
pub type OR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - OR
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR").field("or", &self.or()).finish()
    }
}
impl W {
    ///Bits 0:1 - OR
    #[inline(always)]
    pub fn or(&mut self) -> OR_W<'_, OPTRrs> {
        OR_W::new(self, 0)
    }
}
/**Option Register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#LPTIM1:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {}
