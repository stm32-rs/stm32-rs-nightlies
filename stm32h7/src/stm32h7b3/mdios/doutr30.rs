///Register `DOUTR30` reader
pub type R = crate::R<DOUTR30rs>;
///Register `DOUTR30` writer
pub type W = crate::W<DOUTR30rs>;
///Field `DOUT30` reader - Output data sent to MDIO Master during read frames
pub type DOUT30_R = crate::FieldReader<u16>;
///Field `DOUT30` writer - Output data sent to MDIO Master during read frames
pub type DOUT30_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout30(&self) -> DOUT30_R {
        DOUT30_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR30")
            .field("dout30", &self.dout30())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout30(&mut self) -> DOUT30_W<'_, DOUTR30rs> {
        DOUT30_W::new(self, 0)
    }
}
/**MDIOS output data register 30

You can [`read`](crate::Reg::read) this register and get [`doutr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#MDIOS:DOUTR30)*/
pub struct DOUTR30rs;
impl crate::RegisterSpec for DOUTR30rs {
    type Ux = u32;
}
///`read()` method returns [`doutr30::R`](R) reader structure
impl crate::Readable for DOUTR30rs {}
///`write(|w| ..)` method takes [`doutr30::W`](W) writer structure
impl crate::Writable for DOUTR30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR30 to value 0
impl crate::Resettable for DOUTR30rs {}
