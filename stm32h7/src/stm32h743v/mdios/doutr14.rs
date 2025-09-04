///Register `DOUTR14` reader
pub type R = crate::R<DOUTR14rs>;
///Register `DOUTR14` writer
pub type W = crate::W<DOUTR14rs>;
///Field `DOUT14` reader - Output data sent to MDIO Master during read frames
pub type DOUT14_R = crate::FieldReader<u16>;
///Field `DOUT14` writer - Output data sent to MDIO Master during read frames
pub type DOUT14_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR14")
            .field("dout14", &self.dout14())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout14(&mut self) -> DOUT14_W<DOUTR14rs> {
        DOUT14_W::new(self, 0)
    }
}
/**MDIOS output data register 14

You can [`read`](crate::Reg::read) this register and get [`doutr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#MDIOS:DOUTR14)*/
pub struct DOUTR14rs;
impl crate::RegisterSpec for DOUTR14rs {
    type Ux = u32;
}
///`read()` method returns [`doutr14::R`](R) reader structure
impl crate::Readable for DOUTR14rs {}
///`write(|w| ..)` method takes [`doutr14::W`](W) writer structure
impl crate::Writable for DOUTR14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR14 to value 0
impl crate::Resettable for DOUTR14rs {}
