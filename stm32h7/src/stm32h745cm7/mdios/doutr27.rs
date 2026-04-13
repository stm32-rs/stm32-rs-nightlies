///Register `DOUTR27` reader
pub type R = crate::R<DOUTR27rs>;
///Register `DOUTR27` writer
pub type W = crate::W<DOUTR27rs>;
///Field `DOUT27` reader - Output data sent to MDIO Master during read frames
pub type DOUT27_R = crate::FieldReader<u16>;
///Field `DOUT27` writer - Output data sent to MDIO Master during read frames
pub type DOUT27_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout27(&self) -> DOUT27_R {
        DOUT27_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR27")
            .field("dout27", &self.dout27())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout27(&mut self) -> DOUT27_W<'_, DOUTR27rs> {
        DOUT27_W::new(self, 0)
    }
}
/**MDIOS output data register 27

You can [`read`](crate::Reg::read) this register and get [`doutr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#MDIOS:DOUTR27)*/
pub struct DOUTR27rs;
impl crate::RegisterSpec for DOUTR27rs {
    type Ux = u32;
}
///`read()` method returns [`doutr27::R`](R) reader structure
impl crate::Readable for DOUTR27rs {}
///`write(|w| ..)` method takes [`doutr27::W`](W) writer structure
impl crate::Writable for DOUTR27rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR27 to value 0
impl crate::Resettable for DOUTR27rs {}
