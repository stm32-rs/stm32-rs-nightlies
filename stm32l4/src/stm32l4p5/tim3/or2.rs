///Register `OR2` reader
pub type R = crate::R<OR2rs>;
///Register `OR2` writer
pub type W = crate::W<OR2rs>;
///Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved
pub type ETRSEL_R = crate::FieldReader;
///Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 14:16 - ETR source selection These bits select the ETR input source. Others: Reserved
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR2")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:16 - ETR source selection These bits select the ETR input source. Others: Reserved
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, OR2rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM3 option register 2

You can [`read`](crate::Reg::read) this register and get [`or2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TIM3:OR2)*/
pub struct OR2rs;
impl crate::RegisterSpec for OR2rs {
    type Ux = u32;
}
///`read()` method returns [`or2::R`](R) reader structure
impl crate::Readable for OR2rs {}
///`write(|w| ..)` method takes [`or2::W`](W) writer structure
impl crate::Writable for OR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR2 to value 0
impl crate::Resettable for OR2rs {}
