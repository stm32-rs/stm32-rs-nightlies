#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `CB0OF` writer - Clear buffer 0 overflow flag"]
pub type CB0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CB1OF` writer - Clear buffer 1 overflow flag"]
pub type CB1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CB2OF` writer - Clear buffer 2 overflow flag"]
pub type CB2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CB3OF` writer - Clear buffer 3 overflow flag"]
pub type CB3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMEF` writer - Clear AHB master error flag"]
pub type CAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear buffer 0 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb0of(&mut self) -> CB0OF_W<FCRrs> {
        CB0OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear buffer 1 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb1of(&mut self) -> CB1OF_W<FCRrs> {
        CB1OF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear buffer 2 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb2of(&mut self) -> CB2OF_W<FCRrs> {
        CB2OF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear buffer 3 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb3of(&mut self) -> CB3OF_W<FCRrs> {
        CB3OF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear AHB master error flag"]
    #[inline(always)]
    #[must_use]
    pub fn camef(&mut self) -> CAMEF_W<FCRrs> {
        CAMEF_W::new(self, 4)
    }
}
#[doc = "Graphic MMU flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
