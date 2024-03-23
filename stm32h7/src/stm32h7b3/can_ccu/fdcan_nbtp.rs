#[doc = "Register `FDCAN_NBTP` reader"]
pub type R = crate::R<FDCAN_NBTPrs>;
#[doc = "Register `FDCAN_NBTP` writer"]
pub type W = crate::W<FDCAN_NBTPrs>;
#[doc = "Field `TSEG2` reader - Nominal Time segment after sample point"]
pub type TSEG2_R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Nominal Time segment after sample point"]
pub type TSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type NTSEG1_R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - Bit Rate Prescaler"]
pub type NBRP_R = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Bit Rate Prescaler"]
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_R = crate::FieldReader;
#[doc = "Field `NSJW` writer - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<FDCAN_NBTPrs> {
        TSEG2_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<FDCAN_NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<FDCAN_NBTPrs> {
        NBRP_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<FDCAN_NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_NBTPrs;
impl crate::RegisterSpec for FDCAN_NBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_nbtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_NBTPrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_nbtp::W`](W) writer structure"]
impl crate::Writable for FDCAN_NBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_NBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
