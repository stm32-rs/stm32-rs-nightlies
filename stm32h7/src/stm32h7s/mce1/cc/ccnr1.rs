///Register `CCNR1` reader
pub type R = crate::R<CCNR1rs>;
///Register `CCNR1` writer
pub type W = crate::W<CCNR1rs>;
///Field `SCNONCE` reader - Stream cipher nonce, bits \[63:32\] Refer to the MCE_CCzNR0 register for description of the SCNONCE\[63:0\] bitfield.
pub type SCNONCE_R = crate::FieldReader<u32>;
///Field `SCNONCE` writer - Stream cipher nonce, bits \[63:32\] Refer to the MCE_CCzNR0 register for description of the SCNONCE\[63:0\] bitfield.
pub type SCNONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stream cipher nonce, bits \[63:32\] Refer to the MCE_CCzNR0 register for description of the SCNONCE\[63:0\] bitfield.
    #[inline(always)]
    pub fn scnonce(&self) -> SCNONCE_R {
        SCNONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCNR1")
            .field("scnonce", &self.scnonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stream cipher nonce, bits \[63:32\] Refer to the MCE_CCzNR0 register for description of the SCNONCE\[63:0\] bitfield.
    #[inline(always)]
    pub fn scnonce(&mut self) -> SCNONCE_W<'_, CCNR1rs> {
        SCNONCE_W::new(self, 0)
    }
}
/**MCE cipher context 1 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`ccnr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCNR1rs;
impl crate::RegisterSpec for CCNR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccnr1::R`](R) reader structure
impl crate::Readable for CCNR1rs {}
///`write(|w| ..)` method takes [`ccnr1::W`](W) writer structure
impl crate::Writable for CCNR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCNR1 to value 0
impl crate::Resettable for CCNR1rs {}
