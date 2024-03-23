#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TXBCrs>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TXBCrs>;
#[doc = "Field `TFQM` reader - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TFQM_R = crate::BitReader;
#[doc = "Field `TFQM` writer - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<TXBCrs> {
        TFQM_W::new(self, 24)
    }
}
#[doc = "FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCrs;
impl crate::RegisterSpec for TXBCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TXBCrs {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TXBCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TXBCrs {
    const RESET_VALUE: u32 = 0;
}
