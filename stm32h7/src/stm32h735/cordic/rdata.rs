///Register `RDATA` reader
pub type R = crate::R<RDATArs>;
///Register `RDATA` writer
pub type W = crate::W<RDATArs>;
///Field `RES` reader - Function result
pub type RES_R = crate::FieldReader<u32>;
///Field `RES` writer - Function result
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Function result
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA").field("res", &self.res()).finish()
    }
}
impl W {
    ///Bits 0:31 - Function result
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<RDATArs> {
        RES_W::new(self, 0)
    }
}
/**Result register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#CORDIC:RDATA)*/
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
///`read()` method returns [`rdata::R`](R) reader structure
impl crate::Readable for RDATArs {}
///`write(|w| ..)` method takes [`rdata::W`](W) writer structure
impl crate::Writable for RDATArs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATArs {
    const RESET_VALUE: u32 = 0;
}
