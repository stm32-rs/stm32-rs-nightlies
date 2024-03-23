#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `ENS` reader - ENS"]
pub type ENS_R = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame flag"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `UDR` writer - Update display request"]
pub type UDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDD` reader - Update Display Done"]
pub type UDD_R = crate::BitReader;
#[doc = "Field `RDY` reader - Ready flag"]
pub type RDY_R = crate::BitReader;
#[doc = "Field `FCRSF` reader - LCD Frame Control Register Synchronization flag"]
pub type FCRSF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ENS"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Update Display Done"]
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ready flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Frame Control Register Synchronization flag"]
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Update display request"]
    #[inline(always)]
    #[must_use]
    pub fn udr(&mut self) -> UDR_W<SRrs> {
        UDR_W::new(self, 2)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x20"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x20;
}
