///Register `DOUTR7` reader
pub type R = crate::R<DOUTR7rs>;
///Register `DOUTR7` writer
pub type W = crate::W<DOUTR7rs>;
///Field `DOUT7` reader - Output data sent to MDIO Master during read frames
pub type DOUT7_R = crate::FieldReader<u16>;
///Field `DOUT7` writer - Output data sent to MDIO Master during read frames
pub type DOUT7_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR7")
            .field("dout7", &self.dout7())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout7(&mut self) -> DOUT7_W<'_, DOUTR7rs> {
        DOUT7_W::new(self, 0)
    }
}
/**MDIOS output data register 7

You can [`read`](crate::Reg::read) this register and get [`doutr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#MDIOS:DOUTR7)*/
pub struct DOUTR7rs;
impl crate::RegisterSpec for DOUTR7rs {
    type Ux = u32;
}
///`read()` method returns [`doutr7::R`](R) reader structure
impl crate::Readable for DOUTR7rs {}
///`write(|w| ..)` method takes [`doutr7::W`](W) writer structure
impl crate::Writable for DOUTR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR7 to value 0
impl crate::Resettable for DOUTR7rs {}
