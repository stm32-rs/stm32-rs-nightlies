///Register `DOUTR21` reader
pub type R = crate::R<DOUTR21rs>;
///Register `DOUTR21` writer
pub type W = crate::W<DOUTR21rs>;
///Field `DOUT` reader - output data sent to MDIO Master during read frames This field is written by software. These 16 bits are serially output on the MDIO bus during read frames which address the MDIOS register x.
pub type DOUT_R = crate::FieldReader<u16>;
///Field `DOUT` writer - output data sent to MDIO Master during read frames This field is written by software. These 16 bits are serially output on the MDIO bus during read frames which address the MDIOS register x.
pub type DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - output data sent to MDIO Master during read frames This field is written by software. These 16 bits are serially output on the MDIO bus during read frames which address the MDIOS register x.
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR21")
            .field("dout", &self.dout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - output data sent to MDIO Master during read frames This field is written by software. These 16 bits are serially output on the MDIO bus during read frames which address the MDIOS register x.
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W<DOUTR21rs> {
        DOUT_W::new(self, 0)
    }
}
/**MDIOS output data register 21

You can [`read`](crate::Reg::read) this register and get [`doutr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MDIOS:DOUTR21)*/
pub struct DOUTR21rs;
impl crate::RegisterSpec for DOUTR21rs {
    type Ux = u32;
}
///`read()` method returns [`doutr21::R`](R) reader structure
impl crate::Readable for DOUTR21rs {}
///`write(|w| ..)` method takes [`doutr21::W`](W) writer structure
impl crate::Writable for DOUTR21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOUTR21 to value 0
impl crate::Resettable for DOUTR21rs {}
