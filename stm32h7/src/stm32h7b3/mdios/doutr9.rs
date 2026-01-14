///Register `DOUTR9` reader
pub type R = crate::R<DOUTR9rs>;
///Register `DOUTR9` writer
pub type W = crate::W<DOUTR9rs>;
///Field `DOUT9` reader - Output data sent to MDIO Master during read frames
pub type DOUT9_R = crate::FieldReader<u16>;
///Field `DOUT9` writer - Output data sent to MDIO Master during read frames
pub type DOUT9_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR9")
            .field("dout9", &self.dout9())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout9(&mut self) -> DOUT9_W<'_, DOUTR9rs> {
        DOUT9_W::new(self, 0)
    }
}
/**MDIOS output data register 9

You can [`read`](crate::Reg::read) this register and get [`doutr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#MDIOS:DOUTR9)*/
pub struct DOUTR9rs;
impl crate::RegisterSpec for DOUTR9rs {
    type Ux = u32;
}
///`read()` method returns [`doutr9::R`](R) reader structure
impl crate::Readable for DOUTR9rs {}
///`write(|w| ..)` method takes [`doutr9::W`](W) writer structure
impl crate::Writable for DOUTR9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR9 to value 0
impl crate::Resettable for DOUTR9rs {}
