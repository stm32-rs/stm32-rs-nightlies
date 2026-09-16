///Register `CICR3` reader
pub type R = crate::R<CICR3rs>;
///Register `CICR3` writer
pub type W = crate::W<CICR3rs>;
///Field `PC` reader - Port C channel identification for capture
pub type PC_R = crate::FieldReader<u16>;
///Field `PC` writer - Port C channel identification for capture
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port C channel identification for capture
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CICR3").field("pc", &self.pc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port C channel identification for capture
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<'_, CICR3rs> {
        PC_W::new(self, 0)
    }
}
/**Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR3)*/
pub struct CICR3rs;
impl crate::RegisterSpec for CICR3rs {
    type Ux = u32;
}
///`read()` method returns [`cicr3::R`](R) reader structure
impl crate::Readable for CICR3rs {}
///`write(|w| ..)` method takes [`cicr3::W`](W) writer structure
impl crate::Writable for CICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CICR3 to value 0
impl crate::Resettable for CICR3rs {}
