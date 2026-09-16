///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `RMP` reader - Input 1 remapping capability
pub type RMP_R = crate::FieldReader;
///Field `RMP` writer - Input 1 remapping capability
pub type RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Input 1 remapping capability
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR").field("rmp", &self.rmp()).finish()
    }
}
impl W {
    ///Bits 0:1 - Input 1 remapping capability
    #[inline(always)]
    pub fn rmp(&mut self) -> RMP_W<'_, ORrs> {
        RMP_W::new(self, 0)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#TIM10:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u16;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
