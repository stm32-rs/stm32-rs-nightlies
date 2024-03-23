#[doc = "Register `XIDAM` reader"]
pub type R = crate::R<XIDAMrs>;
#[doc = "Register `XIDAM` writer"]
pub type W = crate::W<XIDAMrs>;
#[doc = "Field `EIDM` reader - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type EIDM_R = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type EIDM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EIDM_W<XIDAMrs> {
        EIDM_W::new(self, 0)
    }
}
#[doc = "FDCAN extended ID and mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDAMrs;
impl crate::RegisterSpec for XIDAMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidam::R`](R) reader structure"]
impl crate::Readable for XIDAMrs {}
#[doc = "`write(|w| ..)` method takes [`xidam::W`](W) writer structure"]
impl crate::Writable for XIDAMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for XIDAMrs {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
