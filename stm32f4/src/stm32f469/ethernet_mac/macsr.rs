#[doc = "Register `MACSR` reader"]
pub type R = crate::R<MACSRrs>;
#[doc = "Register `MACSR` writer"]
pub type W = crate::W<MACSRrs>;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PMTS_R = crate::BitReader;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MMCS_R = crate::BitReader;
#[doc = "Field `MMCRS` reader - MMC receive status"]
pub type MMCRS_R = crate::BitReader;
#[doc = "Field `MMCTS` reader - MMC transmit status"]
pub type MMCTS_R = crate::BitReader;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TSTS_R = crate::BitReader;
#[doc = "Field `TSTS` writer - Time stamp trigger status"]
pub type TSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive status"]
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit status"]
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tsts(&mut self) -> TSTS_W<MACSRrs> {
        TSTS_W::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSRrs;
impl crate::RegisterSpec for MACSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macsr::R`](R) reader structure"]
impl crate::Readable for MACSRrs {}
#[doc = "`write(|w| ..)` method takes [`macsr::W`](W) writer structure"]
impl crate::Writable for MACSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSR to value 0"]
impl crate::Resettable for MACSRrs {
    const RESET_VALUE: u32 = 0;
}
