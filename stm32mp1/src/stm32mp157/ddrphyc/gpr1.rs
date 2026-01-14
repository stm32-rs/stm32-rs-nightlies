///Register `GPR1` reader
pub type R = crate::R<GPR1rs>;
///Register `GPR1` writer
pub type W = crate::W<GPR1rs>;
///Field `GPR1` reader - GPR1
pub type GPR1_R = crate::FieldReader<u32>;
///Field `GPR1` writer - GPR1
pub type GPR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPR1
    #[inline(always)]
    pub fn gpr1(&self) -> GPR1_R {
        GPR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPR1").field("gpr1", &self.gpr1()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPR1
    #[inline(always)]
    pub fn gpr1(&mut self) -> GPR1_W<'_, GPR1rs> {
        GPR1_W::new(self, 0)
    }
}
/**DDRPHYC general purpose register 1

You can [`read`](crate::Reg::read) this register and get [`gpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:GPR1)*/
pub struct GPR1rs;
impl crate::RegisterSpec for GPR1rs {
    type Ux = u32;
}
///`read()` method returns [`gpr1::R`](R) reader structure
impl crate::Readable for GPR1rs {}
///`write(|w| ..)` method takes [`gpr1::W`](W) writer structure
impl crate::Writable for GPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPR1 to value 0
impl crate::Resettable for GPR1rs {}
