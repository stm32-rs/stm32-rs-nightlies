#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TSCCrs>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TSCCrs>;
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TSS_R = crate::FieldReader;
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<TSCCrs> {
        TSS_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<TSCCrs> {
        TCP_W::new(self, 16)
    }
}
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCCrs;
impl crate::RegisterSpec for TSCCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TSCCrs {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TSCCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCCrs {
    const RESET_VALUE: u32 = 0;
}
