///Register `CMR3` reader
pub type R = crate::R<CMR3rs>;
///Register `CMR3` writer
pub type W = crate::W<CMR3rs>;
///Field `PC` reader - Port C channel masking
pub type PC_R = crate::FieldReader<u16>;
///Field `PC` writer - Port C channel masking
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port C channel masking
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMR3").field("pc", &self.pc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port C channel masking
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<'_, CMR3rs> {
        PC_W::new(self, 0)
    }
}
/**Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RI:CMR3)*/
pub struct CMR3rs;
impl crate::RegisterSpec for CMR3rs {
    type Ux = u32;
}
///`read()` method returns [`cmr3::R`](R) reader structure
impl crate::Readable for CMR3rs {}
///`write(|w| ..)` method takes [`cmr3::W`](W) writer structure
impl crate::Writable for CMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMR3 to value 0
impl crate::Resettable for CMR3rs {}
