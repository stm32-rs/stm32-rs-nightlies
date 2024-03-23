#[doc = "Register `FDCAN_CKDIV` reader"]
pub type R = crate::R<FDCAN_CKDIVrs>;
#[doc = "Register `FDCAN_CKDIV` writer"]
pub type W = crate::W<FDCAN_CKDIVrs>;
#[doc = "Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_R = crate::FieldReader;
#[doc = "Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<FDCAN_CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
#[doc = "FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ckdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_CKDIVrs;
impl crate::RegisterSpec for FDCAN_CKDIVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ckdiv::R`](R) reader structure"]
impl crate::Readable for FDCAN_CKDIVrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ckdiv::W`](W) writer structure"]
impl crate::Writable for FDCAN_CKDIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_CKDIV to value 0"]
impl crate::Resettable for FDCAN_CKDIVrs {
    const RESET_VALUE: u32 = 0;
}
