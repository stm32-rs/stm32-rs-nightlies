///Register `DOUTR11` reader
pub type R = crate::R<DOUTR11rs>;
///Register `DOUTR11` writer
pub type W = crate::W<DOUTR11rs>;
///Field `DOUT11` reader - Output data sent to MDIO Master during read frames
pub type DOUT11_R = crate::FieldReader<u16>;
///Field `DOUT11` writer - Output data sent to MDIO Master during read frames
pub type DOUT11_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR11")
            .field("dout11", &self.dout11())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout11(&mut self) -> DOUT11_W<'_, DOUTR11rs> {
        DOUT11_W::new(self, 0)
    }
}
/**MDIOS output data register 11

You can [`read`](crate::Reg::read) this register and get [`doutr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#MDIOS:DOUTR11)*/
pub struct DOUTR11rs;
impl crate::RegisterSpec for DOUTR11rs {
    type Ux = u32;
}
///`read()` method returns [`doutr11::R`](R) reader structure
impl crate::Readable for DOUTR11rs {}
///`write(|w| ..)` method takes [`doutr11::W`](W) writer structure
impl crate::Writable for DOUTR11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR11 to value 0
impl crate::Resettable for DOUTR11rs {}
