///Register `_PSC` reader
pub type R = crate::R<_PSCrs>;
///Register `_PSC` writer
pub type W = crate::W<_PSCrs>;
///Field `PSC` reader - PSC
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - PSC
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PSC
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_PSC").field("psc", &self.psc()).finish()
    }
}
impl W {
    ///Bits 0:15 - PSC
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<'_, _PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM12 prescaler

You can [`read`](crate::Reg::read) this register and get [`_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM12:_PSC)*/
pub struct _PSCrs;
impl crate::RegisterSpec for _PSCrs {
    type Ux = u16;
}
///`read()` method returns [`_psc::R`](R) reader structure
impl crate::Readable for _PSCrs {}
///`write(|w| ..)` method takes [`_psc::W`](W) writer structure
impl crate::Writable for _PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _PSC to value 0
impl crate::Resettable for _PSCrs {}
