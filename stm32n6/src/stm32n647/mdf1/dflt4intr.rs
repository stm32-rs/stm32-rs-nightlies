///Register `DFLT4INTR` reader
pub type R = crate::R<DFLT4INTRrs>;
///Register `DFLT4INTR` writer
pub type W = crate::W<DFLT4INTRrs>;
///Field `INTDIV` reader - Integrator output division
pub type INTDIV_R = crate::FieldReader;
///Field `INTDIV` writer - Integrator output division
pub type INTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INTVAL` reader - Integration value selection
pub type INTVAL_R = crate::FieldReader;
///Field `INTVAL` writer - Integration value selection
pub type INTVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:1 - Integrator output division
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:10 - Integration value selection
    #[inline(always)]
    pub fn intval(&self) -> INTVAL_R {
        INTVAL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT4INTR")
            .field("intdiv", &self.intdiv())
            .field("intval", &self.intval())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Integrator output division
    #[inline(always)]
    pub fn intdiv(&mut self) -> INTDIV_W<DFLT4INTRrs> {
        INTDIV_W::new(self, 0)
    }
    ///Bits 4:10 - Integration value selection
    #[inline(always)]
    pub fn intval(&mut self) -> INTVAL_W<DFLT4INTRrs> {
        INTVAL_W::new(self, 4)
    }
}
/**MDF integrator configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DFLT4INTR)*/
pub struct DFLT4INTRrs;
impl crate::RegisterSpec for DFLT4INTRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt4intr::R`](R) reader structure
impl crate::Readable for DFLT4INTRrs {}
///`write(|w| ..)` method takes [`dflt4intr::W`](W) writer structure
impl crate::Writable for DFLT4INTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT4INTR to value 0
impl crate::Resettable for DFLT4INTRrs {}
