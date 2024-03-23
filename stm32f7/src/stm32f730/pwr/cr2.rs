#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `CWUPF1` reader - Clear Wakeup Pin flag for PA0"]
pub type CWUPF1_R = crate::BitReader;
#[doc = "Field `CWUPF2` reader - Clear Wakeup Pin flag for PA2"]
pub type CWUPF2_R = crate::BitReader;
#[doc = "Field `CWUPF3` reader - Clear Wakeup Pin flag for PC1"]
pub type CWUPF3_R = crate::BitReader;
#[doc = "Field `CWUPF4` reader - Clear Wakeup Pin flag for PC13"]
pub type CWUPF4_R = crate::BitReader;
#[doc = "Field `CWUPF5` reader - Clear Wakeup Pin flag for PI8"]
pub type CWUPF5_R = crate::BitReader;
#[doc = "Field `CWUPF6` reader - Clear Wakeup Pin flag for PI11"]
pub type CWUPF6_R = crate::BitReader;
#[doc = "Field `WUPP1` reader - Wakeup pin polarity bit for PA0"]
pub type WUPP1_R = crate::BitReader;
#[doc = "Field `WUPP1` writer - Wakeup pin polarity bit for PA0"]
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPP2` reader - Wakeup pin polarity bit for PA2"]
pub type WUPP2_R = crate::BitReader;
#[doc = "Field `WUPP2` writer - Wakeup pin polarity bit for PA2"]
pub type WUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPP3` reader - Wakeup pin polarity bit for PC1"]
pub type WUPP3_R = crate::BitReader;
#[doc = "Field `WUPP3` writer - Wakeup pin polarity bit for PC1"]
pub type WUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPP4` reader - Wakeup pin polarity bit for PC13"]
pub type WUPP4_R = crate::BitReader;
#[doc = "Field `WUPP4` writer - Wakeup pin polarity bit for PC13"]
pub type WUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPP5` reader - Wakeup pin polarity bit for PI8"]
pub type WUPP5_R = crate::BitReader;
#[doc = "Field `WUPP5` writer - Wakeup pin polarity bit for PI8"]
pub type WUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPP6` reader - Wakeup pin polarity bit for PI11"]
pub type WUPP6_R = crate::BitReader;
#[doc = "Field `WUPP6` writer - Wakeup pin polarity bit for PI11"]
pub type WUPP6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn cwupf1(&self) -> CWUPF1_R {
        CWUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn cwupf2(&self) -> CWUPF2_R {
        CWUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn cwupf3(&self) -> CWUPF3_R {
        CWUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn cwupf4(&self) -> CWUPF4_R {
        CWUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn cwupf5(&self) -> CWUPF5_R {
        CWUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn cwupf6(&self) -> CWUPF6_R {
        CWUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<CR2rs> {
        WUPP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<CR2rs> {
        WUPP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<CR2rs> {
        WUPP3_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<CR2rs> {
        WUPP4_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<CR2rs> {
        WUPP5_W::new(self, 12)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    #[must_use]
    pub fn wupp6(&mut self) -> WUPP6_W<CR2rs> {
        WUPP6_W::new(self, 13)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
