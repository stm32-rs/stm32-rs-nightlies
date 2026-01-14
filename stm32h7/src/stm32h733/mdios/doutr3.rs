///Register `DOUTR3` reader
pub type R = crate::R<DOUTR3rs>;
///Register `DOUTR3` writer
pub type W = crate::W<DOUTR3rs>;
///Field `DOUT3` reader - Output data sent to MDIO Master during read frames
pub type DOUT3_R = crate::FieldReader<u16>;
///Field `DOUT3` writer - Output data sent to MDIO Master during read frames
pub type DOUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR3")
            .field("dout3", &self.dout3())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout3(&mut self) -> DOUT3_W<'_, DOUTR3rs> {
        DOUT3_W::new(self, 0)
    }
}
/**MDIOS output data register 3

You can [`read`](crate::Reg::read) this register and get [`doutr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#MDIOS:DOUTR3)*/
pub struct DOUTR3rs;
impl crate::RegisterSpec for DOUTR3rs {
    type Ux = u32;
}
///`read()` method returns [`doutr3::R`](R) reader structure
impl crate::Readable for DOUTR3rs {}
///`write(|w| ..)` method takes [`doutr3::W`](W) writer structure
impl crate::Writable for DOUTR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR3 to value 0
impl crate::Resettable for DOUTR3rs {}
