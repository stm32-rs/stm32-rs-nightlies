///Register `DOUTR20` reader
pub type R = crate::R<DOUTR20rs>;
///Register `DOUTR20` writer
pub type W = crate::W<DOUTR20rs>;
///Field `DOUT20` reader - Output data sent to MDIO Master during read frames
pub type DOUT20_R = crate::FieldReader<u16>;
///Field `DOUT20` writer - Output data sent to MDIO Master during read frames
pub type DOUT20_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout20(&self) -> DOUT20_R {
        DOUT20_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR20")
            .field("dout20", &self.dout20())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout20(&mut self) -> DOUT20_W<'_, DOUTR20rs> {
        DOUT20_W::new(self, 0)
    }
}
/**MDIOS output data register 20

You can [`read`](crate::Reg::read) this register and get [`doutr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#MDIOS:DOUTR20)*/
pub struct DOUTR20rs;
impl crate::RegisterSpec for DOUTR20rs {
    type Ux = u32;
}
///`read()` method returns [`doutr20::R`](R) reader structure
impl crate::Readable for DOUTR20rs {}
///`write(|w| ..)` method takes [`doutr20::W`](W) writer structure
impl crate::Writable for DOUTR20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR20 to value 0
impl crate::Resettable for DOUTR20rs {}
