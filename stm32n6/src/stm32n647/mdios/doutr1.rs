///Register `DOUTR1` reader
pub type R = crate::R<DOUTR1rs>;
///Register `DOUTR1` writer
pub type W = crate::W<DOUTR1rs>;
///Field `DOUT` reader - output data sent to MDIO Master during read frames
pub type DOUT_R = crate::FieldReader<u16>;
///Field `DOUT` writer - output data sent to MDIO Master during read frames
pub type DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR1")
            .field("dout", &self.dout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W<'_, DOUTR1rs> {
        DOUT_W::new(self, 0)
    }
}
/**MDIOS output data register 1

You can [`read`](crate::Reg::read) this register and get [`doutr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDIOS:DOUTR1)*/
pub struct DOUTR1rs;
impl crate::RegisterSpec for DOUTR1rs {
    type Ux = u32;
}
///`read()` method returns [`doutr1::R`](R) reader structure
impl crate::Readable for DOUTR1rs {}
///`write(|w| ..)` method takes [`doutr1::W`](W) writer structure
impl crate::Writable for DOUTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR1 to value 0
impl crate::Resettable for DOUTR1rs {}
