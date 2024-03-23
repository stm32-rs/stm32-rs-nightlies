#[doc = "Register `PWR_MCUCR` reader"]
pub type R = crate::R<PWR_MCUCRrs>;
#[doc = "Register `PWR_MCUCR` writer"]
pub type W = crate::W<PWR_MCUCRrs>;
#[doc = "Field `PDDS` reader - PDDS"]
pub type PDDS_R = crate::BitReader;
#[doc = "Field `PDDS` writer - PDDS"]
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - STOPF"]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `SBF` reader - SBF"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `CSSF` reader - CSSF"]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `CSSF` writer - CSSF"]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP` reader - DEEPSLEEP"]
pub type DEEPSLEEP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - DEEPSLEEP"]
    #[inline(always)]
    pub fn deepsleep(&self) -> DEEPSLEEP_R {
        DEEPSLEEP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<PWR_MCUCRrs> {
        PDDS_W::new(self, 0)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<PWR_MCUCRrs> {
        CSSF_W::new(self, 9)
    }
}
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_MCUCRrs;
impl crate::RegisterSpec for PWR_MCUCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_mcucr::R`](R) reader structure"]
impl crate::Readable for PWR_MCUCRrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_mcucr::W`](W) writer structure"]
impl crate::Writable for PWR_MCUCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_MCUCR to value 0"]
impl crate::Resettable for PWR_MCUCRrs {
    const RESET_VALUE: u32 = 0;
}
