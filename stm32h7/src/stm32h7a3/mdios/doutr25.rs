///Register `DOUTR25` reader
pub type R = crate::R<DOUTR25rs>;
///Register `DOUTR25` writer
pub type W = crate::W<DOUTR25rs>;
///Field `DOUT25` reader - Output data sent to MDIO Master during read frames
pub type DOUT25_R = crate::FieldReader<u16>;
///Field `DOUT25` writer - Output data sent to MDIO Master during read frames
pub type DOUT25_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout25(&self) -> DOUT25_R {
        DOUT25_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR25")
            .field("dout25", &self.dout25())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout25(&mut self) -> DOUT25_W<DOUTR25rs> {
        DOUT25_W::new(self, 0)
    }
}
/**MDIOS output data register 25

You can [`read`](crate::Reg::read) this register and get [`doutr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#MDIOS:DOUTR25)*/
pub struct DOUTR25rs;
impl crate::RegisterSpec for DOUTR25rs {
    type Ux = u32;
}
///`read()` method returns [`doutr25::R`](R) reader structure
impl crate::Readable for DOUTR25rs {}
///`write(|w| ..)` method takes [`doutr25::W`](W) writer structure
impl crate::Writable for DOUTR25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR25 to value 0
impl crate::Resettable for DOUTR25rs {}
