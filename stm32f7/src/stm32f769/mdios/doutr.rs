///Register `DOUTR%s` reader
pub type R = crate::R<DOUTRrs>;
///Register `DOUTR%s` writer
pub type W = crate::W<DOUTRrs>;
///Field `DOUT` reader - Output data sent to MDIO Master during read frames
pub type DOUT_R = crate::FieldReader<u16>;
///Field `DOUT` writer - Output data sent to MDIO Master during read frames
pub type DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR").field("dout", &self.dout()).finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W<'_, DOUTRrs> {
        DOUT_W::new(self, 0)
    }
}
/**MDIOS output data register %s

You can [`read`](crate::Reg::read) this register and get [`doutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#MDIOS:DOUTR[0])*/
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`doutr::R`](R) reader structure
impl crate::Readable for DOUTRrs {}
///`write(|w| ..)` method takes [`doutr::W`](W) writer structure
impl crate::Writable for DOUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR%s to value 0
impl crate::Resettable for DOUTRrs {}
