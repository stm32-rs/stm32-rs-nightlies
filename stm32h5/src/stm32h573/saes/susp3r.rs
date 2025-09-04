///Register `SUSP3R` reader
pub type R = crate::R<SUSP3Rrs>;
///Register `SUSP3R` writer
pub type W = crate::W<SUSP3Rrs>;
///Field `SUSP` reader - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP3R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<SUSP3Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAES:SUSP3R)*/
pub struct SUSP3Rrs;
impl crate::RegisterSpec for SUSP3Rrs {
    type Ux = u32;
}
///`read()` method returns [`susp3r::R`](R) reader structure
impl crate::Readable for SUSP3Rrs {}
///`write(|w| ..)` method takes [`susp3r::W`](W) writer structure
impl crate::Writable for SUSP3Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSP3R to value 0
impl crate::Resettable for SUSP3Rrs {}
