#[doc = "Register `MTLOMR` reader"]
pub type R = crate::R<MTLOMRrs>;
#[doc = "Register `MTLOMR` writer"]
pub type W = crate::W<MTLOMRrs>;
#[doc = "Field `DTXSTS` reader - Drop Transmit Status"]
pub type DTXSTS_R = crate::BitReader;
#[doc = "Field `DTXSTS` writer - Drop Transmit Status"]
pub type DTXSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub type CNTPRST_R = crate::BitReader;
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub type CNTPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCLR` reader - Counters Reset"]
pub type CNTCLR_R = crate::BitReader;
#[doc = "Field `CNTCLR` writer - Counters Reset"]
pub type CNTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<MTLOMRrs> {
        DTXSTS_W::new(self, 1)
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<MTLOMRrs> {
        CNTPRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<MTLOMRrs> {
        CNTCLR_W::new(self, 9)
    }
}
#[doc = "Operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLOMRrs;
impl crate::RegisterSpec for MTLOMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlomr::R`](R) reader structure"]
impl crate::Readable for MTLOMRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlomr::W`](W) writer structure"]
impl crate::Writable for MTLOMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLOMR to value 0"]
impl crate::Resettable for MTLOMRrs {
    const RESET_VALUE: u32 = 0;
}
