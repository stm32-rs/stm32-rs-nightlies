///Register `DOUTR19` reader
pub type R = crate::R<DOUTR19rs>;
///Register `DOUTR19` writer
pub type W = crate::W<DOUTR19rs>;
///Field `DOUT19` reader - Output data sent to MDIO Master during read frames
pub type DOUT19_R = crate::FieldReader<u16>;
///Field `DOUT19` writer - Output data sent to MDIO Master during read frames
pub type DOUT19_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout19(&self) -> DOUT19_R {
        DOUT19_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR19")
            .field("dout19", &self.dout19())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout19(&mut self) -> DOUT19_W<'_, DOUTR19rs> {
        DOUT19_W::new(self, 0)
    }
}
/**MDIOS output data register 19

You can [`read`](crate::Reg::read) this register and get [`doutr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#MDIOS:DOUTR19)*/
pub struct DOUTR19rs;
impl crate::RegisterSpec for DOUTR19rs {
    type Ux = u32;
}
///`read()` method returns [`doutr19::R`](R) reader structure
impl crate::Readable for DOUTR19rs {}
///`write(|w| ..)` method takes [`doutr19::W`](W) writer structure
impl crate::Writable for DOUTR19rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR19 to value 0
impl crate::Resettable for DOUTR19rs {}
