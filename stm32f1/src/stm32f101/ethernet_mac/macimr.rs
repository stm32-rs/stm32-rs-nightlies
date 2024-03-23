#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MACIMRrs>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MACIMRrs>;
#[doc = "Field `PMTIM` reader - PMT interrupt mask"]
pub type PMTIM_R = crate::BitReader;
#[doc = "Field `PMTIM` writer - PMT interrupt mask"]
pub type PMTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTIM` reader - Time stamp trigger interrupt mask"]
pub type TSTIM_R = crate::BitReader;
#[doc = "Field `TSTIM` writer - Time stamp trigger interrupt mask"]
pub type TSTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmtim(&mut self) -> PMTIM_W<MACIMRrs> {
        PMTIM_W::new(self, 3)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tstim(&mut self) -> TSTIM_W<MACIMRrs> {
        TSTIM_W::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIMRrs;
impl crate::RegisterSpec for MACIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MACIMRrs {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MACIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMRrs {
    const RESET_VALUE: u32 = 0;
}
