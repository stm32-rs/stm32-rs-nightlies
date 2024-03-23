#[doc = "Register `FDCAN_TSCC` reader"]
pub type R = crate::R<FDCAN_TSCCrs>;
#[doc = "Register `FDCAN_TSCC` writer"]
pub type W = crate::W<FDCAN_TSCCrs>;
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader;
#[doc = "Field `TSS` writer - TSS"]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - TCP"]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - TCP"]
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<FDCAN_TSCCrs> {
        TSS_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<FDCAN_TSCCrs> {
        TCP_W::new(self, 16)
    }
}
#[doc = "FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TSCCrs;
impl crate::RegisterSpec for FDCAN_TSCCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TSCCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TSCCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TSCC to value 0"]
impl crate::Resettable for FDCAN_TSCCrs {
    const RESET_VALUE: u32 = 0;
}
