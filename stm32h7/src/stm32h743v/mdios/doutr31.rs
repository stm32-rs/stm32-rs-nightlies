///Register `DOUTR31` reader
pub type R = crate::R<DOUTR31rs>;
///Register `DOUTR31` writer
pub type W = crate::W<DOUTR31rs>;
///Field `DOUT31` reader - Output data sent to MDIO Master during read frames
pub type DOUT31_R = crate::FieldReader<u16>;
///Field `DOUT31` writer - Output data sent to MDIO Master during read frames
pub type DOUT31_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout31(&self) -> DOUT31_R {
        DOUT31_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR31")
            .field("dout31", &self.dout31())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout31(&mut self) -> DOUT31_W<'_, DOUTR31rs> {
        DOUT31_W::new(self, 0)
    }
}
/**MDIOS output data register 31

You can [`read`](crate::Reg::read) this register and get [`doutr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#MDIOS:DOUTR31)*/
pub struct DOUTR31rs;
impl crate::RegisterSpec for DOUTR31rs {
    type Ux = u32;
}
///`read()` method returns [`doutr31::R`](R) reader structure
impl crate::Readable for DOUTR31rs {}
///`write(|w| ..)` method takes [`doutr31::W`](W) writer structure
impl crate::Writable for DOUTR31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR31 to value 0
impl crate::Resettable for DOUTR31rs {}
