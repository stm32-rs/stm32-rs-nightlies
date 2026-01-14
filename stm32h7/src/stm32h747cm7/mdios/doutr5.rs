///Register `DOUTR5` reader
pub type R = crate::R<DOUTR5rs>;
///Register `DOUTR5` writer
pub type W = crate::W<DOUTR5rs>;
///Field `DOUT5` reader - Output data sent to MDIO Master during read frames
pub type DOUT5_R = crate::FieldReader<u16>;
///Field `DOUT5` writer - Output data sent to MDIO Master during read frames
pub type DOUT5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR5")
            .field("dout5", &self.dout5())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT5_W<'_, DOUTR5rs> {
        DOUT5_W::new(self, 0)
    }
}
/**MDIOS output data register 5

You can [`read`](crate::Reg::read) this register and get [`doutr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#MDIOS:DOUTR5)*/
pub struct DOUTR5rs;
impl crate::RegisterSpec for DOUTR5rs {
    type Ux = u32;
}
///`read()` method returns [`doutr5::R`](R) reader structure
impl crate::Readable for DOUTR5rs {}
///`write(|w| ..)` method takes [`doutr5::W`](W) writer structure
impl crate::Writable for DOUTR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR5 to value 0
impl crate::Resettable for DOUTR5rs {}
