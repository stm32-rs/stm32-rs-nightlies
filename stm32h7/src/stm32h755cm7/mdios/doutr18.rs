///Register `DOUTR18` reader
pub type R = crate::R<DOUTR18rs>;
///Register `DOUTR18` writer
pub type W = crate::W<DOUTR18rs>;
///Field `DOUT18` reader - Output data sent to MDIO Master during read frames
pub type DOUT18_R = crate::FieldReader<u16>;
///Field `DOUT18` writer - Output data sent to MDIO Master during read frames
pub type DOUT18_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout18(&self) -> DOUT18_R {
        DOUT18_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR18")
            .field("dout18", &self.dout18())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout18(&mut self) -> DOUT18_W<'_, DOUTR18rs> {
        DOUT18_W::new(self, 0)
    }
}
/**MDIOS output data register 18

You can [`read`](crate::Reg::read) this register and get [`doutr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#MDIOS:DOUTR18)*/
pub struct DOUTR18rs;
impl crate::RegisterSpec for DOUTR18rs {
    type Ux = u32;
}
///`read()` method returns [`doutr18::R`](R) reader structure
impl crate::Readable for DOUTR18rs {}
///`write(|w| ..)` method takes [`doutr18::W`](W) writer structure
impl crate::Writable for DOUTR18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR18 to value 0
impl crate::Resettable for DOUTR18rs {}
