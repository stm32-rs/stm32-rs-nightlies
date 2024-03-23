#[doc = "Register `C2APB3ENR` reader"]
pub type R = crate::R<C2APB3ENRrs>;
#[doc = "Register `C2APB3ENR` writer"]
pub type W = crate::W<C2APB3ENRrs>;
#[doc = "Field `BLEEN` reader - CPU2 BLE interface clock enable"]
pub type BLEEN_R = crate::BitReader;
#[doc = "Field `BLEEN` writer - CPU2 BLE interface clock enable"]
pub type BLEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN802` reader - CPU2 802.15.4 interface clock enable"]
pub type EN802_R = crate::BitReader;
#[doc = "Field `EN802` writer - CPU2 802.15.4 interface clock enable"]
pub type EN802_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&self) -> BLEEN_R {
        BLEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&self) -> EN802_R {
        EN802_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bleen(&mut self) -> BLEEN_W<C2APB3ENRrs> {
        BLEEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn en802(&mut self) -> EN802_W<C2APB3ENRrs> {
        EN802_W::new(self, 1)
    }
}
#[doc = "CPU2 APB3ENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB3ENRrs;
impl crate::RegisterSpec for C2APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb3enr::R`](R) reader structure"]
impl crate::Readable for C2APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2apb3enr::W`](W) writer structure"]
impl crate::Writable for C2APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB3ENR to value 0"]
impl crate::Resettable for C2APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
