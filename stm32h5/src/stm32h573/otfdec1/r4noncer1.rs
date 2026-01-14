///Register `R4NONCER1` reader
pub type R = crate::R<R4NONCER1rs>;
///Register `R4NONCER1` writer
pub type W = crate::W<R4NONCER1rs>;
///Field `REGx_NONCE` reader - Region nonce, bits \[63:32\] Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\[63:0\] bitfield.
pub type REGX_NONCE_R = crate::FieldReader<u32>;
///Field `REGx_NONCE` writer - Region nonce, bits \[63:32\] Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\[63:0\] bitfield.
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region nonce, bits \[63:32\] Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\[63:0\] bitfield.
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R4NONCER1")
            .field("regx_nonce", &self.regx_nonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region nonce, bits \[63:32\] Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\[63:0\] bitfield.
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<'_, R4NONCER1rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region 4 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`r4noncer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4noncer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#OTFDEC1:R4NONCER1)*/
pub struct R4NONCER1rs;
impl crate::RegisterSpec for R4NONCER1rs {
    type Ux = u32;
}
///`read()` method returns [`r4noncer1::R`](R) reader structure
impl crate::Readable for R4NONCER1rs {}
///`write(|w| ..)` method takes [`r4noncer1::W`](W) writer structure
impl crate::Writable for R4NONCER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R4NONCER1 to value 0
impl crate::Resettable for R4NONCER1rs {}
