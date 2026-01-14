///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `VDDCOREEN` reader - VDDCOREEN
pub type VDDCOREEN_R = crate::BitReader;
///Field `VDDCOREEN` writer - VDDCOREEN
pub type VDDCOREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VDDCOREEN
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("vddcoreen", &self.vddcoreen())
            .finish()
    }
}
impl W {
    ///Bit 0 - VDDCOREEN
    #[inline(always)]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W<'_, ORrs> {
        VDDCOREEN_W::new(self, 0)
    }
}
/**ADC2 option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC2:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
