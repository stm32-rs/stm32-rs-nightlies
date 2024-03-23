#[doc = "Register `TX_PAYSZ` reader"]
pub type R = crate::R<TX_PAYSZrs>;
#[doc = "Register `TX_PAYSZ` writer"]
pub type W = crate::W<TX_PAYSZrs>;
#[doc = "Field `TXPAYSZ` reader - TXPAYSZ"]
pub type TXPAYSZ_R = crate::FieldReader<u16>;
#[doc = "Field `TXPAYSZ` writer - TXPAYSZ"]
pub type TXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    #[must_use]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W<TX_PAYSZrs> {
        TXPAYSZ_W::new(self, 0)
    }
}
#[doc = "UCPD Tx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_paysz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_paysz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PAYSZrs;
impl crate::RegisterSpec for TX_PAYSZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_paysz::R`](R) reader structure"]
impl crate::Readable for TX_PAYSZrs {}
#[doc = "`write(|w| ..)` method takes [`tx_paysz::W`](W) writer structure"]
impl crate::Writable for TX_PAYSZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_PAYSZ to value 0"]
impl crate::Resettable for TX_PAYSZrs {
    const RESET_VALUE: u32 = 0;
}
