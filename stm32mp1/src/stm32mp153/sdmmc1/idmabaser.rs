///Register `IDMABASER` reader
pub type R = crate::R<IDMABASERrs>;
///Register `IDMABASER` writer
pub type W = crate::W<IDMABASERrs>;
///Field `IDMABASE` reader - IDMABASE
pub type IDMABASE_R = crate::FieldReader<u32>;
///Field `IDMABASE` writer - IDMABASE
pub type IDMABASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IDMABASE
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABASER")
            .field("idmabase", &self.idmabase())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IDMABASE
    #[inline(always)]
    pub fn idmabase(&mut self) -> IDMABASE_W<IDMABASERrs> {
        IDMABASE_W::new(self, 0)
    }
}
/**The SDMMC_IDMABASER register contains the memory buffer base address in single buffer configuration and linked list configuration.

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:IDMABASER)*/
pub struct IDMABASERrs;
impl crate::RegisterSpec for IDMABASERrs {
    type Ux = u32;
}
///`read()` method returns [`idmabaser::R`](R) reader structure
impl crate::Readable for IDMABASERrs {}
///`write(|w| ..)` method takes [`idmabaser::W`](W) writer structure
impl crate::Writable for IDMABASERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABASER to value 0
impl crate::Resettable for IDMABASERrs {}
