#[doc = "Register `PTPTSCR` reader"]
pub type R = crate::R<PTPTSCRrs>;
#[doc = "Register `PTPTSCR` writer"]
pub type W = crate::W<PTPTSCRrs>;
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSFCU` reader - Time stamp fine or coarse update"]
pub type TSFCU_R = crate::BitReader;
#[doc = "Field `TSFCU` writer - Time stamp fine or coarse update"]
pub type TSFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTI` reader - Time stamp system time initialize"]
pub type TSSTI_R = crate::BitReader;
#[doc = "Field `TSSTI` writer - Time stamp system time initialize"]
pub type TSSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTU` reader - Time stamp system time update"]
pub type TSSTU_R = crate::BitReader;
#[doc = "Field `TSSTU` writer - Time stamp system time update"]
pub type TSSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSITE` reader - Time stamp interrupt trigger enable"]
pub type TSITE_R = crate::BitReader;
#[doc = "Field `TSITE` writer - Time stamp interrupt trigger enable"]
pub type TSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSARU` reader - Time stamp addend register update"]
pub type TSARU_R = crate::BitReader;
#[doc = "Field `TSARU` writer - Time stamp addend register update"]
pub type TSARU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tsaru(&self) -> TSARU_R {
        TSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<PTPTSCRrs> {
        TSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    #[must_use]
    pub fn tsfcu(&mut self) -> TSFCU_W<PTPTSCRrs> {
        TSFCU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tssti(&mut self) -> TSSTI_W<PTPTSCRrs> {
        TSSTI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    #[must_use]
    pub fn tsstu(&mut self) -> TSSTU_W<PTPTSCRrs> {
        TSSTU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsite(&mut self) -> TSITE_W<PTPTSCRrs> {
        TSITE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    #[must_use]
    pub fn tsaru(&mut self) -> TSARU_W<PTPTSCRrs> {
        TSARU_W::new(self, 5)
    }
}
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSCRrs;
impl crate::RegisterSpec for PTPTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptscr::R`](R) reader structure"]
impl crate::Readable for PTPTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`ptptscr::W`](W) writer structure"]
impl crate::Writable for PTPTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSCR to value 0"]
impl crate::Resettable for PTPTSCRrs {
    const RESET_VALUE: u32 = 0;
}
