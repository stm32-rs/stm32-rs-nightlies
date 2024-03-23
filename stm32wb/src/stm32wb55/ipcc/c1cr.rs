#[doc = "Register `C1CR` reader"]
pub type R = crate::R<C1CRrs>;
#[doc = "Register `C1CR` writer"]
pub type W = crate::W<C1CRrs>;
#[doc = "Field `RXOIE` reader - processor 1 Receive channel occupied interrupt enable"]
pub type RXOIE_R = crate::BitReader;
#[doc = "Field `RXOIE` writer - processor 1 Receive channel occupied interrupt enable"]
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIE` reader - processor 1 Transmit channel free interrupt enable"]
pub type TXFIE_R = crate::BitReader;
#[doc = "Field `TXFIE` writer - processor 1 Transmit channel free interrupt enable"]
pub type TXFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - processor 1 Receive channel occupied interrupt enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel free interrupt enable"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - processor 1 Receive channel occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<C1CRrs> {
        RXOIE_W::new(self, 0)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel free interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<C1CRrs> {
        TXFIE_W::new(self, 16)
    }
}
#[doc = "Control register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1CRrs;
impl crate::RegisterSpec for C1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1cr::R`](R) reader structure"]
impl crate::Readable for C1CRrs {}
#[doc = "`write(|w| ..)` method takes [`c1cr::W`](W) writer structure"]
impl crate::Writable for C1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1CR to value 0"]
impl crate::Resettable for C1CRrs {
    const RESET_VALUE: u32 = 0;
}
