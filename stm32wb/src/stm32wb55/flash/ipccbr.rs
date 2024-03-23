#[doc = "Register `IPCCBR` reader"]
pub type R = crate::R<IPCCBRrs>;
#[doc = "Register `IPCCBR` writer"]
pub type W = crate::W<IPCCBRrs>;
#[doc = "Field `IPCCDBA` reader - PCC mailbox data buffer base address"]
pub type IPCCDBA_R = crate::FieldReader<u16>;
#[doc = "Field `IPCCDBA` writer - PCC mailbox data buffer base address"]
pub type IPCCDBA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - PCC mailbox data buffer base address"]
    #[inline(always)]
    pub fn ipccdba(&self) -> IPCCDBA_R {
        IPCCDBA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PCC mailbox data buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn ipccdba(&mut self) -> IPCCDBA_W<IPCCBRrs> {
        IPCCDBA_W::new(self, 0)
    }
}
#[doc = "IPCC mailbox data buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipccbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipccbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCCBRrs;
impl crate::RegisterSpec for IPCCBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipccbr::R`](R) reader structure"]
impl crate::Readable for IPCCBRrs {}
#[doc = "`write(|w| ..)` method takes [`ipccbr::W`](W) writer structure"]
impl crate::Writable for IPCCBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCCBR to value 0xffff_c000"]
impl crate::Resettable for IPCCBRrs {
    const RESET_VALUE: u32 = 0xffff_c000;
}
