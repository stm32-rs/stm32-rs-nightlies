///Register `DOUTR6` reader
pub type R = crate::R<DOUTR6rs>;
///Register `DOUTR6` writer
pub type W = crate::W<DOUTR6rs>;
///Field `DOUT6` reader - Output data sent to MDIO Master during read frames
pub type DOUT6_R = crate::FieldReader<u16>;
///Field `DOUT6` writer - Output data sent to MDIO Master during read frames
pub type DOUT6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout6(&self) -> DOUT6_R {
        DOUT6_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR6")
            .field("dout6", &self.dout6())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout6(&mut self) -> DOUT6_W<'_, DOUTR6rs> {
        DOUT6_W::new(self, 0)
    }
}
/**MDIOS output data register 6

You can [`read`](crate::Reg::read) this register and get [`doutr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#MDIOS:DOUTR6)*/
pub struct DOUTR6rs;
impl crate::RegisterSpec for DOUTR6rs {
    type Ux = u32;
}
///`read()` method returns [`doutr6::R`](R) reader structure
impl crate::Readable for DOUTR6rs {}
///`write(|w| ..)` method takes [`doutr6::W`](W) writer structure
impl crate::Writable for DOUTR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR6 to value 0
impl crate::Resettable for DOUTR6rs {}
