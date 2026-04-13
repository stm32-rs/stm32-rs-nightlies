///Register `DOUTR26` reader
pub type R = crate::R<DOUTR26rs>;
///Register `DOUTR26` writer
pub type W = crate::W<DOUTR26rs>;
///Field `DOUT26` reader - Output data sent to MDIO Master during read frames
pub type DOUT26_R = crate::FieldReader<u16>;
///Field `DOUT26` writer - Output data sent to MDIO Master during read frames
pub type DOUT26_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout26(&self) -> DOUT26_R {
        DOUT26_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR26")
            .field("dout26", &self.dout26())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout26(&mut self) -> DOUT26_W<'_, DOUTR26rs> {
        DOUT26_W::new(self, 0)
    }
}
/**MDIOS output data register 26

You can [`read`](crate::Reg::read) this register and get [`doutr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#MDIOS:DOUTR26)*/
pub struct DOUTR26rs;
impl crate::RegisterSpec for DOUTR26rs {
    type Ux = u32;
}
///`read()` method returns [`doutr26::R`](R) reader structure
impl crate::Readable for DOUTR26rs {}
///`write(|w| ..)` method takes [`doutr26::W`](W) writer structure
impl crate::Writable for DOUTR26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR26 to value 0
impl crate::Resettable for DOUTR26rs {}
