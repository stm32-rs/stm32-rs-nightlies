#[doc = "Register `ETH_DMAC0SFCSR` reader"]
pub type R = crate::R<ETH_DMAC0SFCSRrs>;
#[doc = "Register `ETH_DMAC0SFCSR` writer"]
pub type W = crate::W<ETH_DMAC0SFCSRrs>;
#[doc = "Field `ESC` reader - ESC"]
pub type ESC_R = crate::BitReader;
#[doc = "Field `ESC` writer - ESC"]
pub type ESC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC` reader - ASC"]
pub type ASC_R = crate::BitReader;
#[doc = "Field `ASC` writer - ASC"]
pub type ASC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSN` reader - RSN"]
pub type RSN_R = crate::FieldReader;
#[doc = "Field `RSN` writer - RSN"]
pub type RSN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    #[must_use]
    pub fn esc(&mut self) -> ESC_W<ETH_DMAC0SFCSRrs> {
        ESC_W::new(self, 0)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    #[must_use]
    pub fn asc(&mut self) -> ASC_W<ETH_DMAC0SFCSRrs> {
        ASC_W::new(self, 1)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    #[must_use]
    pub fn rsn(&mut self) -> RSN_W<ETH_DMAC0SFCSRrs> {
        RSN_W::new(self, 16)
    }
}
#[doc = "Channel i slot function control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0sfcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac0sfcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0SFCSRrs;
impl crate::RegisterSpec for ETH_DMAC0SFCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0sfcsr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0SFCSRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac0sfcsr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC0SFCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0SFCSR to value 0"]
impl crate::Resettable for ETH_DMAC0SFCSRrs {
    const RESET_VALUE: u32 = 0;
}
