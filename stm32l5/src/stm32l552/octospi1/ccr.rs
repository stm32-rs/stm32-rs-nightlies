#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `DCYC` reader - Number of dummy cycles"]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles"]
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader;
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSHIFT` reader - Sample shift"]
pub type SSHIFT_R = crate::BitReader;
#[doc = "Field `SSHIFT` writer - Sample shift"]
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<CCRrs> {
        DCYC_W::new(self, 0)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<CCRrs> {
        DHQC_W::new(self, 28)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<CCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
#[doc = "communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
