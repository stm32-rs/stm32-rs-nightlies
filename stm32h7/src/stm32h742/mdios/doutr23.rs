///Register `DOUTR23` reader
pub type R = crate::R<DOUTR23rs>;
///Register `DOUTR23` writer
pub type W = crate::W<DOUTR23rs>;
///Field `DOUT23` reader - Output data sent to MDIO Master during read frames
pub type DOUT23_R = crate::FieldReader<u16>;
///Field `DOUT23` writer - Output data sent to MDIO Master during read frames
pub type DOUT23_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout23(&self) -> DOUT23_R {
        DOUT23_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR23")
            .field("dout23", &self.dout23())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout23(&mut self) -> DOUT23_W<'_, DOUTR23rs> {
        DOUT23_W::new(self, 0)
    }
}
/**MDIOS output data register 23

You can [`read`](crate::Reg::read) this register and get [`doutr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#MDIOS:DOUTR23)*/
pub struct DOUTR23rs;
impl crate::RegisterSpec for DOUTR23rs {
    type Ux = u32;
}
///`read()` method returns [`doutr23::R`](R) reader structure
impl crate::Readable for DOUTR23rs {}
///`write(|w| ..)` method takes [`doutr23::W`](W) writer structure
impl crate::Writable for DOUTR23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR23 to value 0
impl crate::Resettable for DOUTR23rs {}
