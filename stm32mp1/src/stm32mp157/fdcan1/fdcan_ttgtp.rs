#[doc = "Register `FDCAN_TTGTP` reader"]
pub type R = crate::R<FDCAN_TTGTPrs>;
#[doc = "Register `FDCAN_TTGTP` writer"]
pub type W = crate::W<FDCAN_TTGTPrs>;
#[doc = "Field `TP` reader - TP"]
pub type TP_R = crate::FieldReader<u16>;
#[doc = "Field `TP` writer - TP"]
pub type TP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CTP` reader - CTP"]
pub type CTP_R = crate::FieldReader<u16>;
#[doc = "Field `CTP` writer - CTP"]
pub type CTP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TP"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CTP"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TP"]
    #[inline(always)]
    #[must_use]
    pub fn tp(&mut self) -> TP_W<FDCAN_TTGTPrs> {
        TP_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CTP"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<FDCAN_TTGTPrs> {
        CTP_W::new(self, 16)
    }
}
#[doc = "If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttgtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttgtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTGTPrs;
impl crate::RegisterSpec for FDCAN_TTGTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttgtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTGTPrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttgtp::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTGTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTGTP to value 0"]
impl crate::Resettable for FDCAN_TTGTPrs {
    const RESET_VALUE: u32 = 0;
}
