///Register `DOUTR12` reader
pub type R = crate::R<DOUTR12rs>;
///Register `DOUTR12` writer
pub type W = crate::W<DOUTR12rs>;
///Field `DOUT12` reader - Output data sent to MDIO Master during read frames
pub type DOUT12_R = crate::FieldReader<u16>;
///Field `DOUT12` writer - Output data sent to MDIO Master during read frames
pub type DOUT12_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout12(&self) -> DOUT12_R {
        DOUT12_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR12")
            .field("dout12", &self.dout12())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout12(&mut self) -> DOUT12_W<'_, DOUTR12rs> {
        DOUT12_W::new(self, 0)
    }
}
/**MDIOS output data register 12

You can [`read`](crate::Reg::read) this register and get [`doutr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#MDIOS:DOUTR12)*/
pub struct DOUTR12rs;
impl crate::RegisterSpec for DOUTR12rs {
    type Ux = u32;
}
///`read()` method returns [`doutr12::R`](R) reader structure
impl crate::Readable for DOUTR12rs {}
///`write(|w| ..)` method takes [`doutr12::W`](W) writer structure
impl crate::Writable for DOUTR12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR12 to value 0
impl crate::Resettable for DOUTR12rs {}
