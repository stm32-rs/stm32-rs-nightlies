///Register `DOUTR17` reader
pub type R = crate::R<DOUTR17rs>;
///Register `DOUTR17` writer
pub type W = crate::W<DOUTR17rs>;
///Field `DOUT17` reader - Output data sent to MDIO Master during read frames
pub type DOUT17_R = crate::FieldReader<u16>;
///Field `DOUT17` writer - Output data sent to MDIO Master during read frames
pub type DOUT17_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout17(&self) -> DOUT17_R {
        DOUT17_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR17")
            .field("dout17", &self.dout17())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout17(&mut self) -> DOUT17_W<'_, DOUTR17rs> {
        DOUT17_W::new(self, 0)
    }
}
/**MDIOS output data register 17

You can [`read`](crate::Reg::read) this register and get [`doutr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#MDIOS:DOUTR17)*/
pub struct DOUTR17rs;
impl crate::RegisterSpec for DOUTR17rs {
    type Ux = u32;
}
///`read()` method returns [`doutr17::R`](R) reader structure
impl crate::Readable for DOUTR17rs {}
///`write(|w| ..)` method takes [`doutr17::W`](W) writer structure
impl crate::Writable for DOUTR17rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR17 to value 0
impl crate::Resettable for DOUTR17rs {}
