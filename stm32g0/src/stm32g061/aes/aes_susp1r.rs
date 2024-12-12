///Register `AES_SUSP1R` reader
pub type R = crate::R<AES_SUSP1Rrs>;
///Register `AES_SUSP1R` writer
pub type W = crate::W<AES_SUSP1Rrs>;
///Field `SUSP` reader - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers.
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers.
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_SUSP1R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers.
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<AES_SUSP1Rrs> {
        SUSP_W::new(self, 0)
    }
}
/**AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP1R)*/
pub struct AES_SUSP1Rrs;
impl crate::RegisterSpec for AES_SUSP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`aes_susp1r::R`](R) reader structure
impl crate::Readable for AES_SUSP1Rrs {}
///`write(|w| ..)` method takes [`aes_susp1r::W`](W) writer structure
impl crate::Writable for AES_SUSP1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_SUSP1R to value 0
impl crate::Resettable for AES_SUSP1Rrs {
    const RESET_VALUE: u32 = 0;
}
