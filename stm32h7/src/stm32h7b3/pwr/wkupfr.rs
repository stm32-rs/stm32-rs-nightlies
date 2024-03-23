#[doc = "Register `WKUPFR` reader"]
pub type R = crate::R<WKUPFRrs>;
#[doc = "Register `WKUPFR` writer"]
pub type W = crate::W<WKUPFRrs>;
#[doc = "Field `WKUPF1` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF1_R = crate::BitReader;
#[doc = "Field `WKUPF1` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF2` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF2_R = crate::BitReader;
#[doc = "Field `WKUPF2` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF3` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF3_R = crate::BitReader;
#[doc = "Field `WKUPF3` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF4` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF4_R = crate::BitReader;
#[doc = "Field `WKUPF4` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF5` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF5_R = crate::BitReader;
#[doc = "Field `WKUPF5` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF6` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF6_R = crate::BitReader;
#[doc = "Field `WKUPF6` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf1(&mut self) -> WKUPF1_W<WKUPFRrs> {
        WKUPF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf2(&mut self) -> WKUPF2_W<WKUPFRrs> {
        WKUPF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf3(&mut self) -> WKUPF3_W<WKUPFRrs> {
        WKUPF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf4(&mut self) -> WKUPF4_W<WKUPFRrs> {
        WKUPF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf5(&mut self) -> WKUPF5_W<WKUPFRrs> {
        WKUPF5_W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf6(&mut self) -> WKUPF6_W<WKUPFRrs> {
        WKUPF6_W::new(self, 5)
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKUPFRrs;
impl crate::RegisterSpec for WKUPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupfr::R`](R) reader structure"]
impl crate::Readable for WKUPFRrs {}
#[doc = "`write(|w| ..)` method takes [`wkupfr::W`](W) writer structure"]
impl crate::Writable for WKUPFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKUPFR to value 0"]
impl crate::Resettable for WKUPFRrs {
    const RESET_VALUE: u32 = 0;
}
