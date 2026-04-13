///Register `INIT4` reader
pub type R = crate::R<INIT4rs>;
///Register `INIT4` writer
pub type W = crate::W<INIT4rs>;
///Field `EMR3` reader - EMR3
pub type EMR3_R = crate::FieldReader<u16>;
///Field `EMR3` writer - EMR3
pub type EMR3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EMR2` reader - EMR2
pub type EMR2_R = crate::FieldReader<u16>;
///Field `EMR2` writer - EMR2
pub type EMR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EMR3
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EMR2
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT4")
            .field("emr3", &self.emr3())
            .field("emr2", &self.emr2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - EMR3
    #[inline(always)]
    pub fn emr3(&mut self) -> EMR3_W<'_, INIT4rs> {
        EMR3_W::new(self, 0)
    }
    ///Bits 16:31 - EMR2
    #[inline(always)]
    pub fn emr2(&mut self) -> EMR2_W<'_, INIT4rs> {
        EMR2_W::new(self, 16)
    }
}
/**DDRCTRL SDRAM initialization register 4

You can [`read`](crate::Reg::read) this register and get [`init4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:INIT4)*/
pub struct INIT4rs;
impl crate::RegisterSpec for INIT4rs {
    type Ux = u32;
}
///`read()` method returns [`init4::R`](R) reader structure
impl crate::Readable for INIT4rs {}
///`write(|w| ..)` method takes [`init4::W`](W) writer structure
impl crate::Writable for INIT4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT4 to value 0
impl crate::Resettable for INIT4rs {}
