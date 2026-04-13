///Register `DOUTR24` reader
pub type R = crate::R<DOUTR24rs>;
///Register `DOUTR24` writer
pub type W = crate::W<DOUTR24rs>;
///Field `DOUT24` reader - Output data sent to MDIO Master during read frames
pub type DOUT24_R = crate::FieldReader<u16>;
///Field `DOUT24` writer - Output data sent to MDIO Master during read frames
pub type DOUT24_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout24(&self) -> DOUT24_R {
        DOUT24_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR24")
            .field("dout24", &self.dout24())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout24(&mut self) -> DOUT24_W<'_, DOUTR24rs> {
        DOUT24_W::new(self, 0)
    }
}
/**MDIOS output data register 24

You can [`read`](crate::Reg::read) this register and get [`doutr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#MDIOS:DOUTR24)*/
pub struct DOUTR24rs;
impl crate::RegisterSpec for DOUTR24rs {
    type Ux = u32;
}
///`read()` method returns [`doutr24::R`](R) reader structure
impl crate::Readable for DOUTR24rs {}
///`write(|w| ..)` method takes [`doutr24::W`](W) writer structure
impl crate::Writable for DOUTR24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR24 to value 0
impl crate::Resettable for DOUTR24rs {}
