///Register `DOUTR22` reader
pub type R = crate::R<DOUTR22rs>;
///Register `DOUTR22` writer
pub type W = crate::W<DOUTR22rs>;
///Field `DOUT22` reader - Output data sent to MDIO Master during read frames
pub type DOUT22_R = crate::FieldReader<u16>;
///Field `DOUT22` writer - Output data sent to MDIO Master during read frames
pub type DOUT22_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout22(&self) -> DOUT22_R {
        DOUT22_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR22")
            .field("dout22", &self.dout22())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout22(&mut self) -> DOUT22_W<'_, DOUTR22rs> {
        DOUT22_W::new(self, 0)
    }
}
/**MDIOS output data register 22

You can [`read`](crate::Reg::read) this register and get [`doutr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#MDIOS:DOUTR22)*/
pub struct DOUTR22rs;
impl crate::RegisterSpec for DOUTR22rs {
    type Ux = u32;
}
///`read()` method returns [`doutr22::R`](R) reader structure
impl crate::Readable for DOUTR22rs {}
///`write(|w| ..)` method takes [`doutr22::W`](W) writer structure
impl crate::Writable for DOUTR22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR22 to value 0
impl crate::Resettable for DOUTR22rs {}
