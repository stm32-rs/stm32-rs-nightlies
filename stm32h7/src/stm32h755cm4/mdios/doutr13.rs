///Register `DOUTR13` reader
pub type R = crate::R<DOUTR13rs>;
///Register `DOUTR13` writer
pub type W = crate::W<DOUTR13rs>;
///Field `DOUT13` reader - Output data sent to MDIO Master during read frames
pub type DOUT13_R = crate::FieldReader<u16>;
///Field `DOUT13` writer - Output data sent to MDIO Master during read frames
pub type DOUT13_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR13")
            .field("dout13", &self.dout13())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout13(&mut self) -> DOUT13_W<'_, DOUTR13rs> {
        DOUT13_W::new(self, 0)
    }
}
/**MDIOS output data register 13

You can [`read`](crate::Reg::read) this register and get [`doutr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#MDIOS:DOUTR13)*/
pub struct DOUTR13rs;
impl crate::RegisterSpec for DOUTR13rs {
    type Ux = u32;
}
///`read()` method returns [`doutr13::R`](R) reader structure
impl crate::Readable for DOUTR13rs {}
///`write(|w| ..)` method takes [`doutr13::W`](W) writer structure
impl crate::Writable for DOUTR13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR13 to value 0
impl crate::Resettable for DOUTR13rs {}
