///Register `DOUTR2` reader
pub type R = crate::R<DOUTR2rs>;
///Register `DOUTR2` writer
pub type W = crate::W<DOUTR2rs>;
///Field `DOUT2` reader - Output data sent to MDIO Master during read frames
pub type DOUT2_R = crate::FieldReader<u16>;
///Field `DOUT2` writer - Output data sent to MDIO Master during read frames
pub type DOUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR2")
            .field("dout2", &self.dout2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout2(&mut self) -> DOUT2_W<'_, DOUTR2rs> {
        DOUT2_W::new(self, 0)
    }
}
/**MDIOS output data register 2

You can [`read`](crate::Reg::read) this register and get [`doutr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#MDIOS:DOUTR2)*/
pub struct DOUTR2rs;
impl crate::RegisterSpec for DOUTR2rs {
    type Ux = u32;
}
///`read()` method returns [`doutr2::R`](R) reader structure
impl crate::Readable for DOUTR2rs {}
///`write(|w| ..)` method takes [`doutr2::W`](W) writer structure
impl crate::Writable for DOUTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR2 to value 0
impl crate::Resettable for DOUTR2rs {}
