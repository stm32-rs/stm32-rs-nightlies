///Register `XIDAM` reader
pub type R = crate::R<XIDAMrs>;
///Register `XIDAM` writer
pub type W = crate::W<XIDAMrs>;
///Field `EIDM` reader - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type EIDM_R = crate::FieldReader<u32>;
///Field `EIDM` writer - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type EIDM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIDAM").field("eidm", &self.eidm()).finish()
    }
}
impl W {
    ///Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W<'_, XIDAMrs> {
        EIDM_W::new(self, 0)
    }
}
/**FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FDCAN1:XIDAM)*/
pub struct XIDAMrs;
impl crate::RegisterSpec for XIDAMrs {
    type Ux = u32;
}
///`read()` method returns [`xidam::R`](R) reader structure
impl crate::Readable for XIDAMrs {}
///`write(|w| ..)` method takes [`xidam::W`](W) writer structure
impl crate::Writable for XIDAMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets XIDAM to value 0x1fff_ffff
impl crate::Resettable for XIDAMrs {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
