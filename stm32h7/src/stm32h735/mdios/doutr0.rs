///Register `DOUTR0` reader
pub type R = crate::R<DOUTR0rs>;
///Register `DOUTR0` writer
pub type W = crate::W<DOUTR0rs>;
///Field `DOUT0` reader - Output data sent to MDIO Master during read frames
pub type DOUT0_R = crate::FieldReader<u16>;
///Field `DOUT0` writer - Output data sent to MDIO Master during read frames
pub type DOUT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR0")
            .field("dout0", &self.dout0())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout0(&mut self) -> DOUT0_W<'_, DOUTR0rs> {
        DOUT0_W::new(self, 0)
    }
}
/**MDIOS output data register 0

You can [`read`](crate::Reg::read) this register and get [`doutr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#MDIOS:DOUTR0)*/
pub struct DOUTR0rs;
impl crate::RegisterSpec for DOUTR0rs {
    type Ux = u32;
}
///`read()` method returns [`doutr0::R`](R) reader structure
impl crate::Readable for DOUTR0rs {}
///`write(|w| ..)` method takes [`doutr0::W`](W) writer structure
impl crate::Writable for DOUTR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR0 to value 0
impl crate::Resettable for DOUTR0rs {}
