///Register `DOUTR16` reader
pub type R = crate::R<DOUTR16rs>;
///Register `DOUTR16` writer
pub type W = crate::W<DOUTR16rs>;
///Field `DOUT16` reader - Output data sent to MDIO Master during read frames
pub type DOUT16_R = crate::FieldReader<u16>;
///Field `DOUT16` writer - Output data sent to MDIO Master during read frames
pub type DOUT16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout16(&self) -> DOUT16_R {
        DOUT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR16")
            .field("dout16", &self.dout16())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout16(&mut self) -> DOUT16_W<'_, DOUTR16rs> {
        DOUT16_W::new(self, 0)
    }
}
/**MDIOS output data register 16

You can [`read`](crate::Reg::read) this register and get [`doutr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#MDIOS:DOUTR16)*/
pub struct DOUTR16rs;
impl crate::RegisterSpec for DOUTR16rs {
    type Ux = u32;
}
///`read()` method returns [`doutr16::R`](R) reader structure
impl crate::Readable for DOUTR16rs {}
///`write(|w| ..)` method takes [`doutr16::W`](W) writer structure
impl crate::Writable for DOUTR16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR16 to value 0
impl crate::Resettable for DOUTR16rs {}
