#[doc = "Register `MMCTIR` reader"]
pub type R = crate::R<MMCTIRrs>;
#[doc = "Register `MMCTIR` writer"]
pub type W = crate::W<MMCTIRrs>;
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub type TGFSCS_R = crate::BitReader;
#[doc = "Field `TGFSCS` writer - Transmitted good frames single collision status"]
pub type TGFSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more single collision status"]
pub type TGFMSCS_R = crate::BitReader;
#[doc = "Field `TGFMSCS` writer - Transmitted good frames more single collision status"]
pub type TGFMSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub type TGFS_R = crate::BitReader;
#[doc = "Field `TGFS` writer - Transmitted good frames status"]
pub type TGFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfscs(&mut self) -> TGFSCS_W<MMCTIRrs> {
        TGFSCS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W<MMCTIRrs> {
        TGFMSCS_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfs(&mut self) -> TGFS_W<MMCTIRrs> {
        TGFS_W::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIRrs;
impl crate::RegisterSpec for MMCTIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctir::R`](R) reader structure"]
impl crate::Readable for MMCTIRrs {}
#[doc = "`write(|w| ..)` method takes [`mmctir::W`](W) writer structure"]
impl crate::Writable for MMCTIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIRrs {
    const RESET_VALUE: u32 = 0;
}
