///Register `HASH_DIN` reader
pub type R = crate::R<HASH_DINrs>;
///Register `HASH_DIN` writer
pub type W = crate::W<HASH_DINrs>;
///Field `DATAIN` reader - Data input
pub type DATAIN_R = crate::FieldReader<u32>;
///Field `DATAIN` writer - Data input
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_DIN")
            .field("datain", &self.datain())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W<'_, HASH_DINrs> {
        DATAIN_W::new(self, 0)
    }
}
/**HASH data input register

You can [`read`](crate::Reg::read) this register and get [`hash_din::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_din::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HASH:HASH_DIN)*/
pub struct HASH_DINrs;
impl crate::RegisterSpec for HASH_DINrs {
    type Ux = u32;
}
///`read()` method returns [`hash_din::R`](R) reader structure
impl crate::Readable for HASH_DINrs {}
///`write(|w| ..)` method takes [`hash_din::W`](W) writer structure
impl crate::Writable for HASH_DINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_DIN to value 0
impl crate::Resettable for HASH_DINrs {}
