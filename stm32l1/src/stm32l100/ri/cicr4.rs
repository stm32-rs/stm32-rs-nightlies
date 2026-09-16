///Register `CICR4` reader
pub type R = crate::R<CICR4rs>;
///Register `CICR4` writer
pub type W = crate::W<CICR4rs>;
///Field `PF` reader - Port F channel identification for capture
pub type PF_R = crate::FieldReader<u16>;
///Field `PF` writer - Port F channel identification for capture
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port F channel identification for capture
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CICR4").field("pf", &self.pf()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port F channel identification for capture
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<'_, CICR4rs> {
        PF_W::new(self, 0)
    }
}
/**Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR4)*/
pub struct CICR4rs;
impl crate::RegisterSpec for CICR4rs {
    type Ux = u32;
}
///`read()` method returns [`cicr4::R`](R) reader structure
impl crate::Readable for CICR4rs {}
///`write(|w| ..)` method takes [`cicr4::W`](W) writer structure
impl crate::Writable for CICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR4 to value 0
impl crate::Resettable for CICR4rs {}
