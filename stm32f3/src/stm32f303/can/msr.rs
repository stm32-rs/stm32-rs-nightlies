#[doc = "Register `MSR` reader"]
pub type R = crate::R<MSRrs>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MSRrs>;
#[doc = "Field `INAK` reader - INAK"]
pub type INAK_R = crate::BitReader;
#[doc = "Field `SLAK` reader - SLAK"]
pub type SLAK_R = crate::BitReader;
#[doc = "Field `ERRI` reader - ERRI"]
pub type ERRI_R = crate::BitReader;
#[doc = "Field `ERRI` writer - ERRI"]
pub type ERRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUI` reader - WKUI"]
pub type WKUI_R = crate::BitReader;
#[doc = "Field `WKUI` writer - WKUI"]
pub type WKUI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAKI` reader - SLAKI"]
pub type SLAKI_R = crate::BitReader;
#[doc = "Field `SLAKI` writer - SLAKI"]
pub type SLAKI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXM` reader - TXM"]
pub type TXM_R = crate::BitReader;
#[doc = "Field `RXM` reader - RXM"]
pub type RXM_R = crate::BitReader;
#[doc = "Field `SAMP` reader - SAMP"]
pub type SAMP_R = crate::BitReader;
#[doc = "Field `RX` reader - RX"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INAK"]
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLAK"]
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TXM"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXM"]
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ERRI_W<MSRrs> {
        ERRI_W::new(self, 2)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    #[must_use]
    pub fn wkui(&mut self) -> WKUI_W<MSRrs> {
        WKUI_W::new(self, 3)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    #[must_use]
    pub fn slaki(&mut self) -> SLAKI_W<MSRrs> {
        SLAKI_W::new(self, 4)
    }
}
#[doc = "master status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSRrs;
impl crate::RegisterSpec for MSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MSRrs {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSR to value 0x0c02"]
impl crate::Resettable for MSRrs {
    const RESET_VALUE: u32 = 0x0c02;
}
